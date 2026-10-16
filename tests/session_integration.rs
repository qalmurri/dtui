use std::{
    fs,
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};

use tuicli::{
    domain::{
        desktop::workspace::Workspace,
        session::{SessionState, SessionWindow},
        window_manager::Geometry,
    },
    infrastructure::session as session_storage,
};

static TEST_ID: AtomicU64 = AtomicU64::new(0);

fn temporary_session_path() -> PathBuf {
    let id = TEST_ID.fetch_add(1, Ordering::Relaxed);

    std::env::temp_dir().join(format!(
        "tuicli-session-integration-{}-{id}.toml",
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
fn session_round_trip_preserves_all_state() {
    let path = temporary_session_path();
    let expected = sample_session();

    session_storage::save_to_file(&expected, &path).expect("save session");

    let actual = session_storage::load_from_file(&path).expect("load session");

    assert_eq!(actual, expected);

    fs::remove_file(path).expect("remove temporary session");
}

#[test]
fn session_load_rejects_unknown_active_workspace() {
    let path = temporary_session_path();
    let mut session = sample_session();

    session.active_workspace_id = 99;
    session_storage::save_to_file(&session, &path).expect("save invalid session");

    assert!(
        session_storage::load_from_file(&path).is_err(),
        "unknown active workspace should be rejected"
    );

    fs::remove_file(path).expect("remove temporary session");
}

#[test]
fn session_load_rejects_focus_in_another_workspace() {
    let path = temporary_session_path();
    let mut session = sample_session();

    session.focused_window_id = Some(7);
    session_storage::save_to_file(&session, &path).expect("save invalid session");

    assert!(
        session_storage::load_from_file(&path).is_err(),
        "focus outside active workspace should be rejected"
    );

    fs::remove_file(path).expect("remove temporary session");
}
