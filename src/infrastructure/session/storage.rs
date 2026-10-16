use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use directories::ProjectDirs;

use super::{SessionState, validation};

pub fn load_default() -> Result<Option<SessionState>, Box<dyn Error>> {
    let path = default_path()?;

    if !path.exists() {
        return Ok(None);
    }

    load_from_path(&path).map(Some)
}

pub fn save_default(session: &SessionState) -> Result<(), Box<dyn Error>> {
    save_to_path(session, &default_path()?)
}

fn default_path() -> Result<PathBuf, Box<dyn Error>> {
    let dirs = ProjectDirs::from("com", "qalmurri", "tuicli")
        .ok_or("Tidak dapat menentukan direktori konfigurasi")?;

    Ok(dirs.config_dir().join("session.toml"))
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
    let temporary = path.with_extension(format!("toml.tmp.{}", std::process::id()));

    fs::write(&temporary, contents)?;
    fs::rename(temporary, path)?;

    Ok(())
}

#[cfg(test)]
#[path = "../../../tests/unit/session/storage.rs"]
mod tests;
