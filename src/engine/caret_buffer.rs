#[derive(Debug, Clone, Default)]
pub struct CaretBuffer {
    /// Raw keys typed in the current word
    raw_keys: Vec<char>,
    /// Resulting transformed string currently displayed on screen
    display_str: String,
}

#[allow(dead_code)]
impl CaretBuffer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.raw_keys.is_empty()
    }

    pub fn len(&self) -> usize {
        self.raw_keys.len()
    }

    pub fn display_len(&self) -> usize {
        self.display_str.chars().count()
    }

    pub fn raw_keys(&self) -> &[char] {
        &self.raw_keys
    }

    pub fn display_str(&self) -> &str {
        &self.display_str
    }

    pub fn push_raw(&mut self, c: char) {
        self.raw_keys.push(c);
    }

    pub fn set_display_str(&mut self, s: impl Into<String>) {
        self.display_str = s.into();
    }

    pub fn pop_raw(&mut self) -> Option<char> {
        self.raw_keys.pop()
    }

    pub fn clear(&mut self) {
        self.raw_keys.clear();
        self.display_str.clear();
    }
}
