use datasources::{DatasourceError, Result};
use std::collections::HashMap;
use std::sync::Mutex;

pub trait FileStore: Send + Sync {
    fn read(&self, path: &str) -> Result<String>;
    fn write(&self, path: &str, contents: &str) -> Result<()>;
    fn exists(&self, path: &str) -> bool;
}

#[derive(Default)]
pub struct MemoryFileStore {
    files: Mutex<HashMap<String, String>>,
}

impl MemoryFileStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn seed(path: impl Into<String>, contents: impl Into<String>) -> Self {
        let store = Self::new();
        store
            .files
            .lock()
            .unwrap()
            .insert(path.into(), contents.into());
        store
    }
}

impl FileStore for MemoryFileStore {
    fn read(&self, path: &str) -> Result<String> {
        self.files
            .lock()
            .unwrap()
            .get(path)
            .cloned()
            .ok_or_else(|| DatasourceError::message(format!("File not found: {path}")))
    }

    fn write(&self, path: &str, contents: &str) -> Result<()> {
        self.files
            .lock()
            .unwrap()
            .insert(path.to_string(), contents.to_string());
        Ok(())
    }

    fn exists(&self, path: &str) -> bool {
        self.files.lock().unwrap().contains_key(path)
    }
}
