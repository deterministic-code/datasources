use crate::command::Command;
use crate::datasource_error::DatasourceError;
use crate::connection::Connection;
use crate::converters::{ConvertSchema, RowTypeConverter, TypeConverter};
use crate::create_crud_repository_options::create_crud_repository_options;
use crate::crud_repository::CRUDRepository;
use crate::crud_repository_options::CRUDRepositoryOptions;
use crate::driver::Driver;
use crate::readonly_repository::ReadonlyRepository;
use crate::readonly_repository_options::ReadonlyRepositoryOptions;
use crate::repository::{ICRUDRepository, IReadonlyRepository};
use crate::result::Result;
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
    fn row_converter(&self, columns: &HashMap<String, String>) -> RowTypeConverter {
        RowTypeConverter::from(|| self.row_type_converters(), columns)
    }
    fn repository_from(
        &self,
        connection: Arc<dyn Connection>,
        options: CRUDRepositoryOptions,
    ) -> Arc<dyn ICRUDRepository>;
    fn readonly_repository_from(
        &self,
        connection: Arc<dyn Connection>,
        options: ReadonlyRepositoryOptions,
    ) -> Arc<dyn IReadonlyRepository>;
}

impl dyn Datasource {
    pub fn create_repository<T: ConvertSchema + 'static>(
        &self,
        connection: Option<Arc<dyn Connection>>,
        options: Option<CRUDRepositoryOptions>,
    ) -> Arc<dyn ICRUDRepository> {
        let bound = self.bind_connection(connection);
        self.repository_from(bound, repository_options::<T>(self, options))
    }

    pub fn create_readonly_repository<T: ConvertSchema + 'static>(
        &self,
        connection: Option<Arc<dyn Connection>>,
        options: Option<ReadonlyRepositoryOptions>,
    ) -> Arc<dyn IReadonlyRepository> {
        let custom = options.map(|read| CRUDRepositoryOptions {
            read,
            ..create_crud_repository_options::<T>()
        });
        let bound = self.bind_connection(connection);
        self.readonly_repository_from(bound, repository_options::<T>(self, custom).read)
    }
}

fn repository_options<T: ConvertSchema + 'static>(
    datasource: &dyn Datasource,
    custom: Option<CRUDRepositoryOptions>,
) -> CRUDRepositoryOptions {
    let built = create_crud_repository_options::<T>();
    let types = datasource.row_type_converters();
    let columns = T::schema()
        .into_iter()
        .map(|(name, ty)| (name.to_string(), ty.to_string()))
        .collect();
    let rows = datasource.row_converter(&columns);
    match custom {
        None => {
            let mut options = built;
            options.row_converter = Some(rows);
            options.row_type_converters = types;
            options
        }
        Some(mut crud) => {
            if crud.entity.is_empty() {
                crud.entity = built.entity.clone();
            }
            if crud.row_converter.is_none() {
                crud.row_converter = Some(rows);
            }
            crud.row_type_converters = types;
            crud
        }
    }
}

pub struct BuiltDatasource {
    provider: String,
    driver: Arc<dyn Driver>,
    options: Option<HashMap<String, String>>,
    connection: Mutex<Option<Arc<dyn Connection>>>,
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
        Arc::new(CRUDRepository::new(connection, options).expect("repository"))
    }

    fn readonly_repository_from(
        &self,
        connection: Arc<dyn Connection>,
        options: ReadonlyRepositoryOptions,
    ) -> Arc<dyn IReadonlyRepository> {
        Arc::new(ReadonlyRepository::new(connection, options).expect("repository"))
    }
}
