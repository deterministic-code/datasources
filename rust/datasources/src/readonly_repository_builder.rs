use crate::built_repository::BuiltRepository;
use crate::connection::Connection;
use crate::converters::MetaDataRowTypeConverter;
use crate::mappers::Mapper;
use crate::readonly_repository_options::ReadonlyRepositoryOptions;
use crate::readonly_repository_statements::ReadonlyRepositoryStatements;
use crate::result::Result;
use crate::statement_mappings::StatementMappings;
use std::sync::Arc;

pub struct ReadonlyRepositoryBuilder;

impl ReadonlyRepositoryBuilder {
    pub fn parts(
        connection: Arc<dyn Connection>,
        options: ReadonlyRepositoryOptions,
    ) -> Result<BuiltRepository> {
        assemble(connection, options, Vec::new())
    }
}

pub(crate) fn assemble(
    connection: Arc<dyn Connection>,
    options: ReadonlyRepositoryOptions,
    ids: Vec<String>,
) -> Result<BuiltRepository> {
    let ReadonlyRepositoryOptions {
        entity,
        entity_mapper,
        property_mappings,
        row_converter,
        object_converter,
        row_type_converters,
    } = options;
    let row_converter = row_converter.map(|mapping| mapping.with_types(row_type_converters));
    let object_converter =
        object_converter.unwrap_or_else(|| Box::new(MetaDataRowTypeConverter::new(Vec::new())));
    let entity_mapper: Arc<dyn Mapper<String, String> + Send + Sync> = Arc::from(entity_mapper);
    let property_mappings: Arc<dyn Mapper<String, String> + Send + Sync> =
        Arc::from(property_mappings);
    let table = Mapper::map(&entity_mapper, entity);
    let mut names: Vec<String> = row_converter
        .as_ref()
        .map(|converter| {
            converter
                .column_names()
                .iter()
                .map(|name| Mapper::map(&property_mappings, name.clone()))
                .collect()
        })
        .unwrap_or_default();
    names.sort();
    let mappings = StatementMappings::new(Arc::clone(&entity_mapper), Arc::clone(&property_mappings));
    Ok(BuiltRepository {
        connection,
        row_converter,
        object_converter,
        property_mappings,
        entity_mapper,
        statements: ReadonlyRepositoryStatements::new(table, names, mappings),
        ids,
    })
}
