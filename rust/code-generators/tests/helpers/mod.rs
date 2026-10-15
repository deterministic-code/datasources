use datasources_code_generators::{register_dialect, SqlGenerateDriver};
use std::sync::Once;

pub fn register_sql() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        bind(
            "sqlite",
            datasources_sqlite::QUOTE_LEFT,
            datasources_sqlite::QUOTE_RIGHT,
            &*datasources_sqlite::TYPES,
            &*datasources_sqlite::DEFAULTS,
            datasources_sqlite::SqliteDriver::statement_text_renderer(),
        );
        bind(
            "mysql",
            datasources_mysql::QUOTE_LEFT,
            datasources_mysql::QUOTE_RIGHT,
            &*datasources_mysql::TYPES,
            &*datasources_mysql::DEFAULTS,
            datasources_mysql::MysqlDriver::statement_text_renderer(),
        );
        bind(
            "postgres",
            datasources_postgres::QUOTE_LEFT,
            datasources_postgres::QUOTE_RIGHT,
            &*datasources_postgres::TYPES,
            &*datasources_postgres::DEFAULTS,
            datasources_postgres::PostgresDriver::statement_text_renderer(),
        );
        bind(
            "sqlserver",
            datasources_sqlserver::QUOTE_LEFT,
            datasources_sqlserver::QUOTE_RIGHT,
            &*datasources_sqlserver::TYPES,
            &*datasources_sqlserver::DEFAULTS,
            datasources_sqlserver::SqlServerDriver::statement_text_renderer(),
        );
        bind(
            "oracle",
            datasources_oracle::QUOTE_LEFT,
            datasources_oracle::QUOTE_RIGHT,
            &*datasources_oracle::TYPES,
            &*datasources_oracle::DEFAULTS,
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
