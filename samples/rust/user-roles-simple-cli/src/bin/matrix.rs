use datasources::{ConsoleCommandMiddleware, Driver};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use user_roles_simple_cli::app::App;
use user_roles_simple_cli::migrate;
use user_roles_simple_cli::parse;

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
    let provider = std::env::var("PROVIDER").unwrap_or_else(|_| "sqlite".into());
    let url = std::env::var("DATASOURCE_URL").unwrap_or_else(|_| "sqlite:///:memory:".into());
    let log = std::env::var("E2E_MATRIX_LOG").expect("E2E_MATRIX_LOG");
    let spec: CliSpec = serde_yaml::from_str(
        &std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../user-roles-simple-cli-shared/cli.yaml"),
        )
        .expect("spec"),
    )
    .expect("yaml");
    let mut options = HashMap::new();
    options.insert("DATASOURCE_URL".into(), url);
    let driver: Box<dyn Driver> = match provider.as_str() {
        "sqlite" => Box::new(datasources_sqlite::SqliteDriver),
        "mysql" => Box::new(datasources_mysql::MysqlDriver),
        "postgres" => Box::new(datasources_postgres::PostgresDriver),
        name => panic!("{name}"),
    };
    let connection = driver.connection(Some(&options));
    connection.open().await.expect("open");
    let command = connection.create_command();
    migrate::apply(command.as_ref(), connection.provider())
        .await
        .expect("migrate");
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
    let now = spec.now.clone();
    let app = App::new(command.as_ref(), connection.provider())
        .procedures(false)
        .with_clock(move || now.clone(), |_| Ok("hash".into()), || vec![1, 2, 3]);
    let result = async {
        for step in spec.steps {
            app.run(parse::args(&step.argv)?)
                .await?;
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
