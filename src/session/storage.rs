use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use directories::ProjectDirs;

use super::{SessionState, validation};

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

        validation::validate(&session)?;

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
}

#[cfg(test)]
mod tests {
    use crate::session::state::SessionWindow;
    use std::{
        fs,
        path::PathBuf,
        sync::atomic::{AtomicU64, Ordering},
    };

    use crate::{desktop::workspace::Workspace, window_manager::Geometry};

    use super::*;

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
}
