use evdev::Key;

/// Convert evdev::Key and shift modifier to standard ASCII char if applicable
pub fn key_to_char(key: Key, is_shift: bool, is_capslock: bool) -> Option<char> {
    let effective_shift = is_shift ^ is_capslock;
    match key {
        Key::KEY_A => Some(if effective_shift { 'A' } else { 'a' }),
        Key::KEY_B => Some(if effective_shift { 'B' } else { 'b' }),
        Key::KEY_C => Some(if effective_shift { 'C' } else { 'c' }),
        Key::KEY_D => Some(if effective_shift { 'D' } else { 'd' }),
        Key::KEY_E => Some(if effective_shift { 'E' } else { 'e' }),
        Key::KEY_F => Some(if effective_shift { 'F' } else { 'f' }),
        Key::KEY_G => Some(if effective_shift { 'G' } else { 'g' }),
        Key::KEY_H => Some(if effective_shift { 'H' } else { 'h' }),
        Key::KEY_I => Some(if effective_shift { 'I' } else { 'i' }),
        Key::KEY_J => Some(if effective_shift { 'J' } else { 'j' }),
        Key::KEY_K => Some(if effective_shift { 'K' } else { 'k' }),
        Key::KEY_L => Some(if effective_shift { 'L' } else { 'l' }),
        Key::KEY_M => Some(if effective_shift { 'M' } else { 'm' }),
        Key::KEY_N => Some(if effective_shift { 'N' } else { 'n' }),
        Key::KEY_O => Some(if effective_shift { 'O' } else { 'o' }),
        Key::KEY_P => Some(if effective_shift { 'P' } else { 'p' }),
        Key::KEY_Q => Some(if effective_shift { 'Q' } else { 'q' }),
        Key::KEY_R => Some(if effective_shift { 'R' } else { 'r' }),
        Key::KEY_S => Some(if effective_shift { 'S' } else { 's' }),
        Key::KEY_T => Some(if effective_shift { 'T' } else { 't' }),
        Key::KEY_U => Some(if effective_shift { 'U' } else { 'u' }),
        Key::KEY_V => Some(if effective_shift { 'V' } else { 'v' }),
        Key::KEY_W => Some(if effective_shift { 'W' } else { 'w' }),
        Key::KEY_X => Some(if effective_shift { 'X' } else { 'x' }),
        Key::KEY_Y => Some(if effective_shift { 'Y' } else { 'y' }),
        Key::KEY_Z => Some(if effective_shift { 'Z' } else { 'z' }),

        // Number row
        Key::KEY_1 => Some(if is_shift { '!' } else { '1' }),
        Key::KEY_2 => Some(if is_shift { '@' } else { '2' }),
        Key::KEY_3 => Some(if is_shift { '#' } else { '3' }),
        Key::KEY_4 => Some(if is_shift { '$' } else { '4' }),
        Key::KEY_5 => Some(if is_shift { '%' } else { '5' }),
        Key::KEY_6 => Some(if is_shift { '^' } else { '6' }),
        Key::KEY_7 => Some(if is_shift { '&' } else { '7' }),
        Key::KEY_8 => Some(if is_shift { '*' } else { '8' }),
        Key::KEY_9 => Some(if is_shift { '(' } else { '9' }),
        Key::KEY_0 => Some(if is_shift { ')' } else { '0' }),

        _ => None,
    }
}

/// Check if a key is a Word Boundary key (Decision 002) that MUST reset the Caret Buffer immediately.
pub fn is_word_boundary_key(key: Key) -> bool {
    matches!(
        key,
        Key::KEY_SPACE
            | Key::KEY_ENTER
            | Key::KEY_TAB
            | Key::KEY_ESC
            | Key::KEY_LEFT
            | Key::KEY_RIGHT
            | Key::KEY_UP
            | Key::KEY_DOWN
            | Key::KEY_HOME
            | Key::KEY_END
            | Key::KEY_PAGEUP
            | Key::KEY_PAGEDOWN
            | Key::KEY_DELETE
            | Key::KEY_INSERT
            | Key::KEY_COMMA
            | Key::KEY_DOT
            | Key::KEY_SLASH
            | Key::KEY_SEMICOLON
            | Key::KEY_APOSTROPHE
            | Key::KEY_LEFTBRACE
            | Key::KEY_RIGHTBRACE
            | Key::KEY_BACKSLASH
            | Key::KEY_MINUS
            | Key::KEY_EQUAL
            | Key::KEY_GRAVE
    )
}

/// Helper mapping an ASCII character back to evdev::Key and shift requirement
pub fn char_to_evdev_key(c: char) -> Option<(Key, bool)> {
    match c {
        'a' => Some((Key::KEY_A, false)),
        'A' => Some((Key::KEY_A, true)),
        'b' => Some((Key::KEY_B, false)),
        'B' => Some((Key::KEY_B, true)),
        'c' => Some((Key::KEY_C, false)),
        'C' => Some((Key::KEY_C, true)),
        'd' => Some((Key::KEY_D, false)),
        'D' => Some((Key::KEY_D, true)),
        'e' => Some((Key::KEY_E, false)),
        'E' => Some((Key::KEY_E, true)),
        'f' => Some((Key::KEY_F, false)),
        'F' => Some((Key::KEY_F, true)),
        'g' => Some((Key::KEY_G, false)),
        'G' => Some((Key::KEY_G, true)),
        'h' => Some((Key::KEY_H, false)),
        'H' => Some((Key::KEY_H, true)),
        'i' => Some((Key::KEY_I, false)),
        'I' => Some((Key::KEY_I, true)),
        'j' => Some((Key::KEY_J, false)),
        'J' => Some((Key::KEY_J, true)),
        'k' => Some((Key::KEY_K, false)),
        'K' => Some((Key::KEY_K, true)),
        'l' => Some((Key::KEY_L, false)),
        'L' => Some((Key::KEY_L, true)),
        'm' => Some((Key::KEY_M, false)),
        'M' => Some((Key::KEY_M, true)),
        'n' => Some((Key::KEY_N, false)),
        'N' => Some((Key::KEY_N, true)),
        'o' => Some((Key::KEY_O, false)),
        'O' => Some((Key::KEY_O, true)),
        'p' => Some((Key::KEY_P, false)),
        'P' => Some((Key::KEY_P, true)),
        'q' => Some((Key::KEY_Q, false)),
        'Q' => Some((Key::KEY_Q, true)),
        'r' => Some((Key::KEY_R, false)),
        'R' => Some((Key::KEY_R, true)),
        's' => Some((Key::KEY_S, false)),
        'S' => Some((Key::KEY_S, true)),
        't' => Some((Key::KEY_T, false)),
        'T' => Some((Key::KEY_T, true)),
        'u' => Some((Key::KEY_U, false)),
        'U' => Some((Key::KEY_U, true)),
        'v' => Some((Key::KEY_V, false)),
        'V' => Some((Key::KEY_V, true)),
        'w' => Some((Key::KEY_W, false)),
        'W' => Some((Key::KEY_W, true)),
        'x' => Some((Key::KEY_X, false)),
        'X' => Some((Key::KEY_X, true)),
        'y' => Some((Key::KEY_Y, false)),
        'Y' => Some((Key::KEY_Y, true)),
        'z' => Some((Key::KEY_Z, false)),
        'Z' => Some((Key::KEY_Z, true)),

        '0' => Some((Key::KEY_0, false)),
        '1' => Some((Key::KEY_1, false)),
        '2' => Some((Key::KEY_2, false)),
        '3' => Some((Key::KEY_3, false)),
        '4' => Some((Key::KEY_4, false)),
        '5' => Some((Key::KEY_5, false)),
        '6' => Some((Key::KEY_6, false)),
        '7' => Some((Key::KEY_7, false)),
        '8' => Some((Key::KEY_8, false)),
        '9' => Some((Key::KEY_9, false)),

        _ => None,
    }
}
