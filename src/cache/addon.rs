use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::fs;

const CURRENT_VERSION: u8 = 1;
const CACHE_FILE_NAME: &str = ".modcache.json";

#[derive(Deserialize, Serialize)]
pub struct ModCache {
    #[serde(skip)]
    mods_folder:    PathBuf,
    version:        u8,
    pub mods:           Vec<Mod>,
}

#[derive(Serialize, Deserialize)]
pub struct Mod {
    pub id:         String,
    pub name:       String,
    pub filename:   String,
    pub authors:    Vec<String>
}

impl ModCache {
    pub fn read<P: AsRef<Path>>(mods_folder: P) -> Result<Self> {
        let cache_file = mods_folder.as_ref().join(CACHE_FILE_NAME);
        if !cache_file.exists() {
            return Ok(Self {
                mods_folder: mods_folder.as_ref().to_owned(),
                version: CURRENT_VERSION,
                mods: Vec::default()
            });
        }

        let f = fs::File::open(&cache_file)?;
        let cache: ModCache = serde_json::from_reader(&f)?;

        if cache.version == CURRENT_VERSION {
            Ok(cache)
        } else {
            fs::remove_file(&cache_file)?;
            Ok(Self {
                mods_folder: mods_folder.as_ref().to_owned(),
                version: CURRENT_VERSION,
                mods: Vec::default()
            })
        }
    }

    pub fn cache_mod<S: AsRef<str>>(&mut self, addon: &crate::install::addons::Mod, id: S) -> Result<()> {
        let cache_file = self.mods_folder.join(CACHE_FILE_NAME);
        let mut f = fs::File::create(&cache_file)?;

        self.mods.push(Mod {
            id: id.as_ref().to_string(),
            authors: addon.info.authors.iter().map(|f| f.name.clone()).collect(),
            name: addon.info.name.clone(),
            filename: addon.file.file_name.clone()
        });

        serde_json::to_writer(&mut f, &self)?;

        Ok(())
    }

    pub fn is_cached<S: AsRef<str>>(&self, id: S) -> bool {
        self.mods.iter().any(|f| f.id.eq(id.as_ref()))
    }
}