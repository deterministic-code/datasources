# Connection

`CreateConnection` / `createConnection` / `create_connection` is async so a dialect can do async work later. Today it only constructs a closed named connection. The caller opens it.

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

- `isolation` — [IsolationLevel](#isolationlevel); omitted uses serializable

**Returns** `Promise<[ITransaction](#itransaction)>`

#### ITransaction

```typescript
interface ITransaction
```

**Properties**

##### connection

The open [IConnection](#iconnection) that started this transaction.

```typescript
readonly connection: IConnection
```

##### isolation

The isolation level for this transaction.

```typescript
readonly isolation: IsolationLevel
```

**Methods**

##### commit()

Commits the transaction.

```typescript
commit(): Promise<void>
```

##### abort()

Rolls the transaction back.

```typescript
abort(): Promise<void>
```

#### IsolationLevel

```typescript
const IsolationLevel
```

**Properties**

##### ReadUncommitted

Dirty reads are allowed.

```typescript
ReadUncommitted: "readUncommitted"
```

##### ReadCommitted

Committed data only.

```typescript
ReadCommitted: "readCommitted"
```

##### RepeatableRead

Repeatable reads within the transaction.

```typescript
RepeatableRead: "repeatableRead"
```

##### Serializable

Full isolation. The default for [startTransaction](#starttransaction-isolation).

```typescript
Serializable: "serializable"
```

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

- `isolation` — [IsolationLevel](#isolationlevel); default is serializable

**Returns** `Task<[ITransaction](#itransaction)>`

#### ITransaction

```csharp
public interface ITransaction
```

**Properties**

##### Connection

The open [IConnection](#iconnection) that started this transaction.

```csharp
IConnection Connection { get; }
```

##### Isolation

The isolation level for this transaction.

```csharp
IsolationLevel Isolation { get; }
```

**Methods**

##### Commit()

Commits the transaction.

```csharp
Task Commit()
```

##### Abort()

Rolls the transaction back.

```csharp
Task Abort()
```

#### IsolationLevel

```csharp
public enum IsolationLevel
```

**Properties**

##### ReadUncommitted

Dirty reads are allowed.

```csharp
ReadUncommitted
```

##### ReadCommitted

Committed data only.

```csharp
ReadCommitted
```

##### RepeatableRead

Repeatable reads within the transaction.

```csharp
RepeatableRead
```

##### Serializable

Full isolation. The default for [StartTransaction](#starttransaction-isolation).

```csharp
Serializable
```

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

- `isolation` — [IsolationLevel](#isolationlevel); `None` uses serializable

**Returns** `Result<Arc<dyn `[Transaction](#transaction)`>>`

#### Transaction

```rust
pub trait Transaction: Send + Sync
```

**Methods**

##### isolation()

The isolation level for this transaction.

```rust
fn isolation(&self) -> IsolationLevel
```

**Returns** [IsolationLevel](#isolationlevel)

##### commit()

Commits the transaction.

```rust
async fn commit(&self) -> Result<()>
```

**Returns** `Result<()>`

##### abort()

Rolls the transaction back.

```rust
async fn abort(&self) -> Result<()>
```

**Returns** `Result<()>`

#### IsolationLevel

```rust
pub enum IsolationLevel
```

**Properties**

##### ReadUncommitted

Dirty reads are allowed.

```rust
ReadUncommitted
```

##### ReadCommitted

Committed data only.

```rust
ReadCommitted
```

##### RepeatableRead

Repeatable reads within the transaction.

```rust
RepeatableRead
```

##### Serializable

Full isolation. The default for [start_transaction](#start_transaction-isolation).

```rust
Serializable
```
:::

## Transaction {#transaction}

`StartTransaction` / `startTransaction` / `start_transaction` begins a transaction on an open connection.

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
