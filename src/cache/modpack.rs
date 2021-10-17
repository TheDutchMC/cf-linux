use std::path::{Path, PathBuf};
use anyhow::Result;
use serde::{Serialize, Deserialize};
use std::fs;
use crate::install::manifest::Manifest;

const FILE_NAME: &str = ".modpack.json";

#[derive(Serialize, Deserialize)]
pub struct Modpack {
    #[serde(skip)]
    path:           PathBuf,
    pub name:       String,
    pub version:    String,
    pub author:     String,
    pub index:      u32
}

impl Modpack {
    pub fn read<P: AsRef<Path>>(dir: P) -> Result<Self> {
        let path = dir.as_ref().join(FILE_NAME);
        if !path.exists() {
            return Ok(Self {
                path,
                name: dir.as_ref().to_str().expect("Missing modpack path").to_string(),
                author: "Unknown".to_string(),
                version: "Unknown".to_string(),
                index: 0
            });
        }

        let f = fs::File::open(&path)?;
        let mut this: Self = serde_json::from_reader(&f)?;
        this.path = path;

        Ok(this)
    }

    pub fn read_index<P: AsRef<Path>>(modpack_dir: P, index: u32) -> Result<Option<Self>> {
        for f in fs::read_dir(modpack_dir.as_ref())? {
            let path = f?.path();
            if !path.is_dir() {
                continue;
            }

            let fpath = path.join(FILE_NAME);
            if !fpath.exists() {
                continue;
            }

            let f = fs::File::open(&fpath)?;
            let mut this: Self = serde_json::from_reader(&f)?;
            this.path = fpath.clone();

            if this.index == index {
                return Ok(Some(this));
            }
        }

        Ok(None)
    }

    pub fn create<P: AsRef<Path>>(dir: P, manifest: &Manifest) -> Result<()> {
        let path = dir.as_ref().join(FILE_NAME);
        let this = Self {
            path: path.clone(),
            name: manifest.name.clone(),
            author: manifest.author.clone(),
            version: manifest.version.clone(),
            index: 0
        };

        let mut f = fs::File::create(&path)?;
        serde_json::to_writer(&mut f, &this)?;

        Ok(())
    }

    pub fn update_index(&mut self, index: u32) -> Result<()> {
        self.index = index;
        let mut f = fs::File::create(&self.path)?;
        serde_json::to_writer(&mut f, &self)?;
        Ok(())
    }
}