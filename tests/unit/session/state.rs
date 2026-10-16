use super::*;
use crate::domain::desktop::workspace::WorkspaceId;

#[test]
fn window_from_preserves_window_fields() {
    let window = Window::new(
        5,
        "Terminal",
        Geometry {
            x: 1,
            y: 2,
            width: 30,
            height: 10,
        },
        WorkspaceId(3),
    );

    let saved = SessionState::window_from(&window);

    assert_eq!(saved.id, 5);
    assert_eq!(saved.title, "Terminal");
    assert_eq!(saved.geometry, window.geometry);
    assert_eq!(saved.workspace_id, 3);
}
