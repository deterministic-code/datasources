# Readonly repository

Read-only typed read over a connection. {{ReadSingle|readSingle|read_single}} returns the first row or none. {{Read|read|read}} yields every row.

Construct it with a connection, then register each fill with {{AddEnriched|addEnriched|add_enriched}}. That fill runs after convert-then-map. Lookups use {{ReadSingle|readSingle|read_single}} (cached per distinct key). Lists use {{Read|read|read}}. Missing lookups leave the destination default. This type does not write. Writes are [CRUDRepository](./crud-repository.md).

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
readSingle(query?: ISelectStatement | IWhere | IExecuteStoredProcedureStatement): Promise<T | undefined>
```

- `query` — optional query, where, or stored procedure; omitted uses the default select

**Returns** `Promise<T | undefined>`

##### read(query?)

Yields every item.

```typescript
read(query?: ISelectStatement | IWhere | IExecuteStoredProcedureStatement): AsyncIterable<T>
```

- `query` — optional query, where, or stored procedure; omitted uses the default select

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

##### ReadonlyRepository(inner, strategy?)

Wraps an inner repository. `read` reads that repository, then fills related fields. Omitted strategy uses the default. See [IEnriched](#ienriched).

```typescript
constructor(inner: IReadonlyRepository<T>, strategy?: IReadonlyEnrichmentStrategy<T>)
```

- `inner` — [IReadonlyRepository](#ireadonlyrepository)
- `strategy` — optional enrichment strategy

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
abstract read(query?: ISelectStatement | IWhere | IExecuteStoredProcedureStatement): AsyncIterable<T>
```

- `query` — optional query, where, or stored procedure; omitted uses the default select

**Returns** `AsyncIterable<T>`

##### readSingle(query?)

Returns the first item from [read](#readonlyrepository-read-query), or `undefined`.

```typescript
readSingle(query?: ISelectStatement | IWhere | IExecuteStoredProcedureStatement): Promise<T | undefined>
```

- `query` — optional query, where, or stored procedure; omitted uses the default select

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

##### columnMappings

Maps property names to stored columns. Default is [IdentityMapper](./identity-mapper.md). The repository maps where identifiers and write parameters. Read reverse-maps each result key before the row converter.

```typescript
columnMappings?: IMapper<string, string>
```

##### query

Rewrites the finished select. Omitted leaves the statement unchanged.

```typescript
query?: IMapper<ISelectStatement, ISelectStatement>
```

##### rowsMapper

Maps each converted item. Default is [IdentityMapper](./identity-mapper.md).

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

Provider converters keyed by `bool`, `datetime`, `uuid`, `decimal`, and `binary`, merged into the row converter. The driver applies its converters when it binds a parameter. A field converter reverse-converts that property in the repository.

```typescript
rowTypeConverters?: Record<string, TypeConverter<Cell, Cell>>
```

##### readColumns

SELECT column names. When omitted, public fields of `type` are used.

```typescript
readColumns?: string[]
```

#### IEnriched

```typescript
interface IEnriched<T>
```

**Methods**

##### addEnriched(enrichment, via, strategy?)

Adds a related fill. Fluent for siblings. A related repository fills its own fields. Does not write.

```typescript
addEnriched<TRelated extends object>(enrichment: Enrichment, via: IReadonlyRepository<TRelated>, strategy?: IReadonlyEnrichmentStrategy<T & object>): this
```

- `enrichment` — [Enrichment](#enrichment)
- `via` — related [IReadonlyRepository](./readonly-repository.md)
- `strategy` — optional read strategy for this edge. Omitted uses the default strategy

**Returns** `this`

#### IRepositoryTree

```typescript
interface IRepositoryTree
```

**Properties**

##### parent

Root [IRepositoryTreeNode](#irepositorytreenode).

```typescript
readonly parent: IRepositoryTreeNode
```

#### IRepositoryTreeNode

```typescript
interface IRepositoryTreeNode
```

**Properties**

##### value

Related repository.

```typescript
readonly value: object
```

##### propertyName

Destination property name.

```typescript
readonly propertyName: string
```

##### parent

Parent node, if any.

```typescript
readonly parent?: IRepositoryTreeNode
```

##### items

Child nodes.

```typescript
readonly items: IRepositoryTreeNode[]
```

##### enrichment

[Enrichment](#enrichment) for this node.

```typescript
readonly enrichment?: Enrichment
```

**Methods**

##### include(via, into, from, by, select?, many?)

Adds a child node.

```typescript
include<TRelated extends object>(via: IReadonlyRepository<TRelated>, into: string, from: string, by: string, select?: string, many?: boolean): IRepositoryTreeNode
```

- `via` — related [IReadonlyRepository](./readonly-repository.md)
- `into` — destination property
- `from` — parent key
- `by` — related key
- `select` — optional related property to copy
- `many` — collect a list

**Returns** [IRepositoryTreeNode](#irepositorytreenode)

##### enrichFrom(parents)

Fills `parents` from this node.

```typescript
enrichFrom(parents: object[]): Promise<void>
```

- `parents` — parent objects

#### Enrichment

```typescript
type Enrichment
```

One related fill.

**Properties**

##### into

Destination property.

```typescript
into: string
```

##### from

Parent key.

```typescript
from: string
```

##### by

Related key.

```typescript
by: string
```

##### select

Optional related property to copy.

```typescript
select?: string
```

##### many

Collect a list.

```typescript
many?: boolean
```

#### IReadonlyEnrichmentStrategy

```typescript
interface IReadonlyEnrichmentStrategy<T extends object> extends IReadonlyEnrichmentBinder
```

**Methods**

##### read(inner, tree, query?)

Reads through `inner`, then enriches.

```typescript
read(inner: IReadonlyRepository<T>, tree: IRepositoryTree, query?: ISelectStatement | IWhere | IExecuteStoredProcedureStatement): AsyncIterable<T>
```

- `inner` — [IReadonlyRepository](./readonly-repository.md)
- `tree` — [IRepositoryTree](#irepositorytree)
- `query` — optional query, where, or stored procedure

**Returns** `AsyncIterable<T>`

##### enrich(tree, items)

Fills `items` from the tree.

```typescript
enrich(tree: IRepositoryTree, items: T[]): Promise<void>
```

- `tree` — [IRepositoryTree](#irepositorytree)
- `items` — parents to fill

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

- `query` — optional query, where, or stored-procedure statement; omitted uses the default select

**Returns** `Task<T?>`

##### Read(query)

Yields every item.

```csharp
IAsyncEnumerable<T> Read(IStatement? query = null)
```

- `query` — optional query, where, or stored-procedure statement; omitted uses the default select

**Returns** `IAsyncEnumerable<T>`

#### ReadonlyRepository

```csharp
public class ReadonlyRepository<T> : IReadonlyRepository<T>, IEnriched<T>
```

**Constructor**

##### ReadonlyRepository(inner, strategy)

Wraps an inner repository. `Read` reads that repository, then fills related fields. Omitted strategy uses the default. See [IEnriched](#ienriched).

```csharp
public ReadonlyRepository(IReadonlyRepository<T> inner, IReadonlyEnrichmentStrategy<T>? strategy = null)
```

- `inner` — [IReadonlyRepository](#ireadonlyrepository)
- `strategy` — optional enrichment strategy

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

- `query` — optional query, where, or stored-procedure statement; omitted uses the default select

**Returns** `IAsyncEnumerable<T>`

##### ReadSingle(query)

Returns the first item from [Read](#readonlyrepository-read-query), or `default`.

```csharp
public virtual Task<T?> ReadSingle(IStatement? query = null)
```

- `query` — optional query, where, or stored-procedure statement; omitted uses the default select

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

##### ColumnMappings

Maps property names to stored columns. Default is [IdentityMapper](./identity-mapper.md). The repository maps where identifiers and write parameters. Read reverse-maps each result key before the row converter.

```csharp
IMapper<string, string> ColumnMappings { get; init; }
```

##### Query

Rewrites the finished select. Omitted leaves the statement unchanged.

```csharp
IMapper<ISelectStatement, ISelectStatement>? Query { get; init; }
```

##### RowsMapper

Maps each converted item. Default is [IdentityMapper](./identity-mapper.md).

```csharp
IMapper<T, T> RowsMapper { get; init; }
```

##### RowConverter

Row [TypeConverter](./type-converters.md).

```csharp
TypeConverter<T, Row>? RowConverter { get; init; }
```

##### RowTypeConverters

Provider converters keyed by `bool`, `datetime`, `uuid`, `decimal`, and `binary`, merged into the row converter. The driver applies its converters when it binds a parameter. A field converter reverse-converts that property in the repository.

```csharp
IReadOnlyDictionary<string, TypeConverter<Value, Value>>? RowTypeConverters { get; init; }
```

##### ReadColumns

SELECT column names. When omitted, public properties of `T` are used.

```csharp
IReadOnlyList<string>? ReadColumns { get; init; }
```

#### IEnriched

```csharp
public interface IEnriched<T>
```

**Methods**

##### AddEnriched(enrichment, via, strategy)

Adds a related fill. Fluent for siblings. A related repository fills its own fields. Does not write.

```csharp
IEnriched<T> AddEnriched<TRelated>(Enrichment enrichment, IReadonlyRepository<TRelated> via, IReadonlyEnrichmentStrategy<T>? strategy = null)
```

- `enrichment` — [Enrichment](#enrichment)
- `via` — related [IReadonlyRepository](./readonly-repository.md)
- `strategy` — optional read strategy for this edge. Omitted uses the default strategy

**Returns** [IEnriched](#ienriched)`<T>`

#### IRepositoryTree

```csharp
public interface IRepositoryTree
```

**Properties**

##### Parent

Root [IRepositoryTreeNode](#irepositorytreenode).

```csharp
IRepositoryTreeNode Parent { get; }
```

#### IRepositoryTreeNode

```csharp
public interface IRepositoryTreeNode
```

**Properties**

##### Value

Related repository.

```csharp
object Value { get; }
```

##### PropertyName

Destination property name.

```csharp
string PropertyName { get; }
```

##### Parent

Parent node, if any.

```csharp
IRepositoryTreeNode? Parent { get; }
```

##### Items

Child nodes.

```csharp
IReadOnlyList<IRepositoryTreeNode> Items { get; }
```

##### Enrichment

[Enrichment](#enrichment) for this node.

```csharp
Enrichment? Enrichment { get; }
```

**Methods**

##### Include(via, into, from, by, select, many)

Adds a child node.

```csharp
IRepositoryTreeNode Include<TRelated>(IReadonlyRepository<TRelated> via, string into, string from, string by, string? select = null, bool many = false)
```

- `via` — related [IReadonlyRepository](./readonly-repository.md)
- `into` — destination property
- `from` — parent key
- `by` — related key
- `select` — optional related property to copy
- `many` — collect a list

**Returns** [IRepositoryTreeNode](#irepositorytreenode)

##### EnrichFrom(parents)

Fills `parents` from this node.

```csharp
Task EnrichFrom(IReadOnlyList<object> parents)
```

- `parents` — parent objects

#### Enrichment

```csharp
public sealed record Enrichment
```

One related fill.

**Properties**

##### Into

Destination property.

```csharp
string Into { get; }
```

##### From

Parent key.

```csharp
string From { get; }
```

##### By

Related key.

```csharp
string By { get; }
```

##### Select

Optional related property to copy.

```csharp
string? Select { get; }
```

##### Many

Collect a list.

```csharp
bool Many { get; }
```

#### IReadonlyEnrichmentStrategy

```csharp
public interface IReadonlyEnrichmentStrategy<T> : IReadonlyEnrichmentBinder
```

**Methods**

##### Read(inner, tree, query)

Reads through `inner`, then enriches.

```csharp
IAsyncEnumerable<T> Read(IReadonlyRepository<T> inner, IRepositoryTree tree, IStatement? query)
```

- `inner` — [IReadonlyRepository](./readonly-repository.md)
- `tree` — [IRepositoryTree](#irepositorytree)
- `query` — optional statement or where

**Returns** `IAsyncEnumerable<T>`

##### Enrich(tree, items, sources)

Fills `items` from the tree. `sources` supplies the unread columns for each parent.

```csharp
Task Enrich(IRepositoryTree tree, IList<T> items, IReadOnlyList<Row?>? sources = null)
```

- `tree` — [IRepositoryTree](#irepositorytree)
- `items` — parents to fill
- `sources` — optional source rows aligned with `items`

### rust
#### IReadonlyRepository

```rust
pub trait IReadonlyRepository: IRepository
```

**Methods**

##### read_single(query)

Returns the first row, or `None` when the result is empty.

```rust
async fn read_single(&self, query: Option<&dyn ReadInput>) -> Result<Option<Row>>
```

- `query` — optional query, where, or stored procedure; omitted uses the default select

**Returns** `Result<Option<Row>>`

##### read(query)

Returns every row.

```rust
async fn read(&self, query: Option<&dyn ReadInput>) -> Result<Vec<Row>>
```

- `query` — optional query, where, or stored procedure; omitted uses the default select

**Returns** `Result<Vec<Row>>`

#### ReadonlyRepository

```rust
pub struct ReadonlyRepository
```

**Constructor**

##### from_repository(inner)

Wraps an inner repository. `read` reads that repository, then fills related fields. Uses the default strategy. See [IEnriched](#ienriched).

```rust
pub fn from_repository(inner: Arc<dyn IReadonlyRepository>) -> Self
```

- `inner` — [IReadonlyRepository](#ireadonlyrepository)

**Returns** [ReadonlyRepository](#readonlyrepository)

##### with_strategy(inner, strategy)

Wraps an inner repository with an enrichment strategy.

```rust
pub fn with_strategy(inner: Arc<dyn IReadonlyRepository>, strategy: Arc<dyn ReadonlyEnrichmentStrategy>) -> Self
```

- `inner` — [IReadonlyRepository](#ireadonlyrepository)
- `strategy` — enrichment strategy

**Returns** [ReadonlyRepository](#readonlyrepository)

**Methods**

##### add_enriched(enrichment, via, strategy)

Adds a related fill. A related repository fills its own fields. Does not write.

```rust
pub fn add_enriched(&mut self, enrichment: Enrichment, via: impl EnrichmentVia, strategy: Option<Arc<dyn ReadonlyEnrichmentStrategy>>) -> &mut Self
```

- `enrichment` — [Enrichment](#enrichment)
- `via` — related [IReadonlyRepository](#ireadonlyrepository). The readonly path keeps the read repository only
- `strategy` — optional read strategy for this edge. Omitted uses the default strategy

**Returns** `&mut Self`

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

##### column_mappings

Maps property names to stored columns. Default is [IdentityMapper](./identity-mapper.md). The repository maps where identifiers and write parameters. Read reverse-maps each result key before the row converter.

```rust
pub column_mappings: Box<dyn Mapper<String, String> + Send + Sync>
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

Provider converters keyed by `bool`, `datetime`, `uuid`, `decimal`, and `binary`, merged into the row converter. The driver applies its converters when it binds a parameter. A field converter reverse-converts that property in the repository.

```rust
pub row_type_converters: HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>
```

#### RepositoryTree

```rust
pub struct RepositoryTree
```

**Properties**

##### parent

Root [RepositoryTreeNode](#repositorytreenode).

```rust
pub parent: RepositoryTreeNode
```

#### RepositoryTreeNode

```rust
pub struct RepositoryTreeNode
```

**Properties**

##### value

Related [ReadonlyRepository](./readonly-repository.md).

```rust
pub value: Arc<dyn ReadonlyRepository>
```

##### property_name

Destination field name.

```rust
pub property_name: String
```

##### items

Child nodes.

```rust
pub items: Vec<RepositoryTreeNode>
```

##### enrichment

[Enrichment](#enrichment) for this node.

```rust
pub enrichment: Option<Enrichment>
```

**Methods**

##### include(via, into, from, by, select, many)

Adds a child node.

```rust
pub fn include(&mut self, via: impl EnrichmentVia, into: impl Into<String>, from: impl Into<String>, by: impl Into<String>, select: Option<String>, many: bool) -> &mut Self
```

- `via` — related [IReadonlyRepository](#ireadonlyrepository)
- `into` — destination field
- `from` — parent key
- `by` — related key
- `select` — optional related field to copy
- `many` — collect a list

**Returns** `&mut Self`

#### Enrichment

```rust
pub struct Enrichment
```

One related fill.

**Constructor**

##### new(into, from, by, select, many)

Builds an enrichment.

```rust
pub fn new(into: impl Into<String>, from: impl Into<String>, by: impl Into<String>, select: Option<String>, many: bool) -> Self
```

- `into` — destination field
- `from` — parent key
- `by` — related key
- `select` — optional related field to copy
- `many` — collect a list

**Returns** [Enrichment](#enrichment)

**Properties**

##### into

Destination field.

```rust
pub into: String
```

##### from

Parent key.

```rust
pub from: String
```

##### by

Related key.

```rust
pub by: String
```

##### select

Optional related field to copy.

```rust
pub select: Option<String>
```

##### many

Collect a list.

```rust
pub many: bool
```

#### ReadonlyEnrichmentStrategy

```rust
pub trait ReadonlyEnrichmentStrategy
```

**Methods**

##### read(inner, tree, query)

Reads through `inner`, then enriches.

```rust
async fn read(&self, inner: &dyn ReadonlyRepository, tree: &RepositoryTree, query: Option<&dyn ReadInput>) -> Result<Vec<Row>>
```

- `inner` — [ReadonlyRepository](./readonly-repository.md)
- `tree` — [RepositoryTree](#repositorytree)
- `query` — optional query, where, or stored procedure

**Returns** `Result<Vec<Row>>`

##### enrich(tree, items)

Fills `items` from the tree.

```rust
async fn enrich(&self, tree: &RepositoryTree, items: &mut [Row]) -> Result<()>
```

- `tree` — [RepositoryTree](#repositorytree)
- `items` — parents to fill

**Returns** `Result<()>`


:::

## Remarks

{{AddEnriched|addEnriched|add_enriched}} registers a read fill. With no enrichments, {{Read|read|read}} stays streaming. With enrichments, the read is collected, filled, then yielded. A `many` list is walked. A `select` scalar is not. Depth follows each related repository's own enrichments. Writes, including `many` children and associations, are [CRUDRepository](./crud-repository.md).

`AddEnriched` stays fluent for siblings. Register the child repository's own fills on that repository:

```
childrenRepo.AddEnriched(new("type_name", "type_id", "id", "name"), childTypesRepo);
parentsRepo.AddEnriched(new("children", "id", "parent_id", Many: true), childrenRepo);
```

Repository read is convert, then map. See [Type Converters](./type-converters.md) and [Mappers](./mappers.md).

`read` and `read_single` accept a query, a where, or an [ExecuteStoredProcedureStatement](./execute-stored-procedure-statement.md). Result rows still convert, then map. Procedure argument names are not rewritten as columns.

## Examples

### Read users by id and by username

::: example
```csharp csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#ReadsUsersByIdAndByUsernameAndEnabled
```
```rust rust/providers/sqlite/tests/readonly_repository.rs#reads_users_by_id_and_by_username_and_enabled
```
```typescript typescript/providers/sqlite/tests/readonly.repository.integration.test.ts#reads-users-by-id-and-by-username-and-enabled
```

```output [After: by id]
{
  id: 1,
  username: 'ada',
  is_enabled: true
}
```

```output:typescript [After: by id]
{
  id: 1,
  username: 'ada',
  is_enabled: 1
}
```

```output:rust [After: by id]
{
  id: 1,
  username: 'ada',
  is_enabled: 1
}
```

```output [After: disabled]
[
  {
    id: 2,
    username: 'grace',
    is_enabled: false
  }
]
```

```output:typescript [After: disabled]
[
  {
    id: 2,
    username: 'grace',
    is_enabled: 0
  }
]
```

```output:rust [After: disabled]
[
  {
    id: 2,
    username: 'grace',
    is_enabled: 0
  }
]
```
:::

### Read a user by guid, username, and created

::: example
```csharp csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#ReadsUserByGuidUsernameAndCreated
```
```rust rust/providers/sqlite/tests/readonly_repository.rs#reads_user_by_guid_username_and_created
```
```typescript typescript/providers/sqlite/tests/readonly.repository.integration.test.ts#reads-user-by-guid-username-and-created
```

```output [After: by id]
{
  Key: '11111111-1111-1111-1111-111111111111',
  Login: 'ada',
  Active: true,
  Opened: '2020-01-01T00:00:00.000Z'
}
```

```output:typescript [After: by id]
{
  Key: '11111111-1111-1111-1111-111111111111',
  Login: 'ada',
  Active: 1,
  Opened: '2020-01-01T00:00:00.000Z'
}
```

```output:rust [After: by id]
{
  Key: '11111111-1111-1111-1111-111111111111',
  Login: 'ada',
  Active: 1,
  Opened: '2020-01-01T00:00:00.000Z'
}
```

```output [After: by login]
[
  {
    Key: '11111111-1111-1111-1111-111111111111',
    Login: 'ada',
    Active: true,
    Opened: '2020-01-01T00:00:00.000Z'
  }
]
```

```output:typescript [After: by login]
[
  {
    Key: '11111111-1111-1111-1111-111111111111',
    Login: 'ada',
    Active: 1,
    Opened: '2020-01-01T00:00:00.000Z'
  }
]
```

```output:rust [After: by login]
[
  {
    Key: '11111111-1111-1111-1111-111111111111',
    Login: 'ada',
    Active: 1,
    Opened: '2020-01-01T00:00:00.000Z'
  }
]
```

```output [After: later]
[
  {
    Key: '22222222-2222-2222-2222-222222222222',
    Login: 'grace',
    Active: true,
    Opened: '2024-06-01T00:00:00.000Z'
  }
]
```

```output:typescript [After: later]
[
  {
    Key: '22222222-2222-2222-2222-222222222222',
    Login: 'grace',
    Active: 1,
    Opened: '2024-06-01T00:00:00.000Z'
  }
]
```

```output:rust [After: later]
[
  {
    Key: '22222222-2222-2222-2222-222222222222',
    Login: 'grace',
    Active: 1,
    Opened: '2024-06-01T00:00:00.000Z'
  }
]
```
:::

### Call a stored procedure

::: example
```csharp csharp/Datasources.Tests/RepositoryTests.cs#ReadCallsStoredProcedure
```
```rust rust/datasources/src/tests/mod.rs#evaluates_where_statement
```
```typescript typescript/datasources/src/repository.test.ts#calls-a-stored-procedure-and-maps-rows
```

```output [After: rows]
[
  { id: 1, name: 'a' }
]
```
:::

### Read and read single

::: example
```csharp csharp/Datasources.Tests/RepositoryTests.cs#GetBuildsSelectAndMapsRows
```
```rust rust/datasources/src/tests/mod.rs#evaluates_where_statement
```
```typescript typescript/datasources/src/repository.test.ts#builds-select-and-maps-rows
```

```output [After: read]
[
  { id: 1, name: 'a' }
]
```

```output [After: read single]
{ id: 1, name: 'a' }
```
:::

### Fill a lookup

::: example
```csharp csharp/Datasources.Tests/EnrichedReadonlyRepositoryTests.cs#ReadFillsLookupSelectAndLeavesMissingDefault
```
```rust rust/datasources/tests/enriched.rs#enriched_fills_lookup_select_and_leaves_missing_default
```
```typescript typescript/datasources/src/EnrichedReadonlyRepository.test.ts#fills-lookup-select-and-leaves-missing-default
```
:::

### Fill user type on a user view

`UsersView` maps onto `user`. `UserType` stays on the row as `user_type_id` until enrichment replaces it with the type name. The lookup repository's entity is `UserType`, and [ChangeCasingMapper](./change-casing-mapper.md) maps that name, plus `Id` and `Name`, to `user_type`, `id`, and `name`.

::: example
```csharp csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#FillsUserTypeOnUsersView
```
```rust rust/providers/sqlite/tests/readonly_repository.rs#fills_user_type_on_user_view
```
```typescript typescript/providers/sqlite/tests/readonly.repository.integration.test.ts#fills-user-type-on-a-user-view
```

```output [User view]
{
  Key: '11111111-1111-1111-1111-111111111111',
  Login: 'ada',
  Active: true,
  Opened: '2020-01-01T00:00:00.000Z',
  UserType: 'admin'
}
```

```output:typescript [User view]
{
  Key: '11111111-1111-1111-1111-111111111111',
  Login: 'ada',
  Active: 1,
  Opened: '2020-01-01T00:00:00.000Z',
  UserType: 'admin'
}
```

```output:rust [User view]
{
  Key: '11111111-1111-1111-1111-111111111111',
  Login: 'ada',
  Active: 1,
  Opened: '2020-01-01T00:00:00.000Z',
  UserType: 'admin'
}
```
:::

### Fill roles from one join

The repository reads one join of `user`, `user_type`, `user_roles`, and `roles`. [FlatToNestedMapper](./flat-to-nested-mapper.md) groups each user's `Roles`.

::: example
```csharp csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#FillsRolesFromAJoin
```
```rust rust/providers/sqlite/tests/readonly_repository.rs#fills_roles_from_a_join
```
```typescript typescript/providers/sqlite/tests/users-view-roles.repository.integration.test.ts#fills-roles-from-a-join
```

```output [Users with roles]
[
  {
    Key: '11111111-1111-1111-1111-111111111111',
    Login: 'ada',
    Active: true,
    Opened: '2020-01-01T00:00:00.000Z',
    UserType: 'admin',
    Roles: [
      { UserId: '11111111-1111-1111-1111-111111111111', Name: 'admin' },
      { UserId: '11111111-1111-1111-1111-111111111111', Name: 'editor' }
    ]
  },
  {
    Key: '22222222-2222-2222-2222-222222222222',
    Login: 'grace',
    Active: true,
    Opened: '2024-06-01T00:00:00.000Z',
    UserType: 'user',
    Roles: [
      { UserId: '22222222-2222-2222-2222-222222222222', Name: 'viewer' }
    ]
  },
  {
    Key: '33333333-3333-3333-3333-333333333333',
    Login: 'lin',
    Active: false,
    Opened: '2024-06-01T00:00:00.000Z',
    UserType: 'service',
    Roles: []
  }
]
```

```output:typescript [Users with roles]
[
  {
    Key: '11111111-1111-1111-1111-111111111111',
    Login: 'ada',
    Active: 1,
    Opened: '2020-01-01T00:00:00.000Z',
    UserType: 'admin',
    Roles: [
      { RoleId: 1, UserId: '11111111-1111-1111-1111-111111111111', Name: 'admin' },
      { RoleId: 2, UserId: '11111111-1111-1111-1111-111111111111', Name: 'editor' }
    ]
  },
  {
    Key: '22222222-2222-2222-2222-222222222222',
    Login: 'grace',
    Active: 1,
    Opened: '2024-06-01T00:00:00.000Z',
    UserType: 'user',
    Roles: [
      { RoleId: 3, UserId: '22222222-2222-2222-2222-222222222222', Name: 'viewer' }
    ]
  },
  {
    Key: '33333333-3333-3333-3333-333333333333',
    Login: 'lin',
    Active: 0,
    Opened: '2024-06-01T00:00:00.000Z',
    UserType: 'service',
    Roles: []
  }
]
```

```output:rust [Users with roles]
[
  {
    Key: '11111111-1111-1111-1111-111111111111',
    Login: 'ada',
    Active: 1,
    Opened: '2020-01-01T00:00:00.000Z',
    UserType: 'admin',
    Roles: 'Name=admin,UserId=11111111-1111-1111-1111-111111111111|Name=editor,UserId=11111111-1111-1111-1111-111111111111'
  },
  {
    Key: '22222222-2222-2222-2222-222222222222',
    Login: 'grace',
    Active: 1,
    Opened: '2024-06-01T00:00:00.000Z',
    UserType: 'user',
    Roles: 'Name=viewer,UserId=22222222-2222-2222-2222-222222222222'
  },
  {
    Key: '33333333-3333-3333-3333-333333333333',
    Login: 'lin',
    Active: 0,
    Opened: '2024-06-01T00:00:00.000Z',
    UserType: 'service',
    Roles: ''
  }
]
```
:::
