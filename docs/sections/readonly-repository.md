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

Yields every item. Table repositories convert then map; subclasses may override.

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

#### ReadonlyRepositoryOptions

```typescript
type ReadonlyRepositoryOptions<T = Cells>
```

Constructor bag for a read repository. [CRUDRepositoryOptions](./crud-repository.md#crudrepositoryoptions) extends it.

**Properties**

##### entity

Table or entity name.

```typescript
entity?: string
```

##### entityMapper

Maps the table name when the repository is built. Default is [IdentityMapper](./identity-mapper.md).

```typescript
entityMapper?: IMapper<string, string>
```

##### columns

Maps column names when the repository is built. Default is [IdentityMapper](./identity-mapper.md).

```typescript
columns?: IMapper<string, string>
```

##### query

Rewrites the finished select. Omitted leaves the statement unchanged.

```typescript
query?: IMapper<IQuery, IQuery>
```

##### rowsMapper

Maps each converted item. `Repository.mapper` is this mapper. Default is [IdentityMapper](./identity-mapper.md).

```typescript
rowsMapper?: IMapper<T, T>
```

##### rowConverter

Row [TypeConverter](./type-converters.md).

```typescript
rowConverter?: TypeConverter<T, Cells>
```

##### type

Item constructor. Supplies the default entity name and column list.

```typescript
type?: Constructable<T>
```

##### rowTypeConverters

Per-column [TypeConverter](./type-converters.md)s merged into the row converter.

```typescript
rowTypeConverters?: Record<string, TypeConverter<Cell, Cell>>
```

##### readColumns

SELECT column names. When omitted, public fields of `type` are used.

```typescript
readColumns?: string[]
```

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
public class ReadonlyRepository<T> : IReadonlyRepository<T>
```

**Properties**

##### Type

Item CLR type. `typeof(T)`.

```csharp
Type Type { get; }
```

**Methods**

##### Read(query)

Yields every item. Table repositories convert then map; subclasses may override.

```csharp
public virtual IAsyncEnumerable<T> Read(IStatement? query = null)
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

#### ReadonlyRepositoryOptions

```csharp
public class ReadonlyRepositoryOptions<T> where T : new()
```

Constructor bag for a read repository. [CRUDRepositoryOptions](./crud-repository.md#crudrepositoryoptions) extends it.

**Properties**

##### Entity

Table or entity name. When omitted, `typeof(T).Name` is used.

```csharp
string? Entity { get; init; }
```

##### EntityMapper

Maps the table name when the repository is built. Default is [IdentityMapper](./identity-mapper.md).

```csharp
IMapper<string, string> EntityMapper { get; init; }
```

##### Columns

Maps column names when the repository is built. Default is [IdentityMapper](./identity-mapper.md).

```csharp
IMapper<string, string> Columns { get; init; }
```

##### Query

Rewrites the finished select. Omitted leaves the statement unchanged.

```csharp
IMapper<IQuery, IQuery>? Query { get; init; }
```

##### RowsMapper

Maps each converted item. `Repository.Mapper` is this mapper. Default is [IdentityMapper](./identity-mapper.md).

```csharp
IMapper<T, T> RowsMapper { get; init; }
```

##### RowConverter

Row [TypeConverter](./type-converters.md).

```csharp
TypeConverter<T, Row>? RowConverter { get; init; }
```

##### RowTypeConverters

Per-column [TypeConverter](./type-converters.md)s merged into the row converter.

```csharp
IReadOnlyDictionary<string, TypeConverter<Value, Value>>? RowTypeConverters { get; init; }
```

##### ReadColumns

SELECT column names. When omitted, public properties of `T` are used.

```csharp
IReadOnlyList<string>? ReadColumns { get; init; }
```

### rust
#### IReadonlyRepository

```rust
pub trait IReadonlyRepository: IRepository
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

#### ReadonlyRepositoryOptions

```rust
pub struct ReadonlyRepositoryOptions
```

Constructor bag for a read repository. [CRUDRepositoryOptions](./crud-repository.md#crudrepositoryoptions) embeds this struct and derefs to it. `Default` sets `entity` to `"items"`.

**Properties**

##### entity

Table or entity name. Default `"items"`.

```rust
pub entity: String
```

##### entity_mapper

Maps the table name when the repository is built. Default is [IdentityMapper](./identity-mapper.md).

```rust
pub entity_mapper: Box<dyn Mapper<String, String> + Send + Sync>
```

##### columns

Maps column names when the repository is built. Default is [IdentityMapper](./identity-mapper.md).

```rust
pub columns: Box<dyn Mapper<String, String> + Send + Sync>
```

##### query

Rewrites finished select SQL. `None` leaves the statement unchanged.

```rust
pub query: Option<Arc<TransformMapper<String, String>>>
```

##### rows_mapper

Maps each converted row. Default is [IdentityMapper](./identity-mapper.md).

```rust
pub rows_mapper: Box<dyn Mapper<Row, Row> + Send + Sync>
```

##### row_converter

Row [RowTypeConverter](./row-type-converter.md).

```rust
pub row_converter: Option<RowTypeConverter>
```

##### row_type_converters

Per-column converters merged into the row converter.

```rust
pub row_type_converters: HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>
```
:::

## Remarks

Repository read is convert, then map. See [TypeConverter](./type-converters.md) and [Mapper](./mappers.md).

C# and TypeScript accept `IQuery` / `IWhere` (C#: `IStatement`). Rust splits where into `read_where` / `read_single_where`.

## Examples

### Read and read single

::: example
```csharp csharp/Datasources.Tests/RepositoryTests.cs#L61-L71
```
```rust rust/datasources/src/tests/mod.rs#L892-L898
```
```typescript typescript/datasources/src/repository.test.ts#L63-L73
```
:::
