use crate::connection::Connection;
use crate::converters::ConvertSchema;
use crate::table_repository_builder_options::RepositoryBuilderOptions;
use std::sync::Arc;

pub fn create_repository_builder_options<T: ConvertSchema>(
    connection: Arc<dyn Connection>,
) -> RepositoryBuilderOptions {
    RepositoryBuilderOptions {
        ids: default_id::<T>(),
        assign_identity_on_create: true,
        entity: std::any::type_name::<T>()
            .rsplit("::")
            .next()
            .unwrap_or_default()
            .into(),
        mapping: None,
        mapper: None,
        statement_mapper: None,
        types: Default::default(),
        fields: Default::default(),
        connection,
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
