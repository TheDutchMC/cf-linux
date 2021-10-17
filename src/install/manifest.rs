use std::path::Path;
use serde::Deserialize;
use anyhow::Result;
use std::fs;

#[derive(Deserialize)]
pub struct Manifest {
    pub minecraft:  Minecraft,
    pub name:       String,
    pub version:    String,
    pub author:     String,
    pub files:      Vec<ModpackFile>,
    pub overrides:  String
}

impl Manifest {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let f = fs::File::open(path.as_ref())?;
        let this: Self = serde_json::from_reader(&f)?;

        Ok(this)
    }
}

#[derive(Deserialize)]
pub struct Minecraft {
    pub version:        String,
    #[serde(rename(deserialize = "modLoaders"))]
    pub mod_loaders:    Vec<ModLoader>,
}

#[derive(Deserialize)]
pub struct ModLoader {
    pub id: String
}

#[derive(Deserialize)]
pub struct ModpackFile {
    #[serde(rename(deserialize = "projectID"))]
    pub project_id:     i64,
    #[serde(rename(deserialize = "fileID"))]
    pub file_id:        i64,
    pub required:       bool,
}