use crate::builders::StatementBuilder;
use crate::command::Command;
use crate::converters::{mapping_with, table, Cells};
use crate::mappers::StringSourceDestinationMapper;
use crate::parameter::Parameter;
use crate::result::Result;
use crate::statement::Statement;
use crate::table_repository::TableRepository;
use crate::table_repository_builder_options::TableRepositoryBuilderOptions;
use crate::where_trait::Where;
use std::sync::Arc;

pub struct TableRepositoryCommands {
    pub get: Arc<dyn Command>,
    pub add: Arc<dyn Command>,
    pub update: Arc<dyn Command>,
    pub delete: Arc<dyn Command>,
}

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
        TableRepositoryCommands,
        crate::converters::TypeMapping,
        StringSourceDestinationMapper,
        TableRepositoryStatements,
        Vec<String>,
        bool,
    )> {
        let converters = match options.mapping {
            Some(mapping) => mapping,
            None => mapping_with::<Cells>(options.connection.provider(), options.converters)?,
        };
        table(options.connection.provider())?;
        let mut columns = converters.field_names();
        columns.sort();
        let mapping = StringSourceDestinationMapper::new(options.connection.provider(), "rust");
        let statements = TableRepositoryStatements {
            table: converters.name.clone(),
            columns,
        };
        let commands = TableRepositoryCommands {
            get: options.connection.create_command(),
            add: options.connection.create_command(),
            update: options.connection.create_command(),
            delete: options.connection.create_command(),
        };
        Ok((
            commands,
            converters,
            mapping,
            statements,
            options.ids,
            options.assign_identity_on_add,
        ))
    }

    pub fn build(options: TableRepositoryBuilderOptions) -> Result<TableRepository> {
        let (commands, converters, mapping, statements, ids, assign_identity_on_add) =
            Self::parts(options)?;
        Ok(TableRepository::from_parts(
            commands,
            converters,
            mapping,
            statements,
            ids,
            assign_identity_on_add,
        ))
    }
}
