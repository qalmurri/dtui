mod state;
mod storage;
mod validation;

pub use state::{SessionState, SessionWindow};

use std::{error::Error, path::Path};

impl SessionState {
    pub fn load_from_file(path: &Path) -> Result<Self, Box<dyn Error>> {
        storage::load_from_path(path)
    }

    pub fn save_to_file(&self, path: &Path) -> Result<(), Box<dyn Error>> {
        storage::save_to_path(self, path)
    }
}
