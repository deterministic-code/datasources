use crate::built_repository::BuiltRepository;
use crate::connection::Connection;
use crate::converters::{Cells, RowTypeConverter};
use crate::crud_mapper_options::CRUDMapperOptions;
use crate::mappers::Mapper;
use crate::repository_options::RepositoryOptions;
use crate::result::Result;
use crate::table_repository_statements::RepositoryStatements;
use std::sync::Arc;

pub struct RepositoryBuilder;

impl RepositoryBuilder {
    pub fn parts(
        connection: Arc<dyn Connection>,
        options: RepositoryOptions,
    ) -> Result<BuiltRepository> {
        let row_converter = match options.row_converter {
            Some(mapping) => mapping,
            None => RowTypeConverter::of::<Cells>(),
        }
        .with_types(options.row_type_converters);
        let crud_mapper = options
            .crud_mapper
            .unwrap_or_else(|| CRUDMapperOptions::identity(connection.provider(), "rust"));
        let mut columns: Vec<String> = row_converter
            .column_names()
            .iter()
            .map(|name| Mapper::map(&crud_mapper.columns, name.clone()))
            .collect();
        columns.sort();
        let entity = Mapper::map(&crud_mapper.entity, options.entity);
        let crud_mapper = Arc::new(crud_mapper);
        Ok(BuiltRepository {
            connection,
            row_converter,
            crud_mapper: crud_mapper.clone(),
            statements: RepositoryStatements::with_mapper(entity, columns, crud_mapper),
            ids: options.ids,
        })
    }
}
