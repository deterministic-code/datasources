use super::command::SqliteCommand;
use super::connection::SqliteConnection;
use super::sqlite_path::{sqlite_path, JournalMode, SqliteOptions};
use datasources::{
    downcast_connection, Command, Connection, InsertStatement, Native, Parameter, SelectStatement,
    SelectStatementOptions, Value,
};
use std::fs;
use std::future::Future;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::thread::JoinHandle;

fn spawn_os_thread<F, Fut, T>(work: F) -> JoinHandle<T>
where
    F: FnOnce() -> Fut + Send + 'static,
    Fut: Future<Output = T> + Send,
    T: Send + 'static,
{
    let handle = tokio::runtime::Handle::current();
    std::thread::spawn(move || handle.block_on(work()))
}

struct TempDb(PathBuf);

impl TempDb {
    fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/sqlite-concurrency");
        fs::create_dir_all(&dir).unwrap();
        let id = NEXT.fetch_add(1, Ordering::Relaxed);
        let path = dir.join(format!("datasources-sqlite-{id}.db"));
        let _ = fs::remove_file(&path);
        let _ = fs::remove_file(format!("{}-wal", path.display()));
        let _ = fs::remove_file(format!("{}-shm", path.display()));
        Self(path)
    }

    fn path_string(&self) -> String {
        self.0.to_string_lossy().into_owned()
    }
}

impl Drop for TempDb {
    fn drop(&mut self) {
        let path = self.0.to_string_lossy();
        let _ = fs::remove_file(&self.0);
        let _ = fs::remove_file(format!("{path}-wal"));
        let _ = fs::remove_file(format!("{path}-shm"));
    }
}

async fn open_file(options: SqliteOptions) -> (TempDb, Arc<SqliteConnection>) {
    let temp = TempDb::new();
    let connection = Arc::new(SqliteConnection::with_options(
        Some(temp.path_string()),
        options,
    ));
    connection.open().await.unwrap();
    (temp, connection)
}

async fn exec(connection: &Arc<SqliteConnection>, sql: &str) {
    SqliteCommand::new(connection.clone())
        .execute_native(&Native::new(sql), None)
        .await
        .unwrap();
}

async fn read_select(connection: &Arc<SqliteConnection>, table: &str, field: &str) -> Option<i64> {
    let reader = SqliteCommand::new(connection.clone())
        .execute_reader_statement(
            &SelectStatement::new(SelectStatementOptions::new(table, &[field])),
            None,
        )
        .await
        .unwrap();
    if !reader.next().await.unwrap() {
        return None;
    }
    match reader.current().and_then(|row| row.into_values().next()) {
        Some(Value::Int(value)) => Some(value),
        _ => None,
    }
}

async fn read_int(connection: &Arc<SqliteConnection>, sql: &str) -> Option<i64> {
    let reader = SqliteCommand::new(connection.clone())
        .execute_reader_native(&Native::new(sql), None)
        .await
        .unwrap();
    if !reader.next().await.unwrap() {
        return None;
    }
    match reader.current().and_then(|row| row.into_values().next()) {
        Some(Value::Int(value)) => Some(value),
        _ => None,
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn twenty_concurrent_reads_succeed() {
    let (_temp, connection) = open_file(SqliteOptions::default()).await;
    exec(&connection, "CREATE TABLE t (id INTEGER PRIMARY KEY, n INTEGER)").await;
    exec(&connection, "INSERT INTO t (n) VALUES (1), (2), (3)").await;
    let mut threads = Vec::new();
    for _ in 0..20 {
        let connection = connection.clone();
        threads.push(spawn_os_thread(move || async move {
            SqliteCommand::new(connection)
                .execute_reader_statement(
                    &SelectStatement::new(SelectStatementOptions::new("t", &["n"])),
                    None,
                )
                .await
                .map(|_| ())
        }));
    }
    for thread in threads {
        thread.join().unwrap().unwrap();
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn pooled_reads_overlap() {
    let (_temp, connection) = open_file(SqliteOptions::default()).await;
    exec(&connection, "CREATE TABLE t (id INTEGER PRIMARY KEY)").await;
    exec(&connection, "INSERT INTO t (id) VALUES (1)").await;
    let threads: Vec<_> = (0..4)
        .map(|_| {
            let connection = connection.clone();
            spawn_os_thread(move || async move { read_select(&connection, "t", "id").await })
        })
        .collect();
    for thread in threads {
        assert_eq!(thread.join().unwrap(), Some(1));
    }
}

#[tokio::test]
async fn insert_id_and_committed_row_visible_to_pool() {
    let (_temp, connection) = open_file(SqliteOptions::default()).await;
    exec(&connection, "CREATE TABLE t (id INTEGER PRIMARY KEY, name TEXT)").await;
    exec(&connection, "INSERT INTO t (name) VALUES ('ada')").await;
    assert_eq!(
        read_int(&connection, "SELECT last_insert_rowid() AS id").await,
        Some(1)
    );
    assert_eq!(read_select(&connection, "t", "id").await, Some(1));
}

#[tokio::test]
async fn transaction_read_sees_uncommitted_pooled_read_does_not() {
    let (_temp, connection) = open_file(SqliteOptions::default()).await;
    exec(&connection, "CREATE TABLE t (id INTEGER PRIMARY KEY, name TEXT)").await;
    let txn = connection.start_transaction(None).await.unwrap();
    let txn_conn = downcast_connection::<SqliteConnection>(txn.connection()).unwrap();
    exec(&txn_conn, "INSERT INTO t (name) VALUES ('hidden')").await;
    assert_eq!(
        read_int(&txn_conn, "SELECT COUNT(*) FROM t WHERE name = 'hidden'").await,
        Some(1)
    );
    assert_eq!(read_select(&connection, "t", "id").await, None);
    txn.commit().await.unwrap();
    assert_eq!(read_select(&connection, "t", "id").await, Some(1));
}

#[tokio::test]
async fn memory_works_and_close_rejects_reads() {
    let connection = Arc::new(SqliteConnection::new(Some(":memory:".into())));
    connection.open().await.unwrap();
    exec(&connection, "CREATE TABLE t (id INTEGER PRIMARY KEY)").await;
    exec(&connection, "INSERT INTO t (id) VALUES (1)").await;
    assert_eq!(read_int(&connection, "SELECT id FROM t").await, Some(1));
    connection.close().await.unwrap();
    let error = match SqliteCommand::new(connection)
        .execute_reader_native(&Native::new("SELECT 1"), None)
        .await
    {
        Ok(_) => panic!("expected closed connection"),
        Err(error) => error,
    };
    assert!(error.to_string().contains("Connection is not open"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn pool_size_zero_serializes_without_errors() {
    let options = SqliteOptions {
        pool_size: Some(0),
        ..SqliteOptions::default()
    };
    let (_temp, connection) = open_file(options).await;
    exec(&connection, "CREATE TABLE t (id INTEGER PRIMARY KEY)").await;
    exec(&connection, "INSERT INTO t (id) VALUES (1)").await;
    let mut threads = Vec::new();
    for _ in 0..8 {
        let connection = connection.clone();
        threads.push(spawn_os_thread(move || async move {
            read_select(&connection, "t", "id").await
        }));
    }
    for thread in threads {
        assert_eq!(thread.join().unwrap(), Some(1));
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn concurrent_inserts_return_their_own_ids() {
    let (_temp, connection) = open_file(SqliteOptions::default()).await;
    exec(&connection, "CREATE TABLE t (id INTEGER PRIMARY KEY, n INTEGER)").await;
    let mut threads = Vec::new();
    for n in 0..40 {
        let connection = connection.clone();
        threads.push(spawn_os_thread(move || async move {
            let fields = [Parameter::new("n", n as i64)];
            let insert = InsertStatement::new("t", &fields).returning("id");
            let response = SqliteCommand::new(connection.clone())
                .execute_statement(&insert, Some(insert.fields()))
                .await
                .unwrap();
            let Value::Int(id) = response.result else {
                panic!("insert result was {:?}", response.result);
            };
            let found = read_int(&connection, &format!("SELECT n FROM t WHERE id = {id}")).await;
            (id, found, n)
        }));
    }
    let mut ids = Vec::new();
    for thread in threads {
        let (id, found, n) = thread.join().unwrap();
        assert_eq!(found, Some(n));
        ids.push(id);
    }
    ids.sort();
    ids.dedup();
    assert_eq!(ids.len(), 40);
}

#[test]
fn option_parsing_from_url() {
    let (filename, options) = sqlite_path("sqlite:///./app.sqlite").unwrap();
    assert_eq!(filename, "./app.sqlite");
    let resolved = options.unwrap().resolved("./app.sqlite");
    assert_eq!(resolved.journal_mode, Some(JournalMode::Wal));
    assert_eq!(resolved.busy_timeout, 5000);
    assert_eq!(resolved.pool_size, 8);
    assert!(sqlite_path("sqlite:///app.db?journal_mode=nope")
        .unwrap()
        .1
        .is_err());
    let (filename, _) =
        sqlite_path("sqlite:///file:x?mode=memory&cache=shared&journal_mode=wal").unwrap();
    assert_eq!(filename, "file:x?mode=memory&cache=shared");
    let options = sqlite_path("sqlite:///app.db?pool_size=0")
        .unwrap()
        .1
        .unwrap()
        .resolved("app.db");
    assert_eq!(options.pool_size, 0);
}

#[tokio::test]
async fn bad_option_fails_open() {
    let mut options = std::collections::HashMap::new();
    options.insert(
        "DATASOURCE_URL".into(),
        "sqlite:///app.db?journal_mode=fast".into(),
    );
    let connection = SqliteConnection::from_options(Some(&options));
    let error = connection.open().await.unwrap_err();
    assert!(error.to_string().contains("journal_mode=fast"));
}
