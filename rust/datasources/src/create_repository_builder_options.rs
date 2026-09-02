use crate::connection::Connection;
use crate::converters::ConvertSchema;
use crate::table_repository_builder_options::TableRepositoryBuilderOptions;
use std::sync::Arc;

pub fn create_repository_builder_options<T: ConvertSchema>(
    connection: Arc<dyn Connection>,
) -> TableRepositoryBuilderOptions {
    TableRepositoryBuilderOptions {
        ids: default_id::<T>(),
        assign_identity_on_add: true,
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
