use anyhow::{anyhow, Result};
use evdev::{Device, Key};
use log::info;
use std::collections::HashMap;
use std::fs;

/// Find and return ALL event devices belonging to the selected physical keyboard
pub fn find_keyboard_devices() -> Result<Vec<Device>> {
    let mut candidate_devices: Vec<(std::path::PathBuf, Device, String)> = Vec::new();

    for entry in fs::read_dir("/dev/input")? {
        let entry = entry?;
        let path = entry.path();
        let path_str = path.to_string_lossy();

        if path_str.contains("event") {
            if let Ok(device) = Device::open(&path) {
                let name = device.name().unwrap_or("Unknown").to_string();

                // Exclude vknetd's own virtual keyboard
                if name.contains("vknetd Virtual Keyboard") {
                    continue;
                }

                // Check if device supports key events and contains main keyboard keys
                if let Some(keys) = device.supported_keys() {
                    if is_valid_keyboard(&keys) {
                        info!("Found keyboard candidate: '{}' at {:?}", name, path);
                        candidate_devices.push((path, device, name));
                    }
                }
            }
        }
    }

    if candidate_devices.is_empty() {
        return Err(anyhow!("No physical keyboard device found in /dev/input/event*"));
    }

    // Group candidates by device name
    let mut name_counts: HashMap<String, usize> = HashMap::new();
    for (_, _, name) in &candidate_devices {
        *name_counts.entry(name.clone()).or_insert(0) += 1;
    }

    // Determine target keyboard name (prefer name with "keyboard" or "kbd")
    let mut selected_name = candidate_devices[0].2.clone();
    for (_, _, name) in &candidate_devices {
        let lower = name.to_lowercase();
        if lower.contains("keyboard") || lower.contains("kbd") {
            selected_name = name.clone();
            break;
        }
    }

    info!("🎯 Selected primary keyboard target name: '{}'", selected_name);

    // Collect ALL devices sharing the target keyboard name
    let mut selected_devices = Vec::new();
    for (path, dev, name) in candidate_devices {
        if name == selected_name {
            info!("🔒 Adding device node to grab list: '{}' at {:?}", name, path);
            selected_devices.push(dev);
        }
    }

    if selected_devices.is_empty() {
        return Err(anyhow!("Could not select suitable keyboard devices"));
    }

    Ok(selected_devices)
}

fn is_valid_keyboard(keys: &evdev::AttributeSetRef<Key>) -> bool {
    // Must have standard alphanumeric keys (KEY_A, KEY_Z, KEY_ENTER, KEY_SPACE)
    keys.contains(Key::KEY_A)
        && keys.contains(Key::KEY_Z)
        && keys.contains(Key::KEY_ENTER)
        && keys.contains(Key::KEY_SPACE)
}
