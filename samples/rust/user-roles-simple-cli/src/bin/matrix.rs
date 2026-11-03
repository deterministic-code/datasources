use datasources::ConsoleCommandMiddleware;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use user_roles_simple_cli::program;

#[derive(Deserialize)]
struct CliSpec {
    now: String,
    steps: Vec<Step>,
}

#[derive(Deserialize)]
struct Step {
    argv: Vec<String>,
}

#[tokio::main]
async fn main() {
    let log = std::env::var("E2E_MATRIX_LOG").expect("E2E_MATRIX_LOG");
    let spec: CliSpec = serde_yaml::from_str(
        &std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../user-roles-simple-cli-shared/cli.yaml"),
        )
        .expect("spec"),
    )
    .expect("yaml");
    datasources::register("sqlite", Arc::new(datasources_sqlite::SqliteDriver));
    datasources::register("mysql", Arc::new(datasources_mysql::MysqlDriver));
    datasources::register("postgres", Arc::new(datasources_postgres::PostgresDriver));
    let ds = datasources::default().expect("datasource");
    let connection = ds.create_connection().await.expect("connection");
    connection.open().await.expect("open");
    let command = ds.create_command(Some(connection.clone())).await.expect("command");
    let lines = Arc::new(Mutex::new(Vec::<String>::new()));
    let stamp = spec.now.clone();
    command.add_middleware(Arc::new(
        ConsoleCommandMiddleware::with_sink(
            {
                let lines = Arc::clone(&lines);
                move |line| lines.lock().expect("log").push(line.to_string())
            },
            move || stamp.clone(),
        )
        .options(HashMap::from([("isVerbose".into(), "true".into())])),
    ));
    let result = async {
        for step in spec.steps {
            program::run(command.as_ref(), &step.argv, |_| Ok("hash".into()), false).await?;
        }
        Ok::<(), datasources::DatasourceError>(())
    }
    .await;
    if let Some(dir) = std::path::Path::new(&log).parent() {
        std::fs::create_dir_all(dir).ok();
    }
    std::fs::write(&log, format!("{}\n", lines.lock().expect("log").join("\n"))).expect("write");
    result.expect("steps");
}
