mod storage;
mod validation;

use std::{
    error::Error,
    fs, io,
    path::{Path, PathBuf},
};

pub use crate::domain::session::{SessionState, SessionWindow};

#[derive(Debug, Default, Clone, Copy)]
pub struct FileSessionRepository;

pub fn session_path() -> Result<PathBuf, Box<dyn Error>> {
    storage::default_path()
}

/// Remove the saved layout. Returns false if there was no session file.
pub fn remove_default() -> Result<bool, Box<dyn Error>> {
    match fs::remove_file(session_path()?) {
        Ok(()) => Ok(true),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(error.into()),
    }
}

impl crate::application::ports::SessionRepository for FileSessionRepository {
    fn load(&self) -> Result<Option<SessionState>, Box<dyn std::error::Error>> {
        storage::load_default()
    }

    fn save(&self, session: &SessionState) -> Result<(), Box<dyn std::error::Error>> {
        storage::save_default(session)
    }
}

pub fn load_from_file(path: &Path) -> Result<SessionState, Box<dyn Error>> {
    storage::load_from_path(path)
}

pub fn save_to_file(session: &SessionState, path: &Path) -> Result<(), Box<dyn Error>> {
    storage::save_to_path(session, path)
}
