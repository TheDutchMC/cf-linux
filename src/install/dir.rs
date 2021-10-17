use anyhow::Result;
use std::fs;
use std::path::PathBuf;

pub fn create_modpack_dirs<S: AsRef<str>>(modpack_name: S) -> Result<PathBuf> {
    let home = std::env::var("HOME")?;

    let modpack_dir = PathBuf::from(&home).join("modpacks").join(modpack_name.as_ref());
    if !modpack_dir.exists() {
        fs::create_dir_all(&modpack_dir)?;
    }

    let mod_dir = modpack_dir.join("mods");
    if !mod_dir.exists() {
        fs::create_dir_all(&mod_dir)?;
    }

    Ok(modpack_dir)
}