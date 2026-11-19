use std::path::Path;

#[tokio::main]
async fn main() {
    let log = std::env::var("E2E_MATRIX_LOG").expect("E2E_MATRIX_LOG");
    let lines = outliers::run().await.expect("run");
    if let Some(dir) = Path::new(&log).parent() {
        std::fs::create_dir_all(dir).expect("dir");
    }
    std::fs::write(&log, if lines.is_empty() { String::new() } else { format!("{}\n", lines.join("\n")) }).expect("log");
}
