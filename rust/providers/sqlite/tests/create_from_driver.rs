use datasources::{BuiltDatasource, ConvertSchema, Datasource};
use std::collections::HashMap;
use std::sync::Arc;

use crate::SqliteDriver;

struct Item;

impl ConvertSchema for Item {
    fn schema() -> Vec<(&'static str, &'static str)> {
        vec![("id", "INTEGER"), ("name", "TEXT")]
    }
}

#[test]
fn create_from_driver() {
    let driver = Arc::new(SqliteDriver);
    let ds: Arc<dyn Datasource> = Arc::new(BuiltDatasource::new(
        "sqlite",
        driver,
        Some(HashMap::from([(
            "DATASOURCE_URL".into(),
            "sqlite:///:memory:".into(),
        )])),
    ));
    let repo = ds.create_repository::<Item>(None, None);
    assert_eq!(Datasource::provider(&*ds), "sqlite");
    assert_eq!(Arc::strong_count(&repo) > 0, true);
}
