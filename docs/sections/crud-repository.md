# CRUD repository

[Readonly repository](./readonly-repository.md) plus `create`, `update`, and `delete`. SQL providers typically build a table `CRUDRepository` from `RepositoryBuilder` and a [RowTypeConverter](./row-type-converter.md).

Read is convert, then map. Writes reverse-map, then reverse-convert.

## Classes

::: reference
### typescript
#### ICRUDRepository

```typescript
interface ICRUDRepository<T = unknown> extends IReadonlyRepository<T>
```

**Methods**

##### create(item)

Inserts one item.

```typescript
create(item: T): Promise<Response>
```

- `item` — row to insert

**Returns** `Promise<Response>`

##### update(item, byWhere?)

Updates one item. Omitted `byWhere` uses id columns.

```typescript
update(item: T, byWhere?: IWhere): Promise<Response>
```

- `item` — row to update
- `byWhere` — optional predicate; omitted uses id values on `item`

**Returns** `Promise<Response>`

##### delete(item, byWhere?)

Deletes one item. Omitted `byWhere` uses id columns.

```typescript
delete(item: T, byWhere?: IWhere): Promise<Response>
```

- `item` — row to delete
- `byWhere` — optional predicate; omitted uses id values on `item`

**Returns** `Promise<Response>`

#### CRUDRepository

```typescript
class CRUDRepository<T = Cells> extends ReadonlyRepository<T> implements ICRUDRepository<T>
```

**Constructor**

##### CRUDRepository(type, connection)

Builds a table repository from a type and [IConnection](./connection.md).

```typescript
constructor(type: Constructable<T>, connection: IConnection)
```

- `type` — item constructor
- `connection` — open [IConnection](./connection.md)

##### CRUDRepository(connection, options)

Builds a table repository from an [IConnection](./connection.md) and [CRUDRepositoryOptions](#crudrepositoryoptions).

```typescript
constructor(connection: IConnection, options: CRUDRepositoryOptions<T>)
```

- `connection` — open [IConnection](./connection.md)
- `options` — [CRUDRepositoryOptions](#crudrepositoryoptions)

##### CRUDRepository(built)

Builds a table repository from [BuiltRepository](#builtrepository).

```typescript
constructor(built: BuiltRepository<T>)
```

- `built` — [BuiltRepository](#builtrepository)

**Properties**

##### rowConverter

Row [TypeConverter](./type-converters.md). Convert uses this, then [mapper](#repository-mapper).

```typescript
readonly rowConverter: TypeConverter<T, Cells>
```

##### mapper

Read-row [Mapper](./mappers.md). This is [ReadonlyRepositoryOptions.rowsMapper](./readonly-repository.md#rowsmapper).

```typescript
readonly mapper: IMapper<T, T>
```

##### statements

SQL fragments for query / insert / update / delete.

```typescript
readonly statements: RepositoryStatements
```

**Methods**

##### addMiddleware(middleware)

Adds command middleware. Returns this repository.

```typescript
addMiddleware(middleware: ICommandMiddleware): this
```

- `middleware` — command middleware

**Returns** `this`

##### read(query?)

Reads rows through [rowConverter](#repository-rowconverter) then [mapper](#repository-mapper).

```typescript
read(query?: IQuery | IWhere): AsyncIterable<T>
```

- `query` — optional query or where; omitted uses the default select

**Returns** `AsyncIterable<T>`

##### create(item)

Inserts one item.

```typescript
create(item: T): Promise<Response>
```

- `item` — row to insert

**Returns** `Promise<Response>`

##### update(item, byWhere?)

Updates one item.

```typescript
update(item: T, byWhere?: IWhere): Promise<Response>
```

- `item` — row to update
- `byWhere` — optional predicate; omitted uses id values on `item`

**Returns** `Promise<Response>`

##### delete(item, byWhere?)

Deletes one item.

```typescript
delete(item: T, byWhere?: IWhere): Promise<Response>
```

- `item` — row to delete
- `byWhere` — optional predicate; omitted uses id values on `item`

**Returns** `Promise<Response>`

#### CRUDRepositoryOptions

```typescript
type CRUDRepositoryOptions<T = Cells> = ReadonlyRepositoryOptions<T> & { ... }
```

Extends [ReadonlyRepositoryOptions](./readonly-repository.md#readonlyrepositoryoptions). Constructor bag for [CRUDRepository](#repository).

**Properties**

##### ids

Id column names. When omitted, `["id"]`, or `["Id"]` when `type` has that field and no `id`.

```typescript
ids?: string[]
```

##### insert

Rewrites the finished insert. Omitted leaves the statement unchanged.

```typescript
insert?: IMapper<IInsertStatement, IInsertStatement>
```

##### update

Rewrites the finished update. Omitted leaves the statement unchanged.

```typescript
update?: IMapper<IUpdateStatement, IUpdateStatement>
```

##### delete

Rewrites the finished delete. Omitted leaves the statement unchanged.

```typescript
delete?: IMapper<IDeleteStatement, IDeleteStatement>
```

##### return

Rewrites a finished [ReturnStatement](./return-statement.md). Omitted leaves it unchanged.

```typescript
return?: IMapper<IReturnStatement, IReturnStatement>
```

##### create

Maps `Response.result` onto the created item. Default writes the value to `id`.

```typescript
create?: IMapper<Scalar, T>
```

#### BuiltRepository

```typescript
type BuiltRepository<T = Cells>
```

Builder output used by [CRUDRepository](#repository).

**Properties**

##### connection

Open [IConnection](./connection.md).

```typescript
connection: IConnection
```

##### rowConverter

Row [TypeConverter](./type-converters.md).

```typescript
rowConverter: TypeConverter<T, Cells>
```

##### rowsMapper

Read-row [Mapper](./mappers.md).

```typescript
rowsMapper: IMapper<T, T>
```

##### statements

SQL fragments.

```typescript
statements: RepositoryStatements
```

##### ids

Id column names.

```typescript
ids: string[]
```

##### type

Item constructor.

```typescript
type: Constructable<T>
```

### csharp
#### ICRUDRepository

```csharp
public interface ICRUDRepository<T> : IReadonlyRepository<T>
```

**Methods**

##### Create(item)

Inserts one item.

```csharp
Task<Response> Create(T item)
```

- `item` — row to insert

**Returns** `Task<Response>`

##### Update(item, byWhere)

Updates one item. Omitted `byWhere` uses id columns.

```csharp
Task<Response> Update(T item, IWhere? byWhere = null)
```

- `item` — row to update
- `byWhere` — optional predicate; omitted uses id values on `item`

**Returns** `Task<Response>`

##### Delete(item, byWhere)

Deletes one item. Omitted `byWhere` uses id columns.

```csharp
Task<Response> Delete(T item, IWhere? byWhere = null)
```

- `item` — row to delete
- `byWhere` — optional predicate; omitted uses id values on `item`

**Returns** `Task<Response>`

#### CRUDRepository

```csharp
public sealed class CRUDRepository<T> : ReadonlyRepository<T>, ICRUDRepository<T> where T : new()
```

**Constructor**

##### CRUDRepository(connection)

Builds a table repository from an [IConnection](./connection.md).

```csharp
public Repository(IConnection connection)
```

- `connection` — open [IConnection](./connection.md)

##### CRUDRepository(connection, options)

Builds a table repository from an [IConnection](./connection.md) and [CRUDRepositoryOptions](#crudrepositoryoptions).

```csharp
public CRUDRepository(IConnection connection, CRUDRepositoryOptions<T> options)
```

- `connection` — open [IConnection](./connection.md)
- `options` — [CRUDRepositoryOptions](#crudrepositoryoptions)

##### CRUDRepository(built)

Builds a table repository from [BuiltRepository](#builtrepository).

```csharp
public Repository(BuiltRepository<T> built)
```

- `built` — [BuiltRepository](#builtrepository)

**Properties**

##### Mapper

Read-row [Mapper](./mappers.md). This is [ReadonlyRepositoryOptions.RowsMapper](./readonly-repository.md#rowsmapper).

```csharp
IMapper<T, T> Mapper { get; }
```

**Methods**

##### AddMiddleware(middleware)

Adds command middleware. Returns this repository.

```csharp
public Repository<T> AddMiddleware(ICommandMiddleware middleware)
```

- `middleware` — command middleware

**Returns** [CRUDRepository](#crudrepository)`<T>`

##### Read(query)

Reads rows through the row converter then [Mapper](#repository-mapper).

```csharp
public override IAsyncEnumerable<T> Read(IStatement? query = null)
```

- `query` — optional statement or where; omitted uses the default select

**Returns** `IAsyncEnumerable<T>`

##### Create(item)

Inserts one item.

```csharp
public Task<Response> Create(T item)
```

- `item` — row to insert

**Returns** `Task<Response>`

##### Update(item, byWhere)

Updates one item.

```csharp
public Task<Response> Update(T item, IWhere? byWhere = null)
```

- `item` — row to update
- `byWhere` — optional predicate; omitted uses id values on `item`

**Returns** `Task<Response>`

##### Delete(item, byWhere)

Deletes one item.

```csharp
public Task<Response> Delete(T item, IWhere? byWhere = null)
```

- `item` — row to delete
- `byWhere` — optional predicate; omitted uses id values on `item`

**Returns** `Task<Response>`

#### CRUDRepositoryOptions

```csharp
public sealed class CRUDRepositoryOptions<T> : ReadonlyRepositoryOptions<T> where T : new()
```

Extends [ReadonlyRepositoryOptions](./readonly-repository.md#readonlyrepositoryoptions). Constructor bag for [CRUDRepository](#repository).

**Properties**

##### Ids

Id column names. Default `["id"]`.

```csharp
IReadOnlyList<string> Ids { get; init; }
```

##### Insert

Rewrites the finished insert. Omitted leaves the statement unchanged.

```csharp
IMapper<IInsertStatement, IInsertStatement>? Insert { get; init; }
```

##### Update

Rewrites the finished update. Omitted leaves the statement unchanged.

```csharp
IMapper<IUpdateStatement, IUpdateStatement>? Update { get; init; }
```

##### Delete

Rewrites the finished delete. Omitted leaves the statement unchanged.

```csharp
IMapper<IDeleteStatement, IDeleteStatement>? Delete { get; init; }
```

##### Return

Rewrites a finished [ReturnStatement](./return-statement.md). Omitted leaves it unchanged.

```csharp
IMapper<IReturnStatement, IReturnStatement>? Return { get; init; }
```

##### Create

Maps `Response.Result` onto the created item. Default writes the value to `id` or `Id`.

```csharp
IMapper<Value, T> Create { get; init; }
```

#### BuiltRepository

```csharp
public sealed record BuiltRepository<T> where T : new()
```

Builder output used by [CRUDRepository](#repository).

**Properties**

##### Connection

Open [IConnection](./connection.md).

```csharp
IConnection Connection { get; init; }
```

##### RowConverter

Row [TypeConverter](./type-converters.md).

```csharp
TypeConverter<T, Row> RowConverter { get; init; }
```

##### RowsMapper

Read-row [Mapper](./mappers.md).

```csharp
IMapper<T, T> RowsMapper { get; init; }
```

##### Statements

SQL fragments.

```csharp
RepositoryStatements Statements { get; init; }
```

##### Ids

Id column names.

```csharp
IReadOnlyList<string> Ids { get; init; }
```

### rust
#### ICRUDRepository

```rust
pub trait ICRUDRepository: IReadonlyRepository
```

**Methods**

##### create(item)

Inserts one row.

```rust
async fn create(&self, item: &mut Row) -> Result<Response>
```

- `item` — row to insert

**Returns** `Result<Response>`

##### update(item, by_where)

Updates one row. `None` `by_where` uses id columns.

```rust
async fn update(&self, item: &Row, by_where: Option<&dyn Where>) -> Result<Response>
```

- `item` — row to update
- `by_where` — optional predicate; omitted uses id values on `item`

**Returns** `Result<Response>`

##### delete(item, by_where)

Deletes one row. `None` `by_where` uses id columns.

```rust
async fn delete(&self, item: &Row, by_where: Option<&dyn Where>) -> Result<Response>
```

- `item` — row to delete
- `by_where` — optional predicate; omitted uses id values on `item`

**Returns** `Result<Response>`

#### CRUDRepository

```rust
pub struct CRUDRepository
```

**Constructor**

##### from_type(connection)

Builds a table repository from `T: ConvertSchema`.

```rust
pub fn from_type<T: ConvertSchema + 'static>(connection: Arc<dyn Connection>) -> Result<Self>
```

- `connection` — [Connection](./connection.md)

**Returns** `Result<`[CRUDRepository](#crudrepository)`>`

##### new(connection, options)

Builds a table repository from a [Connection](./connection.md) and [CRUDRepositoryOptions](#crudrepositoryoptions).

```rust
pub fn new(connection: Arc<dyn Connection>, options: CRUDRepositoryOptions) -> Result<Self>
```

- `connection` — [Connection](./connection.md)
- `options` — [CRUDRepositoryOptions](#crudrepositoryoptions)

**Returns** `Result<`[CRUDRepository](#crudrepository)`>`

##### from_built(built)

Builds a table repository from [BuiltRepository](#builtrepository).

```rust
pub fn from_built(built: BuiltRepository) -> Self
```

- `built` — [BuiltRepository](#builtrepository)

**Returns** [CRUDRepository](#crudrepository)

**Methods**

##### add_middleware(middleware)

Adds command middleware.

```rust
pub fn add_middleware(&mut self, middleware: Arc<dyn CommandMiddleware>)
```

- `middleware` — command middleware

#### CRUDRepositoryOptions

```rust
pub struct CRUDRepositoryOptions
```

Embeds [ReadonlyRepositoryOptions](./readonly-repository.md#readonlyrepositoryoptions) and derefs to it. Constructor bag for [CRUDRepository](#repository). `Default` sets `ids` to `["id"]`.

**Properties**

##### ids

Id column names. Default `["id"]`.

```rust
pub ids: Vec<String>
```

##### insert

Rewrites finished insert SQL. `None` leaves the statement unchanged.

```rust
pub insert: Option<Arc<TransformMapper<String, String>>>
```

##### update

Rewrites finished update SQL. `None` leaves the statement unchanged.

```rust
pub update: Option<Arc<TransformMapper<String, String>>>
```

##### delete

Rewrites finished delete SQL. `None` leaves the statement unchanged.

```rust
pub delete: Option<Arc<TransformMapper<String, String>>>
```

##### return_

Rewrites finished return SQL. `None` leaves it unchanged.

```rust
pub return_: Option<Arc<TransformMapper<String, String>>>
```

##### create

Maps `Response.result` onto the created row. Default writes the value to `id`.

```rust
pub create: TransformMapper<Value, Row>
```

##### read

The embedded [ReadonlyRepositoryOptions](./readonly-repository.md#readonlyrepositoryoptions). Field access goes through `Deref`.

```rust
pub read: ReadonlyRepositoryOptions
```

#### BuiltRepository

```rust
pub struct BuiltRepository
```

Builder output used by [CRUDRepository](#repository).

**Properties**

##### connection

[Connection](./connection.md).

```rust
pub connection: Arc<dyn Connection>
```

##### row_converter

[RowTypeConverter](./row-type-converter.md).

```rust
pub row_converter: RowTypeConverter
```

##### rows_mapper

Read-row [Mapper](./mappers.md).

```rust
pub rows_mapper: Box<dyn Mapper<Row, Row> + Send + Sync>
```

##### statements

SQL fragments.

```rust
pub statements: RepositoryStatements
```

##### ids

Id column names.

```rust
pub ids: Vec<String>
```
:::

## Remarks

Update requires at least one non-id field. Missing id values throw. Insert always omits configured id columns. After execute, `Create` / `create` maps `Response.Result` onto the created item.

`Mapper` / `mapper` is [ReadonlyRepositoryOptions.RowsMapper](./readonly-repository.md#rowsmapper).

## Examples

### Create, update, and delete

::: example
```csharp csharp/Datasources.Tests/RepositoryTests.cs#L100-L108
```
```rust rust/datasources/src/tests/mod.rs#L1468-L1482
```
```typescript typescript/datasources/src/repository.test.ts#L103-L113
```
:::
