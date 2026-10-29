use crate::command::Command;
use crate::datasource_error::DatasourceError;
use crate::connection::Connection;
use crate::converters::{RowTypeConverter, TypeConverter};
use crate::from_schema::FromSchema;
use crate::create_crud_repository_options::create_crud_repository_options;
use crate::create_readonly_repository_mapping_options::create_readonly_repository_mapping_options;
use crate::crud_repository::CRUDRepository;
use crate::crud_repository_options::CRUDRepositoryOptions;
use crate::driver::Driver;
use crate::readonly_repository::ReadonlyRepository;
use crate::readonly_repository_mapping_options::ReadonlyRepositoryMappingOptions;
use crate::repository::{ICRUDRepository, IReadonlyRepository};
use crate::repository_access::RepositoryAccess;
use crate::repository_factory::RepositoryFactory;
use crate::result::Result;
use crate::row::Row;
use crate::value::Value;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[async_trait]
pub trait Datasource: Send + Sync {
    fn provider(&self) -> &str;
    async fn create_connection(&self) -> Result<Arc<dyn Connection>>;
    async fn create_command(
        &self,
        connection: Option<Arc<dyn Connection>>,
    ) -> Result<Arc<dyn Command>>;
    fn bind_connection(&self, connection: Option<Arc<dyn Connection>>) -> Arc<dyn Connection>;
    fn row_type_converters(
        &self,
    ) -> HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>;
    fn row_converter(&self, schema: &crate::schema::Schema) -> RowTypeConverter {
        RowTypeConverter::from(|| self.row_type_converters(), &schema.columns)
    }
    fn repository_from(
        &self,
        connection: Arc<dyn Connection>,
        options: CRUDRepositoryOptions,
    ) -> Arc<dyn ICRUDRepository<Row>>;
    fn readonly_repository_from(
        &self,
        connection: Arc<dyn Connection>,
        options: ReadonlyRepositoryMappingOptions,
    ) -> Arc<dyn IReadonlyRepository<Row>>;
    fn repositories(&self) -> &RepositoryFactory;
    async fn close(&self) -> Result<()>;
}

impl dyn Datasource {
    pub fn create_repository<T: 'static>(
        &self,
        connection: Option<Arc<dyn Connection>>,
        options: Option<CRUDRepositoryOptions>,
    ) -> Arc<dyn ICRUDRepository<Row>> {
        let bound = self.bind_connection(connection);
        self.repository_from(bound, apply_crud::<T>(self, options))
    }

    pub fn create_typed_repository<T: FromSchema + 'static>(
        &self,
        connection: Option<Arc<dyn Connection>>,
        options: Option<CRUDRepositoryOptions>,
    ) -> Arc<dyn ICRUDRepository<T>> {
        let bound = self.bind_connection(connection);
        let options = options.unwrap_or_else(create_crud_repository_options::<T>);
        Arc::new(
            CRUDRepository::<T>::new(bound, apply_crud::<T>(self, Some(options))).expect("repository"),
        )
    }

    pub fn create_readonly_repository<T: 'static>(
        &self,
        connection: Option<Arc<dyn Connection>>,
        options: Option<ReadonlyRepositoryMappingOptions>,
    ) -> Arc<dyn IReadonlyRepository<Row>> {
        let bound = self.bind_connection(connection);
        self.readonly_repository_from(bound, apply_readonly::<T>(self, options))
    }

    pub fn create_repository_by_name<T: FromSchema>(
        &self,
        name: &str,
        connection: Option<Arc<dyn Connection>>,
    ) -> Result<Arc<dyn ICRUDRepository<T>>> {
        self.repositories()
            .create_named(name, self.bind_connection(connection))
    }

    pub fn create_readonly_repository_by_name<T: FromSchema>(
        &self,
        name: &str,
        connection: Option<Arc<dyn Connection>>,
    ) -> Result<Arc<dyn IReadonlyRepository<T>>> {
        self.repositories()
            .create_readonly_named(name, self.bind_connection(connection))
    }

    pub fn access(&self, name: &str, connection: Option<Arc<dyn Connection>>) -> RepositoryAccess {
        self.repositories()
            .access(name, self.bind_connection(connection))
    }
}

fn apply_crud<T: 'static>(
    datasource: &dyn Datasource,
    custom: Option<CRUDRepositoryOptions>,
) -> CRUDRepositoryOptions {
    let built = create_crud_repository_options::<T>();
    let types = datasource.row_type_converters();
    let entity = built.entity.clone();
    let mut options = custom.unwrap_or(built);
    if options.entity.is_empty() {
        options.entity = entity;
    }
    if options.row_converter.is_none() {
        options.row_converter = Some(datasource.row_converter(&options.schema));
    }
    options.row_type_converters = types;
    options
}

fn apply_readonly<T: 'static>(
    datasource: &dyn Datasource,
    custom: Option<ReadonlyRepositoryMappingOptions>,
) -> ReadonlyRepositoryMappingOptions {
    let built = create_readonly_repository_mapping_options::<T>();
    let types = datasource.row_type_converters();
    let entity = built.entity.clone();
    let mut options = custom.unwrap_or(built);
    if options.entity.is_empty() {
        options.entity = entity;
    }
    if options.row_converter.is_none() {
        options.row_converter = Some(datasource.row_converter(&options.schema));
    }
    options.row_type_converters = types;
    options
}

pub struct BuiltDatasource {
    provider: String,
    driver: Arc<dyn Driver>,
    options: Option<HashMap<String, String>>,
    connection: Mutex<Option<Arc<dyn Connection>>>,
    repositories: RepositoryFactory,
}

impl BuiltDatasource {
    pub fn new(
        provider: impl Into<String>,
        driver: Arc<dyn Driver>,
        options: Option<HashMap<String, String>>,
    ) -> Self {
        Self {
            provider: provider.into(),
            driver,
            options,
            connection: Mutex::new(None),
            repositories: RepositoryFactory::new(),
        }
    }

    fn default_connection(&self) -> Arc<dyn Connection> {
        let mut slot = self.connection.lock().expect("connection");
        slot.get_or_insert_with(|| self.driver.connection(self.options.as_ref()))
            .clone()
    }
}

#[async_trait]
impl Datasource for BuiltDatasource {
    fn provider(&self) -> &str {
        &self.provider
    }

    async fn create_connection(&self) -> Result<Arc<dyn Connection>> {
        Ok(self.default_connection())
    }

    async fn create_command(
        &self,
        connection: Option<Arc<dyn Connection>>,
    ) -> Result<Arc<dyn Command>> {
        match connection {
            Some(connection) => Ok(self.driver.command(connection)),
            None => {
                let connection = self.default_connection();
                if connection.is_open() {
                    return Err(DatasourceError::message("Connection is already open"));
                }
                connection.open().await?;
                Ok(self.driver.command(connection))
            }
        }
    }

    fn bind_connection(&self, connection: Option<Arc<dyn Connection>>) -> Arc<dyn Connection> {
        connection.unwrap_or_else(|| self.default_connection())
    }

    fn row_type_converters(
        &self,
    ) -> HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>> {
        self.driver.row_type_converters()
    }

    fn repository_from(
        &self,
        connection: Arc<dyn Connection>,
        options: CRUDRepositoryOptions,
    ) -> Arc<dyn ICRUDRepository> {
        Arc::new(CRUDRepository::<Row>::new(connection, options).expect("repository"))
    }

    fn readonly_repository_from(
        &self,
        connection: Arc<dyn Connection>,
        options: ReadonlyRepositoryMappingOptions,
    ) -> Arc<dyn IReadonlyRepository> {
        Arc::new(ReadonlyRepository::<Row>::new(connection, options).expect("repository"))
    }

    fn repositories(&self) -> &RepositoryFactory {
        &self.repositories
    }

    async fn close(&self) -> Result<()> {
        let connection = self.connection.lock().expect("connection").take();
        match connection {
            Some(connection) => connection.close().await,
            None => Ok(()),
        }
    }
}
