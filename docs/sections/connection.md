# Connection

Connection represents a connection to a datasource. You can create commands and transactions from the connection.

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

Opens the connection. Throws if it is already open.

```typescript
open(): Promise<void>
```

##### close()

Closes the connection.

```typescript
close(): Promise<void>
```

##### `Symbol.asyncDispose`

Closes the connection. `await using` calls this.

```typescript
[Symbol.asyncDispose](): Promise<void>
```

##### startTransaction(isolation?)

Begins a transaction. The connection must be open.

```typescript
startTransaction(isolation?: IsolationLevel): Promise<ITransaction>
```

- `isolation` — [IsolationLevel](./transaction.md#isolationlevel); omitted uses read committed

**Returns** `Promise<[ITransaction](./transaction.md)>`

### csharp
#### IConnection

```csharp
public interface IConnection : IAsyncDisposable
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

Opens the connection. Throws if it is already open.

```csharp
Task Open()
```

##### Close()

Closes the connection.

```csharp
Task Close()
```

##### DisposeAsync()

Closes the connection. `await using` calls this.

```csharp
ValueTask DisposeAsync()
```

**Returns** `ValueTask`

##### StartTransaction(isolation)

Begins a transaction. The connection must be open.

```csharp
Task<ITransaction> StartTransaction(IsolationLevel isolation = IsolationLevel.ReadCommitted)
```

- `isolation` — [IsolationLevel](./transaction.md#isolationlevel); default is read committed

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

Opens the connection. Throws if it is already open.

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

- `isolation` — [IsolationLevel](./transaction.md#isolationlevel); `None` uses read committed

**Returns** `Result<Arc<dyn `[Transaction](./transaction.md)`>>`
:::

## Remarks

{{CreateConnection|createConnection|create_connection}} returns a closed connection. It is not open until {{Open|open|open}}. Calling {{Open|open|open}} on an open connection throws. Repository {{Read|read|read}}, {{Create|create|create}}, {{Update|update|update}}, and {{Delete|delete|delete}} open a closed connection before they run.

::: prose
### typescript
Close the connection at the end of an `await using` block, or call `close`.
### csharp
Close the connection at the end of an `await using` block, or call `Close`.
### rust
There is no `using`. Call `close().await`.
:::

{{StartTransaction|startTransaction|start_transaction}} requires an open connection. Memory, JSON, and SQLite throw when the connection is closed.

Working dialects (`json`, `memory`) open against injected ports ({{IFileStore|IFileStore|FileStore}}, `MemoryStore`). SQLite, MySQL, and Postgres open the database client on the connection. Stub dialects throw not-implemented on open, close, and start-transaction.

## Examples

### Create from Default

::: example
```csharp csharp/Datasources.Tests/ConnectionTests.cs#CreateFromDefault
```
```rust rust/datasources/src/tests/connection.rs#create_from_default
```
```typescript typescript/datasources/src/Connection.test.ts#create-from-default
```
:::

### Create and run a command

::: example
```csharp csharp/Providers/Memory.Tests/ConnectionLifecycleTests.cs#CreateAndRunCommand
```
```rust rust/providers/memory/src/lib.rs#create_and_run_a_command
```
```typescript typescript/providers/memory/src/index.test.ts#create-and-run-a-command
```
:::

### Close with using

::: prose
### rust
There is no `using`. Call `close`.
:::

::: example
```csharp csharp/Providers/Memory.Tests/ConnectionLifecycleTests.cs#CloseWithUsing
```
```rust rust/providers/memory/src/lib.rs#open_and_close
```
```typescript typescript/providers/memory/src/index.test.ts#closes-with-using
```
:::

### Start a transaction

::: example
```csharp csharp/Providers/Memory.Tests/ConnectionLifecycleTests.cs#StartTransaction
```
```rust rust/providers/memory/src/lib.rs#start_a_transaction
```
```typescript typescript/providers/memory/src/index.test.ts#start-a-transaction
```
:::
