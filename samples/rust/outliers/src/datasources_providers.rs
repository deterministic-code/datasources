pub fn register() {
    datasources::register("sqlite", std::sync::Arc::new(datasources_sqlite::SqliteDriver));
    datasources::register("mysql", std::sync::Arc::new(datasources_mysql::MysqlDriver));
    datasources::register("postgres", std::sync::Arc::new(datasources_postgres::PostgresDriver));
}
