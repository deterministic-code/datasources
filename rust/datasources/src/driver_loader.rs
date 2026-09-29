use crate::datasource_error::DatasourceError;
use crate::driver::Driver;
use crate::loaded_driver::LoadedDriver;
use crate::provider_catalog::ProviderCatalog;
use crate::result::Result;
use crate::sabi_driver::SabiDriverBox;
use libloading::Library;
use std::path::PathBuf;
use std::sync::Arc;

pub struct DriverLoader;

impl DriverLoader {
    pub fn load(catalog: &ProviderCatalog) -> Result<Arc<dyn Driver>> {
        let path = find(&catalog.location).ok_or_else(|| DatasourceError::not_installed(&catalog.name))?;
        let lib = unsafe { Library::new(&path) }
            .map_err(|_| DatasourceError::not_installed(&catalog.name))?;
        let symbol = format!("{}\0", catalog.module);
        let ctor = unsafe {
            lib.get::<extern "C" fn() -> SabiDriverBox>(symbol.as_bytes())
                .map_err(|_| DatasourceError::not_installed(&catalog.name))?
        };
        let sabi = ctor();
        Ok(Arc::new(LoadedDriver::new(sabi, lib)))
    }
}

fn find(location: &str) -> Option<PathBuf> {
    let file = lib_name(location);
    search_dirs().into_iter().find_map(|dir| {
        let path = dir.join(&file);
        path.is_file().then_some(path)
    })
}

fn search_dirs() -> Vec<PathBuf> {
    if let Some(dir) = std::env::var_os("DATASOURCES_PLUGIN_DIR") {
        return vec![PathBuf::from(dir)];
    }
    let mut dirs = Vec::new();
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            dirs.push(dir.to_path_buf());
            if let Some(parent) = dir.parent() {
                dirs.push(parent.to_path_buf());
            }
        }
    }
    dirs.push(PathBuf::from("plugins"));
    dirs
}

fn lib_name(location: &str) -> String {
    match std::env::consts::OS {
        "windows" => format!("{location}.dll"),
        "macos" => format!("lib{location}.dylib"),
        _ => format!("lib{location}.so"),
    }
}
