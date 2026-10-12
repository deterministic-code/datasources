<!-- Generated from docs/. Edit the markdown in docs/ and run npm run docs:skill. -->

# CRUD repository

Builds on the [readonly repository](readonly-repository.md) and adds create, update, and delete. The object needs a unique id in one or more fields. Custom property names use the repository mappers.

Read is convert, then map. Writes reverse-map, then reverse-convert.

## Classes

#### TypeScript

#### ICRUDRepository {#typescript-icrudrepository}

```typescript
interface ICRUDRepository<T = unknown> extends IReadonlyRepository<T>
```

**Methods**

| Name | Description |
| --- | --- |
| [create(item)](#typescript-icrudrepository-create-item) | Inserts one item. |
| [update(item, byWhere?)](#typescript-icrudrepository-update-item-bywhere) | Updates one item. Omitted `byWhere` uses id columns. |
| [delete(item, byWhere?)](#typescript-icrudrepository-delete-item-bywhere) | Deletes one item. Omitted `byWhere` uses id columns. |
| [readSingleById(id)](#typescript-icrudrepository-readsinglebyid-id) | Returns the row for one id column, or for a composite key. A value requires exactly one configured id column. A row must include every configured id column. No match returns `undefined`. |

##### create(item) {#typescript-icrudrepository-create-item}

Inserts one item.

**Definition**

```typescript
create(item: T): Promise<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `item` | row to insert |

**Returns**

`Promise<Response>`

##### update(item, byWhere?) {#typescript-icrudrepository-update-item-bywhere}

Updates one item. Omitted `byWhere` uses id columns.

**Definition**

```typescript
update(item: T, byWhere?: IWhere): Promise<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `item` | row to update |
| `byWhere` | optional predicate; omitted uses id values on `item` |

**Returns**

`Promise<Response>`

##### delete(item, byWhere?) {#typescript-icrudrepository-delete-item-bywhere}

Deletes one item. Omitted `byWhere` uses id columns.

**Definition**

```typescript
delete(item: T, byWhere?: IWhere): Promise<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `item` | row to delete |
| `byWhere` | optional predicate; omitted uses id values on `item` |

**Returns**

`Promise<Response>`

##### readSingleById(id) {#typescript-icrudrepository-readsinglebyid-id}

Returns the row for one id column, or for a composite key. A value requires exactly one configured id column. A row must include every configured id column. No match returns `undefined`.

**Definition**

```typescript
readSingleById(id: Scalar | Cells): Promise<T | undefined>
```

**Parameters**

| Name | Description |
| --- | --- |
| `id` | value of the single id column, or a row keyed by each id column |

**Returns**

`Promise<T | undefined>`

#### CRUDRepository {#typescript-crudrepository}

```typescript
class CRUDRepository<T = Cells> extends ReadonlyRepository<T> implements ICRUDRepository<T>, IEnriched<T>
```

**Constructor**

| Name | Description |
| --- | --- |
| [CRUDRepository(type, connection)](#typescript-crudrepository-crudrepository-type-connection) | Builds a table repository from a type and [IConnection](docs/sections/connection.md). |
| [CRUDRepository(connection, options)](#typescript-crudrepository-crudrepository-connection-options) | Builds a table repository from an [IConnection](docs/sections/connection.md) and [CRUDRepositoryOptions](#typescript-crudrepositoryoptions). |
| [CRUDRepository(inner, strategy?)](#typescript-crudrepository-crudrepository-inner-strategy) | Wraps an inner [ICRUDRepository](#typescript-icrudrepository). Parent writes delegate to `inner`. Omitted strategy uses the default. |

**Properties**

| Name | Description |
| --- | --- |
| ids | Configured id property names. |

**Methods**

| Name | Description |
| --- | --- |
| [addMiddleware(middleware)](#typescript-crudrepository-addmiddleware-middleware) | Adds command middleware. Returns this repository. |
| [read(query?)](#typescript-crudrepository-read-query) | Reads rows. Convert the database row, then map it. |
| [create(item)](#typescript-crudrepository-create-item) | Inserts one item. |
| [update(item, byWhere?)](#typescript-crudrepository-update-item-bywhere) | Updates one item. |
| [delete(item, byWhere?)](#typescript-crudrepository-delete-item-bywhere) | Deletes one item. |
| [readSingleById(id)](#typescript-crudrepository-readsinglebyid-id) | Returns the row for one id column, or for a composite key. |
| [addEnriched(enrichment, via, strategy?)](#typescript-crudrepository-addenriched-enrichment-via-strategy) | Registers the same read fill as [ReadonlyRepository](readonly-repository.md#typescript-addenrichedenrichment-via-strategy). When `via` is a CRUD repository, `List` children are written with the parent. |
| [addLinked(enrichment, via, link, strategy?)](#typescript-crudrepository-addlinked-enrichment-via-link-strategy) | Registers an association. `via` is read. `link` is the CRUD writer. |

##### CRUDRepository(type, connection) {#typescript-crudrepository-crudrepository-type-connection}

Builds a table repository from a type and [IConnection](docs/sections/connection.md).

**Definition**

```typescript
constructor(type: Constructable<T>, connection: IConnection)
```

**Parameters**

| Name | Description |
| --- | --- |
| `type` | item constructor |
| `connection` | open [IConnection](docs/sections/connection.md) |

##### CRUDRepository(connection, options) {#typescript-crudrepository-crudrepository-connection-options}

Builds a table repository from an [IConnection](docs/sections/connection.md) and [CRUDRepositoryOptions](#typescript-crudrepositoryoptions).

**Definition**

```typescript
constructor(connection: IConnection, options: CRUDRepositoryOptions<T>)
```

**Parameters**

| Name | Description |
| --- | --- |
| `connection` | open [IConnection](docs/sections/connection.md) |
| `options` | [CRUDRepositoryOptions](#typescript-crudrepositoryoptions) |

##### CRUDRepository(inner, strategy?) {#typescript-crudrepository-crudrepository-inner-strategy}

Wraps an inner [ICRUDRepository](#typescript-icrudrepository). Parent writes delegate to `inner`. Omitted strategy uses the default.

**Definition**

```typescript
constructor(inner: ICRUDRepository<T>, strategy?: IReadonlyEnrichmentStrategy<T>)
```

**Parameters**

| Name | Description |
| --- | --- |
| `inner` | [ICRUDRepository](#typescript-icrudrepository) |
| `strategy` | optional read enrichment strategy |

##### addMiddleware(middleware) {#typescript-crudrepository-addmiddleware-middleware}

Adds command middleware. Returns this repository.

**Definition**

```typescript
addMiddleware(middleware: ICommandMiddleware): this
```

**Parameters**

| Name | Description |
| --- | --- |
| `middleware` | command middleware |

**Returns**

`this`

##### read(query?) {#typescript-crudrepository-read-query}

Reads rows. Convert the database row, then map it.

**Definition**

```typescript
read(query?: ISelectStatement | IWhere | IExecuteStoredProcedureStatement): AsyncIterable<T>
```

**Parameters**

| Name | Description |
| --- | --- |
| `query` | optional query, where, or stored procedure; omitted uses the default select |

**Returns**

`AsyncIterable<T>`

##### create(item) {#typescript-crudrepository-create-item}

Inserts one item.

**Definition**

```typescript
create(item: T): Promise<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `item` | row to insert |

**Returns**

`Promise<Response>`

##### update(item, byWhere?) {#typescript-crudrepository-update-item-bywhere}

Updates one item.

**Definition**

```typescript
update(item: T, byWhere?: IWhere): Promise<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `item` | row to update |
| `byWhere` | optional predicate; omitted uses id values on `item` |

**Returns**

`Promise<Response>`

##### delete(item, byWhere?) {#typescript-crudrepository-delete-item-bywhere}

Deletes one item.

**Definition**

```typescript
delete(item: T, byWhere?: IWhere): Promise<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `item` | row to delete |
| `byWhere` | optional predicate; omitted uses id values on `item` |

**Returns**

`Promise<Response>`

##### readSingleById(id) {#typescript-crudrepository-readsinglebyid-id}

Returns the row for one id column, or for a composite key.

**Definition**

```typescript
readSingleById(id: Scalar | Cells): Promise<T | undefined>
```

**Parameters**

| Name | Description |
| --- | --- |
| `id` | value of the single id column, or a row keyed by each id column |

**Returns**

`Promise<T | undefined>`

##### addEnriched(enrichment, via, strategy?) {#typescript-crudrepository-addenriched-enrichment-via-strategy}

Registers the same read fill as [ReadonlyRepository](readonly-repository.md#typescript-addenrichedenrichment-via-strategy). When `via` is a CRUD repository, `List` children are written with the parent.

**Definition**

```typescript
addEnriched<TRelated extends object>(enrichment: Enrichment, via: IReadonlyRepository<TRelated>, strategy?: IReadonlyEnrichmentStrategy<T & object>): this
```

**Parameters**

| Name | Description |
| --- | --- |
| `enrichment` | [Enrichment](enrichment.md) |
| `via` | related repository. A CRUD repository is kept so `List` children can be written |
| `strategy` | optional read strategy for this edge. Omitted uses the default strategy |

**Returns**

`this`

##### addLinked(enrichment, via, link, strategy?) {#typescript-crudrepository-addlinked-enrichment-via-link-strategy}

Registers an association. `via` is read. `link` is the CRUD writer.

**Definition**

```typescript
addLinked<TRelated extends object, TLink extends object>(enrichment: Enrichment, via: IReadonlyRepository<TRelated>, link: ICRUDRepository<TLink>, strategy?: IReadonlyEnrichmentStrategy<T & object>): this
```

**Parameters**

| Name | Description |
| --- | --- |
| `enrichment` | [Enrichment](enrichment.md) with `association` |
| `via` | related [IReadonlyRepository](readonly-repository.md) |
| `link` | [ICRUDRepository](#typescript-icrudrepository) that stores the association rows |
| `strategy` | optional read strategy for this edge. Omitted uses the default strategy |

**Returns**

`this`

#### CRUDRepositoryOptions {#typescript-crudrepositoryoptions}

```typescript
type CRUDRepositoryOptions<T = Cells> = ReadonlyRepositoryMappingOptions<T> & { ... }
```

**Properties**

| Name | Description |
| --- | --- |
| ids | Id column names. When omitted, `["id"]`, or `["Id"]` when `type` has that field and no `id`. |
| write | Write enrichment strategy. Omitted uses the default strategy. |

#### BuiltRepository {#typescript-builtrepository}

```typescript
type BuiltRepository<T = Cells>
```

**Properties**

| Name | Description |
| --- | --- |
| connection | Open [IConnection](docs/sections/connection.md). |
| rowConverter | `Cells` to `Cells`. Omitted leaves each cell unchanged. |
| objectConverter | `Cells` to `T`, after `rowConverter`. Default is [MetaDataRowTypeConverter](type-converters.md). |
| propertyMappings | Property name [Mapper](mappings.md). |
| entityMapper | Entity name [Mapper](mappings.md). |
| statements | SQL fragments. |
| ids | Id column names. |
| type | Item constructor. |

#### C#

#### ICRUDRepository {#csharp-icrudrepository}

```csharp
public interface ICRUDRepository<T> : IReadonlyRepository<T>
```

**Methods**

| Name | Description |
| --- | --- |
| [Create(item)](#csharp-icrudrepository-create-item) | Inserts one item. |
| [Update(item, byWhere)](#csharp-icrudrepository-update-item-bywhere) | Updates one item. Omitted `byWhere` uses id columns. |
| [Delete(item, byWhere)](#csharp-icrudrepository-delete-item-bywhere) | Deletes one item. Omitted `byWhere` uses id columns. |
| [ReadSingleById(id)](#csharp-icrudrepository-readsinglebyid-id) | Returns the row for the single configured id column. Zero id columns, or more than one, throw. |
| [ReadSingleById(ids)](#csharp-icrudrepository-readsinglebyid-ids) | Returns the row whose configured id columns match `ids`. A missing id column throws. No match returns `null`. |

##### Create(item) {#csharp-icrudrepository-create-item}

Inserts one item.

**Definition**

```csharp
Task<Response> Create(T item)
```

**Parameters**

| Name | Description |
| --- | --- |
| `item` | row to insert |

**Returns**

`Task<Response>`

##### Update(item, byWhere) {#csharp-icrudrepository-update-item-bywhere}

Updates one item. Omitted `byWhere` uses id columns.

**Definition**

```csharp
Task<Response> Update(T item, IWhere? byWhere = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `item` | row to update |
| `byWhere` | optional predicate; omitted uses id values on `item` |

**Returns**

`Task<Response>`

##### Delete(item, byWhere) {#csharp-icrudrepository-delete-item-bywhere}

Deletes one item. Omitted `byWhere` uses id columns.

**Definition**

```csharp
Task<Response> Delete(T item, IWhere? byWhere = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `item` | row to delete |
| `byWhere` | optional predicate; omitted uses id values on `item` |

**Returns**

`Task<Response>`

##### ReadSingleById(id) {#csharp-icrudrepository-readsinglebyid-id}

Returns the row for the single configured id column. Zero id columns, or more than one, throw.

**Definition**

```csharp
Task<T?> ReadSingleById(Value id)
```

**Parameters**

| Name | Description |
| --- | --- |
| `id` | value of the single id column |

**Returns**

`Task<T?>`

##### ReadSingleById(ids) {#csharp-icrudrepository-readsinglebyid-ids}

Returns the row whose configured id columns match `ids`. A missing id column throws. No match returns `null`.

**Definition**

```csharp
Task<T?> ReadSingleById(Row ids)
```

**Parameters**

| Name | Description |
| --- | --- |
| `ids` | id column names and values |

**Returns**

`Task<T?>`

#### CRUDRepository {#csharp-crudrepository}

```csharp
public class CRUDRepository<T> : ReadonlyRepository<T>, ICRUDRepository<T>, IEnriched<T> where T : new()
```

**Constructor**

| Name | Description |
| --- | --- |
| [CRUDRepository(connection)](#csharp-crudrepository-crudrepository-connection) | Builds a table repository from an [IConnection](docs/sections/connection.md). |
| [CRUDRepository(connection, options)](#csharp-crudrepository-crudrepository-connection-options) | Builds a table repository from an [IConnection](docs/sections/connection.md) and [CRUDRepositoryOptions](#csharp-crudrepositoryoptions). |
| [CRUDRepository(inner, strategy)](#csharp-crudrepository-crudrepository-inner-strategy) | Wraps an inner [ICRUDRepository](#csharp-icrudrepository). Parent writes delegate to `inner`. Omitted strategy uses the default. |

**Properties**

| Name | Description |
| --- | --- |
| Ids | Configured id property names. |

**Methods**

| Name | Description |
| --- | --- |
| [AddMiddleware(middleware)](#csharp-crudrepository-addmiddleware-middleware) | Adds command middleware. Returns this repository. |
| [Read(query)](#csharp-crudrepository-read-query) | Reads rows. Convert the database row, then map it. |
| [Create(item)](#csharp-crudrepository-create-item) | Inserts one item. |
| [Update(item, byWhere)](#csharp-crudrepository-update-item-bywhere) | Updates one item. |
| [Delete(item, byWhere)](#csharp-crudrepository-delete-item-bywhere) | Deletes one item. |
| [ReadSingleById(id)](#csharp-crudrepository-readsinglebyid-id) | Returns the row for the single configured id column. |
| [ReadSingleById(ids)](#csharp-crudrepository-readsinglebyid-ids) | Returns the row whose configured id columns match `ids`. |
| [AddEnriched(enrichment, via, strategy)](#csharp-crudrepository-addenriched-enrichment-via-strategy) | Registers the same read fill as [ReadonlyRepository](readonly-repository.md#csharp-addenrichedenrichment-via-strategy). When `via` is a CRUD repository, `List` children are written with the parent. |
| [AddLinked(enrichment, via, link, strategy)](#csharp-crudrepository-addlinked-enrichment-via-link-strategy) | Registers an association. `via` is read. `link` is the CRUD writer. |

##### CRUDRepository(connection) {#csharp-crudrepository-crudrepository-connection}

Builds a table repository from an [IConnection](docs/sections/connection.md).

**Definition**

```csharp
public Repository(IConnection connection)
```

**Parameters**

| Name | Description |
| --- | --- |
| `connection` | open [IConnection](docs/sections/connection.md) |

##### CRUDRepository(connection, options) {#csharp-crudrepository-crudrepository-connection-options}

Builds a table repository from an [IConnection](docs/sections/connection.md) and [CRUDRepositoryOptions](#csharp-crudrepositoryoptions).

**Definition**

```csharp
public CRUDRepository(IConnection connection, CRUDRepositoryOptions<T> options)
```

**Parameters**

| Name | Description |
| --- | --- |
| `connection` | open [IConnection](docs/sections/connection.md) |
| `options` | [CRUDRepositoryOptions](#csharp-crudrepositoryoptions) |

##### CRUDRepository(inner, strategy) {#csharp-crudrepository-crudrepository-inner-strategy}

Wraps an inner [ICRUDRepository](#csharp-icrudrepository). Parent writes delegate to `inner`. Omitted strategy uses the default.

**Definition**

```csharp
public CRUDRepository(ICRUDRepository<T> inner, IReadonlyEnrichmentStrategy<T>? strategy = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `inner` | [ICRUDRepository](#csharp-icrudrepository) |
| `strategy` | optional read enrichment strategy |

##### AddMiddleware(middleware) {#csharp-crudrepository-addmiddleware-middleware}

Adds command middleware. Returns this repository.

**Definition**

```csharp
public Repository<T> AddMiddleware(ICommandMiddleware middleware)
```

**Parameters**

| Name | Description |
| --- | --- |
| `middleware` | command middleware |

**Returns**

[CRUDRepository](#csharp-crudrepository)`<T>`

##### Read(query) {#csharp-crudrepository-read-query}

Reads rows. Convert the database row, then map it.

**Definition**

```csharp
public override IAsyncEnumerable<T> Read(IStatement? query = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `query` | optional statement or where; omitted uses the default select |

**Returns**

`IAsyncEnumerable<T>`

##### Create(item) {#csharp-crudrepository-create-item}

Inserts one item.

**Definition**

```csharp
public Task<Response> Create(T item)
```

**Parameters**

| Name | Description |
| --- | --- |
| `item` | row to insert |

**Returns**

`Task<Response>`

##### Update(item, byWhere) {#csharp-crudrepository-update-item-bywhere}

Updates one item.

**Definition**

```csharp
public Task<Response> Update(T item, IWhere? byWhere = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `item` | row to update |
| `byWhere` | optional predicate; omitted uses id values on `item` |

**Returns**

`Task<Response>`

##### Delete(item, byWhere) {#csharp-crudrepository-delete-item-bywhere}

Deletes one item.

**Definition**

```csharp
public Task<Response> Delete(T item, IWhere? byWhere = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `item` | row to delete |
| `byWhere` | optional predicate; omitted uses id values on `item` |

**Returns**

`Task<Response>`

##### ReadSingleById(id) {#csharp-crudrepository-readsinglebyid-id}

Returns the row for the single configured id column.

**Definition**

```csharp
public Task<T?> ReadSingleById(Value id)
```

**Parameters**

| Name | Description |
| --- | --- |
| `id` | value of the single id column |

**Returns**

`Task<T?>`

##### ReadSingleById(ids) {#csharp-crudrepository-readsinglebyid-ids}

Returns the row whose configured id columns match `ids`.

**Definition**

```csharp
public Task<T?> ReadSingleById(Row ids)
```

**Parameters**

| Name | Description |
| --- | --- |
| `ids` | id column names and values |

**Returns**

`Task<T?>`

##### AddEnriched(enrichment, via, strategy) {#csharp-crudrepository-addenriched-enrichment-via-strategy}

Registers the same read fill as [ReadonlyRepository](readonly-repository.md#csharp-addenrichedenrichment-via-strategy). When `via` is a CRUD repository, `List` children are written with the parent.

**Definition**

```csharp
public IEnriched<T> AddEnriched<TRelated>(Enrichment enrichment, IReadonlyRepository<TRelated> via, IReadonlyEnrichmentStrategy<T>? strategy = null) where TRelated : new()
```

**Parameters**

| Name | Description |
| --- | --- |
| `enrichment` | [Enrichment](enrichment.md) |
| `via` | related repository. A CRUD repository is kept so `List` children can be written |
| `strategy` | optional read strategy for this edge. Omitted uses the default strategy |

**Returns**

[IEnriched](enrichment.md)`<T>`

##### AddLinked(enrichment, via, link, strategy) {#csharp-crudrepository-addlinked-enrichment-via-link-strategy}

Registers an association. `via` is read. `link` is the CRUD writer.

**Definition**

```csharp
public IEnriched<T> AddLinked<TRelated, TLink>(Enrichment enrichment, IReadonlyRepository<TRelated> via, ICRUDRepository<TLink> link, IReadonlyEnrichmentStrategy<T>? strategy = null) where TLink : new()
```

**Parameters**

| Name | Description |
| --- | --- |
| `enrichment` | [Enrichment](enrichment.md) with `Association` |
| `via` | related [IReadonlyRepository](readonly-repository.md) |
| `link` | [ICRUDRepository](#csharp-icrudrepository) that stores the association rows |
| `strategy` | optional read strategy for this edge. Omitted uses the default strategy |

**Returns**

[IEnriched](enrichment.md)`<T>`

#### CRUDRepositoryOptions {#csharp-crudrepositoryoptions}

```csharp
public sealed class CRUDRepositoryOptions<T> : ReadonlyRepositoryMappingOptions<T> where T : new()
```

**Properties**

| Name | Description |
| --- | --- |
| Ids | Id column names. Default `["id"]`. |
| Write | Write enrichment strategy. Omitted uses the default strategy. |

#### BuiltRepository {#csharp-builtrepository}

```csharp
public sealed record BuiltRepository<T> where T : new()
```

**Properties**

| Name | Description |
| --- | --- |
| Connection | Open [IConnection](docs/sections/connection.md). |
| RowConverter | Row [TypeConverter](type-converters.md). |
| PropertyMappings | Property name [Mapper](mappings.md). |
| EntityMapper | Entity name [Mapper](mappings.md). |
| Statements | SQL fragments. |
| Ids | Id column names. |

#### Rust

#### ICRUDRepository {#rust-icrudrepository}

```rust
pub trait ICRUDRepository: IReadonlyRepository
```

**Methods**

| Name | Description |
| --- | --- |
| [create(item)](#rust-icrudrepository-create-item) | Inserts one row. |
| [update(item, by_where)](#rust-icrudrepository-update-item-by_where) | Updates one row. `None` `by_where` uses id columns. |
| [delete(item, by_where)](#rust-icrudrepository-delete-item-by_where) | Deletes one row. `None` `by_where` uses id columns. |
| [read_single_by_id(id)](#rust-icrudrepository-read_single_by_id-id) | Returns the row for one id column, or for a composite key. A value requires exactly one configured id column. A row must include every configured id column. No match returns `None`. |

##### create(item) {#rust-icrudrepository-create-item}

Inserts one row.

**Definition**

```rust
async fn create(&self, item: &mut Row) -> Result<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `item` | row to insert |

**Returns**

`Result<Response>`

##### update(item, by_where) {#rust-icrudrepository-update-item-by_where}

Updates one row. `None` `by_where` uses id columns.

**Definition**

```rust
async fn update(&self, item: &Row, by_where: Option<&dyn IWhere>) -> Result<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `item` | row to update |
| `by_where` | optional predicate; omitted uses id values on `item` |

**Returns**

`Result<Response>`

##### delete(item, by_where) {#rust-icrudrepository-delete-item-by_where}

Deletes one row. `None` `by_where` uses id columns.

**Definition**

```rust
async fn delete(&self, item: &Row, by_where: Option<&dyn IWhere>) -> Result<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `item` | row to delete |
| `by_where` | optional predicate; omitted uses id values on `item` |

**Returns**

`Result<Response>`

##### read_single_by_id(id) {#rust-icrudrepository-read_single_by_id-id}

Returns the row for one id column, or for a composite key. A value requires exactly one configured id column. A row must include every configured id column. No match returns `None`.

**Definition**

```rust
async fn read_single_by_id(&self, id: &dyn IdKey) -> Result<Option<Row>>
```

**Parameters**

| Name | Description |
| --- | --- |
| `id` | `Value` for the single id column, or a `Row` keyed by each id column |

**Returns**

`Result<Option<Row>>`

#### CRUDRepository {#rust-crudrepository}

```rust
pub struct CRUDRepository
```

**Constructor**

| Name | Description |
| --- | --- |
| [from_type(connection)](#rust-crudrepository-from_type-connection) | Builds a table repository from `T: ConvertSchema`. |
| [new(connection, options)](#rust-crudrepository-new-connection-options) | Builds a table repository from a [Connection](docs/sections/connection.md) and [CRUDRepositoryOptions](#rust-crudrepositoryoptions). |
| [from_repository(inner)](#rust-crudrepository-from_repository-inner) | Wraps an inner [ICRUDRepository](#rust-icrudrepository). Parent writes delegate to `inner`. |

**Methods**

| Name | Description |
| --- | --- |
| [add_enriched(enrichment, via, strategy)](#rust-crudrepository-add_enriched-enrichment-via-strategy) | Registers the same read fill as [ReadonlyRepository](readonly-repository.md#rust-add_enrichedenrichment-via-strategy). When `via` is a CRUD repository, `List` children are written with the parent. |
| [add_linked(enrichment, via, link, strategy)](#rust-crudrepository-add_linked-enrichment-via-link-strategy) | Registers an association. `via` is read. `link` is the CRUD writer. |
| [add_middleware(middleware)](#rust-crudrepository-add_middleware-middleware) | Adds command middleware. |
| [read_single_by_id(id)](#rust-crudrepository-read_single_by_id-id) | Returns the row for one id column, or for a composite key. |

##### from_type(connection) {#rust-crudrepository-from_type-connection}

Builds a table repository from `T: ConvertSchema`.

**Definition**

```rust
pub fn from_type<T: ConvertSchema + 'static>(connection: Arc<dyn Connection>) -> Result<Self>
```

**Parameters**

| Name | Description |
| --- | --- |
| `connection` | [Connection](docs/sections/connection.md) |

**Returns**

`Result<`[CRUDRepository](#rust-crudrepository)`>`

##### new(connection, options) {#rust-crudrepository-new-connection-options}

Builds a table repository from a [Connection](docs/sections/connection.md) and [CRUDRepositoryOptions](#rust-crudrepositoryoptions).

**Definition**

```rust
pub fn new(connection: Arc<dyn Connection>, options: CRUDRepositoryOptions) -> Result<Self>
```

**Parameters**

| Name | Description |
| --- | --- |
| `connection` | [Connection](docs/sections/connection.md) |
| `options` | [CRUDRepositoryOptions](#rust-crudrepositoryoptions) |

**Returns**

`Result<`[CRUDRepository](#rust-crudrepository)`>`

##### from_repository(inner) {#rust-crudrepository-from_repository-inner}

Wraps an inner [ICRUDRepository](#rust-icrudrepository). Parent writes delegate to `inner`.

**Definition**

```rust
pub fn from_repository(inner: Arc<dyn ICRUDRepository>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `inner` | [ICRUDRepository](#rust-icrudrepository) |

**Returns**

[CRUDRepository](#rust-crudrepository)

##### add_enriched(enrichment, via, strategy) {#rust-crudrepository-add_enriched-enrichment-via-strategy}

Registers the same read fill as [ReadonlyRepository](readonly-repository.md#rust-add_enrichedenrichment-via-strategy). When `via` is a CRUD repository, `List` children are written with the parent.

**Definition**

```rust
pub fn add_enriched(&mut self, enrichment: Enrichment, via: impl EnrichmentVia, strategy: Option<Arc<dyn ReadonlyEnrichmentStrategy>>) -> &mut Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `enrichment` | [Enrichment](enrichment.md) |
| `via` | related repository. A CRUD repository is kept so `List` children can be written |
| `strategy` | optional read strategy for this edge. Omitted uses the default strategy |

**Returns**

`&mut Self`

##### add_linked(enrichment, via, link, strategy) {#rust-crudrepository-add_linked-enrichment-via-link-strategy}

Registers an association. `via` is read. `link` is the CRUD writer.

**Definition**

```rust
pub fn add_linked(&mut self, enrichment: Enrichment, via: impl EnrichmentVia, link: Arc<dyn ICRUDRepository>, strategy: Option<Arc<dyn ReadonlyEnrichmentStrategy>>) -> &mut Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `enrichment` | [Enrichment](enrichment.md) with `association` |
| `via` | related readonly repository |
| `link` | [ICRUDRepository](#rust-icrudrepository) that stores the association rows |
| `strategy` | optional read strategy for this edge. Omitted uses the default strategy |

**Returns**

`&mut Self`

##### add_middleware(middleware) {#rust-crudrepository-add_middleware-middleware}

Adds command middleware.

**Definition**

```rust
pub fn add_middleware(&mut self, middleware: Arc<dyn CommandMiddleware>)
```

**Parameters**

| Name | Description |
| --- | --- |
| `middleware` | command middleware |

##### read_single_by_id(id) {#rust-crudrepository-read_single_by_id-id}

Returns the row for one id column, or for a composite key.

**Definition**

```rust
pub async fn read_single_by_id(&self, id: &dyn IdKey) -> Result<Option<Row>>
```

**Parameters**

| Name | Description |
| --- | --- |
| `id` | `Value` for the single id column, or a `Row` keyed by each id column |

**Returns**

`Result<Option<Row>>`

#### CRUDRepositoryOptions {#rust-crudrepositoryoptions}

```rust
pub struct CRUDRepositoryOptions
```

**Properties**

| Name | Description |
| --- | --- |
| ids | Id column names. Default `["id"]`. |
| write | Write enrichment strategy. `None` uses the default strategy. |
| read | The embedded [ReadonlyRepositoryMappingOptions](readonly-repository.md#rust-readonlyrepositoryoptions). Field access goes through `Deref`. |

#### BuiltRepository {#rust-builtrepository}

```rust
pub struct BuiltRepository
```

**Properties**

| Name | Description |
| --- | --- |
| connection | [Connection](docs/sections/connection.md). |
| row_converter | `Row` to `Row` cell step. `None` leaves each cell unchanged. |
| object_converter | `Row` to `Row`, after `row_converter`. Default is [MetaDataRowTypeConverter](type-converters.md). A handwritten converter assigns each column. |
| property_mappings | Property name [Mapper](mappings.md). |
| entity_mapper | Entity name [Mapper](mappings.md). |
| statements | SQL fragments. |
| ids | Id column names. |

## Remarks

Update requires at least one non-id field. Missing id values throw. Insert always omits configured id columns. After a table insert with one id column, create reads `SelectInsertedId`, converts the row, and copies the id onto the entity. The driver statement factory formats that statement. A null cell is omitted on create and update, so a SQL default applies. A present value is written.

`ReadSingleById` / `readSingleById` / `read_single_by_id` takes a value when the repository has one id column, and a row keyed by each id column when the key is composite.

[AddEnriched](enrichment.md) registers the same read fill as [ReadonlyRepository](readonly-repository.md). Lookups are filled on read and are not written. When `via` is a CRUD repository, `List` children are created, replace-synced, and deleted with the parent. `AddLinked` reads `via` and writes the association through `link`. Eager write replaces that junction set. A null or empty list clears the association and leaves the others. Related rows stay. [LazyWriteEnrichmentStrategy](docs/sections/lazy-write-enrichment-strategy.md) skips a null list on create and update. Delete still removes the junction rows. Passing `byWhere` to `Update` replaces the id predicate, so a version check includes both the id and the version.

[DefaultWriteEnrichmentStrategy](docs/sections/default-write-enrichment-strategy.md) takes `useTransaction`, off by default. When it is on, `Create`, `Update`, and `Delete` commit the parent write and nested persist together. A bulk list uses one transaction for the whole list. Nested writes join when they use the same connection.

- **Create** — write the parent, write the generated identity onto the parent's id column, stamp each child's `RelatedKey` from the parent `SourceProperty`, then create children (and nested lists).
- **Create** — [DefaultCreateEnrichmentStrategy](docs/sections/default-create-enrichment-strategy.md) is the default. Write the parent, stamp each child's related key, then create children and links.
- **Update** — [DefaultUpdateEnrichmentStrategy](docs/sections/default-update-enrichment-strategy.md) is the default. Write the parent, then replace-sync each list (delete missing, update matches by the configured id, create the rest). [CascadeDeleteUpdateEnrichmentStrategy](docs/sections/cascade-delete-update-enrichment-strategy.md) deletes every existing child, then inserts the payload as new rows. Recreated children get new ids. Links are replaced the same way. Related rows stay.
- **Delete** — [CascadeDeleteEnrichmentStrategy](docs/sections/cascade-delete-enrichment-strategy.md) is the default. Delete nested list rows first, then children, then the parent. Loads existing children by FK; does not trust payload lists.

See [Bulk repository](bulk-repository.md) for list writes.

## Examples

### Simple CRUD

The table, columns, and id come from `User`. Create omits `id`. The repository reads the inserted id and writes it onto `user.id`.

#### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#L621-L653)

```csharp
var ds = Datasource.Default;
var casing = new ChangeCasingMapper(Casing.Pascal, Casing.Snake);
var usersRepo = ds.CreateRepository<User>(connection, new CRUDRepositoryOptions<User>
{
    PropertyMappings = casing,
    Ids = ["Id"],
});
var created = await usersRepo.Create(new User { Username = "ada", IsEnabled = true });
var byId = await usersRepo.ReadSingleById(created.Id);
var matches = new List<User>();
await foreach (var row in usersRepo.Read(new Where("Username", eq, "ada").And("IsEnabled", eq, true)))
    matches.Add(row);
created.Username = "ada-v2";
var updated = await usersRepo.Update(created);
var renamed = await usersRepo.ReadSingleById(created.Id);
var removed = await usersRepo.Delete(created);
var gone = await usersRepo.ReadSingleById(created.Id);
```

**Created**

```
id > 0
```

**Read by id**

```
{
  id: 1,
  username: 'ada',
  is_enabled: true
}
```

**Read by username and enabled**

```
ada
```

**After update**

```
ada-v2
```

**After delete**

```
no row
```

#### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/readonly_repository.rs#L653-L705)

```rust
let ds = default().unwrap();
let mut options = create_crud_repository_options::<User>();
options.property_mappings = Some(std::sync::Arc::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake)));
options.ids = vec!["Id".into()];
let users_repo = ds.create_repository::<User>(Some(connection), Some(options));
let mut created = datasources::Row::new();
created.insert("Username".into(), Value::from("ada"));
created.insert("IsEnabled".into(), Value::Bool(true));
let mut created = users_repo.create(&mut created).await.unwrap();
let by_id = users_repo
    .read_single_by_id(created.get("Id").unwrap())
    .await
    .unwrap()
    .unwrap();
let matches = users_repo
    .read(
        Some(&Where::binary("Username", eq, "ada").and(
            "IsEnabled",
            eq,
            Value::Bool(true),
        )),
        None, None
)
    .await
    .unwrap();
created.insert("Username".into(), Value::from("ada-v2"));
let updated = users_repo.update(&created, None).await.unwrap();
let renamed = users_repo
    .read_single_by_id(created.get("Id").unwrap())
    .await
    .unwrap()
    .unwrap();
let removed = users_repo.delete(&created, None).await.unwrap();
let gone = users_repo
    .read_single_by_id(created.get("Id").unwrap())
    .await
    .unwrap();
```

**Created**

```
id > 0
```

**Read by id**

```
{
  id: 1,
  username: 'ada',
  is_enabled: 1
}
```

**Read by username and enabled**

```
ada
```

**After update**

```
ada-v2
```

**After delete**

```
no row
```

#### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/readonly.repository.integration.test.ts#L229-L256)

```typescript
const ds = Datasource.default;
const casing = new ChangeCasingMapper("pascalCase", "snakeCase");
const usersRepo = ds.createRepository(User, connection, {
  ...createCRUDRepositoryOptions(User),
  propertyMappings: casing,
  ids: ["Id"],
});
const created = await usersRepo.create(Object.assign(new User(), { Username: "ada", IsEnabled: true }));
const byId = await usersRepo.readSingleById(created.Id);
const matches: User[] = [];
for await (const row of usersRepo.read(new Where("Username", eq, "ada").and("IsEnabled", eq, true)))
  matches.push(row);
created.Username = "ada-v2";
const updated = await usersRepo.update(created);
const renamed = await usersRepo.readSingleById(created.Id);
const removed = await usersRepo.delete(created);
const gone = await usersRepo.readSingleById(created.Id);
```

**Created**

```
id > 0
```

**Read by id**

```
{
  id: 1,
  username: 'ada',
  is_enabled: 1
}
```

**Read by username and enabled**

```
ada
```

**After update**

```
ada-v2
```

**After delete**

```
no row
```

Schema: [user-pascal](docs/examples.md#user-pascal)

```sql
CREATE TABLE User (id int, is_enabled bool NOT NULL, username string NOT NULL, CONSTRAINT pk_User PRIMARY KEY (id))
```

### Enriched CRUD

`AddLinked` reads roles and groups. Create, update, and delete write `user_roles` and `user_groups`. The related rows stay. Update drops the admin role and adds ops. A null or empty list would clear that association.

#### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/UserViewCRUDRepositoryTests.cs#L135-L210)

```csharp
var ds = Datasource.Default;
var graph = await GraphAsync(connection);
var usersRepo = graph.UsersRepo;
usersRepo.AddEnriched(new("UserType", "UserTypeId", "Id"), graph.UserTypesRepo);
usersRepo.AddLinked(new("Roles", "Id", "Id", Association: new("UserId", "RoleId")), graph.RolesRepo, graph.UserRolesRepo);
usersRepo.AddLinked(new("Groups", "Id", "Id", Association: new("UserId", "GroupId")), graph.GroupsRepo, graph.UserGroupsRepo);
var created = await usersRepo.Create(new UserView
{
    Username = "ada",
    IsEnabled = true,
    UserTypeId = graph.AdminType.Id,
    Roles = [graph.AdminRole, graph.EditorRole],
    Groups = [graph.Engineering],
});
var createdRoleLinks = await graph.UserRolesRepo.Read().ToListAsync();
var createdGroupLinks = await graph.UserGroupsRepo.Read().ToListAsync();
var saved = await usersRepo.Update(new UserView
{
    Id = created.Id,
    Username = "ada",
    IsEnabled = true,
    UserTypeId = graph.AdminType.Id,
    Roles = [graph.EditorRole],
    Groups = [graph.Engineering, graph.Ops],
});
var updated = await usersRepo.ReadSingleById(created.Id);
var updatedRoleLinks = await graph.UserRolesRepo.Read().ToListAsync();
var updatedGroupLinks = await graph.UserGroupsRepo.Read().ToListAsync();
var removed = await usersRepo.Delete(new UserView { Id = created.Id });
```

**Created**

```
{
  UserType: 'admin',
  Roles: ['admin', 'editor'],
  Groups: ['engineering']
}
```

**Updated**

```
{
  Roles: ['editor'],
  Groups: ['engineering', 'ops']
}
```

**After delete**

```
no user
roles and groups remain
```

#### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/tests/user_view.rs#L151-L306)

```rust
let (mut users, seeded) = open(None).await;
users
    .add_enriched(
        Enrichment::new("user_type", "user_type_id", "id", None, datasources::EnrichmentPropertyType::Object),
        seeded.user_types.clone(),
        None,
    )
    .add_linked(
        Enrichment::new("roles", "id", "id", None, datasources::EnrichmentPropertyType::Object)
            .linked(Association::new("user_id", "role_id")),
        seeded.roles.clone(),
        seeded.user_roles.clone(),
        None,
    )
    .add_linked(
        Enrichment::new("groups", "id", "id", None, datasources::EnrichmentPropertyType::Object)
            .linked(Association::new("user_id", "group_id")),
        seeded.groups.clone(),
        seeded.user_groups.clone(),
        None,
    );
let created = users
    .create(&mut ada(
        &seeded,
        encode_rows(&[seeded.admin_role.clone(), seeded.editor_role.clone()]),
        encode_rows(&[seeded.engineering.clone()]),
    ))
    .await
    .unwrap();
let created_role_links = seeded.user_roles.read(None, None, None).await.unwrap();
let created_group_links = seeded.user_groups.read(None, None, None).await.unwrap();
let mut next = ada(
    &seeded,
    encode_rows(&[seeded.editor_role.clone()]),
    encode_rows(&[seeded.engineering.clone(), seeded.ops.clone()]),
);
next.insert("id".into(), created.get("id").unwrap().clone());
let saved = users.update(&next, None).await.unwrap();
let updated = users
    .read_single_by_id(created.get("id").unwrap())
    .await
    .unwrap()
    .unwrap();
let updated_role_links = seeded.user_roles.read(None, None, None).await.unwrap();
let updated_group_links = seeded.user_groups.read(None, None, None).await.unwrap();
let removed = users
    .delete(&record(&[("id", created.get("id").unwrap().clone())]), None)
    .await
    .unwrap();
```

**Created**

```
{
  UserType: 'admin',
  Roles: ['admin', 'editor'],
  Groups: ['engineering']
}
```

**Updated**

```
{
  Roles: ['editor'],
  Groups: ['engineering', 'ops']
}
```

**After delete**

```
no user
roles and groups remain
```

#### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/UserViewCRUDRepository.test.ts#L107-L172)

```typescript
const seeded = await graph();
const usersRepo = seeded.usersRepo;
usersRepo.addEnriched({ destinationProperty: "user_type", sourceProperty: "user_type_id", relatedKey: "id" }, seeded.userTypesRepo);
usersRepo.addLinked(
  { destinationProperty: "roles", sourceProperty: "id", relatedKey: "id", association: { parent: "user_id", related: "role_id" } },
  seeded.rolesRepo,
  seeded.userRolesRepo,
);
usersRepo.addLinked(
  { destinationProperty: "groups", sourceProperty: "id", relatedKey: "id", association: { parent: "user_id", related: "group_id" } },
  seeded.groupsRepo,
  seeded.userGroupsRepo,
);
const created = await usersRepo.create(Object.assign(new UserView(), {
  username: "ada",
  is_enabled: true,
  user_type_id: seeded.adminType.id,
  roles: [seeded.adminRole, seeded.editorRole],
  groups: [seeded.engineering],
}));
const createdRoleLinks = await collectAsync(seeded.userRolesRepo.read());
const createdGroupLinks = await collectAsync(seeded.userGroupsRepo.read());
const saved = await usersRepo.update(Object.assign(new UserView(), {
  id: created.id,
  username: "ada",
  is_enabled: true,
  user_type_id: seeded.adminType.id,
  roles: [seeded.editorRole],
  groups: [seeded.engineering, seeded.ops],
}));
const updated = await usersRepo.readSingleById(created.id);
const updatedRoleLinks = await collectAsync(seeded.userRolesRepo.read());
const updatedGroupLinks = await collectAsync(seeded.userGroupsRepo.read());
const removed = await usersRepo.delete(Object.assign(new UserView(), { id: created.id }));
```

**Created**

```
{
  UserType: 'admin',
  Roles: ['admin', 'editor'],
  Groups: ['engineering']
}
```

**Updated**

```
{
  Roles: ['editor'],
  Groups: ['engineering', 'ops']
}
```

**After delete**

```
no user
roles and groups remain
```

Schema: [user-view-links](docs/examples.md#user-view-links)

```sql
CREATE TABLE users (id int, username string NOT NULL, is_enabled bool NOT NULL, user_type_id int NOT NULL, CONSTRAINT pk_users PRIMARY KEY (id));
CREATE TABLE user_types (id int, name string NOT NULL, CONSTRAINT pk_user_types PRIMARY KEY (id));
CREATE TABLE roles (id int, name string NOT NULL, CONSTRAINT pk_roles PRIMARY KEY (id));
CREATE TABLE groups (id int, name string NOT NULL, CONSTRAINT pk_groups PRIMARY KEY (id));
CREATE TABLE user_roles (user_id int NOT NULL, role_id int NOT NULL, CONSTRAINT pk_user_roles PRIMARY KEY (user_id, role_id));
CREATE TABLE user_groups (user_id int NOT NULL, group_id int NOT NULL, CONSTRAINT pk_user_groups PRIMARY KEY (user_id, group_id))
```

### Multi-tenant

`account` is identified by `tenant_id` and `id`. The same `id` in another tenant is a different row. `ReadSingleById` / `readSingleById` / `read_single_by_id`, update, and delete take both keys. Create omits every configured id column, so this sample seeds the rows.

#### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#L679-L712)

```csharp
var ds = new Datasource("sqlite", new SqliteDriver(), new Dictionary<string, string?> { ["DATASOURCE_URL"] = "sqlite:///:memory:" });
await Execute(connection, """
    INSERT INTO account (tenant_id, id, name) VALUES
        (1, 1, 'ada'),
        (2, 1, 'grace');
    """);
var casing = new ChangeCasingMapper(Casing.Pascal, Casing.Snake);
var accountsRepo = new CRUDRepository<Account>(connection, new CRUDRepositoryOptions<Account>
{
    EntityMapper = casing,
    PropertyMappings = casing,
    Ids = ["TenantId", "Id"],
});
var ada = await accountsRepo.ReadSingleById(new Row { ["TenantId"] = 1, ["Id"] = 1 });
ada!.Name = "ada-v2";
var updated = await accountsRepo.Update(ada);
var renamed = await accountsRepo.ReadSingleById(new Row { ["TenantId"] = 1, ["Id"] = 1 });
var removed = await accountsRepo.Delete(ada);
var gone = await accountsRepo.ReadSingleById(new Row { ["TenantId"] = 1, ["Id"] = 1 });
var grace = await accountsRepo.ReadSingleById(new Row { ["TenantId"] = 2, ["Id"] = 1 });
```

**Tenant 1**

```
ada-v2
```

**Tenant 2**

```
grace
```

#### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/readonly_repository.rs#L928-L974)

```rust
let built = datasource();
let ds: &dyn Datasource = &*built;
execute_sql(
    &connection,
    "
    INSERT INTO account (tenant_id, id, name) VALUES
        (1, 1, 'ada'),
        (2, 1, 'grace');
    ",
)
.await;
let mut options = create_crud_repository_options::<Account>();
let casing = ChangeCasingMapper::new(Casing::Pascal, Casing::Snake);
options.entity_mapper = Some(std::sync::Arc::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake)));
options.property_mappings = Some(std::sync::Arc::new(casing));
options.ids = vec!["TenantId".into(), "Id".into()];
options.row_converter = Some(RowTypeConverter::of::<Account>());
let accounts_repo = CRUDRepository::new(connection, options).unwrap();
let mut key = datasources::Row::new();
key.insert("TenantId".into(), Value::Int(1));
key.insert("Id".into(), Value::Int(1));
let mut ada = accounts_repo.read_single_by_id(&key).await.unwrap().unwrap();
ada.insert("Name".into(), Value::from("ada-v2"));
let updated = accounts_repo.update(&ada, None).await.unwrap();
let renamed = accounts_repo.read_single_by_id(&key).await.unwrap().unwrap();
let removed = accounts_repo.delete(&ada, None).await.unwrap();
let gone = accounts_repo.read_single_by_id(&key).await.unwrap();
let mut other = datasources::Row::new();
other.insert("TenantId".into(), Value::Int(2));
other.insert("Id".into(), Value::Int(1));
let grace = accounts_repo
    .read_single_by_id(&other)
    .await
    .unwrap()
    .unwrap();
```

**Tenant 1**

```
ada-v2
```

**Tenant 2**

```
grace
```

#### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/readonly.repository.integration.test.ts#L379-L409)

```typescript
const ds = new Datasource("sqlite", new SqliteDriver(), { DATASOURCE_URL: "sqlite:///:memory:" });
await executeSql(connection, `
  INSERT INTO account (tenant_id, id, name) VALUES
    (1, 1, 'ada'),
    (2, 1, 'grace');
`);
const casing = new ChangeCasingMapper("pascalCase", "snakeCase");
const accountsRepo = new CRUDRepository(connection, {
  ...createCRUDRepositoryOptions(Account),
  entityMapper: casing,
  propertyMappings: casing,
  ids: ["TenantId", "Id"],
});
const ada = await accountsRepo.readSingleById({ TenantId: 1, Id: 1 });
ada!.Name = "ada-v2";
const updated = await accountsRepo.update(ada!);
const renamed = await accountsRepo.readSingleById({ TenantId: 1, Id: 1 });
const removed = await accountsRepo.delete(ada!);
const gone = await accountsRepo.readSingleById({ TenantId: 1, Id: 1 });
const graceRow = await accountsRepo.readSingleById({ TenantId: 2, Id: 1 });
```

**Tenant 1**

```
ada-v2
```

**Tenant 2**

```
grace
```

Schema: [multi-tenant](docs/examples.md#multi-tenant)

```sql
CREATE TABLE account (tenant_id int NOT NULL, id int NOT NULL, name string NOT NULL, CONSTRAINT pk_account PRIMARY KEY (tenant_id, id))
```

### Enriched contacts write

`AddEnriched` / `addEnriched` / `add_enriched` fills the contact type and source names, and writes the address and phone lists with the contact.

#### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/ContactsRepositoryTests.cs#L74-L150)

```csharp
var conn = new SqliteConnection(new SqliteConnectionOptions
{
    Filename = ":memory:",
});
await conn.Open();
var command = new SqliteCommand(conn);
foreach (var sql in StatementsOf(File.ReadAllText(SeedPath())))
    await command.Execute(new Native(sql));

var contactsRepo = new CRUDRepository<Contact>(conn, new CRUDRepositoryOptions<Contact>
{
    EntityMapper = ContactColumns.ContactEntity,
    PropertyMappings = ContactColumns.Columns,
    Ids = ["Id"],
});
var typesRepo = new CRUDRepository<ContactType>(conn, new CRUDRepositoryOptions<ContactType>
{
    EntityMapper = ContactColumns.Snake,
    PropertyMappings = ContactColumns.Columns,
    Ids = ["Id"],
});
var sourcesRepo = new CRUDRepository<ContactSource>(conn, new CRUDRepositoryOptions<ContactSource>
{
    EntityMapper = ContactColumns.Snake,
    PropertyMappings = ContactColumns.Columns,
    Ids = ["Id"],
});
var addressTypesRepo = new CRUDRepository<AddressType>(conn, new CRUDRepositoryOptions<AddressType>
{
    EntityMapper = ContactColumns.Snake,
    PropertyMappings = ContactColumns.Columns,
    Ids = ["Id"],
});
var phoneTypesRepo = new CRUDRepository<PhoneType>(conn, new CRUDRepositoryOptions<PhoneType>
{
    EntityMapper = ContactColumns.Snake,
    PropertyMappings = ContactColumns.Columns,
    Ids = ["Id"],
});
var addressesRepo = new CRUDRepository<Address>(conn, new CRUDRepositoryOptions<Address>
{
    EntityMapper = ContactColumns.Snake,
    PropertyMappings = ContactColumns.Columns,
    Ids = ["Id"],
});
var phonesRepo = new CRUDRepository<Phone>(conn, new CRUDRepositoryOptions<Phone>
{
    EntityMapper = ContactColumns.Snake,
    PropertyMappings = ContactColumns.Columns,
    Ids = ["Id"],
});
addressesRepo.AddEnriched(new("AddressType", "AddressType", "Id"), addressTypesRepo);
phonesRepo.AddEnriched(new("PhoneType", "PhoneType", "Id"), phoneTypesRepo);
contactsRepo.AddEnriched(new("ContactType", "ContactType", "Id"), typesRepo);
contactsRepo.AddEnriched(new("ContactSource", "ContactSource", "Id"), sourcesRepo);
contactsRepo.AddEnriched(new("Addresses", "Id", "Contact", Type: EnrichmentPropertyType.List), addressesRepo);
contactsRepo.AddEnriched(new("Phones", "Id", "Contact", Type: EnrichmentPropertyType.List), phonesRepo);

var created = await contactsRepo.Create(ExtraContact.New(await ContactLookupIds.Resolve(conn)));
var loaded = await contactsRepo.ReadSingle(new Where("Id", eq, created.Id));
```

**Created**

```
{
  first_name: 'Extra',
  contact_type_name: 'Person',
  addresses: ['9 Extra'],
  phones: ['555-extra']
}
```

#### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/contacts.rs#L130-L274)

```rust
let conn = Arc::new(SqliteConnection::new(Some(":memory:".into())));
conn.open().await.unwrap();
let command = SqliteCommand::new(conn.clone());
let sql = statements_of(&contact_seed("sqlite"));
apply_sql(&command, &sql).await.unwrap();

let field = super::converters::field;
let named = ["Id", "Uuid", "Created", "Updated", "Version", "Name", "Description"];
let address = [
    "Id", "Uuid", "Created", "Updated", "Version", "Contact", "AddressType", "Line1", "Line2",
    "City", "Region", "PostalCode", "Country",
];
let phone = [
    "Id", "Uuid", "Created", "Updated", "Version", "Contact", "PhoneType", "Number",
];
let base = [
    "Id", "Uuid", "Created", "Updated", "Version", "ContactSource", "ContactType", "FirstName",
    "LastName", "Email", "Notes",
];
let mut addresses_repo = contact_repo(conn.clone(), "Address", &address, false, field);
addresses_repo.add_enriched(
    datasources::Enrichment::new("AddressType", "AddressType", "Id", None, datasources::EnrichmentPropertyType::Object),
    as_readonly(Arc::new(contact_repo(conn.clone(), "AddressType", &named, false, field))),
    None,
);
let mut phones_repo = contact_repo(conn.clone(), "Phone", &phone, false, field);
phones_repo.add_enriched(
    datasources::Enrichment::new("PhoneType", "PhoneType", "Id", None, datasources::EnrichmentPropertyType::Object),
    as_readonly(Arc::new(contact_repo(conn.clone(), "PhoneType", &named, false, field))),
    None,
);
let mut contacts_repo = contact_repo(conn.clone(), "Contact", &base, true, field);
contacts_repo
    .add_enriched(
        datasources::Enrichment::new("ContactType", "ContactType", "Id", None, datasources::EnrichmentPropertyType::Object),
        as_readonly(Arc::new(contact_repo(conn.clone(), "ContactType", &named, false, field))),
        None,
    )
    .add_enriched(
        datasources::Enrichment::new("ContactSource", "ContactSource", "Id", None, datasources::EnrichmentPropertyType::Object),
        as_readonly(Arc::new(contact_repo(conn.clone(), "ContactSource", &named, false, field))),
        None,
    )
    .add_enriched(
        datasources::Enrichment::new("Addresses", "Id", "Contact", None, datasources::EnrichmentPropertyType::List),
        Arc::new(addresses_repo),
        None,
    )
    .add_enriched(
        datasources::Enrichment::new("Phones", "Id", "Contact", None, datasources::EnrichmentPropertyType::List),
        Arc::new(phones_repo),
        None,
    );

let ids = contact_lookup_ids(conn).await.unwrap();
let stamp = Value::DateTime(
    chrono::DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z")
        .expect("stamp")
        .with_timezone(&chrono::Utc),
);
let version = Value::Binary(vec![]);
let mut created = contact_row(&[
    ("Uuid", Value::Uuid(uuid::Uuid::new_v4())),
    ("Created", stamp.clone()),
    ("Updated", stamp.clone()),
    ("Version", version.clone()),
    ("ContactSource", Value::Int(ids.contact_source_id)),
    ("ContactType", Value::Int(ids.contact_type_id)),
    ("FirstName", Value::from("Extra")),
    ("LastName", Value::from("Contact")),
    (
        "Addresses",
        encode_rows(&[contact_row(&[
            ("Uuid", Value::Uuid(uuid::Uuid::new_v4())),
            ("Created", stamp.clone()),
            ("Updated", stamp.clone()),
            ("Version", version.clone()),
            ("AddressType", Value::Int(ids.address_type_id)),
            ("Line1", Value::from("9 Extra")),
            ("City", Value::from("Oslo")),
        ])]),
    ),
    (
        "Phones",
        encode_rows(&[contact_row(&[
            ("Uuid", Value::Uuid(uuid::Uuid::new_v4())),
            ("Created", stamp.clone()),
            ("Updated", stamp),
            ("Version", version),
            ("PhoneType", Value::Int(ids.phone_type_id)),
            ("Number", Value::from("555-extra")),
        ])]),
    ),
]);
let created = contacts_repo.create(&mut created).await.unwrap();
let Value::Int(id) = created.get("Id").cloned().unwrap_or(Value::Null) else {
    panic!("expected insert id");
};
let loaded = contacts_repo
    .read_single(
        Some(&Where::new(Operator::equals("Id", Value::Int(id)))),
        None, None
    )
    .await
    .unwrap()
    .unwrap();
```

**Created**

```
{
  first_name: 'Extra',
  contact_type_name: 'Person',
  addresses: ['9 Extra'],
  phones: ['555-extra']
}
```

#### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/contacts.repository.integration.test.ts#L94-L135)

```typescript
const connection = SqliteConnection.from({ DATASOURCE_URL: "sqlite:///:memory:" });
await connection.open();
const command = new SqliteCommand(connection);
for (const sql of statementsOf(readFileSync(seedPath, "utf8")))
  await command.executeNative(new Native(sql));

const contactsRepo = new CRUDRepository(connection, {
  type: Contact,
  entityMapper: contactEntity,
  propertyMappings: columns,
  ids: ["Id"],
});
const typesRepo = new CRUDRepository(connection, { type: ContactType, entityMapper: snake, propertyMappings: columns, ids: ["Id"] });
const sourcesRepo = new CRUDRepository(connection, { type: ContactSource, entityMapper: snake, propertyMappings: columns, ids: ["Id"] });
const addressTypesRepo = new CRUDRepository(connection, { type: AddressType, entityMapper: snake, propertyMappings: columns, ids: ["Id"] });
const phoneTypesRepo = new CRUDRepository(connection, { type: PhoneType, entityMapper: snake, propertyMappings: columns, ids: ["Id"] });
const addressesRepo = new CRUDRepository(connection, { type: Address, entityMapper: snake, propertyMappings: columns, ids: ["Id"] });
const phonesRepo = new CRUDRepository(connection, { type: Phone, entityMapper: snake, propertyMappings: columns, ids: ["Id"] });
addressesRepo.addEnriched({ destinationProperty: "AddressType", sourceProperty: "AddressType", relatedKey: "Id" }, addressTypesRepo);
phonesRepo.addEnriched({ destinationProperty: "PhoneType", sourceProperty: "PhoneType", relatedKey: "Id" }, phoneTypesRepo);
contactsRepo.addEnriched({ destinationProperty: "ContactType", sourceProperty: "ContactType", relatedKey: "Id" }, typesRepo);
contactsRepo.addEnriched({ destinationProperty: "ContactSource", sourceProperty: "ContactSource", relatedKey: "Id" }, sourcesRepo);
contactsRepo.addEnriched({ destinationProperty: "Addresses", sourceProperty: "Id", relatedKey: "Contact", type: "list" }, addressesRepo);
contactsRepo.addEnriched({ destinationProperty: "Phones", sourceProperty: "Id", relatedKey: "Contact", type: "list" }, phonesRepo);

const created = await contactsRepo.create(extraContact(await contactLookupIds(connection)));
const loaded = await contactsRepo.readSingle(new Where(new Operator.Equals("Id", created.Id)));
```

**Created**

```
{
  first_name: 'Extra',
  contact_type_name: 'Person',
  addresses: ['9 Extra'],
  phones: ['555-extra']
}
```

Schema: [contacts](docs/examples.md#contacts)

```sql
CREATE TABLE contact_source (id int, uuid uuid NOT NULL, created datetime NOT NULL, updated datetime NOT NULL, version binary NOT NULL, name string NOT NULL, description string, CONSTRAINT pk_contact_source PRIMARY KEY (id));
CREATE TABLE contact_type (id int, uuid uuid NOT NULL, created datetime NOT NULL, updated datetime NOT NULL, version binary NOT NULL, name string NOT NULL, description string, CONSTRAINT pk_contact_type PRIMARY KEY (id));
CREATE TABLE contacts_base (id int, uuid uuid NOT NULL, created datetime NOT NULL, updated datetime NOT NULL, version binary NOT NULL, contact_source_id int NOT NULL, contact_type_id int NOT NULL, first_name string NOT NULL, last_name string NOT NULL, email string, notes string, CONSTRAINT pk_contacts_base PRIMARY KEY (id), CONSTRAINT fk_contacts_source FOREIGN KEY (contact_source_id) REFERENCES contact_source (id), CONSTRAINT fk_contacts_type FOREIGN KEY (contact_type_id) REFERENCES contact_type (id));
CREATE TABLE address_type (id int, uuid uuid NOT NULL, created datetime NOT NULL, updated datetime NOT NULL, version binary NOT NULL, name string NOT NULL, description string, CONSTRAINT pk_address_type PRIMARY KEY (id));
CREATE TABLE phone_type (id int, uuid uuid NOT NULL, created datetime NOT NULL, updated datetime NOT NULL, version binary NOT NULL, name string NOT NULL, description string, CONSTRAINT pk_phone_type PRIMARY KEY (id));
CREATE TABLE address (id int, uuid uuid NOT NULL, created datetime NOT NULL, updated datetime NOT NULL, version binary NOT NULL, contact_id int NOT NULL, address_type_id int NOT NULL, line1 string NOT NULL, line2 string, city string NOT NULL, region string, postal_code string, country string, CONSTRAINT pk_address PRIMARY KEY (id), CONSTRAINT fk_address_contact FOREIGN KEY (contact_id) REFERENCES contacts_base (id), CONSTRAINT fk_address_type FOREIGN KEY (address_type_id) REFERENCES address_type (id));
CREATE TABLE phone (id int, uuid uuid NOT NULL, created datetime NOT NULL, updated datetime NOT NULL, version binary NOT NULL, contact_id int NOT NULL, phone_type_id int NOT NULL, number string NOT NULL, CONSTRAINT pk_phone PRIMARY KEY (id), CONSTRAINT fk_phone_contact FOREIGN KEY (contact_id) REFERENCES contacts_base (id), CONSTRAINT fk_phone_type FOREIGN KEY (phone_type_id) REFERENCES phone_type (id));
CREATE TABLE contact_groups_base (id int, uuid uuid NOT NULL, created datetime NOT NULL, updated datetime NOT NULL, version binary NOT NULL, name string NOT NULL, description string, CONSTRAINT pk_contact_groups_base PRIMARY KEY (id));
CREATE TABLE contact_group_member (id int, uuid uuid NOT NULL, created datetime NOT NULL, updated datetime NOT NULL, version binary NOT NULL, contact_id int NOT NULL, contact_group_id int NOT NULL, CONSTRAINT pk_contact_group_member PRIMARY KEY (id), CONSTRAINT fk_member_contact FOREIGN KEY (contact_id) REFERENCES contacts_base (id), CONSTRAINT fk_member_group FOREIGN KEY (contact_group_id) REFERENCES contact_groups_base (id), CONSTRAINT uq_contact_group_member UNIQUE (contact_id, contact_group_id));
CREATE TABLE legacy_contact (key string, first_name string NOT NULL, last_name string NOT NULL, email string, imported_at datetime, CONSTRAINT pk_legacy_contact PRIMARY KEY (key));
CREATE TABLE contact_change_log (id int, uuid uuid NOT NULL, created datetime NOT NULL, updated datetime NOT NULL, version binary NOT NULL, occurred_at datetime NOT NULL, contact_id int NOT NULL, changed_by string NOT NULL, change_summary string NOT NULL, CONSTRAINT pk_contact_change_log PRIMARY KEY (id))
```

### Type mapping

`UsersView` maps onto `user`. `Key` is `id`, `Login` is `username`, and `UserTypeId` is `user_type_id`. `UserType` is the enriched name and is not a column. Create writes the generated id back onto `Key`.

#### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#L206-L293)

```csharp
var ds = Datasource.Default;
var casing = new ChangeCasingMapper(Casing.Pascal, Casing.Snake);
var usersRepo = new CRUDRepository<User>(connection, new CRUDRepositoryOptions<User>
{
    Ids = ["Key"],
    EntityMapper = casing,
    PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
    {
        ["Key"] = "id",
        ["Login"] = "username",
        ["Active"] = "is_enabled",
        ["Opened"] = "created",
        ["UserType"] = "user_type_id",
        ["Version"] = "version",
    }),
});
var userTypesRepo = new CRUDRepository<UserType>(connection, new CRUDRepositoryOptions<UserType>
{
    Ids = ["Id"],
    EntityMapper = casing,
    PropertyMappings = casing,
});
var userRolesRepo = new CRUDRepository<UserRole>(connection, new CRUDRepositoryOptions<UserRole>
{
    Ids = [],
    EntityMapper = casing,
    PropertyMappings = casing,
});
usersRepo.AddEnriched(new("UserType", "UserType", "Id"), userTypesRepo);
usersRepo.AddEnriched(new("Roles", "Key", "UserId", Type: EnrichmentPropertyType.List), userRolesRepo);
var admin = await userTypesRepo.Create(new UserType { Name = "admin" });
var user = new User
{
    Login = "ada",
    Active = true,
    Opened = new DateTime(2020, 1, 1, 0, 0, 0, DateTimeKind.Utc),
    UserType = admin,
    Version = 1,
    Roles = [new UserRole { Name = "admin" }, new UserRole { Name = "editor" }],
};
var created = await usersRepo.Create(user);
var grace = await usersRepo.Create(new User
{
    Login = "grace",
    Active = true,
    Opened = new DateTime(2024, 6, 1, 0, 0, 0, DateTimeKind.Utc),
    UserType = admin,
    Version = 1,
    Roles = [new UserRole { Name = "viewer" }],
});
var loaded = await usersRepo.ReadSingle(new Where("Key", eq, user.Key));
var version = loaded!.Version;
var member = await userTypesRepo.Create(new UserType { Name = "member" });
loaded.Login = "ada-v2";
loaded.UserType = member;
loaded.Roles.Add(new UserRole { Name = "owner" });
loaded.Version = version + 1;
var updated = await usersRepo.Update(loaded, new Where("Key", eq, loaded.Key).And("Version", eq, version));
var current = await usersRepo.ReadSingle(new Where("Key", eq, loaded.Key));
var stale = await usersRepo.Update(loaded, new Where("Key", eq, loaded.Key).And("Version", eq, version));
var removed = await usersRepo.Delete(loaded);
var gone = await usersRepo.ReadSingleById(loaded.Key);
```

**Created**

```
{
  id: 1,
  Key: 1
}
```

**Enriched Ada**

```
{
  UserType: 'admin',
  Roles: ['admin', 'editor']
}
```

**Loaded**

```
{
  Key: 1,
  Login: 'ada',
  Active: true,
  UserType: {
    Id: 1,
    Name: 'admin'
  },
  Roles: ['admin', 'editor'],
  Version: 1
}
```

**Versioned update**

```
{
  Key: 1,
  Login: 'ada-v2',
  Active: true,
  UserType: {
    Id: 2,
    Name: 'member'
  },
  Roles: ['admin', 'editor', 'owner'],
  Version: 2
}
```

**Stale update**

```
{
  rowsAffected: 0
}
```

**After delete**

```
no row
```

#### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/readonly_repository.rs#L760-L913)

```rust
let ds = default().unwrap();
let mut users_options = create_crud_repository_options::<User>();
users_options.ids = vec!["Key".into()];
users_options.entity_mapper =
    Some(std::sync::Arc::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake)));
users_options.property_mappings = Some(std::sync::Arc::new(DictionaryMapper::new([
    ("Key".to_string(), "id".to_string()),
    ("Login".to_string(), "username".to_string()),
    ("Active".to_string(), "is_enabled".to_string()),
    ("Opened".to_string(), "created".to_string()),
    ("UserType".to_string(), "user_type_id".to_string()),
    ("Version".to_string(), "version".to_string()),
])));
users_options.row_converter = Some(RowTypeConverter::of::<User>());
users_options.object_converter = Some(Box::new(
    MetaDataRowTypeConverter::of::<User>().with_references(
        vec!["UserType".into()],
        Box::new(DictionaryMapper::new([
            ("Key".to_string(), "id".to_string()),
            ("Login".to_string(), "username".to_string()),
            ("Active".to_string(), "is_enabled".to_string()),
            ("Opened".to_string(), "created".to_string()),
            ("UserType".to_string(), "user_type_id".to_string()),
            ("Version".to_string(), "version".to_string()),
        ])),
    ),
));
let casing = ChangeCasingMapper::new(Casing::Pascal, Casing::Snake);
let mut type_options = create_crud_repository_options::<UserType>();
type_options.entity_mapper =
    Some(std::sync::Arc::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake)));
type_options.property_mappings = Some(std::sync::Arc::new(casing));
type_options.row_converter = Some(RowTypeConverter::of::<UserType>());
let mut role_options = create_crud_repository_options::<UserRole>();
role_options.entity_mapper =
    Some(std::sync::Arc::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake)));
role_options.property_mappings =
    Some(std::sync::Arc::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake)));
role_options.row_converter = Some(RowTypeConverter::of::<UserRole>());
let mut users_repo = CRUDRepository::new(connection.clone(), users_options).unwrap();
let user_types_repo =
    Arc::new(CRUDRepository::new(connection.clone(), type_options).unwrap());
let user_roles_repo = Arc::new(CRUDRepository::new(connection, role_options).unwrap());
users_repo.add_enriched(
    datasources::Enrichment::new(
        "UserType",
        "UserType",
        "Id",
        None,
        datasources::EnrichmentPropertyType::Object,
    ),
    user_types_repo.clone(),
    None,
);
users_repo.add_enriched(
    datasources::Enrichment::new("Roles", "Key", "UserId", None, datasources::EnrichmentPropertyType::List),
    user_roles_repo.clone(),
    None,
);
let mut admin = datasources::Row::new();
admin.insert("Name".into(), Value::from("admin"));
let admin = user_types_repo.create(&mut admin).await.unwrap();
let mut admin_role = datasources::Row::new();
admin_role.insert("Name".into(), Value::from("admin"));
let mut editor_role = datasources::Row::new();
editor_role.insert("Name".into(), Value::from("editor"));
let mut user = datasources::Row::new();
user.insert("Login".into(), Value::from("ada"));
user.insert("Active".into(), Value::Bool(true));
user.insert("Opened".into(), Value::from("2020-01-01T00:00:00.000Z"));
user.insert("UserType".into(), admin.get("Id").cloned().unwrap());
user.insert("Version".into(), Value::Int(1));
user.insert("Roles".into(), encode_rows(&[admin_role, editor_role]));
let user = users_repo.create(&mut user).await.unwrap();
let mut viewer_role = datasources::Row::new();
viewer_role.insert("Name".into(), Value::from("viewer"));
let mut grace = datasources::Row::new();
grace.insert("Login".into(), Value::from("grace"));
grace.insert("Active".into(), Value::Bool(true));
grace.insert("Opened".into(), Value::from("2024-06-01T00:00:00.000Z"));
grace.insert("UserType".into(), admin.get("Id").cloned().unwrap());
grace.insert("Version".into(), Value::Int(1));
grace.insert("Roles".into(), encode_rows(&[viewer_role]));
let grace = users_repo.create(&mut grace).await.unwrap();
let key = user.get("Key").cloned().unwrap();
let mut loaded = users_repo
    .read_single(Some(&Where::binary("Key", eq, key.clone())), None, None)
    .await
    .unwrap()
    .unwrap();
let version = loaded.get("Version").cloned().unwrap();
let mut member = datasources::Row::new();
member.insert("Name".into(), Value::from("member"));
let member = user_types_repo.create(&mut member).await.unwrap();
let mut owner_role = datasources::Row::new();
owner_role.insert("Name".into(), Value::from("owner"));
let mut kept_admin = datasources::Row::new();
kept_admin.insert("Name".into(), Value::from("admin"));
let mut kept_editor = datasources::Row::new();
kept_editor.insert("Name".into(), Value::from("editor"));
loaded.insert("Login".into(), Value::from("ada-v2"));
loaded.insert("UserType".into(), member.get("Id").cloned().unwrap());
loaded.insert(
    "Roles".into(),
    encode_rows(&[kept_admin, kept_editor, owner_role]),
);
loaded.insert("Version".into(), Value::Int(2));
let updated = users_repo
    .update(
        &loaded,
        Some(&Where::binary("Key", eq, key.clone()).and("Version", eq, version.clone())),
    )
    .await
    .unwrap();
let current = users_repo
    .read_single(Some(&Where::binary("Key", eq, key.clone())), None, None)
    .await
    .unwrap()
    .unwrap();
let stale = users_repo
    .update(
        &loaded,
        Some(&Where::binary("Key", eq, key.clone()).and("Version", eq, version)),
    )
    .await
    .unwrap();
let removed = users_repo.delete(&loaded, None).await.unwrap();
let gone = users_repo.read_single_by_id(&key).await.unwrap();
```

**Created**

```
{
  id: 1,
  Key: 1
}
```

**Enriched Ada**

```
{
  UserType: 'admin',
  Roles: ['admin', 'editor']
}
```

**Loaded**

```
{
  Key: 1,
  Login: 'ada',
  Active: true,
  UserType: {
    Id: 1,
    Name: 'admin'
  },
  Roles: ['admin', 'editor'],
  Version: 1
}
```

**Versioned update**

```
{
  Key: 1,
  Login: 'ada-v2',
  Active: true,
  UserType: {
    Id: 2,
    Name: 'member'
  },
  Roles: ['admin', 'editor', 'owner'],
  Version: 2
}
```

**Stale update**

```
{
  rowsAffected: 0
}
```

**After delete**

```
no row
```

#### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/readonly.repository.integration.test.ts#L288-L369)

```typescript
const ds = Datasource.default;
const casing = new ChangeCasingMapper("pascalCase", "snakeCase");
const usersRepo = new CRUDRepository(connection, {
  ...createCRUDRepositoryOptions(User),
  ids: ["Key"],
  entity: "User",
  entityMapper: casing,
  propertyMappings: new DictionaryMapper([
    ["Key", "id"],
    ["Login", "username"],
    ["Active", "is_enabled"],
    ["Opened", "created"],
    ["UserType", "user_type_id"],
    ["Version", "version"],
  ]),
});
const userTypesRepo = new CRUDRepository(connection, {
  ...createCRUDRepositoryOptions(UserType),
  entity: "UserType",
  entityMapper: casing,
  propertyMappings: casing,
});
const userRolesRepo = new CRUDRepository(connection, {
  ...createCRUDRepositoryOptions(UserRole),
  entity: "UserRole",
  entityMapper: casing,
  propertyMappings: casing,
});
usersRepo.addEnriched({ destinationProperty: "UserType", sourceProperty: "UserType", relatedKey: "Id" }, userTypesRepo);
usersRepo.addEnriched({ destinationProperty: "Roles", sourceProperty: "Key", relatedKey: "UserId", type: "list" }, userRolesRepo);
const admin = await userTypesRepo.create(Object.assign(new UserType(), { Name: "admin" }));
const user = Object.assign(new User(), {
  Login: "ada",
  Active: true,
  Opened: "2020-01-01T00:00:00.000Z",
  UserType: admin,
  Version: 1,
  Roles: [Object.assign(new UserRole(), { Name: "admin" }), Object.assign(new UserRole(), { Name: "editor" })],
});
const created = await usersRepo.create(user);
const grace = await usersRepo.create(Object.assign(new User(), {
  Login: "grace",
  Active: true,
  Opened: "2024-06-01T00:00:00.000Z",
  UserType: admin,
  Version: 1,
  Roles: [Object.assign(new UserRole(), { Name: "viewer" })],
}));
const loaded = await usersRepo.readSingle(new Where("Key", eq, user.Key));
const version = loaded!.Version;
const member = await userTypesRepo.create(Object.assign(new UserType(), { Name: "member" }));
loaded!.Login = "ada-v2";
loaded!.UserType = member;
loaded!.Roles.push(Object.assign(new UserRole(), { Name: "owner" }));
loaded!.Version = version + 1;
const updated = await usersRepo.update(loaded!, new Where("Key", eq, loaded!.Key).and("Version", eq, version));
const current = await usersRepo.readSingle(new Where("Key", eq, loaded!.Key));
const stale = await usersRepo.update(loaded!, new Where("Key", eq, loaded!.Key).and("Version", eq, version));
const removed = await usersRepo.delete(loaded!);
const gone = await usersRepo.readSingleById(loaded!.Key);
```

**Created**

```
{
  id: 1,
  Key: 1
}
```

**Enriched Ada**

```
{
  UserType: 'admin',
  Roles: ['admin', 'editor']
}
```

**Loaded**

```
{
  Key: 1,
  Login: 'ada',
  Active: true,
  UserType: {
    Id: 1,
    Name: 'admin'
  },
  Roles: ['admin', 'editor'],
  Version: 1
}
```

**Versioned update**

```
{
  Key: 1,
  Login: 'ada-v2',
  Active: true,
  UserType: {
    Id: 2,
    Name: 'member'
  },
  Roles: ['admin', 'editor', 'owner'],
  Version: 2
}
```

**Stale update**

```
{
  rowsAffected: 0
}
```

**After delete**

```
no row
```

Schema: [user-view-version](docs/examples.md#user-view-version)

```sql
CREATE TABLE user_type (id int, name string NOT NULL, CONSTRAINT pk_user_type PRIMARY KEY (id));
CREATE TABLE user (id int, username string NOT NULL, is_enabled bool NOT NULL, created datetime NOT NULL, user_type_id int NOT NULL, version int NOT NULL, CONSTRAINT pk_user PRIMARY KEY (id));
CREATE TABLE user_role (user_id int NOT NULL, name string NOT NULL)
```
