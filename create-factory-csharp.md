# Create factory (C#)

Register a factory, then create by provider name. Install only the dialect packages you use.

| Dialect | Provider | Package |
| --- | --- | --- |
| json | `json` | `Datasources.Json` |
| memory | `memory` | `Datasources.Memory` |
| sqlite | `sqlite` | `Datasources.Sqlite` (optional engine: `Datasources.Sqlite.Microsoft`) |
| text | `text` | `Datasources.Text` |
| xml | `xml` | `Datasources.Xml` |
| yaml | `yaml` | `Datasources.Yaml` |
| mysql | `mysql` | `Datasources.Mysql` |
| sqlserver | `sqlserver` | `Datasources.SqlServer` |
| postgres | `postgres` | `Datasources.Postgres` |
| oracle | `oracle` | `Datasources.Oracle` |
| mongodb | `mongodb` | `Datasources.MongoDb` |
| dynamodb | `dynamodb` | `Datasources.DynamoDb` |

```csharp
Datasource.Use(
    JsonDatasource.Instance,
    MemoryDatasource.Instance,
    SqliteDatasource.Instance,
    TextDatasource.Instance,
    XmlDatasource.Instance,
    YamlDatasource.Instance,
    MysqlDatasource.Instance,
    SqlServerDatasource.Instance,
    PostgresDatasource.Instance,
    OracleDatasource.Instance,
    MongoDbDatasource.Instance,
    DynamoDbDatasource.Instance);

var json = Datasource.Create("json");
var memory = Datasource.Create("memory");
var sqlite = Datasource.Create("sqlite");
var text = Datasource.Create("text");
var xml = Datasource.Create("xml");
var yaml = Datasource.Create("yaml");
var mysql = Datasource.Create("mysql");
var sqlserver = Datasource.Create("sqlserver");
var postgres = Datasource.Create("postgres");
var oracle = Datasource.Create("oracle");
var mongodb = Datasource.Create("mongodb");
var dynamodb = Datasource.Create("dynamodb");
```

Working dialects (json, memory, sqlite) accept options on `Create`. Stub dialects return a stub datasource.

## Default provider in apps

The library does not load config. The consumer must call `Datasource.Defaults` with a dictionary. `Create()`, `CreateConnection`, `CreateCommand`, and `CreateRepository` then use those defaults.

`Datasources.Hosting` can register several factories from the same dictionary: `providers` / `DATASOURCE_PROVIDERS` (comma list) plus `provider` / `DATASOURCE_PROVIDER`, then sets defaults.

```csharp
var options = new Dictionary<string, string?>
{
    ["provider"] = Environment.GetEnvironmentVariable("DATASOURCE_PROVIDER") ?? "sqlite",
    ["providers"] = Environment.GetEnvironmentVariable("DATASOURCE_PROVIDERS"),
    ["filename"] = Environment.GetEnvironmentVariable("SQLITE_FILENAME") ?? ":memory:",
    ["url"] = Environment.GetEnvironmentVariable("DATABASE_URL"),
    ["schema"] = Environment.GetEnvironmentVariable("DATABASE_SCHEMA")
};
DatasourceInstall.From(options);

var datasource = Datasource.Create();
var connection = await datasource.CreateConnection();
var command = datasource.CreateCommand();
```

