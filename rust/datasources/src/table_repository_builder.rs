use crate::built_repository::BuiltRepository;
use crate::connection::Connection;
use crate::converters::MetaDataRowTypeConverter;
use crate::crud_repository_options::CRUDRepositoryOptions;
use crate::mappers::{Mapper, TransformMapper};
use crate::readonly_repository_options::ReadonlyRepositoryOptions;
use crate::result::Result;
use crate::table_repository_statements::RepositoryStatements;
use std::sync::Arc;

pub struct RepositoryBuilder;

impl RepositoryBuilder {
    pub fn parts(
        connection: Arc<dyn Connection>,
        options: ReadonlyRepositoryOptions,
    ) -> Result<BuiltRepository> {
        assemble(connection, options, Vec::new(), None, None, None)
    }

    pub(crate) fn parts_crud(
        connection: Arc<dyn Connection>,
        options: CRUDRepositoryOptions,
    ) -> Result<BuiltRepository> {
        let CRUDRepositoryOptions {
            ids,
            insert,
            update,
            delete,
            return_: _,
            write,
            read,
        } = options;
        let mut built = assemble(connection, read, ids, insert, update, delete)?;
        built.write = write;
        Ok(built)
    }
}

fn assemble(
    connection: Arc<dyn Connection>,
    options: ReadonlyRepositoryOptions,
    ids: Vec<String>,
    insert: Option<Arc<TransformMapper<String, String>>>,
    update: Option<Arc<TransformMapper<String, String>>>,
    delete: Option<Arc<TransformMapper<String, String>>>,
) -> Result<BuiltRepository> {
    let ReadonlyRepositoryOptions {
        entity,
        entity_mapper,
        column_mappings,
        select,
        rows_mapper,
        row_converter,
        object_converter,
        row_type_converters,
    } = options;
    let row_converter = row_converter.map(|mapping| mapping.with_types(row_type_converters));
    let object_converter =
        object_converter.unwrap_or_else(|| MetaDataRowTypeConverter::new(Vec::new()));
    let mut names: Vec<String> = row_converter
        .as_ref()
        .map(|converter| {
            converter
                .column_names()
                .iter()
                .map(|name| Mapper::map(&column_mappings, name.clone()))
                .collect()
        })
        .unwrap_or_default();
    names.sort();
    Ok(BuiltRepository {
        connection,
        row_converter,
        object_converter,
        rows_mapper,
        column_mappings,
        statements: RepositoryStatements::with_mappers(
            Mapper::map(&entity_mapper, entity),
            names,
            select,
            insert,
            update,
            delete,
        ),
        ids,
        write: None,
    })
}
