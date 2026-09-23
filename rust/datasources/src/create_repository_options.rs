use crate::converters::ConvertSchema;
use crate::repository_options::RepositoryOptions;

pub fn create_repository_options<T: ConvertSchema>() -> RepositoryOptions {
    RepositoryOptions {
        ids: default_id::<T>(),
        entity: std::any::type_name::<T>()
            .rsplit("::")
            .next()
            .unwrap_or_default()
            .into(),
        ..Default::default()
    }
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
