# CRUD repository

[Readonly repository](./readonly-repository.md) plus `create`, `update`, and `delete`. SQL providers typically build a table `Repository` from `RepositoryBuilder` and a [RowTypeConverter](./row-type-converter.md).

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

#### Repository

```typescript
class Repository<T = Cells> extends ReadonlyRepository<T> implements ICRUDRepository<T>
```

**Constructor**

##### Repository(type, connection)

Builds a table repository from a type and [IConnection](./connection.md).

```typescript
constructor(type: Constructable<T>, connection: IConnection)
```

- `type` — item constructor
- `connection` — open [IConnection](./connection.md)

##### Repository(options)

Builds a table repository from [RepositoryBuilderOptions](#repositorybuilderoptions).

```typescript
constructor(options: RepositoryBuilderOptions<T>)
```

- `options` — [RepositoryBuilderOptions](#repositorybuilderoptions)

##### Repository(built)

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

##### statementsMapper

Statement [StatementMapper](./statement-mapper.md).

```typescript
readonly statementsMapper: StatementMapper<T>
```

##### mapper

Read-row [Mapper](./mappers.md). Alias of `statementsMapper.rows`.

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

#### RepositoryBuilderOptions

```typescript
type RepositoryBuilderOptions<T = Cells>
```

Constructor bag for [Repository](#repository).

**Properties**

##### connection

Open [IConnection](./connection.md).

```typescript
connection: IConnection
```

##### ids

Id column names. Default `["id"]`.

```typescript
ids?: string[]
```

##### assignIdentityOnCreate

When true, omit id columns on insert so the provider can generate them. Default `true`.

```typescript
assignIdentityOnCreate?: boolean
```

##### entity

Table or entity name.

```typescript
entity?: string
```

##### mapping

Row [TypeConverter](./type-converters.md).

```typescript
mapping?: TypeConverter<T, Cells>
```

##### mapper

Read-row [Mapper](./mappers.md).

```typescript
mapper?: IMapper<T, T>
```

##### statementMapper

[StatementMapper](./statement-mapper.md).

```typescript
statementMapper?: StatementMapper<T>
```

##### columns

Column names when mapping is omitted.

```typescript
columns?: string[]
```

#### BuiltRepository

```typescript
type BuiltRepository<T = Cells>
```

Builder output used by [Repository](#repository).

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

##### statementMapper

[StatementMapper](./statement-mapper.md).

```typescript
statementMapper: StatementMapper<T>
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

##### assignIdentityOnCreate

When true, omit id columns on insert.

```typescript
assignIdentityOnCreate: boolean
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

#### Repository

```csharp
public sealed class Repository<T> : ReadonlyRepository<T>, ICRUDRepository<T> where T : new()
```

**Constructor**

##### Repository(connection)

Builds a table repository from an [IConnection](./connection.md).

```csharp
public Repository(IConnection connection)
```

- `connection` — open [IConnection](./connection.md)

##### Repository(options)

Builds a table repository from [RepositoryBuilderOptions](#repositorybuilderoptions).

```csharp
public Repository(RepositoryBuilderOptions<T> options)
```

- `options` — [RepositoryBuilderOptions](#repositorybuilderoptions)

##### Repository(built)

Builds a table repository from [BuiltRepository](#builtrepository).

```csharp
public Repository(BuiltRepository<T> built)
```

- `built` — [BuiltRepository](#builtrepository)

**Properties**

##### StatementsMapper

Statement [StatementMapper](./statement-mapper.md).

```csharp
StatementMapper<T> StatementsMapper { get; }
```

##### Mapper

Read-row [Mapper](./mappers.md). Alias of `StatementsMapper.Rows`.

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

**Returns** [Repository](#repository)`<T>`

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

#### RepositoryBuilderOptions

```csharp
public sealed class RepositoryBuilderOptions<T> where T : new()
```

Constructor bag for [Repository](#repository).

**Properties**

##### Connection

Open [IConnection](./connection.md).

```csharp
required IConnection Connection { get; init; }
```

##### Ids

Id column names. Default `["id"]`.

```csharp
IReadOnlyList<string> Ids { get; init; }
```

##### AssignIdentityOnCreate

When true, omit id columns on insert so the provider can generate them. Default `true`.

```csharp
bool AssignIdentityOnCreate { get; init; }
```

##### Entity

Table or entity name.

```csharp
string? Entity { get; init; }
```

##### Mapping

Row [TypeConverter](./type-converters.md).

```csharp
TypeConverter<T, Row>? Mapping { get; init; }
```

##### Mapper

Read-row [Mapper](./mappers.md).

```csharp
IMapper<T, T>? Mapper { get; init; }
```

##### StatementMapper

[StatementMapper](./statement-mapper.md).

```csharp
StatementMapper<T>? StatementMapper { get; init; }
```

##### Columns

Column names when mapping is omitted.

```csharp
IReadOnlyList<string>? Columns { get; init; }
```

#### BuiltRepository

```csharp
public sealed record BuiltRepository<T> where T : new()
```

Builder output used by [Repository](#repository).

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

##### StatementMapper

[StatementMapper](./statement-mapper.md).

```csharp
StatementMapper<T> StatementMapper { get; init; }
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

##### AssignIdentityOnCreate

When true, omit id columns on insert.

```csharp
bool AssignIdentityOnCreate { get; init; }
```

### rust
#### CRUDRepository

```rust
pub trait CRUDRepository: ReadonlyRepository
```

**Methods**

##### create(item)

Inserts one row.

```rust
async fn create(&self, item: &Row) -> Result<Response>
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

#### Repository

```rust
pub struct Repository
```

**Constructor**

##### from_type(connection)

Builds a table repository from `T: ConvertSchema`.

```rust
pub fn from_type<T: ConvertSchema + 'static>(connection: Arc<dyn Connection>) -> Result<Self>
```

- `connection` — [Connection](./connection.md)

**Returns** `Result<`[Repository](#repository)`>`

##### new(options)

Builds a table repository from [RepositoryBuilderOptions](#repositorybuilderoptions).

```rust
pub fn new(options: RepositoryBuilderOptions) -> Result<Self>
```

- `options` — [RepositoryBuilderOptions](#repositorybuilderoptions)

**Returns** `Result<`[Repository](#repository)`>`

##### from_built(built)

Builds a table repository from [BuiltRepository](#builtrepository).

```rust
pub fn from_built(built: BuiltRepository) -> Self
```

- `built` — [BuiltRepository](#builtrepository)

**Returns** [Repository](#repository)

**Methods**

##### add_middleware(middleware)

Adds command middleware.

```rust
pub fn add_middleware(&mut self, middleware: Arc<dyn CommandMiddleware>)
```

- `middleware` — command middleware

#### RepositoryBuilderOptions

```rust
pub struct RepositoryBuilderOptions
```

Constructor bag for [Repository](#repository).

**Constructor**

##### with_factory(connection)

Defaults: `ids = ["id"]`, `assign_identity_on_create = true`, `entity = "items"`.

```rust
pub fn with_factory(connection: Arc<dyn Connection>) -> Self
```

- `connection` — [Connection](./connection.md)

**Returns** [RepositoryBuilderOptions](#repositorybuilderoptions)

**Properties**

##### connection

[Connection](./connection.md).

```rust
pub connection: Arc<dyn Connection>
```

##### ids

Id column names.

```rust
pub ids: Vec<String>
```

##### assign_identity_on_create

When true, omit id columns on insert.

```rust
pub assign_identity_on_create: bool
```

##### entity

Table or entity name.

```rust
pub entity: String
```

##### mapping

Row [RowTypeConverter](./row-type-converter.md).

```rust
pub mapping: Option<RowTypeConverter>
```

##### mapper

Read-row [TransformMapper](./transform-mapper.md).

```rust
pub mapper: Option<TransformMapper<Row, Row>>
```

##### statement_mapper

[StatementMapper](./statement-mapper.md).

```rust
pub statement_mapper: Option<StatementMapper>
```

#### BuiltRepository

```rust
pub struct BuiltRepository
```

Builder output used by [Repository](#repository).

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

##### statement_mapper

[StatementMapper](./statement-mapper.md).

```rust
pub statement_mapper: Arc<StatementMapper>
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

##### assign_identity_on_create

When true, omit id columns on insert.

```rust
pub assign_identity_on_create: bool
```
:::

## Remarks

Update requires at least one non-id field. Missing id values throw.

`Mapper` / `mapper` aliases `StatementMapper.Rows` / `rows`. See [StatementMapper](./statement-mapper.md).

## Examples

### Create, update, and delete

::: example
```csharp csharp/Datasources.Tests/RepositoryTests.cs#L102-L111
```
```rust rust/datasources/src/tests/mod.rs#L886-L899
```
```typescript typescript/datasources/src/repository.test.ts#L100-L111
```
:::
