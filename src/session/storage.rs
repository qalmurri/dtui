use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use directories::ProjectDirs;

use super::{SessionState, validation};

impl SessionState {
    pub fn path() -> Result<PathBuf, Box<dyn Error>> {
        let dirs = ProjectDirs::from("com", "qalmurri", "tuicli")
            .ok_or("Tidak dapat menentukan direktori konfigurasi")?;

        Ok(dirs.config_dir().join("session.toml"))
    }

    pub fn load() -> Result<Option<Self>, Box<dyn Error>> {
        let path = Self::path()?;

        if !path.exists() {
            return Ok(None);
        }

        load_from_path(&path).map(Some)
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let path = Self::path()?;
        save_to_path(self, &path)
    }
}

pub(super) fn load_from_path(path: &Path) -> Result<SessionState, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    let session: SessionState = toml::from_str(&contents)?;

    validation::validate(&session)?;

    Ok(session)
}

pub(super) fn save_to_path(session: &SessionState, path: &Path) -> Result<(), Box<dyn Error>> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let contents = toml::to_string_pretty(session)?;
    let temporary = path.with_extension("toml.tmp");

    fs::write(&temporary, contents)?;
    fs::rename(temporary, path)?;

    Ok(())
}

#[cfg(test)]
mod tests {
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
}
