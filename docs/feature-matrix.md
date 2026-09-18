# Feature Matrix

Parity across C#, Rust, and TypeScript. Standard modules first; opinionated repositories above them.

## Feature stack

| Feature | C# | Rust | TypeScript | Role |
| --- | --- | --- | --- | --- |
| Datasource | `IDatasource` | `Datasource` | `IDatasource` | Factory: `createConnection` (async, construct only), `createCommand`, `createRepository`, `createReadonlyRepository` |
| Mapping | `IMapper<TFrom, TTo>`, `TypeConverter<To, From>` | `Mapper<TFrom, TTo>`, `TypeConverter<To, From>` | `IMapper<TFrom, TTo>`, `TypeConverter` | column → object convert, then map |
| Connection | `IConnection` | `Connection` | `IConnection` | `open`, `close`, `startTransaction(isolation)` |
| Transaction | `ITransaction` | `Transaction` | `ITransaction` | `isolation`, `commit`, `abort` |
| Command | `ICommand` | `Command` | `ICommand` | `executeReader`, `execute` |
| Reader | `IReader` | `Reader` | `IReader` | `current`, `next` |
| Query / Where | `IQuery` / `IWhere` | `Query` / `Where` | `IQuery` / `IWhere` | `getText()` — provider-specific |
| Parameter / Response | `Parameter` / `Response` | `Parameter` / `Response` | `Parameter` / `Response` | `{ name, value }` / `{ result, rowsAffected }` |
| Readonly repository | `IReadonlyRepository<T>` | `ReadonlyRepository` | `IReadonlyRepository<T>` | `connection`, `getSingle`, `get` |
| CRUD repository | `ICRUDRepository<T>` | `CRUDRepository` | `ICRUDRepository<T>` | extends `IReadonlyRepository<T>` with `create`, `update`, `delete` |
| Enriched reading | `EnrichedReadonlyRepository<T>` | `EnrichedReadonlyRepository` | `EnrichedReadonlyRepository<T>` | fill related properties after get |
| Bulk commands | `BulkCRUDRepository<T>` | `BulkCRUDRepository` | `BulkCRUDRepository<T>` | list create / update / delete |

`ICommand` supports `addMiddleware` / `AddMiddleware` / `add_middleware` with `beforeQuery` / `afterQuery`. A `beforeQuery` error skips the query. A query error still runs `afterQuery` with `error` and elapsed time set. `ConsoleCommandMiddleware` logs Start/Finish lines to the console.

## Providers

Working: **json**, **memory**, **sqlite**, **mysql**, **postgres** (inject `IFileStore` / `ISqliteDriver` / `IMysqlDriver` / `IPostgresDriver`). Native engines are opt-in: `better-sqlite3` / `mysql2` / `pg` (optional npm peers), `rusqlite` / `mysql_async` / `tokio-postgres`, or `Datasources.Sqlite.Microsoft` / MySqlConnector / Npgsql.

Stubs (installable, throw “not implemented”): text, xml, yaml, sqlserver, oracle, mongodb, dynamodb.

Package names per language are on [QuickStart](./quickstart.md#packages).
