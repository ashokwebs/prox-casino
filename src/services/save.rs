use anyhow::Result;

use crate::storage::local::{SaveData, SaveStore};

pub struct SaveService {
    store: SaveStore,
}

impl SaveService {
    pub fn new() -> Result<Self> {
        Ok(Self {
            store: SaveStore::new()?,
        })
    }

    pub fn load(&self) -> Result<SaveData> {
        self.store.load()
    }

    pub fn save(&self, data: &SaveData) -> Result<()> {
        self.store.save(data)
    }

    pub fn path(&self) -> String {
        self.store.path().display().to_string()
    }
}
