use std::{error::Error, fs, path::PathBuf};

use mlua::Lua;

use crate::infrastructure::config::{Config, config_dir};

fn scripts_dir() -> Result<PathBuf, Box<dyn Error>> {
    Ok(config_dir()?.join("scripts"))
}

fn ensure_init_script(path: &PathBuf) -> Result<(), Box<dyn Error>> {
    if path.exists() {
        return Ok(());
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let template = r#"-- TUICLI initialization script.
-- This script runs when Lua scripting is enabled in config.toml.

print("TUICLI Lua runtime initialized")
"#;

    fs::write(path, template)?;

    Ok(())
}

pub fn run_init(config: &Config) -> Result<(), Box<dyn Error>> {
    if !config.lua.enabled {
        return Ok(());
    }

    let script_path = scripts_dir()?.join(&config.lua.init_script);

    ensure_init_script(&script_path)?;

    let source = fs::read_to_string(&script_path)?;

    let lua = Lua::new();

    lua.load(&source)
        .set_name(script_path.to_string_lossy())
        .exec()?;

    Ok(())
}
