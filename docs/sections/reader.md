# Reader

Streams rows from [Command](./command.md) {{ExecuteReader|executeReader|execute_reader}}.

## Classes

::: reference
### typescript
#### IReader

```typescript
interface IReader<T>
```

**Properties**

##### current

The row from the last successful [next](#ireader-next).

```typescript
readonly current: T | undefined
```

**Methods**

##### next()

Advances to the next row.

```typescript
next(): Promise<boolean>
```

**Returns** `Promise<boolean>`

##### read()

Reads the current result set. Skips a step whose [current](#ireader-current) is undefined.

```typescript
read(): AsyncIterable<T>
```

**Returns** `AsyncIterable<T>`

##### nextResult()

Moves to the next result set and clears [current](#ireader-current). Returns false when there is no further set. [next](#ireader-next) then reads that set. Calling only [next](#ireader-next) leaves later sets unread.

```typescript
nextResult(): Promise<boolean>
```

**Returns** `Promise<boolean>`

##### dispose()

Releases the reader.

```typescript
dispose(): Promise<void>
```

**Returns** `Promise<void>`

##### [Symbol.asyncDispose]()

`await using` calls [dispose](#ireader-dispose).

```typescript
[Symbol.asyncDispose](): Promise<void>
```

**Returns** `Promise<void>`

### csharp
#### IReader

```csharp
public interface IReader<out T>
```

**Properties**

##### Current

The row from the last successful [Next](#ireader-next).

```csharp
T? Current { get; }
```

**Methods**

##### Next()

Advances to the next row.

```csharp
Task<bool> Next()
```

**Returns** `Task<bool>`

##### Read()

Reads the current result set. Skips a step whose [Current](#ireader-current) is null. `ToListAsync` materializes the sequence.

```csharp
IAsyncEnumerable<T> Read()
```

**Returns** `IAsyncEnumerable<T>`

##### NextResult()

Moves to the next result set and clears [Current](#ireader-current). Returns false when there is no further set. [Next](#ireader-next) then reads that set. A reader with one set returns false. Calling only [Next](#ireader-next) leaves later sets unread.

```csharp
Task<bool> NextResult()
```

**Returns** `Task<bool>`

##### DisposeAsync()

Releases the reader. `await using` calls this.

```csharp
ValueTask DisposeAsync()
```

**Returns** `ValueTask`

### rust
#### Reader

```rust
pub trait Reader<T: Send = Row>: Send + Sync
```

**Methods**

##### current()

The row from the last successful [next](#reader-next).

```rust
fn current(&self) -> Option<T>
```

**Returns** `Option<T>`

##### next()

Advances to the next row.

```rust
async fn next(&self) -> Result<bool>
```

**Returns** `Result<bool>`

##### read()

Reads the current result set. Skips a step whose [current](#reader-current) is `None`. `try_collect` materializes the stream.

```rust
fn read(&self) -> Pin<Box<dyn Stream<Item = Result<T>> + Send + '_>>
```

**Returns** `Pin<Box<dyn Stream<Item = Result<T>> + Send + '_>>`

##### next_result()

Moves to the next result set and clears [current](#reader-current). Returns false when there is no further set. [next](#reader-next) then reads that set. The default returns false. Calling only [next](#reader-next) leaves later sets unread.

```rust
async fn next_result(&self) -> Result<bool>
```

**Returns** `Result<bool>`
:::

## Remarks

{{ExecuteReader|executeReader|execute_reader}} returns a reader for the first result set. {{Next|next|next}} reads that set. {{NextResult|nextResult|next_result}} advances to the next set. Rows stay unread until {{Next|next|next}}.

Dispose the reader with `await using`. Rust drops it.

## Examples

### Read a row

::: example
```csharp csharp/Providers/Sqlite.Tests/SqliteCommandIntegrationTests.cs#RunExecuteReaderOnASimpleList
```
```rust rust/providers/sqlite/tests/command.rs#run_execute_reader_on_a_simple_list
```
```typescript typescript/providers/sqlite/tests/sqlite.integration.test.ts#run-executereader-on-a-simple-list
```

```output [Rows]
[
  { id: 1, name: 'ada' },
  { id: 2, name: 'grace' },
  { id: 3, name: 'bea' }
]
```
:::

### Read and apply case mappings

::: example
```csharp csharp/Providers/Sqlite.Tests/SqliteCommandIntegrationTests.cs#ReadAndApplyCaseMappings
```
```rust rust/providers/sqlite/tests/command.rs#read_and_apply_case_mappings
```
```typescript typescript/providers/sqlite/tests/sqlite.integration.test.ts#read-and-apply-case-mappings
```

```output [Rows]
[
  { UserId: 1, FirstName: 'Ada' }
]
```
:::

### ExecuteReader, then apply flat to nested mapper.

::: example
```csharp csharp/Providers/Sqlite.Tests/SqliteCommandIntegrationTests.cs#ExecuteReaderThenApplyFlatToNestedMapper
```
```rust rust/providers/sqlite/tests/command.rs#execute_reader_then_apply_flat_to_nested_mapper
```
```typescript typescript/providers/sqlite/tests/sqlite.integration.test.ts#executereader-then-apply-flat-to-nested-mapper
```

```output [Contact]
{
  id: 1,
  username: 'bobby',
  contacts: [
    { name: 'bob' },
    { name: 'roberta' }
  ]
}
```
:::

### ExecuteReader maps flat contacts

::: example
```csharp csharp/Providers/Sqlite.Tests/SqliteCommandIntegrationTests.cs#User
```
```csharp csharp/Providers/Sqlite.Tests/SqliteCommandIntegrationTests.cs#ExecuteReaderMapsFlatContacts
```
```rust rust/providers/sqlite/tests/command.rs#Contact
```
```rust rust/providers/sqlite/tests/command.rs#User
```
```rust rust/providers/sqlite/tests/command.rs#execute_reader_maps_flat_contacts
```
```typescript typescript/providers/sqlite/tests/sqlite.integration.test.ts#Contact
```
```typescript typescript/providers/sqlite/tests/sqlite.integration.test.ts#User
```
```typescript typescript/providers/sqlite/tests/sqlite.integration.test.ts#maps-flat-contacts
```

```output [User]
{
  id: 1,
  username: 'bobby',
  email: 'bob@asdf.com',
  contacts: [
    { name: 'bob' },
    { name: 'roberta' }
  ]
}
```
:::

### Read multiple results returned in a single command

::: example
```csharp csharp/Providers/Sqlite.Tests/SqliteCommandIntegrationTests.cs#ReadMultipleResults
```
```rust rust/providers/sqlite/tests/command.rs#read_multiple_results
```
```typescript typescript/providers/sqlite/tests/sqlite.integration.test.ts#reads-multiple-results-returned-in-a-single-command
```

```output [First result]
{
  id: 1
}
```

```output [Second result]
{
  id: 2
}
```
:::

::: prose
### rust
There is no `using`. The reader is dropped at the end of the scope.
:::

### Call a stored procedure that returns two results

::: example
```csharp csharp/Providers/Mysql.Tests/MysqlCommandIntegrationTests.cs#CallProcedureReturnsTwoResults
```
```rust rust/providers/mysql/tests/integration.rs#call_procedure_returns_two_results
```
```typescript typescript/providers/mysql/tests/mysql.integration.test.ts#callProcedureReturnsTwoResults
```

```output [First result]
{
  id: 1
}
```

```output [Second result]
{
  id: 2
}
```
:::
