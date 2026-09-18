use crate::built_repository::BuiltRepository;
use crate::converters::{Cells, DefaultTypeConverters, RowTypeConverter};
use crate::mappers::{Mapper, StatementMapper, TransformMapper};
use crate::result::Result;
use crate::table_repository::Repository;
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
        .with_types(DefaultTypeConverters::for_provider(
            options.connection.provider(),
        ))
        .with_types(options.types)
        .with_fields(options.fields);
        let statement_mapper = match (options.statement_mapper, options.mapper) {
            (Some(mapper), _) => mapper,
            (None, rows) => {
                let mut mapper = StatementMapper::identity(options.connection.provider(), "rust");
                mapper.rows = rows.unwrap_or_else(|| TransformMapper::new(|row| row, |row| row));
                mapper
            }
        };
        let mut columns: Vec<String> = row_converter
            .column_names()
            .iter()
            .map(|name| Mapper::map(&statement_mapper.columns, name.clone()))
            .collect();
        columns.sort();
        let entity = Mapper::map(&statement_mapper.entity, options.entity);
        let statement_mapper = Arc::new(statement_mapper);
        Ok(BuiltRepository {
            connection: options.connection,
            row_converter,
            statement_mapper: statement_mapper.clone(),
            statements: RepositoryStatements::with_mapper(entity, columns, statement_mapper),
            ids: options.ids,
            assign_identity_on_create: options.assign_identity_on_create,
        })
    }

    pub fn build(options: RepositoryBuilderOptions) -> Result<Repository> {
        Ok(Repository::from_built(Self::parts(options)?))
    }
}
