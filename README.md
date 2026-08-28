# Datasources

Async interfaces for reading, writing, updating, and deleting resources. Providers implement the same surface for files, SQL, and (later) document stores. Install only the providers you use.

## Layout

```
typescript/   npm workspaces (datasources + providers/{json,memory,sqlite,stubs})
rust/         cargo workspace (datasources + providers/{json,memory,sqlite,stubs})
csharp/       .NET 10 solution (Datasources + Providers/{Json,Memory,Sqlite,stubs})
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
| `IDatasource` | Factory: `createConnection` (async, construct only), `createCommand`, `createRepository` |
| `IConnection` | `open`, `close`, `startTransaction(isolation)` |
| `ITransaction` | `isolation`, `commit`, `abort` |
| `IsolationLevel` | `readUncommitted`, `readCommitted`, `repeatableRead`, `serializable` |
| `ICommand` | `executeReader`, `execute` |
| `IRepository<T>` | `getSingle`, `get`, `add`, `update`, `delete` |
| `IReader` | `current`, `next` |
| `IQuery` / `Where` | `getText()` — provider-specific |
| `Parameter` / `IResponse` | `{ name, value }` / `{ result, rowsAffected }` |

`IConnection`, `ITransaction`, and `ICommand` support `use(before, after)` middleware. A `before` error skips the operation. An operation error still runs `after` with `ctx.error` set.

## Providers

Working: **json**, **memory**, **sqlite** (inject `IFileStore` / `ISqliteDriver`; production adapters wrap the OS filesystem or a native SQLite module).

Stubs (installable, throw “not implemented”): text, xml, yaml, mysql, sqlserver, postgres, oracle, mongodb, dynamodb.

## Unit tests and coverage

Unit tests use mocks and injectable ports (no real files or network). SQLite Command and Transaction behavior is confirmed across C#, TypeScript, and Rust by shared specs in `specs/sqlite/*.yaml` (same SQL, parameters, and assertions).

- TypeScript: `cd typescript && npm test` (Vitest, 100% line/branch/function/statement)
- Rust: `cd rust && cargo test --workspace` (optional: `./coverage.sh` with cargo-llvm-cov)
- C#: `cd csharp && dotnet test Datasources.slnx`

`Datasources` publishes test doubles (`MockConnection`, `MockCommand`, `MockRepository`, …) for consumers.
