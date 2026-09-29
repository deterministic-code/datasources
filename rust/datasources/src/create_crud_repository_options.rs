use crate::converters::ConvertSchema;
use crate::crud_repository_options::CRUDRepositoryOptions;

pub fn create_crud_repository_options<T: ConvertSchema>() -> CRUDRepositoryOptions {
    let mut options = CRUDRepositoryOptions::default();
    options.ids = default_id::<T>();
    options.entity = std::any::type_name::<T>()
        .rsplit("::")
        .next()
        .unwrap_or_default()
        .into();
    options
}

fn default_id<T: ConvertSchema>() -> Vec<String> {
    let names: Vec<_> = T::schema().into_iter().map(|(name, _)| name).collect();
    if names.contains(&"id") {
        vec!["id".into()]
    } else if names.contains(&"Id") {
        vec!["Id".into()]
    } else {
        vec![]
    }
}
