use anyhow::Result;
use clap::Parser;
use evdev::{EventType, Key};
use log::{error, info, warn};
use std::os::unix::io::AsRawFd;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::{Duration, Instant};

fn set_non_blocking(device: &evdev::Device) {
    let fd = device.as_raw_fd();
    unsafe {
        let flags = libc::fcntl(fd, libc::F_GETFL, 0);
        libc::fcntl(fd, libc::F_SETFL, flags | libc::O_NONBLOCK);
    }
}

mod engine;
mod keyboard;
mod safety;

use engine::{char_to_evdev_key, is_word_boundary_key, EngineAction, ImeEngine, InputMode, VietnameseEngine};
use keyboard::selector::find_keyboard_devices;
use keyboard::uinput_dev::create_virtual_keyboard;
use safety::panic_guard::{setup_panic_and_signal_hooks, RUNNING};

#[derive(Parser, Debug)]
#[command(author, version, about = "vknetd - Vietnamese Kernel Native Input Daemon", long_about = None)]
struct Args {
    /// Auto timeout in seconds to automatically ungrab keyboard and exit (0 = infinite / daemon mode)
    #[arg(short, long, default_value_t = 0)]
    timeout: u64,

    /// Input mode: telex, vni, or off
    #[arg(short, long, default_value = "telex")]
    mode: String,

    /// Enable detailed real-time key event debug logging
    #[arg(short, long, default_value_t = true)]
    debug: bool,
}

fn compute_smart_diff(prev: &str, new: &str) -> (usize, String) {
    let prev_chars: Vec<char> = prev.chars().collect();
    let new_chars: Vec<char> = new.chars().collect();

    let mut common_prefix_len = 0;
    for (c1, c2) in prev_chars.iter().zip(new_chars.iter()) {
        if c1 == c2 {
            common_prefix_len += 1;
        } else {
            break;
        }
    }

    let backspaces_needed = prev_chars.len() - common_prefix_len;
    let new_text_suffix_char_idx = common_prefix_len;

    let byte_idx = new
        .char_indices()
        .nth(new_text_suffix_char_idx)
        .map(|(idx, _)| idx)
        .unwrap_or(new.len());

    let text_to_insert = new[byte_idx..].to_string();

    (backspaces_needed, text_to_insert)
}

fn emit_atomic_text_sequence(
    virt_device: &mut evdev::uinput::VirtualDevice,
    backspace_count: usize,
    text: &str,
) -> Result<()> {
    let syn = evdev::InputEvent::new(EventType::SYNCHRONIZATION, 0, 0);

    // 1. Emit Backspaces first with explicit SYN_REPORT and 2ms DOM flush delay
    if backspace_count > 0 {
        let mut bs_batch: Vec<evdev::InputEvent> = Vec::new();
        for _ in 0..backspace_count {
            bs_batch.push(evdev::InputEvent::new(EventType::KEY, Key::KEY_BACKSPACE.code(), 1));
            bs_batch.push(evdev::InputEvent::new(EventType::KEY, Key::KEY_BACKSPACE.code(), 0));
        }
        bs_batch.push(syn);
        virt_device.emit(&bs_batch)?;
        std::thread::sleep(Duration::from_millis(2));
    }

    if text.is_empty() {
        return Ok(());
    }

    // 2. Emit replacement text
    let mut text_batch: Vec<evdev::InputEvent> = Vec::new();
    for c in text.chars() {
        if let Some((key, shift)) = char_to_evdev_key(c) {
            if shift {
                text_batch.push(evdev::InputEvent::new(EventType::KEY, Key::KEY_LEFTSHIFT.code(), 1));
            }
            text_batch.push(evdev::InputEvent::new(EventType::KEY, key.code(), 1));
            text_batch.push(evdev::InputEvent::new(EventType::KEY, key.code(), 0));
            if shift {
                text_batch.push(evdev::InputEvent::new(EventType::KEY, Key::KEY_LEFTSHIFT.code(), 0));
            }
        } else {
            // ISO 14755 Unicode via Ctrl+Shift+U + hex + ENTER
            let hex = format!("{:x}", c as u32);
            text_batch.push(evdev::InputEvent::new(EventType::KEY, Key::KEY_LEFTCTRL.code(), 1));
            text_batch.push(evdev::InputEvent::new(EventType::KEY, Key::KEY_LEFTSHIFT.code(), 1));
            text_batch.push(evdev::InputEvent::new(EventType::KEY, Key::KEY_U.code(), 1));
            text_batch.push(evdev::InputEvent::new(EventType::KEY, Key::KEY_U.code(), 0));
            text_batch.push(evdev::InputEvent::new(EventType::KEY, Key::KEY_LEFTSHIFT.code(), 0));
            text_batch.push(evdev::InputEvent::new(EventType::KEY, Key::KEY_LEFTCTRL.code(), 0));

            for hex_ch in hex.chars() {
                if let Some((key, _)) = char_to_evdev_key(hex_ch) {
                    text_batch.push(evdev::InputEvent::new(EventType::KEY, key.code(), 1));
                    text_batch.push(evdev::InputEvent::new(EventType::KEY, key.code(), 0));
                }
            }

            // Confirm ISO 14755 with Enter key to avoid Space character pollution in Web editors
            text_batch.push(evdev::InputEvent::new(EventType::KEY, Key::KEY_ENTER.code(), 1));
            text_batch.push(evdev::InputEvent::new(EventType::KEY, Key::KEY_ENTER.code(), 0));
        }
    }

    text_batch.push(syn);
    virt_device.emit(&text_batch)?;

    Ok(())
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let args = Args::parse();
    info!("🚀 Starting vknetd Kernel Native Input Daemon...");
    info!("🛡️ Safety Auto-Timeout set to: {} seconds", args.timeout);

    let input_mode = match args.mode.to_lowercase().as_str() {
        "vni" => InputMode::VNI,
        "off" | "english" => InputMode::Off,
        _ => InputMode::Telex,
    };
    let mut engine = VietnameseEngine::new(input_mode);
    info!("🔤 Engine Layer initialized with Mode: {:?}", input_mode);

    // 1. Find all physical keyboard devices matching target name
    let mut phy_devices = find_keyboard_devices()?;

    // 2. Create uinput virtual keyboard
    let mut virt_device = create_virtual_keyboard()?;

    // 3. Grab ALL physical keyboard devices non-blockingly
    info!("🔒 Grabbing ALL physical keyboard event nodes exclusively (EVIOCGRAB)...");
    for dev in &mut phy_devices {
        set_non_blocking(dev);
        dev.grab()?;
    }

    let devices_arc = Arc::new(std::sync::Mutex::new(Some(phy_devices)));

    // 4. Setup panic hook and signal hooks for safety
    setup_panic_and_signal_hooks(Arc::clone(&devices_arc));

    info!("✅ All physical keyboard nodes grabbed successfully! Running Engine event loop...");
    info!("💡 Emergency Hotkey: LShift + RShift + Esc");

    let start_time = Instant::now();
    let timeout_duration = Duration::from_secs(args.timeout);

    // Hotkey tracker for Emergency exit: Left Shift + Right Shift + Escape
    let mut lshift_pressed = false;
    let mut rshift_pressed = false;
    let mut capslock_active = false;
    let mut active_word_display = String::new();

    while RUNNING.load(Ordering::SeqCst) {
        if args.timeout > 0 && start_time.elapsed() >= timeout_duration {
            info!("⏱️ Safety Auto-Timeout reached ({}s). Shutting down safely...", args.timeout);
            break;
        }

        let mut lock = devices_arc.lock().unwrap();
        if let Some(ref mut devs) = *lock {
            for dev in devs.iter_mut() {
                match dev.fetch_events() {
                    Ok(events) => {
                        for ev in events {
                            if ev.event_type() == EventType::KEY {
                                let key = Key::new(ev.code());
                                let value = ev.value(); // 1 = Press, 0 = Release, 2 = Repeat

                                // Track modifier states
                                if key == Key::KEY_LEFTSHIFT {
                                    lshift_pressed = value != 0;
                                } else if key == Key::KEY_RIGHTSHIFT {
                                    rshift_pressed = value != 0;
                                } else if key == Key::KEY_CAPSLOCK && value == 1 {
                                    capslock_active = !capslock_active;
                                } else if key == Key::KEY_ESC && value == 1 {
                                    if lshift_pressed && rshift_pressed {
                                        warn!("🚨 Emergency Hotkey (LShift+RShift+Esc) triggered!");
                                        RUNNING.store(false, Ordering::SeqCst);
                                        break;
                                    }
                                }

                                if is_word_boundary_key(key) {
                                    active_word_display.clear();
                                } else if key == Key::KEY_BACKSPACE && value == 1 {
                                    active_word_display.pop();
                                }

                                let is_shift = lshift_pressed || rshift_pressed;

                                if args.debug && (value == 1 || key == Key::KEY_LEFTSHIFT || key == Key::KEY_RIGHTSHIFT) {
                                    let val_str = match value {
                                        1 => "Press",
                                        0 => "Release",
                                        2 => "Repeat",
                                        _ => "Unknown",
                                    };
                                    info!("📥 Physical Key: {:?} ({}), Shift: {}, Caps: {}", key, val_str, is_shift, capslock_active);
                                }

                                let action = engine.process_key(key, value, is_shift, capslock_active);

                                if args.debug && value == 1 {
                                    match &action {
                                        EngineAction::PassThrough => {
                                            info!("➡️ Action: PassThrough ({:?})", key);
                                        }
                                        EngineAction::Consumed => {
                                            info!("🚫 Action: Consumed");
                                        }
                                        EngineAction::InjectKeySequence { backspace_count, text } => {
                                            info!("✨ Action: InjectSequence (Backspace x{}, Output: '{}')", backspace_count, text);
                                        }
                                    }
                                }

                                match action {
                                    EngineAction::PassThrough => {
                                        let syn = evdev::InputEvent::new(EventType::SYNCHRONIZATION, 0, 0);
                                        if let Err(e) = virt_device.emit(&[ev, syn]) {
                                            error!("Failed to emit uinput event: {}", e);
                                        }
                                    }
                                    EngineAction::Consumed => {
                                        // Key event completely consumed by Engine
                                    }
                                    EngineAction::InjectKeySequence { backspace_count: _, text } => {
                                        let (bs_needed, suffix) = compute_smart_diff(&active_word_display, &text);
                                        emit_atomic_text_sequence(&mut virt_device, bs_needed, &suffix)?;
                                        active_word_display = text;
                                    }
                                }
                            } else {
                                // Forward non-KEY events (e.g. EV_SYN, EV_MSC)
                                let _ = virt_device.emit(&[ev]);
                            }
                        }
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        // Non-blocking wait
                    }
                    Err(e) => {
                        error!("Error reading evdev events: {}", e);
                        break;
                    }
                }
            }
        } else {
            // Device was ungrabbed by panic or signal handler
            break;
        }
        drop(lock);
        std::thread::sleep(Duration::from_millis(2));
    }

    // Explicit Cleanup
    if let Ok(mut lock) = devices_arc.lock() {
        if let Some(devs) = lock.take() {
            for mut dev in devs {
                let _ = dev.ungrab();
            }
            info!("🔓 All physical keyboard devices successfully ungrabbed. System restored to normal.");
        }
    }

    info!("👋 vknetd PoC exited cleanly.");
    std::process::exit(0);
}
