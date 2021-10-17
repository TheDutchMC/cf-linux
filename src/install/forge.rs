use std::path::Path;
use anyhow::Result;
use crate::CLIENT;
use std::fs;
use std::io;
use std::process::{Command, Stdio};

//const FORGE_INSTALLER_JAVA: &[u8] = include_bytes!("ForgeInstaller.java");
const FORGE_URL: &str = "https://maven.minecraftforge.net/net/minecraftforge/forge";

pub fn download<S: AsRef<str>, T: AsRef<str>, P: AsRef<Path>>(mc_version: S, modloader_id: T, modpack_dir: P) -> Result<()> {
    let tempdir = tempfile::tempdir()?;
    let version = format!("{}-{}", mc_version.as_ref(), modloader_id.as_ref().replace("forge-", ""));
    download_jar(&tempdir.path(), &version)?;
    run_installer(tempdir.path(), &modpack_dir)?;

    std::mem::forget(tempdir);

    Ok(())
}

fn run_installer<P: AsRef<Path>, O: AsRef<Path>>(tmpdir: P, out_dir: O) -> Result<()> {
    let forge_jar = tmpdir.as_ref().join("forge.jar");
    crate::yellow!("Opening Forge Installer at {:?}. Please enter '{}' as output directory.", &forge_jar, out_dir.as_ref().to_str().expect("Missing out_dir path"));
    Command::new("java")
        .arg("-jar")
        .arg(forge_jar.to_str().expect("Missing forge_jar path"))
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?
        .wait()?;

    Ok(())
}

fn download_jar<P: AsRef<Path>, S: AsRef<str>>(dpath: P, version: S) -> Result<()> {
    let mut f = fs::File::create(dpath.as_ref().join("forge.jar"))?;
    let response = CLIENT.get(format!("{}/{v}/forge-{v}-installer.jar", FORGE_URL, v = version.as_ref())).send()?;
    io::copy(&mut response.bytes()?.as_ref(), &mut f)?;

    Ok(())
}