use std::{
    fs,
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};

use crate::{
    desktop::workspace::Workspace,
    session::{SessionState, SessionWindow},
    window_manager::Geometry,
};

use super::*;

static TEST_ID: AtomicU64 = AtomicU64::new(0);

fn temporary_session_path() -> PathBuf {
    let id = TEST_ID.fetch_add(1, Ordering::Relaxed);

    std::env::temp_dir().join(format!(
        "tuicli-session-test-{}-{id}.toml",
        std::process::id()
    ))
}

fn sample_session() -> SessionState {
    SessionState {
        workspaces: vec![Workspace::new(0, "MAIN"), Workspace::new(3, "CODE")],
        active_workspace_id: 3,
        next_workspace_id: 4,
        windows: vec![
            SessionWindow {
                id: 10,
                title: "Editor".into(),
                geometry: Geometry {
                    x: 4,
                    y: 2,
                    width: 60,
                    height: 18,
                },
                workspace_id: 3,
            },
            SessionWindow {
                id: 7,
                title: "Terminal".into(),
                geometry: Geometry {
                    x: 2,
                    y: 1,
                    width: 40,
                    height: 12,
                },
                workspace_id: 0,
            },
        ],
        z_order: vec![7, 10],
        focused_window_id: Some(10),
        next_window_id: 11,
    }
}

#[test]
fn save_and_load_preserves_session_state() {
    let path = temporary_session_path();
    let expected = sample_session();

    save_to_path(&expected, &path).unwrap();
    let actual = load_from_path(&path).unwrap();

    assert_eq!(actual, expected);
    fs::remove_file(path).unwrap();
}

#[test]
fn load_rejects_unknown_active_workspace() {
    let path = temporary_session_path();
    let mut session = sample_session();

    session.active_workspace_id = 99;
    save_to_path(&session, &path).unwrap();

    assert!(load_from_path(&path).is_err());

    fs::remove_file(path).unwrap();
}

#[test]
fn load_rejects_focus_in_another_workspace() {
    let path = temporary_session_path();
    let mut session = sample_session();

    session.focused_window_id = Some(7);
    save_to_path(&session, &path).unwrap();

    assert!(load_from_path(&path).is_err());

    fs::remove_file(path).unwrap();
}
