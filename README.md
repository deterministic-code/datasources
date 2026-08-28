# Datasources

Async interfaces for reading, writing, updating, and deleting resources. Providers implement the same surface for files, SQL, and (later) document stores. Install only the providers you use.

## Layout

```
typescript/   npm workspaces (datasources + providers/{json,memory,sqlite,mysql,postgres,stubs})
rust/         cargo workspace (datasources + providers/{json,memory,sqlite,mysql,postgres,stubs})
csharp/       .NET 10 solution (Datasources + Providers/{Json,Memory,Sqlite,Mysql,Postgres,stubs})
```

## Create

Register a factory, then create by provider name. See:

- [C#](create-factory-csharp.md)
- [TypeScript](create-factory-typescript.md)
- [Rust](create-factory-rust.md)

Create a closed connection, then open it. See:

- [C#](create-connection-csharp.md)
- [TypeScript](create-connection-typescript.md)
- [Rust](create-connection-rust.md)

Create a named command, then execute it. See:

- [C#](create-command-csharp.md)
- [TypeScript](create-command-typescript.md)
- [Rust](create-command-rust.md)

## Interfaces

| Type | Role |
| --- | --- |
| `IDatasource` | Factory: `createConnection` (async, construct only), `createCommand`, `createRepository`, `createReadonlyRepository` |
| `IConnection` | `open`, `close`, `startTransaction(isolation)` |
| `ITransaction` | `isolation`, `commit`, `abort` |
| `IsolationLevel` | `readUncommitted`, `readCommitted`, `repeatableRead`, `serializable` |
| `ICommand` | `executeReader`, `execute` |
| `IReadonlyRepository<T>` | `connection`, `getSingle`, `get` |
| `IRepository<T>` | extends `IReadonlyRepository<T>` with `add`, `update`, `delete` |
| `IReader` | `current`, `next` |
| `IQuery` / `Where` | `getText()` — provider-specific |
| `Parameter` / `IResponse` | `{ name, value }` / `{ result, rowsAffected }` |

`ICommand` supports `use(ICommandMiddleware)` with `beforeQuery` / `afterQuery`. A `beforeQuery` error skips the query. A query error still runs `afterQuery` with `error` and elapsed time set. `ConsoleCommandMiddleware` logs Start/Finish lines to the console.

## Providers

Working: **json**, **memory**, **sqlite**, **mysql**, **postgres** (inject `IFileStore` / `ISqliteDriver` / `IMysqlDriver` / `IPostgresDriver`). Native engines are opt-in: `better-sqlite3` / `mysql2` / `pg` (optional npm peers), `rusqlite` / `mysql_async` / `tokio-postgres`, or `Datasources.Sqlite.Microsoft` / MySqlConnector / Npgsql.

Stubs (installable, throw “not implemented”): text, xml, yaml, sqlserver, oracle, mongodb, dynamodb.

Shared SQL scenarios live in `specs/*.yaml`. Engine-specific DDL and isolation expects are in `specs/dialects/{sqlite,mysql,postgres}.yaml` (fragment names such as `createUserTable` and `dirtyReadRows`).

## Tests

`npm test` is offline: mocks, injectable ports, and in-process SQLite specs. MySQL/Postgres Docker specs are `npm run test:e2e`, which starts the matching Compose services.

```
npm test
npm run test:e2e
```

`test:e2e` defaults to `MYSQL_URL=mysql://ds:ds@127.0.0.1:3306/datasources` and `POSTGRES_URL=postgres://ds:ds@127.0.0.1:5432/datasources`. Override those env vars if the servers are elsewhere.

All languages against Docker (Rust/C# expect Compose already up):

```
npm run test:e2e
npm run test:e2e:rust
npm run test:e2e:csharp
```

- TypeScript: `npm test` / `npm run test:e2e` (Vitest; unit coverage 100% line/branch/function/statement)
- Rust: `cd rust && cargo test --workspace` (optional: `./coverage.sh` with cargo-llvm-cov)
- C#: `cd csharp && dotnet test Datasources.slnx`

CI (`.github/workflows/ci.yml`) runs `npm test` plus Rust/C# unit tests, then a second job that starts Compose and runs `test:e2e:ci` (skips starting Docker) plus Rust/C# e2e.

`Datasources` publishes test doubles (`MockConnection`, `MockCommand`, `MockRepository`, …) for consumers.
