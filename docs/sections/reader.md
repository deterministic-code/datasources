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
pub trait Reader<T = Row>: Send + Sync
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
```csharp csharp/Providers/Sqlite.Tests/SqliteCommandIntegrationTests.cs#RunRawSql
```
```rust rust/providers/sqlite/tests/command.rs#run_raw_sql
```
```typescript typescript/providers/sqlite/tests/sqlite.integration.test.ts#runs-raw-sql
```

```output [Row]
{
  id: 1,
  name: 'ada'
}
```
:::

### Read with case mapping

::: example
```csharp csharp/Providers/Sqlite.Tests/SqliteCommandIntegrationTests.cs#ReadWithCaseMapping
```
```rust rust/providers/sqlite/tests/command.rs#read_with_case_mapping
```
```typescript typescript/providers/sqlite/tests/sqlite.integration.test.ts#reads-with-case-mapping
```

```output [Row]
{
  UserId: 1,
  FirstName: 'Ada'
}
```
:::

### Type conversion and flat to nested

::: example
```csharp csharp/Providers/Sqlite.Tests/SqliteCommandIntegrationTests.cs#ReadFlatRowsToNested
```
```rust rust/providers/sqlite/tests/command.rs#read_flat_rows_to_nested
```
```typescript typescript/providers/sqlite/tests/sqlite.integration.test.ts#reads-flat-rows-to-nested
```

```output [User]
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
