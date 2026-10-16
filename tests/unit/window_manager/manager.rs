use super::*;

fn geometry() -> Geometry {
    Geometry {
        x: 0,
        y: 0,
        width: 30,
        height: 10,
    }
}

#[test]
fn closing_focused_window_focuses_previous_window_in_same_workspace() {
    let mut manager = WindowManager::default();
    let workspace = WorkspaceId(0);

    let first = manager.create("First", geometry(), workspace);
    let second = manager.create("Second", geometry(), workspace);

    manager.close(second);

    assert_eq!(manager.focus.window_id, Some(first));
    assert!(manager.get(second).is_none());
    assert!(!manager.z_order.windows.contains(&second));
}

#[test]
fn closing_non_focused_window_preserves_focus() {
    let mut manager = WindowManager::default();
    let workspace = WorkspaceId(0);

    let first = manager.create("First", geometry(), workspace);
    let second = manager.create("Second", geometry(), workspace);

    manager.close(first);

    assert_eq!(manager.focus.window_id, Some(second));
}

#[test]
fn closing_last_window_clears_focus() {
    let mut manager = WindowManager::default();
    let id = manager.create("Terminal", geometry(), WorkspaceId(0));

    manager.close(id);

    assert!(manager.windows.is_empty());
    assert_eq!(manager.focus.window_id, None);
}

#[test]
fn closing_window_does_not_focus_another_workspace() {
    let mut manager = WindowManager::default();
    let first = manager.create("First", geometry(), WorkspaceId(0));
    let other = manager.create("Other", geometry(), WorkspaceId(1));

    manager.activate_workspace(WorkspaceId(0));
    manager.close(first);

    assert_eq!(manager.focus.window_id, None);
    assert!(manager.get(other).is_some());
}
