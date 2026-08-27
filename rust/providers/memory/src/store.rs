use datasources::Record;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Clone)]
pub enum MemoryData {
    Rows(Vec<Record>),
    Collections(HashMap<String, Vec<Record>>),
}

#[derive(Default)]
pub struct MemoryStore {
    collections: Mutex<HashMap<String, Vec<Record>>>,
}

impl MemoryStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn has_collection(&self, name: &str) -> bool {
        self.collections.lock().unwrap().contains_key(name)
    }

    pub fn collection<R>(&self, name: &str, f: impl FnOnce(&mut Vec<Record>) -> R) -> R {
        let mut map = self.collections.lock().unwrap();
        f(map.entry(name.to_string()).or_default())
    }

    pub fn snapshot(&self) -> HashMap<String, Vec<Record>> {
        self.collections.lock().unwrap().clone()
    }

    pub fn restore(&self, snapshot: HashMap<String, Vec<Record>>) {
        *self.collections.lock().unwrap() = snapshot;
    }

    pub fn seed(&self, data: &MemoryData, collection: &str) {
        match data {
            MemoryData::Rows(rows) if !self.has_collection(collection) => {
                self.collections
                    .lock()
                    .unwrap()
                    .insert(collection.to_string(), rows.clone());
            }
            MemoryData::Collections(named) => {
                let mut map = self.collections.lock().unwrap();
                for (name, rows) in named {
                    map.entry(name.clone()).or_insert_with(|| rows.clone());
                }
            }
            _ => {}
        }
    }
}
