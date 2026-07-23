use evdev::Key;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Off,
    Telex,
    VNI,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EngineAction {
    /// Pass the original key event directly to uinput without modification
    PassThrough,
    /// Consume key event completely without outputting anything to uinput
    Consumed,
    /// Consume key event, send `backspace_count` Backspaces, then type `text`
    InjectKeySequence {
        backspace_count: usize,
        text: String,
    },
}

#[allow(dead_code)]
pub trait ImeEngine: Send + Sync {
    /// Process a physical key event and return the action to be taken
    fn process_key(&mut self, key: Key, event_value: i32, is_shift: bool, is_capslock: bool) -> EngineAction;

    /// Reset the virtual caret buffer (e.g. on Space, Enter, Escape, Arrow keys, etc.)
    fn reset_buffer(&mut self);

    /// Get the current input mode (Off, Telex, VNI)
    fn get_mode(&self) -> InputMode;

    /// Set the input mode
    fn set_mode(&mut self, mode: InputMode);
}
