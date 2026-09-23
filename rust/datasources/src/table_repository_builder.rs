use crate::built_repository::BuiltRepository;
use crate::connection::Connection;
use crate::converters::{Cells, RowTypeConverter};
use crate::crud_repository_options::CRUDRepositoryOptions;
use crate::mappers::{Mapper, TransformMapper};
use crate::readonly_repository_options::ReadonlyRepositoryOptions;
use crate::result::Result;
use crate::row::Row;
use crate::table_repository_statements::RepositoryStatements;
use crate::value::Value;
use std::sync::Arc;

pub struct RepositoryBuilder;

impl RepositoryBuilder {
    pub fn parts(
        connection: Arc<dyn Connection>,
        options: ReadonlyRepositoryOptions,
    ) -> Result<BuiltRepository> {
        assemble(connection, options, Vec::new(), None, None, None, None)
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
            create,
            read,
        } = options;
        assemble(connection, read, ids, insert, update, delete, Some(create))
    }
}

fn assemble(
    connection: Arc<dyn Connection>,
    options: ReadonlyRepositoryOptions,
    ids: Vec<String>,
    insert: Option<Arc<TransformMapper<String, String>>>,
    update: Option<Arc<TransformMapper<String, String>>>,
    delete: Option<Arc<TransformMapper<String, String>>>,
    create: Option<TransformMapper<Value, Row>>,
) -> Result<BuiltRepository> {
    let ReadonlyRepositoryOptions {
        entity,
        entity_mapper,
        columns,
        query,
        rows_mapper,
        row_converter,
        row_type_converters,
    } = options;
    let row_converter = match row_converter {
        Some(mapping) => mapping,
        None => RowTypeConverter::of::<Cells>(),
    }
    .with_types(row_type_converters);
    let mut names: Vec<String> = row_converter
        .column_names()
        .iter()
        .map(|name| Mapper::map(&columns, name.clone()))
        .collect();
    names.sort();
    Ok(BuiltRepository {
        connection,
        row_converter,
        rows_mapper,
        statements: RepositoryStatements::with_mappers(
            Mapper::map(&entity_mapper, entity),
            names,
            query,
            insert,
            update,
            delete,
        ),
        ids,
        create,
    })
}
