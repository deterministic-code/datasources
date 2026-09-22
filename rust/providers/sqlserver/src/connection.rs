use datasources::named_stub_connection;

named_stub_connection!(SqlServerConnection, SqlServerProvider, "sqlserver");
