use std::{
    collections::HashSet,
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::{
    desktop::workspace::Workspace,
    window_manager::{Geometry, Window},
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionWindow {
    pub id: u64,
    pub title: String,
    pub geometry: Geometry,
    pub workspace_id: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionState {
    pub workspaces: Vec<Workspace>,
    pub active_workspace_id: usize,
    pub next_workspace_id: usize,
    pub windows: Vec<SessionWindow>,
    pub z_order: Vec<u64>,
    pub focused_window_id: Option<u64>,
    pub next_window_id: u64,
}

impl SessionState {
    pub fn path() -> Result<PathBuf, Box<dyn Error>> {
        let dirs = ProjectDirs::from("com", "qalmurri", "dtui")
            .ok_or("Tidak dapat menentukan direktori konfigurasi")?;

        Ok(dirs.config_dir().join("session.toml"))
    }

    pub fn load() -> Result<Option<Self>, Box<dyn Error>> {
        let path = Self::path()?;

        if !path.exists() {
            return Ok(None);
        }

        Self::load_from_path(&path).map(Some)
    }

    fn load_from_path(path: &Path) -> Result<Self, Box<dyn Error>> {
        let contents = fs::read_to_string(path)?;
        let session: Self = toml::from_str(&contents)?;

        if session.workspaces.is_empty() {
            return Err("Sesi DTUI tidak memiliki workspace".into());
        }

        let workspace_ids: HashSet<_> = session.workspaces.iter().map(|w| w.id.0).collect();

        if workspace_ids.len() != session.workspaces.len() {
            return Err("Sesi memiliki ID workspace duplikat".into());
        }

        if !workspace_ids.contains(&session.active_workspace_id) {
            return Err("Workspace aktif dalam sesi tidak valid".into());
        }

        let window_ids: HashSet<_> = session.windows.iter().map(|w| w.id).collect();

        if window_ids.len() != session.windows.len() {
            return Err("Sesi memiliki ID jendela duplikat".into());
        }

        if session
            .windows
            .iter()
            .any(|w| !workspace_ids.contains(&w.workspace_id))
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
            let focused_window = session.windows.iter().find(|w| w.id == focused_id);
            if focused_window.is_some_and(|w| w.workspace_id != session.active_workspace_id) {
                return Err("Jendela fokus bukan bagian dari workspace aktif".into());
            }
        }

        Ok(session)
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let path = Self::path()?;
        self.save_to_path(&path)
    }

    fn save_to_path(&self, path: &Path) -> Result<(), Box<dyn Error>> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let contents = toml::to_string_pretty(self)?;
        let temporary = path.with_extension("toml.tmp");

        fs::write(&temporary, contents)?;
        fs::rename(temporary, path)?;

        Ok(())
    }

    pub fn window_from(window: &Window) -> SessionWindow {
        SessionWindow {
            id: window.id,
            title: window.title.clone(),
            geometry: window.geometry,
            workspace_id: window.workspace_id.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        path::PathBuf,
        sync::atomic::{AtomicU64, Ordering},
    };

    use super::*;
    use crate::desktop::workspace::WorkspaceId;

    static TEST_ID: AtomicU64 = AtomicU64::new(0);

    fn temporary_session_path() -> PathBuf {
        let id = TEST_ID.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!(
            "dtui-session-test-{}-{id}.toml",
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

        expected.save_to_path(&path).unwrap();
        let actual = SessionState::load_from_path(&path).unwrap();

        assert_eq!(actual, expected);
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn load_rejects_unknown_active_workspace() {
        let path = temporary_session_path();
        let mut session = sample_session();
        session.active_workspace_id = 99;
        session.save_to_path(&path).unwrap();

        assert!(SessionState::load_from_path(&path).is_err());
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn load_rejects_focus_in_another_workspace() {
        let path = temporary_session_path();
        let mut session = sample_session();
        session.focused_window_id = Some(7);
        session.save_to_path(&path).unwrap();

        assert!(SessionState::load_from_path(&path).is_err());
        fs::remove_file(path).unwrap();
    }

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
}
