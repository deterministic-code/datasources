use std::collections::HashMap;
use std::sync::Mutex;

static DEFAULTS: Mutex<Option<HashMap<String, String>>> = Mutex::new(None);

pub fn defaults(options: HashMap<String, String>) {
    *DEFAULTS.lock().expect("defaults") = Some(options);
}

pub fn snapshot() -> Option<HashMap<String, String>> {
    DEFAULTS.lock().expect("defaults").clone()
}

pub fn clear() {
    *DEFAULTS.lock().expect("defaults") = None;
}
