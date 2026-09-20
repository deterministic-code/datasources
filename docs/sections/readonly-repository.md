# Readonly repository

Read-only typed read over a connection. `ReadSingle` / `readSingle` / `read_single` returns the first row or none. `Read` / `read` yields every row.

## Classes

::: reference
### typescript
#### IReadonlyRepository

```typescript
interface IReadonlyRepository<T = unknown> extends IRepository<T>
```

**Methods**

##### readSingle(query?)

Returns the first item, or `undefined` when the result is empty.

```typescript
readSingle(query?: IQuery | IWhere): Promise<T | undefined>
```

- `query` — optional query or where; omitted uses the default select

**Returns** `Promise<T | undefined>`

##### read(query?)

Yields every item.

```typescript
read(query?: IQuery | IWhere): AsyncIterable<T>
```

- `query` — optional query or where; omitted uses the default select

**Returns** `AsyncIterable<T>`

#### ReadonlyRepository

```typescript
abstract class ReadonlyRepository<T> implements IReadonlyRepository<T>
```

**Constructor**

##### ReadonlyRepository(type)

Stores the item constructor.

```typescript
constructor(type: Constructable<T>)
```

- `type` — item constructor ([IRepository.type](./repository.md#irepository-type))

**Properties**

##### type

Item constructor the repository exposes.

```typescript
readonly type: Constructable<T>
```

**Methods**

##### read(query?)

Yields every item. Implement in a subclass.

```typescript
abstract read(query?: IQuery | IWhere): AsyncIterable<T>
```

- `query` — optional query or where; omitted uses the default select

**Returns** `AsyncIterable<T>`

##### readSingle(query?)

Returns the first item from [read](#readonlyrepository-read-query), or `undefined`.

```typescript
readSingle(query?: IQuery | IWhere): Promise<T | undefined>
```

- `query` — optional query or where; omitted uses the default select

**Returns** `Promise<T | undefined>`

### csharp
#### IReadonlyRepository

```csharp
public interface IReadonlyRepository<T> : IRepository
```

**Methods**

##### ReadSingle(query)

Returns the first item, or `default` when the result is empty.

```csharp
Task<T?> ReadSingle(IStatement? query = null)
```

- `query` — optional statement or where; omitted uses the default select

**Returns** `Task<T?>`

##### Read(query)

Yields every item.

```csharp
IAsyncEnumerable<T> Read(IStatement? query = null)
```

- `query` — optional statement or where; omitted uses the default select

**Returns** `IAsyncEnumerable<T>`

#### ReadonlyRepository

```csharp
public abstract class ReadonlyRepository<T> : IReadonlyRepository<T>
```

**Properties**

##### Type

Item CLR type. `typeof(T)`.

```csharp
Type Type { get; }
```

**Methods**

##### Read(query)

Yields every item. Implement in a subclass.

```csharp
public abstract IAsyncEnumerable<T> Read(IStatement? query = null)
```

- `query` — optional statement or where; omitted uses the default select

**Returns** `IAsyncEnumerable<T>`

##### ReadSingle(query)

Returns the first item from [Read](#readonlyrepository-read-query), or `default`.

```csharp
public virtual Task<T?> ReadSingle(IStatement? query = null)
```

- `query` — optional statement or where; omitted uses the default select

**Returns** `Task<T?>`

### rust
#### ReadonlyRepository

```rust
pub trait ReadonlyRepository: IRepository
```

**Methods**

##### read_single(query)

Returns the first row, or `None` when the result is empty.

```rust
async fn read_single(&self, query: Option<&dyn Query>) -> Result<Option<Row>>
```

- `query` — optional query; omitted uses the default select

**Returns** `Result<Option<Row>>`

##### read(query)

Returns every row.

```rust
async fn read(&self, query: Option<&dyn Query>) -> Result<Vec<Row>>
```

- `query` — optional query; omitted uses the default select

**Returns** `Result<Vec<Row>>`

##### read_single_where(where_clause)

Returns the first row matching a where.

```rust
async fn read_single_where(&self, where_clause: &dyn Where) -> Result<Option<Row>>
```

- `where_clause` — predicate

**Returns** `Result<Option<Row>>`

##### read_where(where_clause)

Returns every row matching a where.

```rust
async fn read_where(&self, where_clause: &dyn Where) -> Result<Vec<Row>>
```

- `where_clause` — predicate

**Returns** `Result<Vec<Row>>`
:::

## Remarks

Repository read is convert, then map. See [TypeConverter](./type-converters.md) and [Mapper](./mappers.md).

C# and TypeScript accept `IQuery` / `IWhere` (C#: `IStatement`). Rust splits where into `read_where` / `read_single_where`.

## Examples

### Read and read single

::: example
```csharp csharp/Datasources.Tests/RepositoryTests.cs#L62-L72
```
```rust rust/datasources/src/tests/mod.rs#L845-L851
```
```typescript typescript/datasources/src/repository.test.ts#L63-L73
```
:::
