use std::path::Path;
use serde::Deserialize;
use anyhow::Result;
use crate::CLIENT;
use std::fs;

const API_ENDPOINT: &str = "https://addons-ecs.forgesvc.net/api/v2";

pub struct Mod {
    pub info:   AddonManifest,
    pub file:   FileManifest
}

impl Mod {
    pub fn get<S: AsRef<str>>(id: S, file_id: S) -> Result<Mod> {
        let addon_man = Self::get_addon_manifest(id.as_ref())?;

        let download_man = Self::get_file_manifest(id.as_ref(), file_id.as_ref())?;

        Ok(Self {
            info: addon_man,
            file: download_man
        })
    }

    pub fn download<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let mut f = fs::File::create(path.as_ref())?;
        let mut response = CLIENT.get(&self.file.download_url)
            .send()?;
        response.copy_to(&mut f)?;

        Ok(())
    }

    fn get_addon_manifest(id: &str) -> Result<AddonManifest> {
        let response: AddonManifest = CLIENT.get(format!("{}/addon/{}", API_ENDPOINT, id))
            .send()?
            .json()?;

        Ok(response)
    }

    fn get_file_manifest(project_id: &str, file_id: &str) -> Result<FileManifest> {
        let response: FileManifest = CLIENT.get(format!("{}/addon/{}/file/{}", API_ENDPOINT, project_id, file_id))
            .send()?
            .json()?;

        Ok(response)
    }
}

#[derive(Deserialize)]
pub struct AddonManifest {
    pub name:       String,
    pub authors:    Vec<AddonAuthor>,
}

#[derive(Deserialize)]
pub struct AddonAuthor {
    pub name: String
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileManifest {
    pub file_name:       String,
    pub download_url:   String
}
