use crate::create_readonly_repository_mapping_options::create_readonly_repository_mapping_options;
use crate::crud_repository_options::CRUDRepositoryOptions;

pub fn create_crud_repository_options<T>() -> CRUDRepositoryOptions {
    let mut options = CRUDRepositoryOptions::default();
    options.read = create_readonly_repository_mapping_options::<T>();
    options.ids = Vec::new();
    options
}
