use datasources_code_generators::{register_dialect, SqlGenerateDriver};
use std::sync::Once;

pub fn register_sql() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        bind(
            "sqlite",
            datasources_sqlite::SqliteDriver::QUOTE_LEFT,
            datasources_sqlite::SqliteDriver::QUOTE_RIGHT,
            datasources_sqlite::SqliteDriver::types(),
            datasources_sqlite::SqliteDriver::defaults(),
            datasources_sqlite::SqliteDriver::statement_text_renderer(),
        );
        bind(
            "mysql",
            datasources_mysql::MysqlDriver::QUOTE_LEFT,
            datasources_mysql::MysqlDriver::QUOTE_RIGHT,
            datasources_mysql::MysqlDriver::types(),
            datasources_mysql::MysqlDriver::defaults(),
            datasources_mysql::MysqlDriver::statement_text_renderer(),
        );
        bind(
            "postgres",
            datasources_postgres::PostgresDriver::QUOTE_LEFT,
            datasources_postgres::PostgresDriver::QUOTE_RIGHT,
            datasources_postgres::PostgresDriver::types(),
            datasources_postgres::PostgresDriver::defaults(),
            datasources_postgres::PostgresDriver::statement_text_renderer(),
        );
        bind(
            "sqlserver",
            datasources_sqlserver::SqlServerDriver::QUOTE_LEFT,
            datasources_sqlserver::SqlServerDriver::QUOTE_RIGHT,
            datasources_sqlserver::SqlServerDriver::types(),
            datasources_sqlserver::SqlServerDriver::defaults(),
            datasources_sqlserver::SqlServerDriver::statement_text_renderer(),
        );
        bind(
            "oracle",
            datasources_oracle::OracleDriver::QUOTE_LEFT,
            datasources_oracle::OracleDriver::QUOTE_RIGHT,
            datasources_oracle::OracleDriver::types(),
            datasources_oracle::OracleDriver::defaults(),
            datasources_oracle::OracleDriver::statement_text_renderer(),
        );
    });
}

fn bind(
    dialect: &str,
    quote_left: &'static str,
    quote_right: &'static str,
    type_converter: &'static datasources::DeterministicTypeConverter,
    default_value_converter: &'static datasources::DefaultValueConverter,
    statement_renderer: &'static datasources::StatementTextRenderer,
) {
    register_dialect(
        dialect,
        SqlGenerateDriver {
            quote_left,
            quote_right,
            type_converter,
            default_value_converter,
            statement_renderer,
        },
    )
    .unwrap();
}
