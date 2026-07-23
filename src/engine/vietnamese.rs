use evdev::Key;
use crate::engine::caret_buffer::CaretBuffer;
use crate::engine::key_mapper::{is_word_boundary_key, key_to_char};
use crate::engine::traits::{EngineAction, ImeEngine, InputMode};

pub struct VietnameseEngine {
    mode: InputMode,
    buffer: CaretBuffer,
}

impl VietnameseEngine {
    pub fn new(mode: InputMode) -> Self {
        Self {
            mode,
            buffer: CaretBuffer::new(),
        }
    }

    fn transform_telex(&self, raw: &[char]) -> String {
        if raw.is_empty() {
            return String::new();
        }

        let mut tone = 0; // 1: sắc, 2: huyền, 3: hỏi, 4: ngã, 5: nặng
        let mut base_keys: Vec<char> = Vec::new();

        for (idx, &c) in raw.iter().enumerate() {
            let lower = c.to_ascii_lowercase();
            let is_tone_key = matches!(lower, 's' | 'f' | 'r' | 'x' | 'j');
            
            if is_tone_key && !base_keys.is_empty() && has_vowel(&base_keys) {
                let t = match lower {
                    's' => 1,
                    'f' => 2,
                    'r' => 3,
                    'x' => 4,
                    'j' => 5,
                    _ => 0,
                };
                if tone == t {
                    tone = 0;
                    base_keys.push(c);
                } else {
                    tone = t;
                }
            } else if lower == 'w' && idx > 0 && has_vowel(&base_keys) {
                // 'w' used as horn modifier after a vowel is consumed
                // Note: It triggers horn transformation below
            } else {
                base_keys.push(c);
            }
        }

        let has_w = raw.iter().enumerate().any(|(idx, &c)| idx > 0 && (c == 'w' || c == 'W'));

        let mut res: Vec<char> = Vec::new();
        let len = base_keys.len();
        let mut i = 0;

        while i < len {
            let ch = base_keys[i];

            // 1. 'dd' -> 'đ'
            if (ch == 'd' || ch == 'D') && i + 1 < len && (base_keys[i + 1] == 'd' || base_keys[i + 1] == 'D') {
                res.push(if ch == 'D' || base_keys[i + 1] == 'D' { 'Đ' } else { 'đ' });
                i += 2;
                continue;
            }

            // 2. 'aa' -> 'â', 'ee' -> 'ê', 'oo' -> 'ô'
            if (ch == 'a' || ch == 'A') && i + 1 < len && (base_keys[i + 1] == 'a' || base_keys[i + 1] == 'A') {
                res.push(if ch.is_uppercase() || base_keys[i + 1].is_uppercase() { 'Â' } else { 'â' });
                i += 2;
                continue;
            }
            if (ch == 'e' || ch == 'E') && i + 1 < len && (base_keys[i + 1] == 'e' || base_keys[i + 1] == 'E') {
                res.push(if ch.is_uppercase() || base_keys[i + 1].is_uppercase() { 'Ê' } else { 'ê' });
                i += 2;
                continue;
            }
            if (ch == 'o' || ch == 'O') && i + 1 < len && (base_keys[i + 1] == 'o' || base_keys[i + 1] == 'O') {
                res.push(if ch.is_uppercase() || base_keys[i + 1].is_uppercase() { 'Ô' } else { 'ô' });
                i += 2;
                continue;
            }

            // 3. 'aw' -> 'ă', 'ow' -> 'ơ', 'uw' -> 'ư'
            if (ch == 'a' || ch == 'A') && i + 1 < len && (base_keys[i + 1] == 'w' || base_keys[i + 1] == 'W') {
                res.push(if ch.is_uppercase() { 'Ă' } else { 'ă' });
                i += 2;
                continue;
            }
            if (ch == 'o' || ch == 'O') && i + 1 < len && (base_keys[i + 1] == 'w' || base_keys[i + 1] == 'W') {
                res.push(if ch.is_uppercase() { 'Ơ' } else { 'ơ' });
                i += 2;
                continue;
            }
            if (ch == 'u' || ch == 'U') && i + 1 < len && (base_keys[i + 1] == 'w' || base_keys[i + 1] == 'W') {
                if i + 2 < len && (base_keys[i + 2] == 'o' || base_keys[i + 2] == 'O') {
                    res.push(if ch.is_uppercase() { 'Ư' } else { 'ư' });
                    res.push(if base_keys[i + 2].is_uppercase() { 'Ơ' } else { 'ơ' });
                    i += 3;
                    continue;
                }
                res.push(if ch.is_uppercase() { 'Ư' } else { 'ư' });
                i += 2;
                continue;
            }

            res.push(ch);
            i += 1;
        }

        // Smart Caret Scan for 'dd' -> 'đ' anywhere in word (e.g. 'duocd' -> 'đuoc')
        let d_indices: Vec<usize> = res.iter().enumerate().filter_map(|(idx, &c)| if c == 'd' || c == 'D' { Some(idx) } else { None }).collect();
        if d_indices.len() >= 2 {
            let first = d_indices[0];
            res[first] = if res[first].is_uppercase() { 'Đ' } else { 'đ' };
            res.remove(d_indices[1]);
        }

        // Smart Caret Scan for 'oo' -> 'ô' anywhere in word (e.g. 'khong' + 'o' -> 'không')
        let o_indices: Vec<usize> = res.iter().enumerate().filter_map(|(idx, &c)| if c == 'o' || c == 'O' { Some(idx) } else { None }).collect();
        if o_indices.len() >= 2 {
            let first = o_indices[0];
            res[first] = if res[first].is_uppercase() { 'Ô' } else { 'ô' };
            res.remove(o_indices[1]);
        }

        // Smart Caret Scan for 'ee' -> 'ê' anywhere in word (e.g. 'len' + 'e' -> 'lên')
        let e_indices: Vec<usize> = res.iter().enumerate().filter_map(|(idx, &c)| if c == 'e' || c == 'E' { Some(idx) } else { None }).collect();
        if e_indices.len() >= 2 {
            let first = e_indices[0];
            res[first] = if res[first].is_uppercase() { 'Ê' } else { 'ê' };
            res.remove(e_indices[1]);
        }

        // Smart Caret Scan for 'tat' + 'a' -> 'tât'
        let a_indices: Vec<usize> = res.iter().enumerate().filter_map(|(idx, &c)| if c == 'a' || c == 'A' { Some(idx) } else { None }).collect();
        if a_indices.len() >= 2 {
            let first = a_indices[0];
            res[first] = if res[first].is_uppercase() { 'Â' } else { 'â' };
            res.remove(a_indices[1]);
        }

        // Smart Caret Scan for 'w' horn/breve modifier on vowels (a -> ă, o -> ơ, u -> ư, uo -> ươ)
        if has_w {
            let mut a_idx = None;
            let mut u_idx = None;
            let mut o_idx = None;

            for (idx, &c) in res.iter().enumerate() {
                if c == 'a' || c == 'A' { a_idx = Some(idx); }
                if c == 'u' || c == 'U' { u_idx = Some(idx); }
                if c == 'o' || c == 'O' { o_idx = Some(idx); }
            }

            if let (Some(ui), Some(oi)) = (u_idx, o_idx) {
                if ui + 1 == oi {
                    res[ui] = if res[ui].is_uppercase() { 'Ư' } else { 'ư' };
                    res[oi] = if res[oi].is_uppercase() { 'Ơ' } else { 'ơ' };
                }
            } else if let Some(ai) = a_idx {
                res[ai] = if res[ai].is_uppercase() { 'Ă' } else { 'ă' };
            } else if let Some(ui) = u_idx {
                res[ui] = if res[ui].is_uppercase() { 'Ư' } else { 'ư' };
            } else if let Some(oi) = o_idx {
                res[oi] = if res[oi].is_uppercase() { 'Ơ' } else { 'ơ' };
            }
        }

        // Apply tone mark using Smart Tone Mark placement (Decision 008)
        if tone != 0 {
            if let Some(idx) = find_tone_vowel_index(&res) {
                res[idx] = apply_tone_to_char(res[idx], tone);
            }
        }

        res.into_iter().collect()
    }

    fn transform_vni(&self, raw: &[char]) -> String {
        if raw.is_empty() {
            return String::new();
        }

        let mut tone = 0; // 1: sắc, 2: huyền, 3: hỏi, 4: ngã, 5: nặng
        let mut vni_modifiers: Vec<u8> = Vec::new(); // 6: Â/Ê/Ô, 7: Ơ/Ư, 8: Ă, 9: Đ
        let mut base_keys: Vec<char> = Vec::new();

        for &c in raw {
            if matches!(c, '1'..='5') && !base_keys.is_empty() && has_vowel(&base_keys) {
                let t = c.to_digit(10).unwrap() as u8;
                if tone == t {
                    tone = 0;
                    base_keys.push(c);
                } else {
                    tone = t;
                }
            } else if matches!(c, '6'..='9') && !base_keys.is_empty() {
                vni_modifiers.push(c.to_digit(10).unwrap() as u8);
            } else {
                base_keys.push(c);
            }
        }

        let mut res: Vec<char> = Vec::new();
        for &ch in &base_keys {
            res.push(ch);
        }

        // Apply all VNI modifiers (6, 7, 8, 9)
        for &mod_num in &vni_modifiers {
            match mod_num {
                6 => {
                    // Â, Ê, Ô
                    let a_count = res.iter().filter(|&&c| c == 'a' || c == 'A').count();
                    if a_count >= 2 {
                        let a_indices: Vec<usize> = res.iter().enumerate().filter_map(|(idx, &c)| if c == 'a' || c == 'A' { Some(idx) } else { None }).collect();
                        res[a_indices[0]] = if res[a_indices[0]].is_uppercase() { 'Â' } else { 'â' };
                        res.remove(a_indices[1]);
                    } else {
                        for c in res.iter_mut().rev() {
                            if *c == 'a' { *c = 'â'; break; }
                            if *c == 'A' { *c = 'Â'; break; }
                            if *c == 'e' { *c = 'ê'; break; }
                            if *c == 'E' { *c = 'Ê'; break; }
                            if *c == 'o' { *c = 'ô'; break; }
                            if *c == 'O' { *c = 'Ô'; break; }
                        }
                    }
                }
                7 => {
                    // Ơ, Ư
                    let mut u_idx = None;
                    let mut o_idx = None;
                    for (idx, &c) in res.iter().enumerate() {
                        if c == 'u' || c == 'U' { u_idx = Some(idx); }
                        if c == 'o' || c == 'O' { o_idx = Some(idx); }
                    }
                    if let (Some(ui), Some(oi)) = (u_idx, o_idx) {
                        if ui + 1 == oi {
                            res[ui] = if res[ui].is_uppercase() { 'Ư' } else { 'ư' };
                            res[oi] = if res[oi].is_uppercase() { 'Ơ' } else { 'ơ' };
                        }
                    } else if let Some(ui) = u_idx {
                        res[ui] = if res[ui].is_uppercase() { 'Ư' } else { 'ư' };
                    } else if let Some(oi) = o_idx {
                        res[oi] = if res[oi].is_uppercase() { 'Ơ' } else { 'ơ' };
                    }
                }
                8 => {
                    // Ă
                    for c in res.iter_mut().rev() {
                        if *c == 'a' { *c = 'ă'; break; }
                        if *c == 'A' { *c = 'Ă'; break; }
                    }
                }
                9 => {
                    // Đ
                    for c in res.iter_mut().rev() {
                        if *c == 'd' { *c = 'đ'; break; }
                        if *c == 'D' { *c = 'Đ'; break; }
                    }
                }
                _ => {}
            }
        }

        if tone != 0 {
            if let Some(idx) = find_tone_vowel_index(&res) {
                res[idx] = apply_tone_to_char(res[idx], tone);
            }
        }

        res.into_iter().collect()
    }
}

fn find_tone_vowel_index(res: &[char]) -> Option<usize> {
    if res.is_empty() {
        return None;
    }

    // Special rule for initial consonant 'gi' (e.g. 'giáo', 'giá', 'gió', 'giúp')
    let start_offset = if res.len() >= 3
        && (res[0] == 'g' || res[0] == 'G')
        && (res[1] == 'i' || res[1] == 'I')
        && is_vietnamese_vowel(res[2])
    {
        2 // Skip 'i' in 'gi' consonant blend so tone mark goes on 'a'/'o'/'u'
    } else {
        0
    };

    // 1. Priority 1: Main hat/horn vowels (ê, ô, ơ, â, ă) - e.g. 'ơ' in 'ươc' gets tone -> 'được'
    for (idx, &c) in res.iter().enumerate().skip(start_offset) {
        if matches!(c, 'ê' | 'Ê' | 'ô' | 'Ô' | 'ơ' | 'Ơ' | 'â' | 'Â' | 'ă' | 'Ă') {
            return Some(idx);
        }
    }

    // Priority 1b: 'ư' or 'Ư'
    for (idx, &c) in res.iter().enumerate().skip(start_offset) {
        if matches!(c, 'ư' | 'Ư') {
            return Some(idx);
        }
    }

    // Collect all vowel indices starting from start_offset
    let vowel_indices: Vec<usize> = res
        .iter()
        .enumerate()
        .skip(start_offset)
        .filter_map(|(idx, &c)| if is_vietnamese_vowel(c) { Some(idx) } else { None })
        .collect();

    if vowel_indices.is_empty() {
        return res.iter().enumerate().position(|(_, &c)| is_vietnamese_vowel(c));
    }

    if vowel_indices.len() == 1 {
        return Some(vowel_indices[0]);
    }

    // Check if word ends with a consonant (e.g. 'c', 't', 'n', 'ng', 'm', 'p', 'ch')
    let ends_with_consonant = res.last().map_or(false, |&c| !is_vietnamese_vowel(c));

    if vowel_indices.len() >= 2 {
        let first_vowel_idx = vowel_indices[0];
        let second_vowel_idx = vowel_indices[1];
        let first_vowel = res[first_vowel_idx].to_ascii_lowercase();
        let second_vowel = res[second_vowel_idx].to_ascii_lowercase();

        // Pairs where tone goes on 2nd vowel: 'oa', 'oe', 'uy', or when word has trailing consonant (e.g. 'uan', 'uyen')
        if (first_vowel == 'o' && second_vowel == 'a')
            || (first_vowel == 'o' && second_vowel == 'e')
            || (first_vowel == 'u' && second_vowel == 'y')
            || (first_vowel == 'u' && second_vowel == 'a' && ends_with_consonant)
            || (first_vowel == 'u' && second_vowel == 'o' && ends_with_consonant)
        {
            return Some(second_vowel_idx);
        }

        // Default for 'au', 'ay', 'ao', 'ai', 'eo', 'iu', 'ia', 'ua': tone goes on 1st vowel
        return Some(first_vowel_idx);
    }

    Some(vowel_indices[0])
}

impl ImeEngine for VietnameseEngine {
    fn process_key(&mut self, key: Key, event_value: i32, is_shift: bool, is_capslock: bool) -> EngineAction {
        if self.mode == InputMode::Off {
            return EngineAction::PassThrough;
        }

        if matches!(
            key,
            Key::KEY_LEFTSHIFT
                | Key::KEY_RIGHTSHIFT
                | Key::KEY_LEFTCTRL
                | Key::KEY_RIGHTCTRL
                | Key::KEY_LEFTALT
                | Key::KEY_RIGHTALT
                | Key::KEY_LEFTMETA
                | Key::KEY_RIGHTMETA
        ) {
            return EngineAction::PassThrough;
        }

        if is_word_boundary_key(key) {
            self.reset_buffer();
            return EngineAction::PassThrough;
        }

        let is_press = event_value == 1;

        if let Some(ch) = key_to_char(key, is_shift, is_capslock) {
            // Decision 007: Standalone 'w' at word start must NOT be converted to 'ư'
            if self.mode == InputMode::Telex && self.buffer.is_empty() && (ch == 'w' || ch == 'W') {
                if is_press {
                    self.buffer.push_raw(ch);
                    self.buffer.set_display_str(ch.to_string());
                }
                return EngineAction::PassThrough;
            }

            if !is_press {
                return EngineAction::Consumed;
            }

            let prev_display_len = self.buffer.display_len();
            self.buffer.push_raw(ch);

            let raw = self.buffer.raw_keys().to_vec();
            let new_display = match self.mode {
                InputMode::Telex => self.transform_telex(&raw),
                InputMode::VNI => self.transform_vni(&raw),
                InputMode::Off => String::new(),
            };
            self.buffer.set_display_str(&new_display);

            return EngineAction::InjectKeySequence {
                backspace_count: prev_display_len,
                text: new_display,
            };
        }

        if key == Key::KEY_BACKSPACE {
            if !is_press {
                return EngineAction::Consumed;
            }
            if !self.buffer.is_empty() {
                self.buffer.pop_raw();
                
                let raw = self.buffer.raw_keys().to_vec();
                let new_display = match self.mode {
                    InputMode::Telex => self.transform_telex(&raw),
                    InputMode::VNI => self.transform_vni(&raw),
                    InputMode::Off => String::new(),
                };
                self.buffer.set_display_str(&new_display);
            }
            return EngineAction::PassThrough;
        }

        EngineAction::PassThrough
    }

    fn reset_buffer(&mut self) {
        self.buffer.clear();
    }

    fn get_mode(&self) -> InputMode {
        self.mode
    }

    fn set_mode(&mut self, mode: InputMode) {
        self.mode = mode;
        self.reset_buffer();
    }
}

fn has_vowel(keys: &[char]) -> bool {
    keys.iter().any(|&c| is_vietnamese_vowel(c))
}

fn is_vietnamese_vowel(c: char) -> bool {
    matches!(
        c,
        'a' | 'A' | 'à' | 'À' | 'á' | 'Á' | 'ả' | 'Ả' | 'ã' | 'Ã' | 'ạ' | 'Ạ'
        | 'ă' | 'Ă' | 'ằ' | 'Ằ' | 'ắ' | 'Ắ' | 'ẳ' | 'Ẳ' | 'ẵ' | 'Ẵ' | 'ặ' | 'Ặ'
        | 'â' | 'Â' | 'ầ' | 'Ầ' | 'ấ' | 'Ấ' | 'ẩ' | 'Ẩ' | 'ẫ' | 'Ẫ' | 'ậ' | 'Ậ'
        | 'e' | 'E' | 'è' | 'È' | 'é' | 'É' | 'ẻ' | 'Ẻ' | 'ẽ' | 'Ẽ' | 'ẹ' | 'Ẹ'
        | 'ê' | 'Ê' | 'ề' | 'Ề' | 'ế' | 'Ế' | 'ể' | 'Ể' | 'ễ' | 'Ễ' | 'ệ' | 'Ệ'
        | 'i' | 'I' | 'ì' | 'Ì' | 'í' | 'Í' | 'ỉ' | 'Ỉ' | 'ĩ' | 'Ĩ' | 'ị' | 'Ị'
        | 'o' | 'O' | 'ò' | 'Ò' | 'ó' | 'Ó' | 'ỏ' | 'Ỏ' | 'õ' | 'Õ' | 'ọ' | 'Ọ'
        | 'ô' | 'Ô' | 'ồ' | 'Ồ' | 'ố' | 'Ố' | 'ổ' | 'Ổ' | 'ỗ' | 'Ỗ' | 'ộ' | 'Ộ'
        | 'ơ' | 'Ơ' | 'ờ' | 'Ờ' | 'ớ' | 'Ớ' | 'ở' | 'Ở' | 'ỡ' | 'Ỡ' | 'ợ' | 'Ợ'
        | 'u' | 'U' | 'ù' | 'Ù' | 'ú' | 'Ú' | 'ủ' | 'Ủ' | 'ũ' | 'Ũ' | 'ụ' | 'Ụ'
        | 'ư' | 'Ư' | 'ừ' | 'Ừ' | 'ứ' | 'Ứ' | 'ử' | 'Ử' | 'ữ' | 'Ữ' | 'ự' | 'Ự'
        | 'y' | 'Y' | 'ỳ' | 'Ỳ' | 'ý' | 'Ý' | 'ỷ' | 'Ỷ' | 'ỹ' | 'Ỹ' | 'ỵ' | 'Ỵ'
    )
}

fn apply_tone_to_char(c: char, tone: u8) -> char {
    match (c, tone) {
        // 'a' family
        ('a', 1) => 'á', ('a', 2) => 'à', ('a', 3) => 'ả', ('a', 4) => 'ã', ('a', 5) => 'ạ',
        ('A', 1) => 'Á', ('A', 2) => 'À', ('A', 3) => 'Ả', ('A', 4) => 'Ã', ('A', 5) => 'Ạ',
        ('â', 1) => 'ấ', ('â', 2) => 'ầ', ('â', 3) => 'ẩ', ('â', 4) => 'ẫ', ('â', 5) => 'ậ',
        ('Â', 1) => 'Ấ', ('Â', 2) => 'Ầ', ('Â', 3) => 'Ẩ', ('Â', 4) => 'Ẫ', ('Â', 5) => 'Ậ',
        ('ă', 1) => 'ắ', ('ă', 2) => 'ằ', ('ă', 3) => 'ẳ', ('ă', 4) => 'ẵ', ('ă', 5) => 'ặ',
        ('Ă', 1) => 'Ắ', ('Ă', 2) => 'Ằ', ('Ă', 3) => 'Ẳ', ('Ă', 4) => 'Ẵ', ('Ă', 5) => 'Ặ',

        // 'e' family
        ('e', 1) => 'é', ('e', 2) => 'è', ('e', 3) => 'ẻ', ('e', 4) => 'ẽ', ('e', 5) => 'ẹ',
        ('E', 1) => 'É', ('E', 2) => 'È', ('E', 3) => 'Ẻ', ('E', 4) => 'Ẽ', ('E', 5) => 'Ẹ',
        ('ê', 1) => 'ế', ('ê', 2) => 'ề', ('ê', 3) => 'ể', ('ê', 4) => 'ễ', ('ê', 5) => 'ệ',
        ('Ê', 1) => 'Ế', ('Ê', 2) => 'Ề', ('Ê', 3) => 'Ể', ('Ê', 4) => 'Ễ', ('Ê', 5) => 'Ệ',

        // 'i' family
        ('i', 1) => 'í', ('i', 2) => 'ì', ('i', 3) => 'ỉ', ('i', 4) => 'ĩ', ('i', 5) => 'ị',
        ('I', 1) => 'Í', ('I', 2) => 'Ì', ('I', 3) => 'Ỉ', ('I', 4) => 'Ĩ', ('I', 5) => 'Ị',

        // 'o' family
        ('o', 1) => 'ó', ('o', 2) => 'ò', ('o', 3) => 'ỏ', ('o', 4) => 'õ', ('o', 5) => 'ọ',
        ('O', 1) => 'Ó', ('O', 2) => 'Ò', ('O', 3) => 'Ỏ', ('O', 4) => 'Õ', ('O', 5) => 'Ọ',
        ('ô', 1) => 'ố', ('ô', 2) => 'ồ', ('ô', 3) => 'ổ', ('ô', 4) => 'ỗ', ('ô', 5) => 'ộ',
        ('Ô', 1) => 'Ố', ('Ô', 2) => 'Ồ', ('Ô', 3) => 'Ổ', ('Ô', 4) => 'Ỗ', ('Ô', 5) => 'Ộ',
        ('ơ', 1) => 'ớ', ('ơ', 2) => 'ờ', ('ơ', 3) => 'ở', ('ơ', 4) => 'ỡ', ('ơ', 5) => 'ợ',
        ('Ơ', 1) => 'Ớ', ('Ơ', 2) => 'Ờ', ('Ơ', 3) => 'Ở', ('Ơ', 4) => 'Ỡ', ('Ơ', 5) => 'Ợ',

        // 'u' family
        ('u', 1) => 'ú', ('u', 2) => 'ù', ('u', 3) => 'ủ', ('u', 4) => 'ũ', ('u', 5) => 'ụ',
        ('U', 1) => 'Ú', ('U', 2) => 'Ù', ('U', 3) => 'Ủ', ('U', 4) => 'Ũ', ('U', 5) => 'Ụ',
        ('ư', 1) => 'ứ', ('ư', 2) => 'ừ', ('ư', 3) => 'ử', ('ư', 4) => 'ữ', ('ư', 5) => 'ự',
        ('Ư', 1) => 'Ứ', ('Ư', 2) => 'Ừ', ('Ư', 3) => 'Ử', ('Ư', 4) => 'Ữ', ('Ư', 5) => 'Ự',

        // 'y' family
        ('y', 1) => 'ý', ('y', 2) => 'ỳ', ('y', 3) => 'ỷ', ('y', 4) => 'ỹ', ('y', 5) => 'ỵ',
        ('Y', 1) => 'Ý', ('Y', 2) => 'Ỳ', ('Y', 3) => 'Ỷ', ('Y', 4) => 'Ỹ', ('Y', 5) => 'Ỵ',

        _ => c,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_telex_basic() {
        let mut engine = VietnameseEngine::new(InputMode::Telex);

        engine.process_key(Key::KEY_V, 1, false, false);
        engine.process_key(Key::KEY_I, 1, false, false);
        engine.process_key(Key::KEY_E, 1, false, false);
        engine.process_key(Key::KEY_E, 1, false, false);
        engine.process_key(Key::KEY_T, 1, false, false);
        let action = engine.process_key(Key::KEY_J, 1, false, false);

        if let EngineAction::InjectKeySequence { text, .. } = action {
            assert_eq!(text, "việt");
        } else {
            panic!("Expected InjectKeySequence");
        }
    }

    #[test]
    fn test_smart_placement_tat_a_s() {
        let mut engine = VietnameseEngine::new(InputMode::Telex);

        engine.process_key(Key::KEY_T, 1, false, false);
        engine.process_key(Key::KEY_A, 1, false, false);
        engine.process_key(Key::KEY_T, 1, false, false);
        engine.process_key(Key::KEY_A, 1, false, false);
        let action = engine.process_key(Key::KEY_S, 1, false, false);

        if let EngineAction::InjectKeySequence { text, .. } = action {
            assert_eq!(text, "tất");
        } else {
            panic!("Expected InjectKeySequence");
        }
    }

    #[test]
    fn test_smart_placement_tat_w_s() {
        let mut engine = VietnameseEngine::new(InputMode::Telex);

        engine.process_key(Key::KEY_T, 1, false, false);
        engine.process_key(Key::KEY_A, 1, false, false);
        engine.process_key(Key::KEY_T, 1, false, false);
        engine.process_key(Key::KEY_W, 1, false, false);
        let action = engine.process_key(Key::KEY_S, 1, false, false);

        if let EngineAction::InjectKeySequence { text, .. } = action {
            assert_eq!(text, "tắt");
        } else {
            panic!("Expected InjectKeySequence");
        }
    }

    #[test]
    fn test_tone_placement_chau_mau() {
        let mut engine = VietnameseEngine::new(InputMode::Telex);

        // Test 'chau' + 's' -> 'cháu'
        engine.process_key(Key::KEY_C, 1, false, false);
        engine.process_key(Key::KEY_H, 1, false, false);
        engine.process_key(Key::KEY_A, 1, false, false);
        engine.process_key(Key::KEY_U, 1, false, false);
        let action1 = engine.process_key(Key::KEY_S, 1, false, false);

        if let EngineAction::InjectKeySequence { text, .. } = action1 {
            assert_eq!(text, "cháu");
        } else {
            panic!("Expected InjectKeySequence");
        }

        // Test 'mau' + 'a' + 'x' -> 'mẫu'
        let mut engine2 = VietnameseEngine::new(InputMode::Telex);
        engine2.process_key(Key::KEY_M, 1, false, false);
        engine2.process_key(Key::KEY_A, 1, false, false);
        engine2.process_key(Key::KEY_U, 1, false, false);
        engine2.process_key(Key::KEY_A, 1, false, false);
        let action2 = engine2.process_key(Key::KEY_X, 1, false, false);

        if let EngineAction::InjectKeySequence { text, .. } = action2 {
            assert_eq!(text, "mẫu");
        } else {
            panic!("Expected InjectKeySequence");
        }
    }

    #[test]
    fn test_tone_placement_giao() {
        let mut engine = VietnameseEngine::new(InputMode::Telex);

        engine.process_key(Key::KEY_G, 1, false, false);
        engine.process_key(Key::KEY_I, 1, false, false);
        engine.process_key(Key::KEY_A, 1, false, false);
        engine.process_key(Key::KEY_O, 1, false, false);
        let action = engine.process_key(Key::KEY_S, 1, false, false);

        if let EngineAction::InjectKeySequence { text, .. } = action {
            assert_eq!(text, "giáo");
        } else {
            panic!("Expected InjectKeySequence");
        }
    }

    #[test]
    fn test_khongo_to_khong() {
        let mut engine = VietnameseEngine::new(InputMode::Telex);

        engine.process_key(Key::KEY_K, 1, false, false);
        engine.process_key(Key::KEY_H, 1, false, false);
        engine.process_key(Key::KEY_O, 1, false, false);
        engine.process_key(Key::KEY_N, 1, false, false);
        engine.process_key(Key::KEY_G, 1, false, false);
        let action = engine.process_key(Key::KEY_O, 1, false, false);

        if let EngineAction::InjectKeySequence { text, .. } = action {
            assert_eq!(text, "không");
        } else {
            panic!("Expected InjectKeySequence");
        }
    }

    #[test]
    fn test_vni_multi_modifier_duoc_day() {
        let mut engine = VietnameseEngine::new(InputMode::VNI);

        // Test d + 9 + u + o + 7 + c + 5 -> được
        engine.process_key(Key::KEY_D, 1, false, false);
        engine.process_key(Key::KEY_9, 1, false, false);
        engine.process_key(Key::KEY_U, 1, false, false);
        engine.process_key(Key::KEY_O, 1, false, false);
        engine.process_key(Key::KEY_7, 1, false, false);
        engine.process_key(Key::KEY_C, 1, false, false);
        let action = engine.process_key(Key::KEY_5, 1, false, false);

        if let EngineAction::InjectKeySequence { text, .. } = action {
            assert_eq!(text, "được");
        } else {
            panic!("Expected InjectKeySequence");
        }
    }

    #[test]
    fn test_single_w_dduocwj() {
        let mut engine = VietnameseEngine::new(InputMode::Telex);

        engine.process_key(Key::KEY_D, 1, false, false);
        engine.process_key(Key::KEY_D, 1, false, false);
        engine.process_key(Key::KEY_U, 1, false, false);
        engine.process_key(Key::KEY_O, 1, false, false);
        engine.process_key(Key::KEY_C, 1, false, false);
        engine.process_key(Key::KEY_W, 1, false, false);
        let action = engine.process_key(Key::KEY_J, 1, false, false);

        if let EngineAction::InjectKeySequence { text, .. } = action {
            assert_eq!(text, "được");
        } else {
            panic!("Expected InjectKeySequence");
        }
    }

    #[test]
    fn test_smart_placement_duocdwj() {
        let mut engine = VietnameseEngine::new(InputMode::Telex);

        engine.process_key(Key::KEY_D, 1, false, false);
        engine.process_key(Key::KEY_U, 1, false, false);
        engine.process_key(Key::KEY_O, 1, false, false);
        engine.process_key(Key::KEY_C, 1, false, false);
        engine.process_key(Key::KEY_D, 1, false, false);
        engine.process_key(Key::KEY_W, 1, false, false);
        let action = engine.process_key(Key::KEY_J, 1, false, false);

        if let EngineAction::InjectKeySequence { text, .. } = action {
            assert_eq!(text, "được");
        } else {
            panic!("Expected InjectKeySequence");
        }
    }

    #[test]
    fn test_decision_007_no_standalone_w_at_start() {
        let mut engine = VietnameseEngine::new(InputMode::Telex);

        let action1 = engine.process_key(Key::KEY_W, 1, false, false);
        assert_eq!(action1, EngineAction::PassThrough);

        engine.process_key(Key::KEY_E, 1, false, false);
        let action3 = engine.process_key(Key::KEY_B, 1, false, false);

        if let EngineAction::InjectKeySequence { text, .. } = action3 {
            assert_eq!(text, "web");
        }
    }

    #[test]
    fn test_vni_basic() {
        let mut engine = VietnameseEngine::new(InputMode::VNI);

        engine.process_key(Key::KEY_T, 1, false, false);
        engine.process_key(Key::KEY_A, 1, false, false);
        engine.process_key(Key::KEY_T, 1, false, false);
        engine.process_key(Key::KEY_6, 1, false, false);
        let action = engine.process_key(Key::KEY_1, 1, false, false);

        if let EngineAction::InjectKeySequence { text, .. } = action {
            assert_eq!(text, "tất");
        } else {
            panic!("Expected InjectKeySequence");
        }
    }
}
