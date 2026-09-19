use crate::result::Result;
use std::collections::HashMap;
use std::sync::OnceLock;

type Install = fn(&HashMap<String, String>) -> Result<()>;

static INSTALL: OnceLock<Install> = OnceLock::new();

pub fn bind_install(install: Install) {
    let _ = INSTALL.set(install);
}

pub fn defaults(options: HashMap<String, String>) -> Result<()> {
    register_listed(&options)
}

pub fn register_listed(options: &HashMap<String, String>) -> Result<()> {
    match INSTALL.get() {
        Some(install) => install(options),
        None => Ok(()),
    }
}
