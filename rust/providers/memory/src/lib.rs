use async_trait::async_trait;
use datasources::{
    identity_filter, parameters_to_map, ArrayReader, Command, Connection, Datasource,
    DatasourceError, DatasourceFactory, MiddlewareAware, MiddlewareHook, MiddlewareHost, Parameter,
    Query, Reader, Record, Repository, Response, Result, Transaction, Value, Where,
};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub enum MemoryData {
    Rows(Vec<Record>),
    Collections(HashMap<String, Vec<Record>>),
}

#[derive(Default)]
pub struct MemoryStore {
    collections: Mutex<HashMap<String, Vec<Record>>>,
}

impl MemoryStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn has_collection(&self, name: &str) -> bool {
        self.collections.lock().unwrap().contains_key(name)
    }

    pub fn collection<R>(&self, name: &str, f: impl FnOnce(&mut Vec<Record>) -> R) -> R {
        let mut map = self.collections.lock().unwrap();
        f(map.entry(name.to_string()).or_default())
    }

    pub fn snapshot(&self) -> HashMap<String, Vec<Record>> {
        self.collections.lock().unwrap().clone()
    }

    pub fn restore(&self, snapshot: HashMap<String, Vec<Record>>) {
        *self.collections.lock().unwrap() = snapshot;
    }

    pub fn seed(&self, data: &MemoryData, collection: &str) {
        match data {
            MemoryData::Rows(rows) if !self.has_collection(collection) => {
                self.collections
                    .lock()
                    .unwrap()
                    .insert(collection.to_string(), rows.clone());
            }
            MemoryData::Collections(named) => {
                let mut map = self.collections.lock().unwrap();
                for (name, rows) in named {
                    map.entry(name.clone()).or_insert_with(|| rows.clone());
                }
            }
            _ => {}
        }
    }
}

struct MemoryState {
    store: Arc<MemoryStore>,
    collection: Mutex<String>,
    opened: Mutex<bool>,
    in_transaction: Mutex<bool>,
    snapshot: Mutex<Option<HashMap<String, Vec<Record>>>>,
    data: Option<MemoryData>,
}

pub struct MemoryConnection {
    host: MiddlewareHost,
    state: Arc<MemoryState>,
}

impl MemoryConnection {
    pub fn new(
        store: Arc<MemoryStore>,
        collection: impl Into<String>,
        data: Option<MemoryData>,
    ) -> Self {
        Self {
            host: MiddlewareHost::new(),
            state: Arc::new(MemoryState {
                store,
                collection: Mutex::new(collection.into()),
                opened: Mutex::new(false),
                in_transaction: Mutex::new(false),
                snapshot: Mutex::new(None),
                data,
            }),
        }
    }

    pub fn set_collection(&self, name: impl Into<String>) {
        *self.state.collection.lock().unwrap() = name.into();
    }

    pub fn collection_name(&self) -> String {
        self.state.collection.lock().unwrap().clone()
    }

    pub fn ensure_open(&self) -> Result<()> {
        if !*self.state.opened.lock().unwrap() {
            return Err(DatasourceError::message("Connection is not open"));
        }
        Ok(())
    }

    pub fn collection<R>(&self, f: impl FnOnce(&mut Vec<Record>) -> R) -> R {
        self.state.store.collection(&self.collection_name(), f)
    }

    pub fn finish_commit(&self) {
        *self.state.in_transaction.lock().unwrap() = false;
        *self.state.snapshot.lock().unwrap() = None;
    }

    pub fn finish_abort(&self) {
        if let Some(snapshot) = self.state.snapshot.lock().unwrap().take() {
            self.state.store.restore(snapshot);
        }
        *self.state.in_transaction.lock().unwrap() = false;
    }

    pub fn in_transaction(&self) -> bool {
        *self.state.in_transaction.lock().unwrap()
    }
}

impl MiddlewareAware for MemoryConnection {
    fn use_middleware(&self, before: Option<MiddlewareHook>, after: Option<MiddlewareHook>) {
        self.host.use_middleware(before, after);
    }
}

#[async_trait]
impl Connection for MemoryConnection {
    async fn open(&self) -> Result<()> {
        self.host
            .run("open", None, || async {
                if *self.state.opened.lock().unwrap() {
                    return Ok(());
                }
                if let Some(data) = &self.state.data {
                    self.state.store.seed(data, &self.collection_name());
                }
                self.collection(|_| {});
                *self.state.opened.lock().unwrap() = true;
                Ok(())
            })
            .await
    }

    async fn close(&self) -> Result<()> {
        self.host
            .run("close", None, || async {
                *self.state.opened.lock().unwrap() = false;
                Ok(())
            })
            .await
    }

    async fn start_transaction(&self) -> Result<Arc<dyn Transaction>> {
        self.host
            .run("startTransaction", None, || async {
                self.ensure_open()?;
                *self.state.snapshot.lock().unwrap() = Some(self.state.store.snapshot());
                *self.state.in_transaction.lock().unwrap() = true;
                Ok(Arc::new(MemoryTransaction {
                    host: MiddlewareHost::new(),
                    state: self.state.clone(),
                }) as Arc<dyn Transaction>)
            })
            .await
    }
}

pub struct MemoryTransaction {
    host: MiddlewareHost,
    state: Arc<MemoryState>,
}

impl MiddlewareAware for MemoryTransaction {
    fn use_middleware(&self, before: Option<MiddlewareHook>, after: Option<MiddlewareHook>) {
        self.host.use_middleware(before, after);
    }
}

#[async_trait]
impl Transaction for MemoryTransaction {
    async fn commit(&self) -> Result<()> {
        self.host
            .run("commit", None, || async {
                *self.state.in_transaction.lock().unwrap() = false;
                *self.state.snapshot.lock().unwrap() = None;
                Ok(())
            })
            .await
    }

    async fn abort(&self) -> Result<()> {
        self.host
            .run("abort", None, || async {
                if let Some(snapshot) = self.state.snapshot.lock().unwrap().take() {
                    self.state.store.restore(snapshot);
                }
                *self.state.in_transaction.lock().unwrap() = false;
                Ok(())
            })
            .await
    }
}

fn parse_filter(text: Option<&str>) -> Result<Record> {
    match text {
        None => Ok(Record::new()),
        Some(value) if value.trim().is_empty() => Ok(Record::new()),
        Some(value) => {
            let parsed: JsonValue = serde_json::from_str(value)
                .map_err(|e| DatasourceError::message(e.to_string()))?;
            match parsed {
                JsonValue::Object(map) => Ok(map
                    .into_iter()
                    .map(|(k, v)| (k, json_to_value(&v)))
                    .collect()),
                _ => Err(DatasourceError::message("Memory filter must be an object")),
            }
        }
    }
}

fn matches(row: &Record, filter: &Record) -> bool {
    filter.iter().all(|(k, v)| row.get(k) == Some(v))
}

fn json_to_value(v: &JsonValue) -> Value {
    match v {
        JsonValue::Null => Value::Null,
        JsonValue::Bool(b) => Value::Bool(*b),
        JsonValue::Number(n) if n.is_i64() => Value::Int(n.as_i64().unwrap()),
        JsonValue::Number(n) if n.is_f64() => Value::Float(n.as_f64().unwrap()),
        JsonValue::Number(n) => Value::Text(n.to_string()),
        JsonValue::String(s) => Value::Text(s.clone()),
        other => Value::Text(other.to_string()),
    }
}

pub struct MemoryCommand {
    host: MiddlewareHost,
    connection: Arc<MemoryConnection>,
}

impl MemoryCommand {
    pub fn new(connection: Arc<MemoryConnection>) -> Self {
        Self {
            host: MiddlewareHost::new(),
            connection,
        }
    }
}

impl MiddlewareAware for MemoryCommand {
    fn use_middleware(&self, before: Option<MiddlewareHook>, after: Option<MiddlewareHook>) {
        self.host.use_middleware(before, after);
    }
}

#[async_trait]
impl Command for MemoryCommand {
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
        let text_owned = text.map(ToOwned::to_owned);
        let params = parameters.map(|items| items.to_vec());
        self.host
            .run("executeReader", extras, move || {
                let connection = connection.clone();
                async move {
                    connection.ensure_open()?;
                    let mut filter = parse_filter(text_owned.as_deref())?;
                    filter.extend(parameters_to_map(params.as_deref()));
                    let rows = connection.collection(|items| {
                        items
                            .iter()
                            .filter(|row| matches(row, &filter))
                            .cloned()
                            .collect::<Vec<_>>()
                    });
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
        let op = text.unwrap_or("").trim().to_ascii_lowercase();
        let params = parameters_to_map(parameters);
        self.host
            .run("execute", extras, move || {
                let connection = connection.clone();
                let op = op.clone();
                async move {
                    connection.ensure_open()?;
                    match op.as_str() {
                        "insert" => {
                            connection.collection(|items| items.push(params.clone()));
                            Ok(Response::new(Value::Null, 1))
                        }
                        "update" => {
                            let mut obj = params.clone();
                            let where_text = obj
                                .remove("$where")
                                .and_then(|v| match v {
                                    Value::Text(text) => Some(text),
                                    _ => None,
                                });
                            let filter = if let Some(text) = where_text {
                                parse_filter(Some(&text))?
                            } else {
                                identity_filter(&obj)
                            };
                            let affected = connection.collection(|items| {
                                let mut count = 0i64;
                                for item in items.iter_mut() {
                                    if matches(item, &filter) {
                                        item.extend(obj.clone());
                                        count += 1;
                                    }
                                }
                                count
                            });
                            Ok(Response::new(Value::Null, affected))
                        }
                        "delete" => {
                            let filter = if params.is_empty() {
                                Record::new()
                            } else {
                                identity_filter(&params)
                            };
                            let affected = connection.collection(|items| {
                                let before = items.len();
                                items.retain(|row| !matches(row, &filter));
                                (before - items.len()) as i64
                            });
                            Ok(Response::new(Value::Null, affected))
                        }
                        _ => Err(DatasourceError::message(format!(
                            "Unsupported Memory command: {op}"
                        ))),
                    }
                }
            })
            .await
    }
}

pub struct MemoryRepository {
    connection: Arc<MemoryConnection>,
}

impl MemoryRepository {
    pub fn new(connection: Arc<MemoryConnection>) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl Repository for MemoryRepository {
    async fn get_single(&self, query: Option<&dyn Query>) -> Result<Option<Record>> {
        let text = query.map(|q| q.get_text());
        let reader = MemoryCommand::new(self.connection.clone())
            .execute_reader(text.as_deref(), None)
            .await?;
        if reader.next().await? {
            Ok(reader.current())
        } else {
            Ok(None)
        }
    }

    async fn get(&self, query: Option<&dyn Query>) -> Result<Vec<Record>> {
        let text = query.map(|q| q.get_text());
        let reader = MemoryCommand::new(self.connection.clone())
            .execute_reader(text.as_deref(), None)
            .await?;
        datasources::collect_reader(&*reader).await
    }

    async fn add(&self, obj: &Record) -> Result<Response> {
        self.connection.ensure_open()?;
        self.connection.collection(|items| items.push(obj.clone()));
        Ok(Response::new(Value::Null, 1))
    }

    async fn update(&self, obj: &Record, by_where: Option<&dyn Where>) -> Result<Response> {
        self.connection.ensure_open()?;
        let filter = if let Some(w) = by_where {
            parse_filter(Some(&w.get_text()))?
        } else {
            identity_filter(obj)
        };
        let affected = self.connection.collection(|items| {
            let mut count = 0i64;
            for item in items.iter_mut() {
                if matches(item, &filter) {
                    item.extend(obj.clone());
                    count += 1;
                }
            }
            count
        });
        Ok(Response::new(Value::Null, affected))
    }

    async fn delete(&self, obj: &Record, by_where: Option<&dyn Where>) -> Result<Response> {
        self.connection.ensure_open()?;
        let filter = if let Some(w) = by_where {
            parse_filter(Some(&w.get_text()))?
        } else {
            identity_filter(obj)
        };
        let affected = self.connection.collection(|items| {
            let before = items.len();
            items.retain(|row| !matches(row, &filter));
            (before - items.len()) as i64
        });
        Ok(Response::new(Value::Null, affected))
    }
}

pub struct MemoryDatasource {
    store: Arc<MemoryStore>,
    collection: Option<String>,
    data: Option<MemoryData>,
}

impl MemoryDatasource {
    pub fn factory() -> Arc<Self> {
        Arc::new(Self::default())
    }

    pub fn new(
        store: Arc<MemoryStore>,
        collection: Option<String>,
        data: Option<MemoryData>,
    ) -> Self {
        Self {
            store,
            collection,
            data,
        }
    }

    fn typed_connection(&self) -> Arc<MemoryConnection> {
        Arc::new(MemoryConnection::new(
            self.store.clone(),
            self.collection.clone().unwrap_or_default(),
            self.data.clone(),
        ))
    }
}

impl Default for MemoryDatasource {
    fn default() -> Self {
        Self::new(Arc::new(MemoryStore::new()), None, None)
    }
}

#[async_trait]
impl Datasource for MemoryDatasource {
    fn provider(&self) -> &str {
        "memory"
    }

    async fn create_connection(&self) -> Result<Arc<dyn Connection>> {
        Ok(self.typed_connection())
    }

    fn create_command(&self, _connection: Option<Arc<dyn Connection>>) -> Arc<dyn Command> {
        Arc::new(MemoryCommand::new(self.typed_connection()))
    }

    fn create_repository(
        &self,
        _connection: Option<Arc<dyn Connection>>,
        table: Option<&str>,
    ) -> Arc<dyn Repository> {
        let conn = self.typed_connection();
        if let Some(name) = table {
            conn.set_collection(name);
        }
        Arc::new(MemoryRepository::new(conn))
    }
}

impl DatasourceFactory for MemoryDatasource {
    fn provider(&self) -> &str {
        "memory"
    }

    fn create(&self, _options: Option<&str>) -> Arc<dyn Datasource> {
        Arc::new(MemoryDatasource::new(
            self.store.clone(),
            self.collection.clone(),
            self.data.clone(),
        ))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use datasources::{create, register, reset, TextQuery, TextWhere};

    fn rec(id: i64, name: &str) -> Record {
        let mut r = Record::new();
        r.insert("id".into(), Value::Int(id));
        r.insert("name".into(), Value::from(name));
        r
    }

    #[tokio::test]
    async fn memory_crud_and_transactions() {
        let store = Arc::new(MemoryStore::new());
        store.seed(&MemoryData::Rows(vec![rec(1, "a")]), "");
        store.seed(&MemoryData::Rows(vec![rec(2, "b")]), "");
        let mut named = HashMap::new();
        named.insert("users".into(), vec![rec(3, "c")]);
        named.insert("posts".into(), vec![rec(4, "d")]);
        store.seed(&MemoryData::Collections(named.clone()), "");
        store.seed(&MemoryData::Collections(named), "");
        assert_eq!(store.collection("", |items| items.len()), 1);
        assert_eq!(store.collection("users", |items| items[0]["id"].clone()), Value::Int(3));

        let conn = Arc::new(MemoryConnection::new(
            store.clone(),
            "",
            Some(MemoryData::Rows(vec![rec(8, "x")])),
        ));
        conn.open().await.unwrap();
        conn.open().await.unwrap();
        assert_eq!(conn.collection_name(), "");
        let repo = MemoryRepository::new(conn.clone());
        assert!(repo.get_single(None).await.unwrap().is_some());
        assert!(repo
            .get_single(Some(&TextQuery::new("{\"id\":9}")))
            .await
            .unwrap()
            .is_none());
        assert!(!repo.get(Some(&TextQuery::new("{\"id\":1}"))).await.unwrap().is_empty());
        repo.add(&rec(2, "b")).await.unwrap();
        repo.update(&rec(2, "B"), None).await.unwrap();
        repo.update(&rec(2, "C"), Some(&TextWhere::new("{\"id\":2}")))
            .await
            .unwrap();
        repo.delete(&rec(2, "C"), Some(&TextWhere::new("{\"id\":2}")))
            .await
            .unwrap();

        let command = MemoryCommand::new(conn.clone());
        command
            .execute(Some("insert"), Some(&[Parameter::new("id", 5_i64)]))
            .await
            .unwrap();
        command
            .execute(
                Some("update"),
                Some(&[
                    Parameter::new("id", 5_i64),
                    Parameter::new("name", "z"),
                    Parameter::new("$where", "{\"id\":5}"),
                ]),
            )
            .await
            .unwrap();
        command
            .execute(Some("update"), Some(&[Parameter::new("id", 5_i64)]))
            .await
            .unwrap();
        command
            .execute(
                Some("update"),
                Some(&[Parameter::new("$where", 1_i64)]),
            )
            .await
            .unwrap();
        command
            .execute(Some("delete"), Some(&[Parameter::new("id", 5_i64)]))
            .await
            .unwrap();
        command.execute(Some("delete"), None).await.unwrap();
        assert!(command.execute(Some("nope"), None).await.is_err());
        assert!(command.execute(None, None).await.is_err());
        command
            .execute_reader(Some("{\"id\":1}"), Some(&[Parameter::new("name", "a")]))
            .await
            .unwrap();
        command.use_middleware(None, None);

        let txn = conn.start_transaction().await.unwrap();
        assert!(conn.in_transaction());
        repo.add(&rec(9, "t")).await.unwrap();
        txn.abort().await.unwrap();
        let txn = conn.start_transaction().await.unwrap();
        repo.add(&rec(8, "c")).await.unwrap();
        txn.commit().await.unwrap();
        txn.use_middleware(None, None);
        let _ = conn.start_transaction().await.unwrap();
        conn.finish_abort();
        assert!(!conn.in_transaction());
        conn.finish_abort();
        conn.close().await.unwrap();
        assert!(conn.ensure_open().is_err());

        let named_conn = Arc::new(MemoryConnection::new(
            Arc::new(MemoryStore::new()),
            "users",
            Some(MemoryData::Collections({
                let mut map = HashMap::new();
                map.insert("users".into(), vec![rec(1, "a")]);
                map
            })),
        ));
        named_conn.open().await.unwrap();
        MemoryRepository::new(named_conn.clone())
            .add(&rec(2, "b"))
            .await
            .unwrap();

        reset();
        register(MemoryDatasource::factory());
        let ds = create("memory", None).unwrap();
        assert_eq!(ds.provider(), "memory");
        ds.create_connection().await.unwrap();
        ds.create_command(None);
        ds.create_command(Some(ds.create_connection().await.unwrap()));
        ds.create_repository(None, Some("users"));
        ds.create_repository(Some(ds.create_connection().await.unwrap()), None);
        let factory = MemoryDatasource::default();
        assert_eq!(DatasourceFactory::provider(&factory), "memory");
        DatasourceFactory::create(&factory, None);
        assert_eq!(create("memory", None).unwrap().provider(), "memory");

        assert!(parse_filter(None).unwrap().is_empty());
        assert!(parse_filter(Some("   ")).unwrap().is_empty());
        assert!(parse_filter(Some("[]")).is_err());
        assert!(parse_filter(Some("not-json")).is_err());
        assert!(!matches(&Record::new(), &{
            let mut f = Record::new();
            f.insert("id".into(), Value::Int(1));
            f
        }));
        json_to_value(&JsonValue::Null);
        json_to_value(&JsonValue::Bool(true));
        json_to_value(&serde_json::json!(1));
        json_to_value(&JsonValue::Number(serde_json::Number::from_f64(1.25).unwrap()));
        json_to_value(&serde_json::json!("x"));
        json_to_value(&serde_json::json!([1]));
        if let JsonValue::Number(n) = serde_json::json!(1) {
            json_to_value(&JsonValue::Number(n));
        }
        json_to_value(&JsonValue::Number(serde_json::Number::from(u64::MAX)));

        let closed = Arc::new(MemoryConnection::new(
            Arc::new(MemoryStore::new()),
            "",
            None,
        ));
        assert!(MemoryCommand::new(closed.clone())
            .execute_reader(None, None)
            .await
            .is_err());
        assert!(MemoryCommand::new(closed.clone())
            .execute(Some("insert"), None)
            .await
            .is_err());
        assert!(MemoryRepository::new(closed.clone())
            .add(&rec(1, "a"))
            .await
            .is_err());
        assert!(MemoryRepository::new(closed.clone())
            .update(&rec(1, "a"), None)
            .await
            .is_err());
        assert!(MemoryRepository::new(closed)
            .delete(&rec(1, "a"), None)
            .await
            .is_err());
        assert!(MemoryConnection::new(Arc::new(MemoryStore::new()), "", None)
            .start_transaction()
            .await
            .is_err());

        conn.use_middleware(None, None);
        let empty = Arc::new(MemoryConnection::new(
            Arc::new(MemoryStore::new()),
            "",
            None,
        ));
        empty.open().await.unwrap();
        empty.finish_commit();
        empty.finish_abort();
    }
}
