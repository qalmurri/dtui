use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn key_to_bytes(key: KeyEvent) -> Vec<u8> {
    let modifiers = key.modifiers;

    if modifiers.contains(KeyModifiers::CONTROL)
        && let KeyCode::Char(ch) = key.code
    {
        let ch = ch.to_ascii_lowercase();
        if ch.is_ascii_lowercase() {
            return vec![ch as u8 - b'a' + 1];
        }

        return match ch {
            '@' | ' ' => vec![0],
            '[' => vec![0x1b],
            '\\' => vec![0x1c],
            ']' => vec![0x1d],
            '^' => vec![0x1e],
            '_' => vec![0x1f],
            _ => ch.to_string().into_bytes(),
        };
    }

    let modified_navigation = matches!(
        key.code,
        KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right | KeyCode::Home | KeyCode::End
    ) && modifiers != KeyModifiers::NONE;

    let mut bytes = if modified_navigation {
        let modifier = xterm_modifier_parameter(modifiers);
        let final_char = match key.code {
            KeyCode::Up => 'A',
            KeyCode::Down => 'B',
            KeyCode::Right => 'C',
            KeyCode::Left => 'D',
            KeyCode::Home => 'H',
            KeyCode::End => 'F',
            _ => unreachable!(),
        };
        format!("\x1b[1;{modifier}{final_char}").into_bytes()
    } else {
        match key.code {
            KeyCode::Char(ch) => ch.to_string().into_bytes(),
            KeyCode::Enter => b"\r".to_vec(),
            KeyCode::Backspace => vec![0x7f],
            KeyCode::Tab => b"\t".to_vec(),
            KeyCode::BackTab => b"\x1b[Z".to_vec(),
            KeyCode::Esc => vec![0x1b],
            KeyCode::Up => b"\x1b[A".to_vec(),
            KeyCode::Down => b"\x1b[B".to_vec(),
            KeyCode::Right => b"\x1b[C".to_vec(),
            KeyCode::Left => b"\x1b[D".to_vec(),
            KeyCode::Home => b"\x1b[H".to_vec(),
            KeyCode::End => b"\x1b[F".to_vec(),
            KeyCode::PageUp => b"\x1b[5~".to_vec(),
            KeyCode::PageDown => b"\x1b[6~".to_vec(),
            KeyCode::Delete => b"\x1b[3~".to_vec(),
            KeyCode::Insert => b"\x1b[2~".to_vec(),
            KeyCode::F(1) => b"\x1bOP".to_vec(),
            KeyCode::F(2) => b"\x1bOQ".to_vec(),
            KeyCode::F(3) => b"\x1bOR".to_vec(),
            KeyCode::F(4) => b"\x1bOS".to_vec(),
            KeyCode::F(n @ 5..=12) => format!("\x1b[{}~", 15 + (n - 5) * 2).into_bytes(),
            _ => Vec::new(),
        }
    };

    if !modified_navigation && modifiers.contains(KeyModifiers::ALT) && !bytes.is_empty() {
        bytes.insert(0, 0x1b);
    }

    bytes
}

fn xterm_modifier_parameter(modifiers: KeyModifiers) -> u8 {
    1 + u8::from(modifiers.contains(KeyModifiers::SHIFT))
        + 2 * u8::from(modifiers.contains(KeyModifiers::ALT))
        + 4 * u8::from(modifiers.contains(KeyModifiers::CONTROL))
        + 8 * u8::from(modifiers.contains(KeyModifiers::SUPER))
        + 16 * u8::from(modifiers.contains(KeyModifiers::HYPER))
        + 32 * u8::from(modifiers.contains(KeyModifiers::META))
}

#[cfg(test)]
mod tests {
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    use super::key_to_bytes;

    #[test]
    fn encodes_regular_control_and_modified_arrow_keys() {
        assert_eq!(
            key_to_bytes(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE)),
            b"a".to_vec()
        );
        assert_eq!(
            key_to_bytes(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL)),
            vec![3]
        );
        assert_eq!(
            key_to_bytes(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE)),
            b"\r".to_vec()
        );
        assert_eq!(
            key_to_bytes(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE)),
            b"\x1b[A".to_vec()
        );
        assert_eq!(
            key_to_bytes(KeyEvent::new(KeyCode::Left, KeyModifiers::ALT)),
            b"\x1b[1;3D".to_vec()
        );
        assert_eq!(
            key_to_bytes(KeyEvent::new(KeyCode::Right, KeyModifiers::CONTROL)),
            b"\x1b[1;5C".to_vec()
        );
    }
}
