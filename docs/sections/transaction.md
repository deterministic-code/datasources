# Transaction

Transaction represents a unit of work on an open connection and handles commit and abort. IsolationLevel selects the isolation for a new transaction.

## Classes

::: reference
### typescript
#### ITransaction

```typescript
interface ITransaction
```

**Properties**

##### connection

The open [IConnection](./connection.md) that started this transaction.

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

Full isolation. The default for [startTransaction](./connection.md#iconnection-starttransaction-isolation).

```typescript
Serializable: "serializable"
```

### csharp
#### ITransaction

```csharp
public interface ITransaction
```

**Properties**

##### Connection

The open [IConnection](./connection.md) that started this transaction.

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

Full isolation. The default for [StartTransaction](./connection.md#iconnection-starttransaction-isolation).

```csharp
Serializable
```

### rust
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

Full isolation. The default for [start_transaction](./connection.md#connection-start_transaction-isolation).

```rust
Serializable
```
:::

## Remarks

`StartTransaction` / `startTransaction` / `start_transaction` requires an open [connection](./connection.md). It returns a transaction that is not committed.

`Commit` / `commit` persists work. `Abort` / `abort` rolls it back.

Omitted isolation is `Serializable` (TypeScript omits the argument, C# defaults, Rust uses `None`). Memory, JSON, and SQLite throw when the connection is closed.

## Examples

### Start a transaction

::: example
```csharp csharp/Datasources.Tests/TransactionTests.cs#L27-L35
```
```rust rust/datasources/src/tests/transaction.rs#L21-L28
```
```typescript typescript/datasources/src/Transaction.test.ts#L31-L37
```
:::

### Commit

::: example
```csharp csharp/Providers/Memory.Tests/TransactionLifecycleTests.cs#L7-L15
```
```rust rust/providers/memory/src/lib.rs#L63-L69
```
```typescript typescript/providers/memory/src/index.test.ts#L52-L58
```
:::

### Abort

::: example
```csharp csharp/Providers/Memory.Tests/TransactionLifecycleTests.cs#L17-L25
```
```rust rust/providers/memory/src/lib.rs#L72-L78
```
```typescript typescript/providers/memory/src/index.test.ts#L60-L66
```
:::
