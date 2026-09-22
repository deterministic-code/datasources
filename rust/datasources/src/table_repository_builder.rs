use crate::built_repository::BuiltRepository;
use crate::converters::{Cells, RowTypeConverter};
use crate::crud_mapper::CRUDMapper;
use crate::mappers::{Mapper, TransformMapper};
use crate::result::Result;
use crate::table_repository_builder_options::RepositoryBuilderOptions;
use crate::table_repository_statements::RepositoryStatements;
use std::sync::Arc;

pub struct RepositoryBuilder;

impl RepositoryBuilder {
    pub fn parts(options: RepositoryBuilderOptions) -> Result<BuiltRepository> {
        let row_converter = match options.mapping {
            Some(mapping) => mapping,
            None => RowTypeConverter::of::<Cells>(),
        }
        .with_types(options.types)
        .with_fields(options.fields);
        let crud_mapper = match (options.crud_mapper, options.mapper) {
            (Some(mapper), _) => mapper,
            (None, rows) => {
                let mut mapper = CRUDMapper::identity(options.connection.provider(), "rust");
                mapper.rows = rows.unwrap_or_else(|| TransformMapper::new(|row| row, |row| row));
                mapper
            }
        };
        let mut columns: Vec<String> = row_converter
            .column_names()
            .iter()
            .map(|name| Mapper::map(&crud_mapper.columns, name.clone()))
            .collect();
        columns.sort();
        let entity = Mapper::map(&crud_mapper.entity, options.entity);
        let crud_mapper = Arc::new(crud_mapper);
        Ok(BuiltRepository {
            connection: options.connection,
            row_converter,
            crud_mapper: crud_mapper.clone(),
            statements: RepositoryStatements::with_mapper(entity, columns, crud_mapper),
            ids: options.ids,
        })
    }

}
