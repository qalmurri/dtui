mod storage;
mod validation;

pub use crate::domain::session::{SessionState, SessionWindow};

#[derive(Debug, Default, Clone, Copy)]
pub struct FileSessionRepository;

impl crate::application::ports::SessionRepository for FileSessionRepository {
    fn load(&self) -> Result<Option<SessionState>, Box<dyn std::error::Error>> {
        storage::load_default()
    }

    fn save(&self, session: &SessionState) -> Result<(), Box<dyn std::error::Error>> {
        storage::save_default(session)
    }
}

use std::{error::Error, path::Path};

pub fn load_from_file(path: &Path) -> Result<SessionState, Box<dyn Error>> {
    storage::load_from_path(path)
}

pub fn save_to_file(session: &SessionState, path: &Path) -> Result<(), Box<dyn Error>> {
    storage::save_to_path(session, path)
}
