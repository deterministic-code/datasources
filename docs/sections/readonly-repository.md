# Readonly repository

Get-only typed read over a connection. `GetSingle` / `getSingle` / `get_single` returns the first row or none. `Get` / `get` yields every row.

## Classes

::: reference
### typescript
#### IReadonlyRepository

```typescript
interface IReadonlyRepository<T = unknown> extends IRepository<T>
```

**Methods**

##### getSingle(query?)

Returns the first item, or `undefined` when the result is empty.

```typescript
getSingle(query?: IQuery | IWhere): Promise<T | undefined>
```

- `query` — optional query or where; omitted uses the default select

**Returns** `Promise<T | undefined>`

##### get(query?)

Yields every item.

```typescript
get(query?: IQuery | IWhere): AsyncIterable<T>
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

##### get(query?)

Yields every item. Implement in a subclass.

```typescript
abstract get(query?: IQuery | IWhere): AsyncIterable<T>
```

- `query` — optional query or where; omitted uses the default select

**Returns** `AsyncIterable<T>`

##### getSingle(query?)

Returns the first item from [get](#readonlyrepository-get-query), or `undefined`.

```typescript
getSingle(query?: IQuery | IWhere): Promise<T | undefined>
```

- `query` — optional query or where; omitted uses the default select

**Returns** `Promise<T | undefined>`

### csharp
#### IReadonlyRepository

```csharp
public interface IReadonlyRepository<T> : IRepository
```

**Methods**

##### GetSingle(query)

Returns the first item, or `default` when the result is empty.

```csharp
Task<T?> GetSingle(IStatement? query = null)
```

- `query` — optional statement or where; omitted uses the default select

**Returns** `Task<T?>`

##### Get(query)

Yields every item.

```csharp
IAsyncEnumerable<T> Get(IStatement? query = null)
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

##### Get(query)

Yields every item. Implement in a subclass.

```csharp
public abstract IAsyncEnumerable<T> Get(IStatement? query = null)
```

- `query` — optional statement or where; omitted uses the default select

**Returns** `IAsyncEnumerable<T>`

##### GetSingle(query)

Returns the first item from [Get](#readonlyrepository-get-query), or `default`.

```csharp
public virtual Task<T?> GetSingle(IStatement? query = null)
```

- `query` — optional statement or where; omitted uses the default select

**Returns** `Task<T?>`

### rust
#### ReadonlyRepository

```rust
pub trait ReadonlyRepository
```

**Methods**

##### entity_type()

Item `TypeId`. Defaults to `Row`.

```rust
fn entity_type(&self) -> TypeId
```

**Returns** `TypeId`

##### get_single(query)

Returns the first row, or `None` when the result is empty.

```rust
async fn get_single(&self, query: Option<&dyn Query>) -> Result<Option<Row>>
```

- `query` — optional query; omitted uses the default select

**Returns** `Result<Option<Row>>`

##### get(query)

Returns every row.

```rust
async fn get(&self, query: Option<&dyn Query>) -> Result<Vec<Row>>
```

- `query` — optional query; omitted uses the default select

**Returns** `Result<Vec<Row>>`

##### get_single_where(where_clause)

Returns the first row matching a where.

```rust
async fn get_single_where(&self, where_clause: &dyn Where) -> Result<Option<Row>>
```

- `where_clause` — predicate

**Returns** `Result<Option<Row>>`

##### get_where(where_clause)

Returns every row matching a where.

```rust
async fn get_where(&self, where_clause: &dyn Where) -> Result<Vec<Row>>
```

- `where_clause` — predicate

**Returns** `Result<Vec<Row>>`
:::

## Remarks

Repository get is convert, then map. See [TypeConverter](./type-converters.md) and [IMapper](./mappers.md).

C# and TypeScript accept `IQuery` / `IWhere` (C#: `IStatement`). Rust splits where into `get_where` / `get_single_where`.

## Examples

### Get and get single

::: example
```csharp csharp/Datasources.Tests/RepositoryTests.cs#L62-L72
```
```rust rust/datasources/src/tests/mod.rs#L845-L851
```
```typescript typescript/datasources/src/repository.test.ts#L63-L73
```
:::
