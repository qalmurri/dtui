use super::{DesktopState, MAX_WORKSPACES};

#[test]
fn starts_with_one_main_workspace() {
    let desktop = DesktopState::default();

    assert_eq!(desktop.workspaces.len(), 1);
    assert_eq!(desktop.active_workspace().unwrap().name, "MAIN");
    assert_eq!(desktop.next_workspace_id(), 1);
}

#[test]
fn creates_workspace_and_activates_it() {
    let mut desktop = DesktopState::default();

    let id = desktop.create_workspace("GAMES").unwrap();

    assert_eq!(desktop.active_workspace().unwrap().name, "GAMES");
    assert_eq!(desktop.active_workspace().unwrap().id, id);
    assert_eq!(desktop.workspaces.len(), 2);
}

#[test]
fn creates_workspace_with_automatic_name() {
    let mut desktop = DesktopState::default();

    let id = desktop.create_workspace_auto().unwrap();

    assert_eq!(desktop.active_workspace().unwrap().name, "WORKSPACE 2");
    assert_eq!(desktop.active_workspace().unwrap().id, id);
    assert_eq!(desktop.workspaces.len(), 2);
}

#[test]
fn automatic_name_skips_existing_names() {
    let mut desktop = DesktopState::default();

    desktop.create_workspace("WORKSPACE 2").unwrap();
    desktop.select_workspace(0);

    let id = desktop.create_workspace_auto().unwrap();

    assert_eq!(desktop.active_workspace().unwrap().id, id);
    assert_eq!(desktop.active_workspace().unwrap().name, "WORKSPACE 3");
}

#[test]
fn rejects_duplicate_workspace_names() {
    let mut desktop = DesktopState::default();

    assert!(desktop.create_workspace("MAIN").is_none());
    assert_eq!(desktop.workspaces.len(), 1);
}

#[test]
fn renames_active_workspace() {
    let mut desktop = DesktopState::default();

    assert!(desktop.rename_active_workspace("CODE"));
    assert_eq!(desktop.active_workspace().unwrap().name, "CODE");
}

#[test]
fn cannot_remove_last_workspace() {
    let mut desktop = DesktopState::default();

    assert!(desktop.remove_active_workspace().is_none());
    assert_eq!(desktop.workspaces.len(), 1);
}

#[test]
fn workspace_ids_are_not_reused() {
    let mut desktop = DesktopState::default();

    let first = desktop.create_workspace("GAMES").unwrap();
    desktop.remove_active_workspace();
    let second = desktop.create_workspace("MUSIC").unwrap();

    assert_ne!(first, second);
}

#[test]
fn navigation_wraps_around() {
    let mut desktop = DesktopState::default();

    desktop.create_workspace("DEV").unwrap();

    desktop.select_previous_workspace();
    assert_eq!(desktop.active_workspace().unwrap().name, "MAIN");

    desktop.select_next_workspace();
    assert_eq!(desktop.active_workspace().unwrap().name, "DEV");

    desktop.select_next_workspace();
    assert_eq!(desktop.active_workspace().unwrap().name, "MAIN");

    desktop.select_previous_workspace();
    assert_eq!(desktop.active_workspace().unwrap().name, "DEV");
}

#[test]
fn rejects_workspace_creation_above_limit() {
    let mut desktop = DesktopState::default();

    for number in 2..=MAX_WORKSPACES {
        desktop
            .create_workspace(&format!("WORKSPACE {number}"))
            .unwrap();
    }

    assert_eq!(desktop.workspaces.len(), MAX_WORKSPACES);
    assert!(desktop.create_workspace_auto().is_none());
    assert!(desktop.create_workspace("EXTRA").is_none());
    assert_eq!(desktop.workspaces.len(), MAX_WORKSPACES);
}

#[test]
fn rejects_restoring_more_than_nine_workspaces() {
    use crate::domain::desktop::workspace::{Workspace, WorkspaceId};

    let mut desktop = DesktopState::default();

    let workspaces = (0..10)
        .map(|id| Workspace::new(id, format!("WORKSPACE {id}")))
        .collect();

    assert!(!desktop.restore_workspaces(workspaces, WorkspaceId(0), 10));
    assert_eq!(desktop.workspaces.len(), 1);
}

#[test]
fn restores_workspaces_and_active_workspace() {
    use crate::domain::desktop::workspace::{Workspace, WorkspaceId};

    let mut desktop = DesktopState::default();
    let restored = vec![Workspace::new(0, "MAIN"), Workspace::new(4, "TOOLS")];

    assert!(desktop.restore_workspaces(restored, WorkspaceId(4), 5));
    assert_eq!(desktop.active_workspace().unwrap().name, "TOOLS");
    assert_eq!(desktop.next_workspace_id(), 5);
}
