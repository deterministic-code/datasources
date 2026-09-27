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

##### readSingleById(id)

Returns the row for one id column, or for a composite key. A value requires exactly one configured id column. A row must include every configured id column. No match returns `undefined`.

```typescript
readSingleById(id: Scalar | Cells): Promise<T | undefined>
```

- `id` — value of the single id column, or a row keyed by each id column

**Returns** `Promise<T | undefined>`

#### CRUDRepository

```typescript
class CRUDRepository<T = Cells> extends ReadonlyRepository<T> implements ICRUDRepository<T>, IEnriched<T>
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

##### CRUDRepository(inner, strategy?)

Wraps an inner [ICRUDRepository](#icrudrepository). Parent writes delegate to `inner`. Omitted strategy uses the default.

```typescript
constructor(inner: ICRUDRepository<T>, strategy?: IReadonlyEnrichmentStrategy<T>)
```

- `inner` — [ICRUDRepository](#icrudrepository)
- `strategy` — optional read enrichment strategy

**Properties**

##### ids

Configured id property names.

```typescript
readonly ids: readonly string[]
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

Reads rows. Convert the database row, then map it.

```typescript
read(query?: ISelectStatement | IWhere | IExecuteStoredProcedureStatement): AsyncIterable<T>
```

- `query` — optional query, where, or stored procedure; omitted uses the default select

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

##### readSingleById(id)

Returns the row for one id column, or for a composite key.

```typescript
readSingleById(id: Scalar | Cells): Promise<T | undefined>
```

- `id` — value of the single id column, or a row keyed by each id column

**Returns** `Promise<T | undefined>`

##### addEnriched(enrichment, via, strategy?)

Registers the same read fill as [ReadonlyRepository](./readonly-repository.md#addenrichedenrichment-via-strategy). When `via` is a CRUD repository, `many` children are written with the parent.

```typescript
addEnriched<TRelated extends object>(enrichment: Enrichment, via: IReadonlyRepository<TRelated>, strategy?: IReadonlyEnrichmentStrategy<T & object>): this
```

- `enrichment` — [Enrichment](./readonly-repository.md#enrichment)
- `via` — related repository. A CRUD repository is kept so `many` children can be written
- `strategy` — optional read strategy for this edge. Omitted uses the default strategy

**Returns** `this`

##### addLinked(enrichment, via, link, strategy?)

Registers an association. `via` is read. `link` is the CRUD writer.

```typescript
addLinked<TRelated extends object, TLink extends object>(enrichment: Enrichment, via: IReadonlyRepository<TRelated>, link: ICRUDRepository<TLink>, strategy?: IReadonlyEnrichmentStrategy<T & object>): this
```

- `enrichment` — [Enrichment](./readonly-repository.md#enrichment) with `association`
- `via` — related [IReadonlyRepository](./readonly-repository.md)
- `link` — [ICRUDRepository](#icrudrepository) that stores the association rows
- `strategy` — optional read strategy for this edge. Omitted uses the default strategy

**Returns** `this`

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

##### ReadSingleById(id)

Returns the row for the single configured id column. Zero id columns, or more than one, throw.

```csharp
Task<T?> ReadSingleById(Value id)
```

- `id` — value of the single id column

**Returns** `Task<T?>`

##### ReadSingleById(ids)

Returns the row whose configured id columns match `ids`. A missing id column throws. No match returns `null`.

```csharp
Task<T?> ReadSingleById(Row ids)
```

- `ids` — id column names and values

**Returns** `Task<T?>`

#### CRUDRepository

```csharp
public class CRUDRepository<T> : ReadonlyRepository<T>, ICRUDRepository<T>, IEnriched<T> where T : new()
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

##### CRUDRepository(inner, strategy)

Wraps an inner [ICRUDRepository](#icrudrepository). Parent writes delegate to `inner`. Omitted strategy uses the default.

```csharp
public CRUDRepository(ICRUDRepository<T> inner, IReadonlyEnrichmentStrategy<T>? strategy = null)
```

- `inner` — [ICRUDRepository](#icrudrepository)
- `strategy` — optional read enrichment strategy

**Properties**

##### Ids

Configured id property names.

```csharp
IReadOnlyList<string> Ids { get; }
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

Reads rows. Convert the database row, then map it.

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

##### ReadSingleById(id)

Returns the row for the single configured id column.

```csharp
public Task<T?> ReadSingleById(Value id)
```

- `id` — value of the single id column

**Returns** `Task<T?>`

##### ReadSingleById(ids)

Returns the row whose configured id columns match `ids`.

```csharp
public Task<T?> ReadSingleById(Row ids)
```

- `ids` — id column names and values

**Returns** `Task<T?>`

##### AddEnriched(enrichment, via, strategy)

Registers the same read fill as [ReadonlyRepository](./readonly-repository.md#addenrichedenrichment-via-strategy). When `via` is a CRUD repository, `many` children are written with the parent.

```csharp
public IEnriched<T> AddEnriched<TRelated>(Enrichment enrichment, IReadonlyRepository<TRelated> via, IReadonlyEnrichmentStrategy<T>? strategy = null)
```

- `enrichment` — [Enrichment](./readonly-repository.md#enrichment)
- `via` — related repository. A CRUD repository is kept so `many` children can be written
- `strategy` — optional read strategy for this edge. Omitted uses the default strategy

**Returns** [IEnriched](./readonly-repository.md#ienriched)`<T>`

##### AddLinked(enrichment, via, link, strategy)

Registers an association. `via` is read. `link` is the CRUD writer.

```csharp
public IEnriched<T> AddLinked<TRelated, TLink>(Enrichment enrichment, IReadonlyRepository<TRelated> via, ICRUDRepository<TLink> link, IReadonlyEnrichmentStrategy<T>? strategy = null)
```

- `enrichment` — [Enrichment](./readonly-repository.md#enrichment) with `Association`
- `via` — related [IReadonlyRepository](./readonly-repository.md)
- `link` — [ICRUDRepository](#icrudrepository) that stores the association rows
- `strategy` — optional read strategy for this edge. Omitted uses the default strategy

**Returns** [IEnriched](./readonly-repository.md#ienriched)`<T>`

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
async fn update(&self, item: &Row, by_where: Option<&dyn IWhere>) -> Result<Response>
```

- `item` — row to update
- `by_where` — optional predicate; omitted uses id values on `item`

**Returns** `Result<Response>`

##### delete(item, by_where)

Deletes one row. `None` `by_where` uses id columns.

```rust
async fn delete(&self, item: &Row, by_where: Option<&dyn IWhere>) -> Result<Response>
```

- `item` — row to delete
- `by_where` — optional predicate; omitted uses id values on `item`

**Returns** `Result<Response>`

##### read_single_by_id(id)

Returns the row for one id column, or for a composite key. A value requires exactly one configured id column. A row must include every configured id column. No match returns `None`.

```rust
async fn read_single_by_id(&self, id: &dyn IdKey) -> Result<Option<Row>>
```

- `id` — `Value` for the single id column, or a `Row` keyed by each id column

**Returns** `Result<Option<Row>>`

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

##### from_repository(inner)

Wraps an inner [ICRUDRepository](#icrudrepository). Parent writes delegate to `inner`.

```rust
pub fn from_repository(inner: Arc<dyn ICRUDRepository>) -> Self
```

- `inner` — [ICRUDRepository](#icrudrepository)

**Returns** [CRUDRepository](#crudrepository)

**Methods**

##### add_enriched(enrichment, via, strategy)

Registers the same read fill as [ReadonlyRepository](./readonly-repository.md#add_enrichedenrichment-via-strategy). When `via` is a CRUD repository, `many` children are written with the parent.

```rust
pub fn add_enriched(&mut self, enrichment: Enrichment, via: impl EnrichmentVia, strategy: Option<Arc<dyn ReadonlyEnrichmentStrategy>>) -> &mut Self
```

- `enrichment` — [Enrichment](./readonly-repository.md#enrichment)
- `via` — related repository. A CRUD repository is kept so `many` children can be written
- `strategy` — optional read strategy for this edge. Omitted uses the default strategy

**Returns** `&mut Self`

##### add_linked(enrichment, via, link, strategy)

Registers an association. `via` is read. `link` is the CRUD writer.

```rust
pub fn add_linked(&mut self, enrichment: Enrichment, via: impl EnrichmentVia, link: Arc<dyn ICRUDRepository>, strategy: Option<Arc<dyn ReadonlyEnrichmentStrategy>>) -> &mut Self
```

- `enrichment` — [Enrichment](./readonly-repository.md#enrichment) with `association`
- `via` — related readonly repository
- `link` — [ICRUDRepository](#icrudrepository) that stores the association rows
- `strategy` — optional read strategy for this edge. Omitted uses the default strategy

**Returns** `&mut Self`

##### add_middleware(middleware)

Adds command middleware.

```rust
pub fn add_middleware(&mut self, middleware: Arc<dyn CommandMiddleware>)
```

- `middleware` — command middleware

##### read_single_by_id(id)

Returns the row for one id column, or for a composite key.

```rust
pub async fn read_single_by_id(&self, id: &dyn IdKey) -> Result<Option<Row>>
```

- `id` — `Value` for the single id column, or a `Row` keyed by each id column

**Returns** `Result<Option<Row>>`

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

Update requires at least one non-id field. Missing id values throw. Insert always omits configured id columns. After execute, insert writes `Response.Result` onto the single id column through the driver's generated-id method and the row converter. A null cell is omitted on create and update, so a SQL default applies. A present value is written.

`ReadSingleById` / `readSingleById` / `read_single_by_id` takes a value when the repository has one id column, and a row keyed by each id column when the key is composite.

`AddEnriched` registers the same read fill as [ReadonlyRepository](./readonly-repository.md). Lookups are filled on read and are not written. When `via` is a CRUD repository, `many` children are created, replace-synced, and deleted with the parent. `AddLinked` reads `via` and writes the association through `link`. Eager write replaces that junction set. A null or empty list clears the association and leaves the others. Related rows stay. `LazyWriteEnrichmentStrategy` skips a null list on create and update. Delete still removes the junction rows. Passing `byWhere` to `Update` replaces the id predicate, so a version check includes both the id and the version.

`DefaultWriteEnrichmentStrategy` takes `useTransaction`, off by default. When it is on, `Create`, `Update`, and `Delete` commit the parent write and nested persist together. A bulk list uses one transaction for the whole list. Nested writes join when they use the same connection.

- **Create** — write the parent, write the generated identity onto the parent's id column, stamp each child's `by` from the parent `from`, then create children (and nested many).
- **Update** — `DefaultUpdateEnrichmentStrategy` is the default. Write the parent, then replace-sync each many list (delete missing, update matches by the configured id, create the rest). `CascadeDeleteUpdateEnrichmentStrategy` deletes every existing child, then inserts the payload as new rows. Recreated children get new ids. Links are replaced the same way. Related rows stay.
- **Delete** — `CascadeDeleteEnrichmentStrategy` is the default. Delete nested many rows first, then children, then the parent. Loads existing children by FK; does not trust payload lists.

See [Bulk repository](./bulk-repository.md) for list writes.

## Examples

### User from the type

The table, columns, and id come from `User`. Create omits `id`. SQLite writes `last_insert_rowid` back onto `user.id`.

::: example
```csharp csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#WritesUserFromTheType
```
```rust rust/providers/sqlite/tests/readonly_repository.rs#updates_users_view_when_the_version_matches
```
```typescript typescript/providers/sqlite/tests/readonly.repository.integration.test.ts#writes-a-user-from-the-type
```

```output [Created]
id: 1
```

```output [Read by id]
{
  id: 1,
  username: 'ada',
  is_enabled: true
}
```

```output:typescript [Read by id]
{
  id: 1,
  username: 'ada',
  is_enabled: 1
}
```

```output:rust [Read by id]
{
  id: 1,
  username: 'ada',
  is_enabled: 1
}
```

```output [Read by username and enabled]
ada
```

```output [After update]
ada-v2
```

```output [After delete]
no row
```
:::

### Users view with enrichment and a version

`UsersView` maps onto `user`. `UserTypeId` is written to `user_type_id`. `UserType` is the enriched name from `user_type` and is not a column. `Roles` is a `many` list from `user_role`. Create writes the generated id back onto `Key`. `Update` passes a where on `Key` and `Version`, so a stale version changes no row. Delete removes the user and its roles.

::: example
```csharp csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#UpdatesUsersViewWhenTheVersionMatches
```
```rust rust/providers/sqlite/tests/readonly_repository.rs#updates_users_view_when_the_version_matches
```
```typescript typescript/providers/sqlite/tests/readonly.repository.integration.test.ts#updates-a-user-view-when-the-version-matches
```

```output [Created]
{
  id: 1,
  Key: 1
}
```

```output [Enriched Ada]
{
  UserType: 'admin',
  Roles: ['admin', 'editor']
}
```

```output [Versioned update]
{
  rowsAffected: 1,
  version: 2,
  Login: 'ada-v2'
}
```

```output [Stale update]
{
  rowsAffected: 0
}
```

```output [After delete]
no row
```
:::

### Users with linked roles and groups

`AddLinked` reads roles and groups. Create, update, and delete write `user_roles` and `user_groups`. The related rows stay. Update drops the admin role and adds ops. A null or empty list would clear that association.

::: example
```csharp csharp/Providers/Sqlite.Tests/UserViewCRUDRepositoryTests.cs#CreateUpdateAndDeleteSyncsLinkRows
```
```rust rust/datasources/tests/user_view.rs#create_update_and_delete_syncs_link_rows
```
```typescript typescript/datasources/src/UserViewCRUDRepository.test.ts#creates-updates-and-deletes-through-link-rows
```

```output [Created]
{
  UserType: 'admin',
  Roles: ['admin', 'editor'],
  Groups: ['engineering']
}
```

```output [Updated]
{
  Roles: ['editor'],
  Groups: ['engineering', 'ops']
}
```

```output [After delete]
no user
roles and groups remain
```
:::

### Null datetime and version

Create and update leave a null `created` and `version` out of the statement. SQLite fills `2020-01-01T00:00:00.000Z` and `X'0102'`. An update of `name` leaves those defaults in place. A later create that sets both stores those values.

::: example
```csharp csharp/Providers/Sqlite.Tests/NullWriteTests.cs#OmitsNullDatetimeAndVersion
```
```rust rust/providers/sqlite/tests/null_write.rs#omits_null_datetime_and_version
```
```typescript typescript/providers/sqlite/tests/null-write.repository.integration.test.ts#omits-null-datetime-and-version
```

```output [Stored default]
{
  name: 'ada-v2',
  created: '2020-01-01T00:00:00.000Z',
  version: '0102'
}
```

```output [Explicit values]
{
  name: 'grace',
  created: '2024-06-01T00:00:00.000Z',
  version: '0909'
}
```
:::
