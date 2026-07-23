use evdev::Device;
use log::{info, warn};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub static RUNNING: AtomicBool = AtomicBool::new(true);

pub fn setup_panic_and_signal_hooks(devices: Arc<std::sync::Mutex<Option<Vec<Device>>>>) {
    // 1. Custom Panic Hook
    let dev_clone = Arc::clone(&devices);
    let orig_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        warn!("🚨 PANIC DETECTED! Releasing all evdev grabs immediately...");
        if let Ok(mut lock) = dev_clone.lock() {
            if let Some(devs) = lock.take() {
                for mut dev in devs {
                    let _ = dev.ungrab();
                }
                info!("✅ All evdev grabs successfully released on panic.");
            }
        }
        orig_hook(panic_info);
    }));

    // 2. Signal Listener Thread (SIGINT, SIGTERM)
    let dev_clone_sig = Arc::clone(&devices);
    std::thread::spawn(move || {
        let mut signals = match signal_hook::iterator::Signals::new(&[
            signal_hook::consts::SIGINT,
            signal_hook::consts::SIGTERM,
        ]) {
            Ok(s) => s,
            Err(e) => {
                warn!("Failed to register signal handler: {}", e);
                return;
            }
        };

        for sig in signals.forever() {
            warn!("⚠️ Signal {} received! Shutting down vknetd...", sig);
            RUNNING.store(false, Ordering::SeqCst);
            if let Ok(mut lock) = dev_clone_sig.lock() {
                if let Some(devs) = lock.take() {
                    for mut dev in devs {
                        let _ = dev.ungrab();
                    }
                    info!("✅ All evdev grabs successfully released on signal.");
                }
            }
            break;
        }
    });
}
