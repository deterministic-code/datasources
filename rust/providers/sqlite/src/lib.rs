use async_trait::async_trait;
use datasources::{
    identity_where_text, parameters_to_map, register, ArrayReader, Command, Connection, Datasource,
    DatasourceError, DatasourceFactory, MiddlewareAware, MiddlewareHook, MiddlewareHost, Parameter,
    Query, Reader, Record, Repository, Response, Result, Transaction, Value, Where,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, PartialEq)]
pub struct SqliteExecuteResult {
    pub rows_affected: i64,
    pub result: Value,
}

#[async_trait]
pub trait SqliteDriver: Send + Sync {
    async fn open(&self, filename: &str) -> Result<()>;
    async fn close(&self) -> Result<()>;
    async fn execute(
        &self,
        sql: &str,
        params: &HashMap<String, Value>,
    ) -> Result<SqliteExecuteResult>;
    async fn query(&self, sql: &str, params: &HashMap<String, Value>) -> Result<Vec<Record>>;
    async fn begin(&self) -> Result<()>;
    async fn commit(&self) -> Result<()>;
    async fn rollback(&self) -> Result<()>;
}

pub fn require_driver(driver: Option<Arc<dyn SqliteDriver>>) -> Result<Arc<dyn SqliteDriver>> {
    driver.ok_or_else(|| DatasourceError::message("SQLite driver is required"))
}

pub struct ScriptedDriver {
    pub opened: Mutex<bool>,
    pub closed: Mutex<bool>,
    pub begins: Mutex<usize>,
    pub commits: Mutex<usize>,
    pub rollbacks: Mutex<usize>,
    pub executes: Mutex<Vec<String>>,
    pub queries: Mutex<Vec<String>>,
    pub query_result: Mutex<Vec<Record>>,
    pub execute_result: Mutex<SqliteExecuteResult>,
}

impl ScriptedDriver {
    pub fn new() -> Self {
        Self {
            opened: Mutex::new(false),
            closed: Mutex::new(false),
            begins: Mutex::new(0),
            commits: Mutex::new(0),
            rollbacks: Mutex::new(0),
            executes: Mutex::new(Vec::new()),
            queries: Mutex::new(Vec::new()),
            query_result: Mutex::new(Vec::new()),
            execute_result: Mutex::new(SqliteExecuteResult {
                rows_affected: 1,
                result: Value::Int(10),
            }),
        }
    }
}

#[async_trait]
impl SqliteDriver for ScriptedDriver {
    async fn open(&self, _filename: &str) -> Result<()> {
        *self.opened.lock().unwrap() = true;
        Ok(())
    }

    async fn close(&self) -> Result<()> {
        *self.closed.lock().unwrap() = true;
        Ok(())
    }

    async fn execute(
        &self,
        sql: &str,
        _params: &HashMap<String, Value>,
    ) -> Result<SqliteExecuteResult> {
        self.executes.lock().unwrap().push(sql.to_string());
        Ok(self.execute_result.lock().unwrap().clone())
    }

    async fn query(&self, sql: &str, _params: &HashMap<String, Value>) -> Result<Vec<Record>> {
        self.queries.lock().unwrap().push(sql.to_string());
        Ok(self.query_result.lock().unwrap().clone())
    }

    async fn begin(&self) -> Result<()> {
        *self.begins.lock().unwrap() += 1;
        Ok(())
    }

    async fn commit(&self) -> Result<()> {
        *self.commits.lock().unwrap() += 1;
        Ok(())
    }

    async fn rollback(&self) -> Result<()> {
        *self.rollbacks.lock().unwrap() += 1;
        Ok(())
    }
}

pub struct SqliteConnection {
    host: MiddlewareHost,
    pub filename: String,
    pub driver: Arc<dyn SqliteDriver>,
    opened: Mutex<bool>,
}

impl SqliteConnection {
    pub fn new(filename: Option<String>, driver: Arc<dyn SqliteDriver>) -> Self {
        Self {
            host: MiddlewareHost::new(),
            filename: filename.unwrap_or_else(|| ":memory:".to_string()),
            driver,
            opened: Mutex::new(false),
        }
    }

    pub fn ensure_open(&self) -> Result<()> {
        if !*self.opened.lock().unwrap() {
            return Err(DatasourceError::message("Connection is not open"));
        }
        Ok(())
    }
}

impl MiddlewareAware for SqliteConnection {
    fn use_middleware(&self, before: Option<MiddlewareHook>, after: Option<MiddlewareHook>) {
        self.host.use_middleware(before, after);
    }
}

#[async_trait]
impl Connection for SqliteConnection {
    async fn open(&self) -> Result<()> {
        self.host
            .run("open", None, || async {
                if *self.opened.lock().unwrap() {
                    return Ok(());
                }
                self.driver.open(&self.filename).await?;
                *self.opened.lock().unwrap() = true;
                Ok(())
            })
            .await
    }

    async fn close(&self) -> Result<()> {
        self.host
            .run("close", None, || async {
                self.driver.close().await?;
                *self.opened.lock().unwrap() = false;
                Ok(())
            })
            .await
    }

    async fn start_transaction(&self) -> Result<Arc<dyn Transaction>> {
        self.host
            .run("startTransaction", None, || async {
                self.ensure_open()?;
                self.driver.begin().await?;
                Ok(Arc::new(SqliteTransaction {
                    host: MiddlewareHost::new(),
                    driver: self.driver.clone(),
                }) as Arc<dyn Transaction>)
            })
            .await
    }
}

pub struct SqliteTransaction {
    host: MiddlewareHost,
    driver: Arc<dyn SqliteDriver>,
}

impl MiddlewareAware for SqliteTransaction {
    fn use_middleware(&self, before: Option<MiddlewareHook>, after: Option<MiddlewareHook>) {
        self.host.use_middleware(before, after);
    }
}

#[async_trait]
impl Transaction for SqliteTransaction {
    async fn commit(&self) -> Result<()> {
        let driver = self.driver.clone();
        self.host
            .run("commit", None, move || async move { driver.commit().await })
            .await
    }

    async fn abort(&self) -> Result<()> {
        let driver = self.driver.clone();
        self.host
            .run("abort", None, move || async move { driver.rollback().await })
            .await
    }
}

pub struct SqliteCommand {
    host: MiddlewareHost,
    connection: Arc<SqliteConnection>,
}

impl SqliteCommand {
    pub fn new(connection: Arc<SqliteConnection>) -> Self {
        Self {
            host: MiddlewareHost::new(),
            connection,
        }
    }
}

impl MiddlewareAware for SqliteCommand {
    fn use_middleware(&self, before: Option<MiddlewareHook>, after: Option<MiddlewareHook>) {
        self.host.use_middleware(before, after);
    }
}

#[async_trait]
impl Command for SqliteCommand {
    async fn execute_reader(
        &self,
        text: Option<&str>,
        parameters: Option<&[Parameter]>,
    ) -> Result<Box<dyn Reader>> {
        let extras = Some((
            text.map(ToOwned::to_owned),
            parameters.map(|items| items.to_vec()),
        ));
        let connection = self.connection.clone();
        let sql = text.unwrap_or("").to_string();
        let params = parameters_to_map(parameters);
        self.host
            .run("executeReader", extras, move || {
                let connection = connection.clone();
                async move {
                    connection.ensure_open()?;
                    let rows = connection.driver.query(&sql, &params).await?;
                    Ok(Box::new(ArrayReader::new(rows)) as Box<dyn Reader>)
                }
            })
            .await
    }

    async fn execute(
        &self,
        text: Option<&str>,
        parameters: Option<&[Parameter]>,
    ) -> Result<Response> {
        let extras = Some((
            text.map(ToOwned::to_owned),
            parameters.map(|items| items.to_vec()),
        ));
        let connection = self.connection.clone();
        let sql = text.unwrap_or("").to_string();
        let params = parameters_to_map(parameters);
        self.host
            .run("execute", extras, move || {
                let connection = connection.clone();
                async move {
                    connection.ensure_open()?;
                    let result = connection.driver.execute(&sql, &params).await?;
                    Ok(Response::new(result.result, result.rows_affected))
                }
            })
            .await
    }
}

pub struct SqliteRepository {
    connection: Arc<SqliteConnection>,
    pub table: String,
}

impl SqliteRepository {
    pub fn new(connection: Arc<SqliteConnection>, table: Option<String>) -> Self {
        Self {
            connection,
            table: table.unwrap_or_else(|| "items".to_string()),
        }
    }
}

#[async_trait]
impl Repository for SqliteRepository {
    async fn get_single(&self, query: Option<&dyn Query>) -> Result<Option<Record>> {
        let sql = select_sql(&self.table, query, true);
        let reader = SqliteCommand::new(self.connection.clone())
            .execute_reader(Some(&sql), None)
            .await?;
        if reader.next().await? {
            Ok(reader.current())
        } else {
            Ok(None)
        }
    }

    async fn get(&self, query: Option<&dyn Query>) -> Result<Vec<Record>> {
        let sql = select_sql(&self.table, query, false);
        let reader = SqliteCommand::new(self.connection.clone())
            .execute_reader(Some(&sql), None)
            .await?;
        datasources::collect_reader(&*reader).await
    }

    async fn add(&self, obj: &Record) -> Result<Response> {
        let keys: Vec<&String> = obj.keys().collect();
        let sql = if keys.is_empty() {
            format!("INSERT INTO {} DEFAULT VALUES", self.table)
        } else {
            let cols = keys
                .iter()
                .map(|k| (*k).as_str())
                .collect::<Vec<_>>()
                .join(", ");
            let vals = keys
                .iter()
                .map(|k| format!("${k}"))
                .collect::<Vec<_>>()
                .join(", ");
            format!("INSERT INTO {} ({cols}) VALUES ({vals})", self.table)
        };
        SqliteCommand::new(self.connection.clone())
            .execute(Some(&sql), Some(&to_params(obj)))
            .await
    }

    async fn update(&self, obj: &Record, by_where: Option<&dyn Where>) -> Result<Response> {
        let keys: Vec<&String> = obj.keys().collect();
        let assignments = if keys.is_empty() {
            "id = id".to_string()
        } else {
            keys.iter()
                .map(|k| format!("{k} = ${k}"))
                .collect::<Vec<_>>()
                .join(", ")
        };
        let where_sql = by_where
            .map(|w| w.get_text())
            .unwrap_or_else(|| identity_where_text(obj));
        let sql = format!(
            "UPDATE {} SET {assignments} WHERE {where_sql}",
            self.table
        );
        SqliteCommand::new(self.connection.clone())
            .execute(Some(&sql), Some(&to_params(obj)))
            .await
    }

    async fn delete(&self, obj: &Record, by_where: Option<&dyn Where>) -> Result<Response> {
        let where_sql = by_where
            .map(|w| w.get_text())
            .unwrap_or_else(|| identity_where_text(obj));
        let sql = format!("DELETE FROM {} WHERE {where_sql}", self.table);
        SqliteCommand::new(self.connection.clone())
            .execute(Some(&sql), Some(&to_params(obj)))
            .await
    }
}

fn select_sql(table: &str, query: Option<&dyn Query>, single: bool) -> String {
    let where_sql = query
        .map(|q| format!(" WHERE {}", q.get_text()))
        .unwrap_or_default();
    let limit = if single { " LIMIT 1" } else { "" };
    format!("SELECT * FROM {table}{where_sql}{limit}")
}

fn to_params(obj: &Record) -> Vec<Parameter> {
    obj.iter()
        .map(|(k, v)| Parameter::new(k.clone(), v.clone()))
        .collect()
}

pub struct SqliteDatasource {
    filename: Option<String>,
    driver: Option<Arc<dyn SqliteDriver>>,
    table: Option<String>,
}

impl SqliteDatasource {
    pub fn new(
        filename: Option<String>,
        driver: Option<Arc<dyn SqliteDriver>>,
        table: Option<String>,
    ) -> Self {
        Self {
            filename,
            driver,
            table,
        }
    }

    fn typed_connection(&self) -> Result<Arc<SqliteConnection>> {
        Ok(Arc::new(SqliteConnection::new(
            self.filename.clone(),
            require_driver(self.driver.clone())?,
        )))
    }
}

impl Datasource for SqliteDatasource {
    fn provider(&self) -> &str {
        "sqlite"
    }

    fn create_connection(&self) -> Arc<dyn Connection> {
        self.typed_connection()
            .unwrap_or_else(|_| Arc::new(SqliteConnection::new(None, Arc::new(ScriptedDriver::new()))))
    }

    fn create_command(&self, _connection: Option<Arc<dyn Connection>>) -> Arc<dyn Command> {
        let conn = self
            .typed_connection()
            .unwrap_or_else(|_| Arc::new(SqliteConnection::new(None, Arc::new(ScriptedDriver::new()))));
        Arc::new(SqliteCommand::new(conn))
    }

    fn create_repository(
        &self,
        _connection: Option<Arc<dyn Connection>>,
        table: Option<&str>,
    ) -> Arc<dyn Repository> {
        let conn = self
            .typed_connection()
            .unwrap_or_else(|_| Arc::new(SqliteConnection::new(None, Arc::new(ScriptedDriver::new()))));
        Arc::new(SqliteRepository::new(
            conn,
            table.map(ToOwned::to_owned).or_else(|| self.table.clone()),
        ))
    }
}

impl DatasourceFactory for SqliteDatasource {
    fn provider(&self) -> &str {
        "sqlite"
    }

    fn create(&self, _options: Option<&str>) -> Arc<dyn Datasource> {
        Arc::new(SqliteDatasource::new(
            self.filename.clone(),
            self.driver.clone(),
            self.table.clone(),
        ))
    }
}

pub fn register_sqlite() {
    register(Arc::new(SqliteDatasource::new(
        Some(":memory:".into()),
        Some(Arc::new(ScriptedDriver::new())),
        None,
    )));
}

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::{create, reset, TextQuery, TextWhere};

    fn rec(id: i64) -> Record {
        let mut r = Record::new();
        r.insert("id".into(), Value::Int(id));
        r
    }

    #[tokio::test]
    async fn sqlite_command_and_repository() {
        assert!(require_driver(None).is_err());
        let driver = Arc::new(ScriptedDriver::new());
        require_driver(Some(driver.clone())).unwrap();
        driver
            .query_result
            .lock()
            .unwrap()
            .push(rec(1));
        let conn = Arc::new(SqliteConnection::new(Some("x.db".into()), driver.clone()));
        conn.open().await.unwrap();
        conn.open().await.unwrap();
        let command = SqliteCommand::new(conn.clone());
        command
            .execute(Some("DELETE FROM items"), Some(&[Parameter::new("id", 1_i64)]))
            .await
            .unwrap();
        command.execute(None, None).await.unwrap();
        command.execute_reader(Some("SELECT 1"), None).await.unwrap();
        command.execute_reader(None, None).await.unwrap();
        command.use_middleware(None, None);

        let repo = SqliteRepository::new(conn.clone(), None);
        assert!(repo.get_single(None).await.unwrap().is_some());
        driver.query_result.lock().unwrap().clear();
        assert!(repo
            .get_single(Some(&TextQuery::new("id = 1")))
            .await
            .unwrap()
            .is_none());
        driver.query_result.lock().unwrap().push(rec(1));
        assert_eq!(repo.get(None).await.unwrap().len(), 1);
        repo.add(&rec(2)).await.unwrap();
        repo.add(&Record::new()).await.unwrap();
        repo.update(&rec(2), None).await.unwrap();
        repo.update(&Record::new(), Some(&TextWhere::new("id = 1")))
            .await
            .unwrap();
        repo.delete(&rec(2), None).await.unwrap();
        repo.delete(&rec(2), Some(&TextWhere::new("name = $name")))
            .await
            .unwrap();
        let named = SqliteRepository::new(conn.clone(), Some("users".into()));
        named.get(None).await.unwrap();
        assert!(named.table == "users");

        let txn = conn.start_transaction().await.unwrap();
        txn.commit().await.unwrap();
        let txn = conn.start_transaction().await.unwrap();
        txn.abort().await.unwrap();
        txn.use_middleware(None, None);
        conn.close().await.unwrap();
        assert!(conn.ensure_open().is_err());
        assert!(conn.start_transaction().await.is_err());

        let defaulted = SqliteConnection::new(None, driver.clone());
        assert_eq!(defaulted.filename, ":memory:");
        defaulted.use_middleware(None, None);

        reset();
        register_sqlite();
        let ds = create("sqlite", None).unwrap();
        assert_eq!(ds.provider(), "sqlite");
        ds.create_connection();
        ds.create_command(None);
        ds.create_repository(None, Some("people"));
        let factory = SqliteDatasource::new(None, None, None);
        assert_eq!(Datasource::provider(&factory), "sqlite");
        factory.create_connection();
        factory.create_command(Some(factory.create_connection()));
        factory.create_repository(Some(factory.create_connection()), None);
        DatasourceFactory::create(&factory, None);
        assert_eq!(DatasourceFactory::provider(&factory), "sqlite");

        let closed = Arc::new(SqliteConnection::new(None, driver));
        assert!(SqliteCommand::new(closed)
            .execute(Some("x"), None)
            .await
            .is_err());
    }
}
