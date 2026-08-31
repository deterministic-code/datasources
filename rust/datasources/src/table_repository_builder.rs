use crate::built_repository::BuiltRepository;
use crate::converters::{Cells, DefaultTypeConverters, RowTypeConverter};
use crate::mappers::NameSourceDestinationMapper;
use crate::result::Result;
use crate::table_repository::TableRepository;
use crate::table_repository_builder_options::TableRepositoryBuilderOptions;
use crate::table_repository_statements::TableRepositoryStatements;

pub struct TableRepositoryBuilder;

impl TableRepositoryBuilder {
    pub fn parts(options: TableRepositoryBuilderOptions) -> Result<BuiltRepository> {
        let converters = match options.mapping {
            Some(mapping) => mapping,
            None => RowTypeConverter::of::<Cells>(),
        }
        .with_types(DefaultTypeConverters::for_provider(
            options.connection.provider(),
        ))
        .with_types(options.types)
        .with_fields(options.fields);
        let mut columns = converters.column_names().to_vec();
        columns.sort();
        let mapping = NameSourceDestinationMapper::new(options.connection.provider(), "rust");
        Ok(BuiltRepository {
            connection: options.connection,
            converters,
            mapping,
            statements: TableRepositoryStatements::new(options.table, columns),
            ids: options.ids,
            assign_identity_on_add: options.assign_identity_on_add,
        })
    }

    pub fn build(options: TableRepositoryBuilderOptions) -> Result<TableRepository> {
        Ok(TableRepository::from_built(Self::parts(options)?))
    }
}
