use std::{fs, path::PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::models::player::OfflinePlayer;
use crate::models::stats::PlayerStats;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveData {
    pub player: OfflinePlayer,
    pub stats: PlayerStats,
    pub version: String,
}

impl Default for SaveData {
    fn default() -> Self {
        Self {
            player: OfflinePlayer::default(),
            stats: PlayerStats::default(),
            version: crate::core::APP_VERSION.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SaveFile {
    pub version: String,
    pub data: SaveData,
}

pub struct SaveStore {
    path: PathBuf,
}

impl SaveStore {
    pub fn new() -> Result<Self> {
        let mut base = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
        base.push("prox-casino");
        fs::create_dir_all(&base).context("creating save directory")?;

        Ok(Self {
            path: base.join("offline_save.json"),
        })
    }

    pub fn load(&self) -> Result<SaveData> {
        if !self.path.exists() {
            return Ok(SaveData::default());
        }
        let bytes = fs::read(&self.path).context("reading save file")?;
        let file: SaveFile = serde_json::from_slice(&bytes).context("parsing save file")?;
        Ok(file.data)
    }

    pub fn save(&self, data: &SaveData) -> Result<()> {
        let file = SaveFile {
            version: crate::core::APP_VERSION.to_string(),
            data: data.clone(),
        };
        let bytes = serde_json::to_vec_pretty(&file).context("serializing save data")?;
        let tmp = self.path.with_extension("json.tmp");
        fs::write(&tmp, &bytes).context("writing temp save file")?;
        fs::rename(&tmp, &self.path).context("finalizing save file")?;
        Ok(())
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}
