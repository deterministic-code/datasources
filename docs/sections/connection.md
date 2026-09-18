# Connection

`CreateConnection` / `createConnection` / `create_connection` is async so a dialect can do async work later. Today it only constructs a closed named connection. The caller opens it.

## IConnection

| C# | Rust | TypeScript |
| --- | --- | --- |
| `IConnection` | `Connection` | `IConnection` |
| `Provider` | `provider()` | `provider` |
| `IsOpen` | `is_open()` | `isOpen` |
| `CreateCommand` | `create_command` | `createCommand` |
| `Open` | `open` | `open` |
| `Close` | `close` | `close` |
| `StartTransaction` | `start_transaction` | `startTransaction` |

Working dialects (`json`, `memory`, `sqlite`) open against injected ports (`IFileStore` / `FileStore`, `MemoryStore`, `ISqliteDriver` / `SqliteDriver`). Stub dialects return a named connection; open / close / start-transaction throw or return not-implemented.

## Transaction

`StartTransaction` / `startTransaction` / `start_transaction` begins a transaction on an open connection.

## Create and open

::: dialects
```csharp
Datasource.Register(JsonDatasource.Instance, MemoryDatasource.Instance, SqliteDatasource.Instance);

var json = Datasource.Create("json", new JsonConnectionOptions { Data = new JsonArray() });
var connection = await json.CreateConnection();
await connection.Open();
```
```rust
use datasources::{create, register};
use datasources_json::JsonDatasource;

register(JsonDatasource::instance());

let json = create("json", None)?;
let connection = json.create_connection().await?;
connection.open().await?;
```
```typescript
import { Datasource } from "@datasources/core";
import { JsonDatasource } from "@datasources/json";

Datasource.register(JsonDatasource.instance);

const json = Datasource.create("json", { data: [] });
const connection = await json.createConnection();
await connection.open();
```
:::

## Startup

Apps create the default datasource and connection at process start, then hold the closed connection. Call `Open` / `open` on first use (a no-op if already open). Do not open during bootstrap.

::: dialects
```csharp
Datasource.Defaults(options);
var datasource = Datasource.Create();
var connection = await datasource.CreateConnection();

Task EnsureOpen() => connection.IsOpen ? Task.CompletedTask : connection.Open();
```
```rust
use std::sync::Arc;
use datasources::{create_default, defaults, Connection, Result};

defaults(options);
let datasource = create_default()?;
let connection = datasource.create_connection().await?;

async fn ensure_open(connection: &Arc<dyn Connection>) -> Result<()> {
    if connection.is_open() {
        return Ok(());
    }
    connection.open().await
}
```
```typescript
Datasource.defaults(options);
const datasource = Datasource.create();
const connection = await datasource.createConnection();

async function ensureOpen() {
  if (!connection.isOpen) await connection.open();
}
```
:::

## Mock

Depend on `IConnection` / `Connection`. Use `MockConnection` / `MockDatasource`. `CreateConnection` returns the configured mock and does not open it.

::: dialects
```csharp
var ds = new MockDatasource();
var connection = await ds.CreateConnection();
await connection.Open();
Assert.True(((MockConnection)connection).IsOpen);

var failing = new MockConnection { OpenError = new InvalidOperationException("open-fail") };
ds.Connection = failing;
await Assert.ThrowsAsync<InvalidOperationException>(() => failing.Open());
```
```rust
use datasources::{Datasource, MockConnection, MockDatasource};

let ds = MockDatasource::new("mock");
let connection = ds.create_connection().await?;
connection.open().await?;

let failing = MockConnection::new();
failing.set_open_error("open-fail");
assert!(failing.open().await.is_err());
```
```typescript
const ds = new MockDatasource();
const connection = await ds.createConnection();
await connection.open();
expect((connection as MockConnection).isOpen).toBe(true);

const failing = new MockConnection();
failing.openError = new Error("open-fail");
await expect(failing.open()).rejects.toThrow("open-fail");
```
:::

Provider tests inject fakes on options (`MemoryFileStore`, `ScriptedDriver`) and then open.
