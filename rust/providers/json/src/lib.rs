use async_trait::async_trait;
use datasources::{
    identity_filter, parameters_to_map, ArrayReader, Command, Connection, Datasource,
    DatasourceError, DatasourceFactory, MiddlewareAware, MiddlewareHook, MiddlewareHost, Parameter,
    Query, Reader, Record, Repository, Response, Result, Transaction, Value, Where,
};
use serde_json::{Map, Value as JsonValue};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub trait FileStore: Send + Sync {
    fn read(&self, path: &str) -> Result<String>;
    fn write(&self, path: &str, contents: &str) -> Result<()>;
    fn exists(&self, path: &str) -> bool;
}

#[derive(Default)]
pub struct MemoryFileStore {
    files: Mutex<HashMap<String, String>>,
}

impl MemoryFileStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn seed(path: impl Into<String>, contents: impl Into<String>) -> Self {
        let store = Self::new();
        store
            .files
            .lock()
            .unwrap()
            .insert(path.into(), contents.into());
        store
    }
}

impl FileStore for MemoryFileStore {
    fn read(&self, path: &str) -> Result<String> {
        self.files
            .lock()
            .unwrap()
            .get(path)
            .cloned()
            .ok_or_else(|| DatasourceError::message(format!("File not found: {path}")))
    }

    fn write(&self, path: &str, contents: &str) -> Result<()> {
        self.files
            .lock()
            .unwrap()
            .insert(path.to_string(), contents.to_string());
        Ok(())
    }

    fn exists(&self, path: &str) -> bool {
        self.files.lock().unwrap().contains_key(path)
    }
}

struct JsonState {
    path: String,
    collection: Mutex<Option<String>>,
    store: Arc<dyn FileStore>,
    data: Mutex<JsonValue>,
    opened: Mutex<bool>,
    in_transaction: Mutex<bool>,
    snapshot: Mutex<Option<JsonValue>>,
    initial: Option<JsonValue>,
}

pub struct JsonConnection {
    host: MiddlewareHost,
    pub(crate) state: Arc<JsonState>,
}

impl JsonConnection {
    pub fn new(
        path: impl Into<String>,
        collection: Option<String>,
        store: Arc<dyn FileStore>,
        initial: Option<JsonValue>,
    ) -> Self {
        Self {
            host: MiddlewareHost::new(),
            state: Arc::new(JsonState {
                path: path.into(),
                collection: Mutex::new(collection),
                store,
                data: Mutex::new(JsonValue::Array(vec![])),
                opened: Mutex::new(false),
                in_transaction: Mutex::new(false),
                snapshot: Mutex::new(None),
                initial,
            }),
        }
    }

    pub fn path(&self) -> &str {
        &self.state.path
    }

    pub fn set_collection(&self, name: impl Into<String>) {
        *self.state.collection.lock().unwrap() = Some(name.into());
    }

    pub fn ensure_open(&self) -> Result<()> {
        if !*self.state.opened.lock().unwrap() {
            return Err(DatasourceError::message("Connection is not open"));
        }
        Ok(())
    }

    fn empty_data(collection: &Option<String>) -> JsonValue {
        if collection.is_some() {
            JsonValue::Object(Map::new())
        } else {
            JsonValue::Array(vec![])
        }
    }

    pub fn collection_mut<R>(&self, f: impl FnOnce(&mut Vec<JsonValue>) -> R) -> R {
        let collection = self.state.collection.lock().unwrap().clone();
        let mut data = self.state.data.lock().unwrap();
        if let Some(name) = collection {
            if !data.is_object() {
                *data = JsonValue::Object(Map::new());
            }
            let obj = data.as_object_mut().unwrap();
            if !obj.get(&name).map(JsonValue::is_array).unwrap_or(false) {
                obj.insert(name.clone(), JsonValue::Array(vec![]));
            }
            f(obj.get_mut(&name).unwrap().as_array_mut().unwrap())
        } else {
            if !data.is_array() {
                *data = JsonValue::Array(vec![]);
            }
            f(data.as_array_mut().unwrap())
        }
    }

    pub fn persist(&self) -> Result<()> {
        if *self.state.in_transaction.lock().unwrap() || self.state.path.is_empty() {
            return Ok(());
        }
        let data = self.state.data.lock().unwrap().to_string();
        self.state.store.write(&self.state.path, &data)
    }

}

impl MiddlewareAware for JsonConnection {
    fn use_middleware(&self, before: Option<MiddlewareHook>, after: Option<MiddlewareHook>) {
        self.host.use_middleware(before, after);
    }
}

#[async_trait]
impl Connection for JsonConnection {
    async fn open(&self) -> Result<()> {
        self.host
            .run("open", None, || async {
                if *self.state.opened.lock().unwrap() {
                    return Ok(());
                }
                let collection = self.state.collection.lock().unwrap().clone();
                let next = if let Some(initial) = &self.state.initial {
                    initial.clone()
                } else if !self.state.path.is_empty() && self.state.store.exists(&self.state.path) {
                    let contents = self.state.store.read(&self.state.path)?;
                    serde_json::from_str(&contents)
                        .map_err(|e| DatasourceError::message(e.to_string()))?
                } else {
                    JsonConnection::empty_data(&collection)
                };
                *self.state.data.lock().unwrap() = next;
                if collection.is_some() {
                    self.collection_mut(|_| {});
                }
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
                *self.state.snapshot.lock().unwrap() =
                    Some(self.state.data.lock().unwrap().clone());
                *self.state.in_transaction.lock().unwrap() = true;
                Ok(Arc::new(JsonTransaction {
                    host: MiddlewareHost::new(),
                    state: self.state.clone(),
                }) as Arc<dyn Transaction>)
            })
            .await
    }
}

pub struct JsonTransaction {
    host: MiddlewareHost,
    state: Arc<JsonState>,
}

impl MiddlewareAware for JsonTransaction {
    fn use_middleware(&self, before: Option<MiddlewareHook>, after: Option<MiddlewareHook>) {
        self.host.use_middleware(before, after);
    }
}

#[async_trait]
impl Transaction for JsonTransaction {
    async fn commit(&self) -> Result<()> {
        self.host
            .run("commit", None, || async {
                *self.state.in_transaction.lock().unwrap() = false;
                *self.state.snapshot.lock().unwrap() = None;
                if !self.state.path.is_empty() {
                    self.state.store.write(
                        &self.state.path,
                        &self.state.data.lock().unwrap().to_string(),
                    )?;
                }
                Ok(())
            })
            .await
    }

    async fn abort(&self) -> Result<()> {
        self.host
            .run("abort", None, || async {
                if let Some(snapshot) = self.state.snapshot.lock().unwrap().take() {
                    *self.state.data.lock().unwrap() = snapshot;
                }
                *self.state.in_transaction.lock().unwrap() = false;
                Ok(())
            })
            .await
    }
}

fn parse_filter(text: Option<&str>) -> Result<Map<String, JsonValue>> {
    match text {
        None => Ok(Map::new()),
        Some(value) if value.trim().is_empty() => Ok(Map::new()),
        Some(value) => {
            let parsed: JsonValue = serde_json::from_str(value)
                .map_err(|e| DatasourceError::message(e.to_string()))?;
            match parsed {
                JsonValue::Object(map) => Ok(map),
                _ => Err(DatasourceError::message("JSON filter must be an object")),
            }
        }
    }
}

fn matches(obj: &JsonValue, filter: &Map<String, JsonValue>) -> bool {
    let JsonValue::Object(map) = obj else {
        return false;
    };
    filter.iter().all(|(k, v)| map.get(k) == Some(v))
}

fn json_from_record(record: &Record) -> JsonValue {
    let mut map = Map::new();
    for (k, v) in record {
        map.insert(k.clone(), value_to_json(v));
    }
    JsonValue::Object(map)
}

fn record_from_json(value: &JsonValue) -> Record {
    let mut record = Record::new();
    if let JsonValue::Object(map) = value {
        for (k, v) in map {
            record.insert(k.clone(), json_to_value(v));
        }
    }
    record
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

fn value_to_json(value: &Value) -> JsonValue {
    match value {
        Value::Null => JsonValue::Null,
        Value::Bool(b) => JsonValue::Bool(*b),
        Value::Int(i) => JsonValue::from(*i),
        Value::Float(f) => JsonValue::from(*f),
        Value::Text(t) => JsonValue::String(t.clone()),
    }
}

pub struct JsonCommand {
    host: MiddlewareHost,
    connection: Arc<JsonConnection>,
}

impl JsonCommand {
    pub fn new(connection: Arc<JsonConnection>) -> Self {
        Self {
            host: MiddlewareHost::new(),
            connection,
        }
    }
}

impl MiddlewareAware for JsonCommand {
    fn use_middleware(&self, before: Option<MiddlewareHook>, after: Option<MiddlewareHook>) {
        self.host.use_middleware(before, after);
    }
}

#[async_trait]
impl Command for JsonCommand {
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
                    for (k, v) in parameters_to_map(params.as_deref()) {
                        filter.insert(k, value_to_json(&v));
                    }
                    let rows = connection.collection_mut(|items| {
                        items
                            .iter()
                            .filter(|row| matches(row, &filter))
                            .map(record_from_json)
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
                    let mut obj: Map<String, JsonValue> = params
                        .iter()
                        .map(|(k, v)| (k.clone(), value_to_json(v)))
                        .collect();
                    match op.as_str() {
                        "insert" => {
                            connection
                                .collection_mut(|items| items.push(JsonValue::Object(obj.clone())));
                            connection.persist()?;
                            Ok(Response::new(Value::Null, 1))
                        }
                        "update" => {
                            let where_text = obj
                                .remove("$where")
                                .and_then(|v| v.as_str().map(ToOwned::to_owned));
                            let filter = if let Some(text) = where_text {
                                parse_filter(Some(&text))?
                            } else {
                                identity_filter(&record_from_json(&JsonValue::Object(obj.clone())))
                                    .into_iter()
                                    .map(|(k, v)| (k, value_to_json(&v)))
                                    .collect()
                            };
                            let affected = connection.collection_mut(|items| {
                                let mut count = 0;
                                for item in items.iter_mut() {
                                    if matches(item, &filter) {
                                        if let JsonValue::Object(map) = item {
                                            for (k, v) in &obj {
                                                map.insert(k.clone(), v.clone());
                                            }
                                        }
                                        count += 1;
                                    }
                                }
                                count
                            });
                            connection.persist()?;
                            Ok(Response::new(Value::Null, affected))
                        }
                        "delete" => {
                            let filter = if obj.is_empty() {
                                Map::new()
                            } else {
                                identity_filter(&record_from_json(&JsonValue::Object(obj.clone())))
                                    .into_iter()
                                    .map(|(k, v)| (k, value_to_json(&v)))
                                    .collect()
                            };
                            let affected = connection.collection_mut(|items| {
                                let before = items.len();
                                items.retain(|row| !matches(row, &filter));
                                (before - items.len()) as i64
                            });
                            connection.persist()?;
                            Ok(Response::new(Value::Null, affected))
                        }
                        _ => Err(DatasourceError::message(format!(
                            "Unsupported JSON command: {op}"
                        ))),
                    }
                }
            })
            .await
    }
}

pub struct JsonRepository {
    connection: Arc<JsonConnection>,
}

impl JsonRepository {
    pub fn new(connection: Arc<JsonConnection>) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl Repository for JsonRepository {
    async fn get_single(&self, query: Option<&dyn Query>) -> Result<Option<Record>> {
        let text = query.map(|q| q.get_text());
        let reader = JsonCommand::new(self.connection.clone())
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
        let reader = JsonCommand::new(self.connection.clone())
            .execute_reader(text.as_deref(), None)
            .await?;
        datasources::collect_reader(&*reader).await
    }

    async fn add(&self, obj: &Record) -> Result<Response> {
        self.connection.ensure_open()?;
        self.connection
            .collection_mut(|items| items.push(json_from_record(obj)));
        self.connection.persist()?;
        Ok(Response::new(Value::Null, 1))
    }

    async fn update(&self, obj: &Record, by_where: Option<&dyn Where>) -> Result<Response> {
        self.connection.ensure_open()?;
        let filter = if let Some(w) = by_where {
            parse_filter(Some(&w.get_text()))?
        } else {
            identity_filter(obj)
                .into_iter()
                .map(|(k, v)| (k, value_to_json(&v)))
                .collect()
        };
        let json = json_from_record(obj);
        let affected = self.connection.collection_mut(|items| {
            let mut count = 0i64;
            for item in items.iter_mut() {
                if matches(item, &filter) {
                    if let (JsonValue::Object(target), JsonValue::Object(src)) = (item, &json) {
                        for (k, v) in src {
                            target.insert(k.clone(), v.clone());
                        }
                    }
                    count += 1;
                }
            }
            count
        });
        self.connection.persist()?;
        Ok(Response::new(Value::Null, affected))
    }

    async fn delete(&self, obj: &Record, by_where: Option<&dyn Where>) -> Result<Response> {
        self.connection.ensure_open()?;
        let filter = if let Some(w) = by_where {
            parse_filter(Some(&w.get_text()))?
        } else {
            identity_filter(obj)
                .into_iter()
                .map(|(k, v)| (k, value_to_json(&v)))
                .collect()
        };
        let affected = self.connection.collection_mut(|items| {
            let before = items.len();
            items.retain(|row| !matches(row, &filter));
            (before - items.len()) as i64
        });
        self.connection.persist()?;
        Ok(Response::new(Value::Null, affected))
    }
}

pub struct JsonDatasource {
    path: String,
    collection: Option<String>,
    store: Arc<dyn FileStore>,
    initial: Option<JsonValue>,
}

impl JsonDatasource {
    pub fn factory() -> Arc<Self> {
        Arc::new(Self::new(
            "",
            None,
            Arc::new(MemoryFileStore::new()),
            Some(JsonValue::Array(vec![])),
        ))
    }

    pub fn new(
        path: impl Into<String>,
        collection: Option<String>,
        store: Arc<dyn FileStore>,
        initial: Option<JsonValue>,
    ) -> Self {
        Self {
            path: path.into(),
            collection,
            store,
            initial,
        }
    }

    fn typed_connection(&self) -> Arc<JsonConnection> {
        Arc::new(JsonConnection::new(
            self.path.clone(),
            self.collection.clone(),
            self.store.clone(),
            self.initial.clone(),
        ))
    }
}

impl Datasource for JsonDatasource {
    fn provider(&self) -> &str {
        "json"
    }

    fn create_connection(&self) -> Arc<dyn Connection> {
        self.typed_connection()
    }

    fn create_command(&self, _connection: Option<Arc<dyn Connection>>) -> Arc<dyn Command> {
        Arc::new(JsonCommand::new(self.typed_connection()))
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
        Arc::new(JsonRepository::new(conn))
    }
}

impl DatasourceFactory for JsonDatasource {
    fn provider(&self) -> &str {
        "json"
    }

    fn create(&self, _options: Option<&str>) -> Arc<dyn Datasource> {
        Arc::new(JsonDatasource::new(
            self.path.clone(),
            self.collection.clone(),
            self.store.clone(),
            self.initial.clone(),
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
    async fn json_crud_and_transactions() {
        let store = Arc::new(MemoryFileStore::seed(
            "db.json",
            "[{\"id\":1,\"name\":\"a\"}]",
        ));
        let conn = Arc::new(JsonConnection::new(
            "db.json",
            None,
            store.clone(),
            None,
        ));
        conn.open().await.unwrap();
        conn.open().await.unwrap();
        assert_eq!(conn.path(), "db.json");
        let repo = JsonRepository::new(conn.clone());
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
        let command = JsonCommand::new(conn.clone());
        command
            .execute(Some("insert"), Some(&[Parameter::new("id", 3_i64)]))
            .await
            .unwrap();
        command
            .execute(
                Some("update"),
                Some(&[
                    Parameter::new("id", 3_i64),
                    Parameter::new("name", "z"),
                    Parameter::new("$where", "{\"id\":3}"),
                ]),
            )
            .await
            .unwrap();
        command
            .execute(Some("update"), Some(&[Parameter::new("id", 3_i64)]))
            .await
            .unwrap();
        command
            .execute(Some("delete"), Some(&[Parameter::new("id", 3_i64)]))
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
        repo.add(&rec(9, "t")).await.unwrap();
        txn.abort().await.unwrap();
        let txn = conn.start_transaction().await.unwrap();
        repo.add(&rec(8, "c")).await.unwrap();
        txn.commit().await.unwrap();
        txn.use_middleware(None, None);
        conn.close().await.unwrap();
        assert!(conn.ensure_open().is_err());

        let memory = Arc::new(JsonConnection::new(
            "",
            Some("users".into()),
            Arc::new(MemoryFileStore::new()),
            Some(serde_json::json!({})),
        ));
        memory.open().await.unwrap();
        JsonRepository::new(memory.clone())
            .add(&rec(1, "a"))
            .await
            .unwrap();
        *memory.state.data.lock().unwrap() = JsonValue::Null;
        memory.collection_mut(|_| {});
        *memory.state.data.lock().unwrap() = JsonValue::Array(vec![]);
        memory.collection_mut(|_| {});
        let txn = memory.start_transaction().await.unwrap();
        txn.commit().await.unwrap();

        reset();
        register(JsonDatasource::factory());
        let ds = create("json", None).unwrap();
        assert_eq!(ds.provider(), "json");
        ds.create_connection();
        ds.create_command(None);
        ds.create_command(Some(ds.create_connection()));
        ds.create_repository(None, Some("users"));
        ds.create_repository(Some(ds.create_connection()), None);
        let factory = JsonDatasource::new("", None, Arc::new(MemoryFileStore::new()), None);
        assert_eq!(DatasourceFactory::provider(&factory), "json");
        DatasourceFactory::create(&factory, None);

        let missing = MemoryFileStore::new();
        assert!(!missing.exists("x"));
        assert!(missing.read("x").is_err());
        missing.write("x", "{}").unwrap();
        assert!(missing.exists("x"));

        assert!(parse_filter(None).unwrap().is_empty());
        assert!(parse_filter(Some("   ")).unwrap().is_empty());
        assert!(parse_filter(Some("[]")).is_err());
        assert!(parse_filter(Some("not-json")).is_err());
        assert!(!matches(&JsonValue::Array(vec![]), &Map::new()));
        let _ = record_from_json(&JsonValue::Array(vec![]));
        let _ = record_from_json(&serde_json::json!({
            "n": 1.5,
            "u": null,
            "b": true,
            "s": "x",
            "o": {"a": 1}
        }));
        let _ = json_from_record(&{
            let mut r = Record::new();
            r.insert("a".into(), Value::Null);
            r.insert("b".into(), Value::Bool(true));
            r.insert("c".into(), Value::Float(1.2));
            r.insert("d".into(), Value::from("t"));
            r
        });
        let closed = Arc::new(JsonConnection::new(
            "",
            None,
            Arc::new(MemoryFileStore::new()),
            None,
        ));
        assert!(JsonCommand::new(closed.clone())
            .execute_reader(None, None)
            .await
            .is_err());
        assert!(JsonCommand::new(closed.clone())
            .execute(Some("insert"), None)
            .await
            .is_err());
        assert!(JsonRepository::new(closed.clone())
            .add(&rec(1, "a"))
            .await
            .is_err());
        assert!(JsonRepository::new(closed.clone())
            .update(&rec(1, "a"), None)
            .await
            .is_err());
        assert!(JsonRepository::new(closed)
            .delete(&rec(1, "a"), None)
            .await
            .is_err());
        assert!(JsonConnection::new("", None, Arc::new(MemoryFileStore::new()), None)
            .start_transaction()
            .await
            .is_err());

        let with_col = Arc::new(JsonConnection::new(
            "c.json",
            Some("users".into()),
            Arc::new(MemoryFileStore::seed("c.json", "{}")),
            None,
        ));
        with_col.open().await.unwrap();
        JsonCommand::new(with_col)
            .execute(Some("update"), None)
            .await
            .unwrap();

        let bad = Arc::new(JsonConnection::new(
            "bad.json",
            None,
            Arc::new(MemoryFileStore::seed("bad.json", "not-json")),
            None,
        ));
        assert!(bad.open().await.is_err());

        let empty_file = Arc::new(JsonConnection::new(
            "none.json",
            None,
            Arc::new(MemoryFileStore::new()),
            None,
        ));
        empty_file.open().await.unwrap();

        conn.use_middleware(None, None);
        json_to_value(&JsonValue::Number(serde_json::Number::from_f64(1.25).unwrap()));
        json_to_value(&serde_json::json!([1]));
        if let JsonValue::Number(n) = serde_json::json!(1) {
            json_to_value(&JsonValue::Number(n));
        }
    }
}
