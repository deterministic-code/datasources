use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Clock {
    now: Arc<Mutex<Arc<dyn Fn() -> String + Send + Sync>>>,
    uuid: Arc<Mutex<Arc<dyn Fn() -> String + Send + Sync>>>,
}

impl Clock {
    pub fn new() -> Self {
        Self {
            now: Arc::new(Mutex::new(Arc::new(system_now))),
            uuid: Arc::new(Mutex::new(Arc::new(|| {
                datasources::uuid::Uuid::new_v4().to_string()
            }))),
        }
    }

    pub fn now(&self) -> String {
        let now = self.now.lock().expect("clock").clone();
        now()
    }

    pub fn uuid(&self) -> String {
        let uuid = self.uuid.lock().expect("clock").clone();
        uuid()
    }
}

impl Default for Clock {
    fn default() -> Self {
        Self::new()
    }
}

fn system_now() -> String {
    chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string()
}
