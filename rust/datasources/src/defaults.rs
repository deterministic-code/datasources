use crate::result::Result;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

type Install = fn(&HashMap<String, String>) -> Result<()>;

static DEFAULTS: Mutex<Option<HashMap<String, String>>> = Mutex::new(None);
static INSTALL: OnceLock<Install> = OnceLock::new();

const ENV_KEYS: [&str; 6] = [
    "INSTALLED_PROVIDERS",
    "providers",
    "DATABASE_URL",
    "url",
    "DATABASE_SCHEMA",
    "provider",
];

pub fn bind_install(install: Install) {
    let _ = INSTALL.set(install);
}

pub fn defaults(options: HashMap<String, String>) -> Result<()> {
    if let Some(install) = INSTALL.get() {
        install(&options)?;
    }
    *DEFAULTS.lock().expect("defaults") = Some(options);
    Ok(())
}

pub fn from_env() -> HashMap<String, String> {
    ENV_KEYS
        .into_iter()
        .filter_map(|key| {
            std::env::var(key)
                .ok()
                .filter(|value| !value.trim().is_empty())
                .map(|value| (key.to_string(), value))
        })
        .collect()
}

pub fn register_listed(options: &HashMap<String, String>) -> Result<()> {
    match INSTALL.get() {
        Some(install) => install(options),
        None => Ok(()),
    }
}

pub fn apply_env() -> Result<()> {
    if snapshot().is_some() {
        return Ok(());
    }
    let env = from_env();
    if env.is_empty() {
        return Ok(());
    }
    defaults(env)
}

pub fn snapshot() -> Option<HashMap<String, String>> {
    DEFAULTS.lock().expect("defaults").clone()
}

pub fn clear() {
    *DEFAULTS.lock().expect("defaults") = None;
}
