# CRUD repository

Builds on the [readonly repository](./readonly-repository.md) and adds create, update, and delete. The object needs a unique id in one or more fields. Custom property names use the repository mappers.

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

- `enrichment` — [Enrichment](./enrichment.md)
- `via` — related repository. A CRUD repository is kept so `many` children can be written
- `strategy` — optional read strategy for this edge. Omitted uses the default strategy

**Returns** `this`

##### addLinked(enrichment, via, link, strategy?)

Registers an association. `via` is read. `link` is the CRUD writer.

```typescript
addLinked<TRelated extends object, TLink extends object>(enrichment: Enrichment, via: IReadonlyRepository<TRelated>, link: ICRUDRepository<TLink>, strategy?: IReadonlyEnrichmentStrategy<T & object>): this
```

- `enrichment` — [Enrichment](./enrichment.md) with `association`
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

##### write

Write enrichment strategy. Omitted uses the default strategy.

```typescript
write?: IWriteEnrichmentStrategy<T>
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

`Cells` to `Cells`. Omitted leaves each cell unchanged.

```typescript
rowConverter?: TypeConverter<Cells, Cells>
```

##### objectConverter

`Cells` to `T`, after `rowConverter`. Default is [MetaDataRowTypeConverter](./metadata-row-type-converter.md).

```typescript
objectConverter: TypeConverter<T, Cells>
```

##### propertyMappings

Property name [Mapper](./mappers.md).

```typescript
propertyMappings: IMapper<string, string>
```

##### entityMapper

Entity name [Mapper](./mappers.md).

```typescript
entityMapper: IMapper<string, string>
```

##### statements

SQL fragments.

```typescript
statements: ReadonlyRepositoryStatements
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

- `enrichment` — [Enrichment](./enrichment.md)
- `via` — related repository. A CRUD repository is kept so `many` children can be written
- `strategy` — optional read strategy for this edge. Omitted uses the default strategy

**Returns** [IEnriched](./i-enriched.md)`<T>`

##### AddLinked(enrichment, via, link, strategy)

Registers an association. `via` is read. `link` is the CRUD writer.

```csharp
public IEnriched<T> AddLinked<TRelated, TLink>(Enrichment enrichment, IReadonlyRepository<TRelated> via, ICRUDRepository<TLink> link, IReadonlyEnrichmentStrategy<T>? strategy = null)
```

- `enrichment` — [Enrichment](./enrichment.md) with `Association`
- `via` — related [IReadonlyRepository](./readonly-repository.md)
- `link` — [ICRUDRepository](#icrudrepository) that stores the association rows
- `strategy` — optional read strategy for this edge. Omitted uses the default strategy

**Returns** [IEnriched](./i-enriched.md)`<T>`

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

##### Write

Write enrichment strategy. Omitted uses the default strategy.

```csharp
IWriteEnrichmentStrategy<T>? Write { get; init; }
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

##### PropertyMappings

Property name [Mapper](./mappers.md).

```csharp
IMapper<string, string> PropertyMappings { get; init; }
```

##### EntityMapper

Entity name [Mapper](./mappers.md).

```csharp
IMapper<string, string> EntityMapper { get; init; }
```

##### Statements

SQL fragments.

```csharp
ReadonlyRepositoryStatements Statements { get; init; }
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

- `enrichment` — [Enrichment](./enrichment.md)
- `via` — related repository. A CRUD repository is kept so `many` children can be written
- `strategy` — optional read strategy for this edge. Omitted uses the default strategy

**Returns** `&mut Self`

##### add_linked(enrichment, via, link, strategy)

Registers an association. `via` is read. `link` is the CRUD writer.

```rust
pub fn add_linked(&mut self, enrichment: Enrichment, via: impl EnrichmentVia, link: Arc<dyn ICRUDRepository>, strategy: Option<Arc<dyn ReadonlyEnrichmentStrategy>>) -> &mut Self
```

- `enrichment` — [Enrichment](./enrichment.md) with `association`
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

##### write

Write enrichment strategy. `None` uses the default strategy.

```rust
pub write: Option<Arc<dyn WriteEnrichmentStrategy>>
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

`Row` to `Row` cell step. `None` leaves each cell unchanged.

```rust
pub row_converter: Option<RowTypeConverter>
```

##### object_converter

`Row` to `Row`, after `row_converter`. Default is [MetaDataRowTypeConverter](./metadata-row-type-converter.md). A handwritten converter assigns each column.

```rust
pub object_converter: Box<dyn TypeConverter<Row, Row> + Send + Sync>
```

##### property_mappings

Property name [Mapper](./mappers.md).

```rust
pub property_mappings: Box<dyn Mapper<String, String> + Send + Sync>
```

##### entity_mapper

Entity name [Mapper](./mappers.md).

```rust
pub entity_mapper: Box<dyn Mapper<String, String> + Send + Sync>
```

##### statements

SQL fragments.

```rust
pub statements: ReadonlyRepositoryStatements
```

##### ids

Id column names.

```rust
pub ids: Vec<String>
```
:::

## Remarks

Update requires at least one non-id field. Missing id values throw. Insert always omits configured id columns. After a table insert with one id column, create reads `SelectInsertedId`, converts the row, and copies the id onto the entity. The driver statement factory formats that statement. A null cell is omitted on create and update, so a SQL default applies. A present value is written.

{{ReadSingleById|readSingleById|read_single_by_id}} takes a value when the repository has one id column, and a row keyed by each id column when the key is composite.

[AddEnriched](./i-enriched.md) registers the same read fill as [ReadonlyRepository](./readonly-repository.md). Lookups are filled on read and are not written. When `via` is a CRUD repository, `many` children are created, replace-synced, and deleted with the parent. `AddLinked` reads `via` and writes the association through `link`. Eager write replaces that junction set. A null or empty list clears the association and leaves the others. Related rows stay. [LazyWriteEnrichmentStrategy](./lazy-write-enrichment-strategy.md) skips a null list on create and update. Delete still removes the junction rows. Passing `byWhere` to `Update` replaces the id predicate, so a version check includes both the id and the version.

[DefaultWriteEnrichmentStrategy](./default-write-enrichment-strategy.md) takes `useTransaction`, off by default. When it is on, `Create`, `Update`, and `Delete` commit the parent write and nested persist together. A bulk list uses one transaction for the whole list. Nested writes join when they use the same connection.

- **Create** — write the parent, write the generated identity onto the parent's id column, stamp each child's `by` from the parent `from`, then create children (and nested many).
- **Update** — [DefaultUpdateEnrichmentStrategy](./default-update-enrichment-strategy.md) is the default. Write the parent, then replace-sync each many list (delete missing, update matches by the configured id, create the rest). [CascadeDeleteUpdateEnrichmentStrategy](./cascade-delete-update-enrichment-strategy.md) deletes every existing child, then inserts the payload as new rows. Recreated children get new ids. Links are replaced the same way. Related rows stay.
- **Delete** — [CascadeDeleteEnrichmentStrategy](./cascade-delete-enrichment-strategy.md) is the default. Delete nested many rows first, then children, then the parent. Loads existing children by FK; does not trust payload lists.

See [Bulk repository](./bulk-repository.md) for list writes.

## Examples

### Simple CRUD

The table, columns, and id come from `User`. Create omits `id`. The repository reads the inserted id and writes it onto `user.id`.

::: example
```csharp csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#WritesUserFromTheType
```
```rust rust/providers/sqlite/tests/readonly_repository.rs#writes_user_from_the_type
```
```typescript typescript/providers/sqlite/tests/readonly.repository.integration.test.ts#writes-a-user-from-the-type
```

```output [Created]
id > 0
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

### Enriched CRUD

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

### Multi-tenant

`account` is identified by `tenant_id` and `id`. The same `id` in another tenant is a different row. {{ReadSingleById|readSingleById|read_single_by_id}}, update, and delete take both keys. Create omits every configured id column, so this sample seeds the rows.

::: example
```csharp csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#ReadsUpdatesAndDeletesByTenantAndId
```
```rust rust/providers/sqlite/tests/readonly_repository.rs#reads_updates_and_deletes_by_tenant_and_id
```
```typescript typescript/providers/sqlite/tests/readonly.repository.integration.test.ts#reads-updates-and-deletes-by-tenant-and-id
```

```output [Tenant 1]
ada-v2
```

```output [Tenant 2]
grace
```
:::

### Enriched contacts write

{{AddEnriched|addEnriched|add_enriched}} fills the contact type and source names, and writes the address and phone lists with the contact.

::: example
```csharp csharp/Providers/Sqlite.Tests/ContactsRepositoryTests.cs#WritesEnrichedContact
```
```rust rust/providers/sqlite/tests/contacts.rs#writes_enriched_contact
```
```typescript typescript/providers/sqlite/tests/contacts.repository.integration.test.ts#writes-enriched-contact
```

```output [Created]
{
  first_name: 'Extra',
  contact_type_name: 'Person',
  addresses: ['9 Extra'],
  phones: ['555-extra']
}
```
:::

### Type mapping

`UsersView` maps onto `user`. `Key` is `id`, `Login` is `username`, and `UserTypeId` is `user_type_id`. `UserType` is the enriched name and is not a column. Create writes the generated id back onto `Key`.

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
