# Create factory (C#)

Register a factory, then create by provider name. Install only the dialect packages you use.

| Dialect | Provider | Package |
| --- | --- | --- |
| json | `json` | `Datasources.Json` |
| memory | `memory` | `Datasources.Memory` |
| sqlite | `sqlite` | `Datasources.Sqlite` |
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
    JsonDatasource.Factory,
    MemoryDatasource.Factory,
    SqliteDatasource.Factory,
    TextDatasource.Factory,
    XmlDatasource.Factory,
    YamlDatasource.Factory,
    MysqlDatasource.Factory,
    SqlServerDatasource.Factory,
    PostgresDatasource.Factory,
    OracleDatasource.Factory,
    MongoDbDatasource.Factory,
    DynamoDbDatasource.Factory);

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
