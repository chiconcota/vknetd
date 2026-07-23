use anyhow::Result;
use evdev::uinput::{VirtualDevice, VirtualDeviceBuilder};
use evdev::{AttributeSet, BusType, InputId, Key};
use log::info;

pub fn create_virtual_keyboard() -> Result<VirtualDevice> {
    let mut keys = AttributeSet::<Key>::new();
    
    // Standard QWERTY keyboard keycodes (1 to 111: includes ESC, A-Z, 0-9, F1-F12, Enter, Space, Shift, Ctrl, Alt...)
    for code in 1..=111 {
        keys.insert(Key::new(code));
    }
    
    // Right Alt, Right Ctrl, Left Meta, Right Meta, Menu (124 to 127)
    for code in 124..=127 {
        keys.insert(Key::new(code));
    }

    let device = VirtualDeviceBuilder::new()?
        .name("vknetd Virtual Keyboard")
        .input_id(InputId::new(BusType::BUS_USB, 0x1234, 0x5678, 1))
        .with_keys(&keys)?
        .build()?;

    info!("Created uinput virtual keyboard: 'vknetd Virtual Keyboard' (QWERTY Keyboard)");
    Ok(device)
}
