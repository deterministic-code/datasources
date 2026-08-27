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

## Interfaces

| Type | Role |
| --- | --- |
| `IDatasource` | Factory: `createConnection`, `createCommand`, `createRepository` |
| `IConnection` | `open`, `close`, `startTransaction` |
| `ITransaction` | `commit`, `abort` |
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

This repo ships **unit tests only** (mocks / injectable ports). No real files, SQLite, or network. Integration and e2e tests are deferred.

- TypeScript: `cd typescript && npm test` (Vitest, 100% line/branch/function/statement)
- Rust: `cd rust && cargo test --workspace` (optional: `./coverage.sh` with cargo-llvm-cov)
- C#: `cd csharp && dotnet test Datasources.slnx`

`Datasources` publishes test doubles (`MockConnection`, `MockCommand`, `MockRepository`, …) for consumers.
