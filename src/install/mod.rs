use std::path::{Path, PathBuf};
use std::process::exit;
use anyhow::Result;
use indicatif::ProgressIterator;
use manifest::ModpackFile;
use crate::cache::addon::ModCache;
use std::fs;
use crate::white;
use std::os::unix::fs::symlink;

mod file;
mod dir;
mod forge;
pub mod addons;
pub mod manifest;

pub fn install<P: AsRef<Path>>(zip_path: P, skip_optional: bool) -> Result<()> {
    white!("Extracting ZIP");
    let extracted = file::extract_zip(&zip_path.as_ref())?;

    white!("Reading manifest");
    let manifest = extracted.path().join("manifest.json");
    let manifest = manifest::Manifest::from_file(&manifest)?;

    let home = std::env::var("HOME")?;
    let modpack_dir = PathBuf::from(&home).join("modpacks").join(&manifest.name);

    if !modpack_dir.exists() {
        white!("Creating modpack directory");
        dir::create_modpack_dirs(&manifest.name)?;
    }

    let addon_dir = modpack_dir.join("mods");

    let mut addon_cache = ModCache::read(&addon_dir)?;
    let addons_to_download: Vec<&ModpackFile> = manifest.files.iter()
        .filter(|f| f.required || !skip_optional)
        .filter(|f| !addon_cache.is_cached(&f.project_id.to_string()))
        .collect();

    white!("Downloading {} mods", addons_to_download.len());
    download_mods(&addons_to_download, &mut addon_cache, &addon_dir)?;

    white!("Copying overrides");
    let overrides = extracted.path().join("overrides");
    file::copy(&overrides, &modpack_dir)?;

    white!("Creating symlinks");
    create_symlinks(&modpack_dir, &manifest.minecraft.version)?;

    white!("Opening Forge installer");
    forge::download(&manifest.minecraft.version, &manifest.minecraft.mod_loaders[0].id, &modpack_dir)?;
    white!("Forge is installed");

    crate::cache::modpack::Modpack::create(&modpack_dir, &manifest)?;

    Ok(())
}

fn create_symlinks<P: AsRef<Path>, S: AsRef<str>>(modpack_dir: P, version: S) -> Result<()> {
    let modpack_dir = modpack_dir.as_ref();

    let home = std::env::var("HOME")?;
    let minecraft_dir = PathBuf::from(&home).join(".minecraft");

    let version_dir = minecraft_dir.join("versions").join(version.as_ref());
    if !version_dir.exists() {
        crate::red!("You need to launch Minecraft {} at least once before you can install this modpack", version.as_ref());
        exit(1);
    }

    let dir = modpack_dir.join("versions").join(version.as_ref());
    if !dir.exists(){
        fs::create_dir_all(modpack_dir.join("versions"))?;
        symlink(version_dir, modpack_dir.join("versions").join(&dir))?;
    }

    let dir = modpack_dir.join("bin");
    if !dir.exists() {
        symlink(minecraft_dir.join("bin"), dir)?;
    }

    let dir = modpack_dir.join("launcher");
    if !dir.exists() {
        symlink(minecraft_dir.join("launcher"), dir)?;
    }

    let dir = modpack_dir.join("assets");
    if !dir.exists() {
        symlink(minecraft_dir.join("assets"), dir)?;
    }

    let dir = modpack_dir.join("libraries");
    if !dir.exists() {
        symlink(minecraft_dir.join("libraries"), dir)?;
    }

    let dir = modpack_dir.join("launcher_accounts.json");
    if !dir.exists() {
        symlink(minecraft_dir.join("launcher_accounts.json"), dir)?;
    }

    let dir = modpack_dir.join("launcher_settings.json");
    if !dir.exists() {
        symlink(minecraft_dir.join("launcher_settings.json"), dir)?;
    }

    let dir = modpack_dir.join("launcher_profiles.json");
    if !dir.exists() {
        symlink(minecraft_dir.join("launcher_profiles.json"), dir)?;
    }

    Ok(())
}

fn download_mods(addons: &[&ModpackFile], cache: &mut ModCache, mod_dir: &Path) -> anyhow::Result<()> {
    for f in addons.iter().progress() {
        let addon = addons::Mod::get(&f.project_id.to_string(), &f.file_id.to_string())?;
        let mod_path = mod_dir.join(&addon.file.file_name);
        if mod_path.exists() {
            continue;
        }

        addon.download(mod_path)?;
        cache.cache_mod(&addon, &f.project_id.to_string())?;
    }

    Ok(())
}
