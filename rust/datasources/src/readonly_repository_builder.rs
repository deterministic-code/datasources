use crate::built_repository::BuiltRepository;
use crate::connection::Connection;
use crate::converters::MetaDataRowTypeConverter;
use crate::mappers::{IdentityMapper, Mapper};
use crate::readonly_repository_mapping_options::ReadonlyRepositoryMappingOptions;
use crate::readonly_repository_statements::ReadonlyRepositoryStatements;
use crate::result::Result;
use crate::statement_mappers::StatementMappers;
use std::sync::Arc;

pub struct ReadonlyRepositoryBuilder;

impl ReadonlyRepositoryBuilder {
    pub fn parts(
        connection: Arc<dyn Connection>,
        options: ReadonlyRepositoryMappingOptions,
    ) -> Result<BuiltRepository> {
        assemble(connection, options, Vec::new())
    }
}

pub(crate) fn assemble(
    connection: Arc<dyn Connection>,
    options: ReadonlyRepositoryMappingOptions,
    ids: Vec<String>,
) -> Result<BuiltRepository> {
    let ReadonlyRepositoryMappingOptions {
        entity,
        entity_mapper,
        property_mappings,
        read_properties,
        row_converter,
        object_converter,
        row_type_converters,
    } = options;
    let row_converter = row_converter.map(|mapping| mapping.with_types(row_type_converters));
    let object_converter =
        object_converter.unwrap_or_else(|| Box::new(MetaDataRowTypeConverter::new(Vec::new())));
    let entity_mapper: Arc<dyn Mapper<String, String> + Send + Sync> =
        entity_mapper.unwrap_or_else(|| Arc::new(IdentityMapper::new()));
    let property_mappings: Arc<dyn Mapper<String, String> + Send + Sync> =
        property_mappings.unwrap_or_else(|| Arc::new(IdentityMapper::new()));
    let table = entity;
    let mut names: Vec<String> = if read_properties.is_empty() {
        row_converter
            .as_ref()
            .map(|converter| converter.column_names().to_vec())
            .unwrap_or_default()
    } else {
        read_properties
    };
    names.sort_by(|left, right| {
        Mapper::map(&property_mappings, left.clone()).cmp(&Mapper::map(&property_mappings, right.clone()))
    });
    let mappings = StatementMappers::new(Arc::clone(&entity_mapper), Arc::clone(&property_mappings));
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
