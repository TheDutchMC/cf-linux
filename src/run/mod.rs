use std::path::PathBuf;
use std::process::{Command, exit, Stdio};
use anyhow::Result;

use crate::red;

pub fn run(index: u32) -> Result<()> {
    let home = std::env::var("HOME")?;
    let modpack_dir = PathBuf::from(&home).join("modpacks");
    if !modpack_dir.exists() {
        red!("You don't have any modpacks installed!");
        exit(1);
    }

    let modpack = crate::cache::modpack::Modpack::read_index(&modpack_dir, index)?;
    if let Some(modpack) = modpack {
        Command::new("minecraft-launcher")
            .current_dir(modpack_dir.join(&modpack.name))
            .arg("--workDir")
            .arg(modpack_dir.join(&modpack.name).to_str().expect("Failed to retreive modpack dir as str"))
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
    } else {
        red!("Modpack does not exist! Use `list` to list available modpacks");
        exit(1);
    }

    Ok(())
}