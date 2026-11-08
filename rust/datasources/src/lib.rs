pub use async_trait::async_trait;
pub use futures_core::Stream;

mod array_reader;
mod association;
mod batch_command_i_bulk_write_strategy;
mod bind_repositories;
pub mod builders;
mod built_repository;
mod bulk_command;
mod bulk_crud_repository;
mod bulk_write_enrichment;
mod bulk_write_strategy;
mod cascade_delete_enrichment_strategy;
mod cascade_delete_update_enrichment_strategy;
mod command;
mod command_middleware;
mod command_middleware_host;
mod connection;
mod console_command_middleware;
pub mod converters;
mod create;
mod create_crud_repository_options;
mod create_enrichment_strategy;
mod create_readonly_repository_mapping_options;
mod crud_repository;
mod crud_repository_options;
mod datasource;
mod datasource_error;
mod datasource_url;
mod datasource_url_parser;
mod default_create_enrichment_strategy;
mod default_readonly_enrichment_strategy;
mod default_update_enrichment_strategy;
mod default_write_enrichment_strategy;
mod defaults;
mod delete_enrichment_strategy;
mod delete_nested;
mod dot_env;
mod downcast_connection;
mod driver;
mod enrichment;
mod enrichment_payload;
mod enrichment_property_type;
mod enrichment_via;
mod entity;
mod erased_driver;
mod expect_console_middleware;
mod first_result_reader;
mod hashset_optimized_readonly_enrichment_strategy;
mod i_repository;
mod i_unit_of_work;
mod id_key;
mod information;
mod isolation_level;
mod lazy_write_enrichment_strategy;
pub mod mappers;
pub mod mocks;
mod native;
mod nested_to_object_enrichment_strategy;
mod parameter;
mod parameter_direction;
mod parameters_to_map;
mod provider_catalog;
mod reader;
mod readonly_enrichment_strategy;
mod readonly_repository;
mod schema;
mod schema_default;
pub(crate) use readonly_repository::push_middleware;
mod crud_repository_builder;
mod crud_repository_statements;
mod readonly_repository_builder;
mod readonly_repository_mapping_options;
mod readonly_repository_statements;
mod register;
mod registered;
mod registry_state;
mod repository;
mod repository_access;
mod repository_builder_options_factory;
mod repository_factory;
mod repository_tree;
mod repository_tree_node;
mod reset;
mod response;
mod result;
mod row;
mod shared_items;
mod single_command_i_bulk_write_strategy;
mod split_statements;
mod statement_mappers;
mod statement_parser;
pub mod statements;
mod stored_enrichment;
mod stored_procedure_bulk_write_strategy;
pub mod stubs;
mod transaction;
mod unit_of_work_transaction;
mod update_enrichment_strategy;
mod value;
mod write_enrichment_strategy;
mod write_mode;

pub use array_reader::ArrayReader;
pub use association::Association;
pub use batch_command_i_bulk_write_strategy::BatchCommandIBulkWriteStrategy;
pub use builders::{Column, Join, JoinKind};
pub use built_repository::BuiltRepository;
pub use bulk_command::BulkCommand;
pub use bulk_crud_repository::BulkCRUDRepository;
pub use bulk_write_strategy::BulkWriteStrategy;
pub use cascade_delete_enrichment_strategy::CascadeDeleteEnrichmentStrategy;
pub use cascade_delete_update_enrichment_strategy::CascadeDeleteUpdateEnrichmentStrategy;
pub use chrono;
pub use command::Command;
pub use command_middleware::CommandMiddleware;
pub use command_middleware_host::CommandMiddlewareHost;
pub use connection::Connection;
pub use console_command_middleware::{format_elapsed, utc_iso_millis, ConsoleCommandMiddleware};
pub use converters::SchemaObject;
pub use converters::{
    Casing, ChangeCasingConverter, FieldSize, Mapping, MetaDataRowTypeConverter,
    PluralizerConverter, RegexConverter, ReplaceConverter, RowTypeConverter, SpecField,
    ToLowerCaseConverter, ToUpperCaseConverter, TypeConverter,
};
pub use create::{create, default};
pub use create_crud_repository_options::create_crud_repository_options;
pub use create_enrichment_strategy::CreateEnrichmentStrategy;
pub use create_readonly_repository_mapping_options::create_readonly_repository_mapping_options;
pub use crud_repository::CRUDRepository;
pub use crud_repository_builder::CRUDRepositoryBuilder;
pub use crud_repository_options::CRUDRepositoryOptions;
pub use crud_repository_statements::CRUDRepositoryStatements;
pub use ctor::{__support, ctor};
pub use datasource::{BuiltDatasource, Datasource};
pub use datasource_error::DatasourceError;
pub use datasource_url::DatasourceUrl;
pub use datasource_url_parser::DatasourceUrlParser;
pub use default_create_enrichment_strategy::DefaultCreateEnrichmentStrategy;
pub use default_readonly_enrichment_strategy::DefaultReadonlyEnrichmentStrategy;
pub use default_update_enrichment_strategy::DefaultUpdateEnrichmentStrategy;
pub use default_write_enrichment_strategy::DefaultWriteEnrichmentStrategy;
pub use default_write_enrichment_strategy::{decode_rows, encode_rows};
pub use defaults::defaults;
pub use delete_enrichment_strategy::DeleteEnrichmentStrategy;
pub use delete_nested::DeleteNested;
pub use downcast_connection::downcast_connection;
pub use driver::Driver;
pub use enrichment::Enrichment;
pub use enrichment_property_type::EnrichmentPropertyType;
pub use enrichment_via::EnrichmentVia;
pub use entity::Entity;
pub use expect_console_middleware::ExpectConsoleMiddleware;
pub use facet::{self, Facet};
pub use first_result_reader::FirstResultReader;
pub use hashset_optimized_readonly_enrichment_strategy::HashsetOptimizedReadonlyEnrichmentStrategy;
pub use i_repository::IRepository;
pub use i_unit_of_work::IUnitOfWork;
pub use id_key::IdKey;
pub use information::Information;
pub use isolation_level::IsolationLevel;
pub use lazy_write_enrichment_strategy::LazyWriteEnrichmentStrategy;
pub use mappers::{
    ChangeCasingMapper, DeterministicTypeMapper, DictionaryMapper, FlatToNestedMapper,
    FlatToNestedMapperTrait, IdentityMapper, MappedRow, MappedValue, Mapper, PluralizerMapper,
    RowTypeMapper, TransformConverterMapper, TransformMapper, TypeMap, TypeMapField, TypeMapGroup,
};
pub use mocks::{
    MockCommand, MockConnection, MockDatasource, MockDriver, MockReader, MockRepository,
    MockSelectStatement, MockTransaction, MockWhere,
};
pub use native::{n, Native};
pub use nested_to_object_enrichment_strategy::NestedToObjectEnrichmentStrategy;
pub use parameter::Parameter;
pub use parameter_direction::ParameterDirection;
pub use parameters_to_map::parameters_to_map;
pub use provider_catalog::ProviderCatalog;
pub use reader::Reader;
pub use readonly_enrichment_strategy::ReadonlyEnrichmentStrategy;
pub use readonly_repository::ReadonlyRepository;
pub use readonly_repository_builder::ReadonlyRepositoryBuilder;
pub use readonly_repository_mapping_options::ReadonlyRepositoryMappingOptions;
pub use readonly_repository_statements::ReadonlyRepositoryStatements;
pub use register::register;
pub use registered::registered;
pub use repository::{as_readonly, ICRUDRepository, IReadonlyRepository};
pub use repository_access::RepositoryAccess;
pub use repository_builder_options_factory::RepositoryBuilderOptionsFactory;
pub use repository_factory::{CRUDRepositoryFactory, ReadonlyRepositoryFactory, RepositoryFactory};
pub use repository_tree::RepositoryTree;
pub use repository_tree_node::RepositoryTreeNode;
pub use reset::reset;
pub use response::Response;
pub use result::Result;
pub use row::Row;
pub use rust_decimal;
pub use schema::{Schema, SchemaColumn};
pub use single_command_i_bulk_write_strategy::SingleCommandIBulkWriteStrategy;
pub use split_statements::{split_statements, statement_parameters, statement_uses_parameter};
pub use statement_mappers::StatementMappers;
pub use statement_parser::StatementParser;
pub use statements::statement;
pub use statements::{
    eq, ge, gt, le, lt, ne, procedure_name_of, sql_procedure_statements_of, Aggregate, AlterAction,
    AlterTableStatement, AlterTableStatementTextRenderer, ArrayExpression, BulkDeleteStatement,
    BulkDeleteStatementTextRenderer, BulkInsertStatement, BulkInsertStatementTextRenderer,
    BulkUpdateRow, BulkUpdateStatement, BulkUpdateStatementTextRenderer, CheckConstraint,
    ColumnDefault, Condition, ConditionRenderer, ConditionVisitor,
    CreateBulkCreateStoredProcedureStatement, CreateBulkCreateStoredProcedureStatementTextRenderer,
    CreateBulkDeleteStoredProcedureStatement, CreateBulkDeleteStoredProcedureStatementTextRenderer,
    CreateBulkUpdateStoredProcedureStatement, CreateBulkUpdateStoredProcedureStatementTextRenderer,
    CreateCreateStoredProcedureStatement, CreateCreateStoredProcedureStatementTextRenderer,
    CreateDeleteStoredProcedureStatement, CreateDeleteStoredProcedureStatementTextRenderer,
    CreateIndexStatement, CreateIndexStatementTextRenderer,
    CreateNestedReadStoredProcedureStatement, CreateNestedReadStoredProcedureStatementTextRenderer,
    CreateReadStoredProcedureStatement, CreateReadStoredProcedureStatementTextRenderer,
    CreateTableStatement, CreateTableStatementTextRenderer, CreateTriggerStatement,
    CreateTriggerStatementTextRenderer, CreateUpdateStoredProcedureStatement,
    CreateUpdateStoredProcedureStatementTextRenderer, DefaultStatementTextRenderer,
    DeleteStatement, DeleteStatementTextRenderer, DropIndexStatement,
    DropIndexStatementTextRenderer, DropRoutineStatement, DropRoutineStatementTextRenderer,
    DropTableStatement, DropTableStatementTextRenderer, DropTriggerStatement,
    DropTriggerStatementTextRenderer, ExecuteStoredProcedureStatement,
    ExecuteStoredProcedureStatementTextRenderer, Expression, ExpressionEvaluator,
    ExpressionRenderer, ForeignKey, ForeignKeyStatement, ICondition, ISelectStatement, IWhere,
    Identifier, IdentityInsertStatement, IdentityInsertStatementTextRenderer, InsertStatement,
    InsertStatementTextRenderer, KeywordCasing, Literal, NestedSelectBuilder,
    NestedSelectBuilderOptions, NestedSelectRelation, NestedSelectRelationKind,
    NestedSelectStatement, NestedSelectType, NestedWhereArgument, NotNull, Operator, OrderBy,
    PrimaryKey, ProcedureByGroup, ProcedureSource, ReferentialAction, ReturnStatement,
    ReturnStatementTextRenderer, RoutineParameter, SelectInsertedIdStatement,
    SelectInsertedIdStatementTextRenderer, SelectStatement, SelectStatementOptions,
    SelectStatementTextRenderer, SortDirection, SqlSeedColumn, SqlSeedStatement, Statement,
    StatementEvaluator, StatementKind, StatementNaming, StatementRenderer, StatementTextRenderer,
    StatementVisitor, TableConstraint, UniqueConstraint, UpdateStatement,
    UpdateStatementTextRenderer, Where, WhereText,
};
pub use stored_procedure_bulk_write_strategy::StoredProcedureBulkWriteStrategy;
pub use stubs::{
    create_stub_datasource, StubCommand, StubConnection, StubDatasource, StubReader,
    StubRepository, StubTransaction,
};
pub use transaction::Transaction;
pub use unit_of_work_transaction::UnitOfWorkTransaction;
pub use update_enrichment_strategy::UpdateEnrichmentStrategy;
pub use uuid;
pub use value::Value;
pub use write_enrichment_strategy::WriteEnrichmentStrategy;
pub use write_mode::WriteMode;

#[cfg(test)]
mod tests;
