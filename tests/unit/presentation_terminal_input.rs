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
