use super::SessionState;
use crate::domain::desktop::desktop::MAX_WORKSPACES;
use crate::domain::window_manager::MAX_WINDOWS;
use std::{collections::HashSet, error::Error};

pub(super) fn validate(session: &SessionState) -> Result<(), Box<dyn Error>> {
    if session.workspaces.is_empty() {
        return Err("Sesi TUICLI tidak memiliki workspace".into());
    }

    if session.workspaces.len() > MAX_WORKSPACES {
        return Err(format!("Sesi melebihi batas maksimal {MAX_WORKSPACES} workspace").into());
    }

    let workspace_ids: HashSet<_> = session
        .workspaces
        .iter()
        .map(|workspace| workspace.id.0)
        .collect();

    if workspace_ids.len() != session.workspaces.len() {
        return Err("Sesi memiliki ID workspace duplikat".into());
    }

    if session.next_workspace_id >= usize::MAX - 1
        || workspace_ids.iter().any(|id| *id >= usize::MAX - 1)
    {
        return Err("Sesi memiliki ID workspace yang tidak dapat dilanjutkan".into());
    }

    if !workspace_ids.contains(&session.active_workspace_id) {
        return Err("Workspace aktif dalam sesi tidak valid".into());
    }

    let window_ids: HashSet<_> = session.windows.iter().map(|window| window.id).collect();

    if window_ids.len() != session.windows.len() {
        return Err("Sesi memiliki ID jendela duplikat".into());
    }

    if session.windows.len() > MAX_WINDOWS {
        return Err(format!("Sesi melebihi batas maksimal {MAX_WINDOWS} jendela").into());
    }

    if session.next_window_id >= u64::MAX - 1 || window_ids.iter().any(|id| *id >= u64::MAX - 1) {
        return Err("Sesi memiliki ID jendela yang tidak dapat dilanjutkan".into());
    }

    let mut z_order_ids = HashSet::new();
    if session
        .z_order
        .iter()
        .any(|id| !window_ids.contains(id) || !z_order_ids.insert(*id))
    {
        return Err("Urutan jendela dalam sesi tidak valid".into());
    }

    if session
        .windows
        .iter()
        .any(|window| !workspace_ids.contains(&window.workspace_id))
    {
        return Err("Jendela mengacu ke workspace yang tidak ada".into());
    }

    if session
        .focused_window_id
        .is_some_and(|id| !window_ids.contains(&id))
    {
        return Err("Fokus mengacu ke jendela yang tidak ada".into());
    }

    if let Some(focused_id) = session.focused_window_id {
        let focused_window = session
            .windows
            .iter()
            .find(|window| window.id == focused_id);

        if focused_window.is_some_and(|window| window.workspace_id != session.active_workspace_id) {
            return Err("Jendela fokus bukan bagian dari workspace aktif".into());
        }
    }

    Ok(())
}
