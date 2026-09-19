# Connection

Connection represents a connection to a datasource and handles opening and closing. A new command or transaction can be created from an existing connection.

## Classes

::: reference
### typescript
#### IConnection

```typescript
interface IConnection
```

**Properties**

##### provider

Registered provider name.

```typescript
readonly provider: string
```

##### isOpen

Whether the connection is open.

```typescript
readonly isOpen: boolean
```

**Methods**

##### createCommand()

Creates a command bound to this connection.

```typescript
createCommand(): ICommand
```

**Returns** [ICommand](./command.md)

##### open()

Opens the connection. A no-op if already open.

```typescript
open(): Promise<void>
```

##### close()

Closes the connection.

```typescript
close(): Promise<void>
```

##### startTransaction(isolation?)

Begins a transaction. The connection must be open.

```typescript
startTransaction(isolation?: IsolationLevel): Promise<ITransaction>
```

- `isolation` — [IsolationLevel](./transaction.md#isolationlevel); omitted uses serializable

**Returns** `Promise<[ITransaction](./transaction.md)>`

### csharp
#### IConnection

```csharp
public interface IConnection
```

**Properties**

##### Provider

Registered provider name.

```csharp
string Provider { get; }
```

##### IsOpen

Whether the connection is open.

```csharp
bool IsOpen { get; }
```

**Methods**

##### CreateCommand()

Creates a command bound to this connection.

```csharp
ICommand CreateCommand()
```

**Returns** [ICommand](./command.md)

##### Open()

Opens the connection. A no-op if already open.

```csharp
Task Open()
```

##### Close()

Closes the connection.

```csharp
Task Close()
```

##### StartTransaction(isolation)

Begins a transaction. The connection must be open.

```csharp
Task<ITransaction> StartTransaction(IsolationLevel isolation = IsolationLevel.Serializable)
```

- `isolation` — [IsolationLevel](./transaction.md#isolationlevel); default is serializable

**Returns** `Task<[ITransaction](./transaction.md)>`

### rust
#### Connection

```rust
pub trait Connection: Send + Sync
```

**Methods**

##### provider()

Registered provider name.

```rust
fn provider(&self) -> &str
```

**Returns** `&str`

##### is_open()

Whether the connection is open.

```rust
fn is_open(&self) -> bool
```

**Returns** `bool`

##### create_command()

Creates a command bound to this connection.

```rust
fn create_command(&self) -> Arc<dyn Command>
```

**Returns** `Arc<dyn `[Command](./command.md)`>`

##### open()

Opens the connection. A no-op if already open.

```rust
async fn open(&self) -> Result<()>
```

**Returns** `Result<()>`

##### close()

Closes the connection.

```rust
async fn close(&self) -> Result<()>
```

**Returns** `Result<()>`

##### start_transaction(isolation)

Begins a transaction. The connection must be open.

```rust
async fn start_transaction(&self, isolation: Option<IsolationLevel>) -> Result<Arc<dyn Transaction>>
```

- `isolation` — [IsolationLevel](./transaction.md#isolationlevel); `None` uses serializable

**Returns** `Result<Arc<dyn `[Transaction](./transaction.md)`>>`
:::

## Remarks

`CreateConnection` / `createConnection` / `create_connection` is async and returns a closed connection. The caller opens it. `Open` / `open` is a no-op if already open. Do not open during process bootstrap.

`StartTransaction` requires an open connection. Memory, JSON, and SQLite throw when the connection is closed.

Working dialects (`json`, `memory`, `sqlite`) open against injected ports (`IFileStore` / `FileStore`, `MemoryStore`, `ISqliteDriver` / `SqliteDriver`). Stub dialects throw not-implemented on open, close, and start-transaction.

## Examples

### Create from Default

::: example
```csharp csharp/Datasources.Tests/ConnectionTests.cs#L27-L33
```
```rust rust/datasources/src/tests/connection.rs#L25-L31
```
```typescript typescript/datasources/src/Connection.test.ts#L31-L35
```
:::

### Open and close

::: example
```csharp csharp/Providers/Memory.Tests/ConnectionLifecycleTests.cs#L7-L14
```
```rust rust/providers/memory/src/lib.rs#L48-L54
```
```typescript typescript/providers/memory/src/index.test.ts#L36-L41
```
:::

### Start a transaction

::: example
```csharp csharp/Providers/Memory.Tests/ConnectionLifecycleTests.cs#L16-L23
```
```rust rust/providers/memory/src/lib.rs#L56-L62
```
```typescript typescript/providers/memory/src/index.test.ts#L43-L48
```
:::
