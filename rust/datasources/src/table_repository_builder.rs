use crate::builders::StatementBuilder;
use crate::connection::Connection;
use crate::converters::{Cells, DefaultTypeConverters, RowTypeConverter};
use crate::mappers::NameSourceDestinationMapper;
use crate::parameter::Parameter;
use crate::result::Result;
use crate::statement::Statement;
use crate::table_repository::TableRepository;
use crate::table_repository_builder_options::TableRepositoryBuilderOptions;
use crate::where_trait::Where;
use std::sync::Arc;

pub struct TableRepositoryStatements {
    table: String,
    columns: Vec<String>,
}

impl TableRepositoryStatements {
    pub fn query(&self, where_clause: Option<&dyn Where>) -> (String, Vec<Parameter>) {
        let refs: Vec<&str> = self.columns.iter().map(String::as_str).collect();
        let built = StatementBuilder::create_query(&self.table, &refs, where_clause);
        (
            built.get_text(),
            where_clause
                .map(|predicate| predicate.parameters())
                .unwrap_or_else(|| built.parameters()),
        )
    }

    pub fn insert(&self, fields: &[Parameter]) -> String {
        StatementBuilder::create_insert(&self.table, fields).get_text()
    }

    pub fn update(&self, fields: &[Parameter], where_clause: &dyn Where) -> String {
        StatementBuilder::create_update(&self.table, fields, Some(where_clause)).get_text()
    }

    pub fn delete(&self, where_clause: &dyn Where) -> String {
        StatementBuilder::create_delete(&self.table, Some(where_clause)).get_text()
    }
}

pub struct TableRepositoryBuilder;

impl TableRepositoryBuilder {
    pub fn parts(options: TableRepositoryBuilderOptions) -> Result<(
        Arc<dyn Connection>,
        crate::converters::RowTypeConverter,
        NameSourceDestinationMapper,
        TableRepositoryStatements,
        Vec<String>,
        bool,
    )> {
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
        let statements = TableRepositoryStatements {
            table: options.table,
            columns,
        };
        Ok((
            options.connection,
            converters,
            mapping,
            statements,
            options.ids,
            options.assign_identity_on_add,
        ))
    }

    pub fn build(options: TableRepositoryBuilderOptions) -> Result<TableRepository> {
        let (connection, converters, mapping, statements, ids, assign_identity_on_add) =
            Self::parts(options)?;
        Ok(TableRepository::from_parts(
            connection,
            converters,
            mapping,
            statements,
            ids,
            assign_identity_on_add,
        ))
    }
}
