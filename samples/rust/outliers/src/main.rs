#[tokio::main]
async fn main() -> datasources::Result<()> {
    for line in outliers::run().await? {
        println!("{line}");
    }
    Ok(())
}
