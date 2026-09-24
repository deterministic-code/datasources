# Enriched repository

A read-only wrapper fills properties already on `T` from related repositories. This is a post-Read step (after convert-then-map). It does not join or collapse rows.

Lookups use `ReadSingle` / `read_single` (cached per distinct key). Lists use `Read` / `read`. Missing lookups leave the destination default.

`EnrichedCRUDRepository` is the write side of the same tree. `Read` still enriches. `select` lookups and 1:1 embeds are not written. Each `many` node must be a CRUD repository.

- **Create** — write the parent, apply generated identity to `id`, stamp each child's `by` from the parent `from`, then create children (and nested many).
- **Update** — write the parent, then replace-sync each many list (delete missing, update matches by `id`, create the rest).
- **Delete** — delete nested many rows first, then children, then the parent. Loads existing children by FK; does not trust payload lists.

See [Bulk repository](./bulk-repository.md) for list writes.

## Classes

::: reference
### typescript
#### IEnriched

```typescript
interface IEnriched<T>
```

**Methods**

##### addEnriched(into, from, via, by, select?, many?, nest?)

Adds a related fill. Fluent for siblings; `nest` receives the child node.

```typescript
addEnriched<TRelated extends object>(into: keyof T & string, from: keyof T & string, via: IReadonlyRepository<TRelated>, by: keyof TRelated & string, select?: keyof TRelated & string, many?: boolean, nest?: (node: IRepositoryTreeNode) => void): this
```

- `into` — destination property on `T`
- `from` — key on the parent
- `via` — related [IReadonlyRepository](./readonly-repository.md)
- `by` — key on the related item
- `select` — optional related property to copy; omitted copies the whole row
- `many` — collect a list; default `false`
- `nest` — optional child-node callback

**Returns** `this`

##### enrich(items)

Fills `items` in place from the tree.

```typescript
enrich(items: T[]): Promise<T[]>
```

- `items` — parents to fill

**Returns** `Promise<T[]>`

#### EnrichedReadonlyRepository

```typescript
class EnrichedReadonlyRepository<T extends object> extends ReadonlyRepository<T> implements IEnriched<T>
```

**Constructor**

##### EnrichedReadonlyRepository(inner, strategy?)

Wraps an inner [IReadonlyRepository](./readonly-repository.md). Omitted strategy uses the default.

```typescript
constructor(inner: IReadonlyRepository<T>, strategy?: IReadonlyEnrichmentStrategy<T>)
```

- `inner` — [IReadonlyRepository](./readonly-repository.md)
- `strategy` — optional [IReadonlyEnrichmentStrategy](#ireadonlyenrichmentstrategy)

**Properties**

##### tree

Enrichment [IRepositoryTree](#irepositorytree).

```typescript
readonly tree: IRepositoryTree
```

**Methods**

##### addEnriched(into, from, via, by, select?, many?, nest?)

Adds a related fill.

```typescript
addEnriched<TRelated extends object>(into: keyof T & string, from: keyof T & string, via: IReadonlyRepository<TRelated>, by: keyof TRelated & string, select?: keyof TRelated & string, many?: boolean, nest?: (node: IRepositoryTreeNode) => void): this
```

- `into` — destination property on `T`
- `from` — key on the parent
- `via` — related [IReadonlyRepository](./readonly-repository.md)
- `by` — key on the related item
- `select` — optional related property to copy
- `many` — collect a list
- `nest` — optional child-node callback

**Returns** `this`

##### enrich(items)

Fills `items` in place from the tree.

```typescript
enrich(items: T[]): Promise<T[]>
```

- `items` — parents to fill

**Returns** `Promise<T[]>`

##### read(query?)

Reads through the inner repository, then enriches.

```typescript
read(query?: IQuery | IWhere | IExecuteStoredProcedureStatement): AsyncIterable<T>
```

- `query` — optional query, where, or stored procedure

**Returns** `AsyncIterable<T>`

#### EnrichedCRUDRepository

```typescript
class EnrichedCRUDRepository<T extends object> extends EnrichedReadonlyRepository<T> implements ICRUDRepository<T>
```

**Constructor**

##### EnrichedCRUDRepository(inner, strategy?)

Wraps an inner [ICRUDRepository](./crud-repository.md). Omitted strategy uses the default.

```typescript
constructor(inner: ICRUDRepository<T>, strategy?: IReadonlyEnrichmentStrategy<T>)
```

- `inner` — [ICRUDRepository](./crud-repository.md)
- `strategy` — optional [IReadonlyEnrichmentStrategy](#ireadonlyenrichmentstrategy)

**Methods**

##### create(item)

Writes the parent, then nested `many` children.

```typescript
create(item: T): Promise<Response>
```

- `item` — parent to insert

**Returns** `Promise<Response>`

##### update(item, byWhere?)

Writes the parent, then replace-syncs nested `many` lists.

```typescript
update(item: T, byWhere?: IWhere): Promise<Response>
```

- `item` — parent to update
- `byWhere` — optional predicate

**Returns** `Promise<Response>`

##### delete(item, byWhere?)

Deletes nested `many` rows, then the parent.

```typescript
delete(item: T, byWhere?: IWhere): Promise<Response>
```

- `item` — parent to delete
- `byWhere` — optional predicate

**Returns** `Promise<Response>`

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
read(inner: IReadonlyRepository<T>, tree: IRepositoryTree, query?: IQuery | IWhere | IExecuteStoredProcedureStatement): AsyncIterable<T>
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
#### IEnriched

```csharp
public interface IEnriched<T>
```

**Methods**

##### AddEnriched(into, from, via, by, select, many, nest)

Adds a related fill. Fluent for siblings; `nest` receives the child node.

```csharp
IEnriched<T> AddEnriched<TRelated>(string into, string from, IReadonlyRepository<TRelated> via, string by, string? select = null, bool many = false, Action<IRepositoryTreeNode>? nest = null)
```

- `into` — destination property on `T`
- `from` — key on the parent
- `via` — related [IReadonlyRepository](./readonly-repository.md)
- `by` — key on the related item
- `select` — optional related property to copy; omitted copies the whole row
- `many` — collect a list
- `nest` — optional child-node callback

**Returns** [IEnriched](#ienriched)`<T>`

##### Enrich(items)

Fills `items` in place from the tree.

```csharp
Task Enrich(IList<T> items)
```

- `items` — parents to fill

#### EnrichedReadonlyRepository

```csharp
public class EnrichedReadonlyRepository<T> : ReadonlyRepository<T>, IEnriched<T>
```

**Constructor**

##### EnrichedReadonlyRepository(inner, strategy)

Wraps an inner [IReadonlyRepository](./readonly-repository.md). Omitted strategy uses the default.

```csharp
public EnrichedReadonlyRepository(IReadonlyRepository<T> inner, IReadonlyEnrichmentStrategy<T>? strategy = null)
```

- `inner` — [IReadonlyRepository](./readonly-repository.md)
- `strategy` — optional [IReadonlyEnrichmentStrategy](#ireadonlyenrichmentstrategy)

**Properties**

##### Tree

Enrichment [IRepositoryTree](#irepositorytree).

```csharp
IRepositoryTree Tree { get; }
```

**Methods**

##### AddEnriched(into, from, via, by, select, many, nest)

Adds a related fill.

```csharp
public IEnriched<T> AddEnriched<TRelated>(string into, string from, IReadonlyRepository<TRelated> via, string by, string? select = null, bool many = false, Action<IRepositoryTreeNode>? nest = null)
```

- `into` — destination property on `T`
- `from` — key on the parent
- `via` — related [IReadonlyRepository](./readonly-repository.md)
- `by` — key on the related item
- `select` — optional related property to copy
- `many` — collect a list
- `nest` — optional child-node callback

**Returns** [IEnriched](#ienriched)`<T>`

##### Enrich(items)

Fills `items` in place from the tree.

```csharp
public Task Enrich(IList<T> items)
```

- `items` — parents to fill

##### Read(query)

Reads through the inner repository, then enriches.

```csharp
public override IAsyncEnumerable<T> Read(IStatement? query = null)
```

- `query` — optional statement or where

**Returns** `IAsyncEnumerable<T>`

#### EnrichedCRUDRepository

```csharp
public class EnrichedCRUDRepository<T> : EnrichedReadonlyRepository<T>, ICRUDRepository<T>
```

**Constructor**

##### EnrichedCRUDRepository(inner, strategy)

Wraps an inner [ICRUDRepository](./crud-repository.md). Omitted strategy uses the default.

```csharp
public EnrichedCRUDRepository(ICRUDRepository<T> inner, IReadonlyEnrichmentStrategy<T>? strategy = null)
```

- `inner` — [ICRUDRepository](./crud-repository.md)
- `strategy` — optional [IReadonlyEnrichmentStrategy](#ireadonlyenrichmentstrategy)

**Methods**

##### Create(item)

Writes the parent, then nested `many` children.

```csharp
public Task<Response> Create(T item)
```

- `item` — parent to insert

**Returns** `Task<Response>`

##### Update(item, byWhere)

Writes the parent, then replace-syncs nested `many` lists.

```csharp
public Task<Response> Update(T item, IWhere? byWhere = null)
```

- `item` — parent to update
- `byWhere` — optional predicate

**Returns** `Task<Response>`

##### Delete(item, byWhere)

Deletes nested `many` rows, then the parent.

```csharp
public Task<Response> Delete(T item, IWhere? byWhere = null)
```

- `item` — parent to delete
- `byWhere` — optional predicate

**Returns** `Task<Response>`

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

##### Enrich(tree, items)

Fills `items` from the tree.

```csharp
Task Enrich(IRepositoryTree tree, IList<T> items)
```

- `tree` — [IRepositoryTree](#irepositorytree)
- `items` — parents to fill

### rust
#### Enriched

```rust
pub trait Enriched
```

**Methods**

##### enrich(items)

Fills `items` in place from the tree.

```rust
async fn enrich(&self, items: &mut [Row]) -> Result<()>
```

- `items` — parents to fill

**Returns** `Result<()>`

#### EnrichedReadonlyRepository

```rust
pub struct EnrichedReadonlyRepository
```

**Constructor**

##### new(inner)

Wraps an inner [ReadonlyRepository](./readonly-repository.md). Uses the default strategy.

```rust
pub fn new(inner: Arc<dyn ReadonlyRepository>) -> Self
```

- `inner` — [ReadonlyRepository](./readonly-repository.md)

**Returns** [EnrichedReadonlyRepository](#enrichedreadonlyrepository)

##### with_strategy(inner, strategy)

Wraps an inner repository with a [ReadonlyEnrichmentStrategy](#readonlyenrichmentstrategy).

```rust
pub fn with_strategy(inner: Arc<dyn ReadonlyRepository>, strategy: Arc<dyn ReadonlyEnrichmentStrategy>) -> Self
```

- `inner` — [ReadonlyRepository](./readonly-repository.md)
- `strategy` — [ReadonlyEnrichmentStrategy](#readonlyenrichmentstrategy)

**Returns** [EnrichedReadonlyRepository](#enrichedreadonlyrepository)

**Properties**

##### tree

Enrichment [RepositoryTree](#repositorytree).

```rust
pub tree: RepositoryTree
```

**Methods**

##### add_enriched(into, from, via, by, select, many, nest)

Adds a related fill. Pass `()` for `nest` when the child has no further fill.

```rust
pub fn add_enriched(&mut self, into: impl Into<String>, from: impl Into<String>, via: impl EnrichmentVia, by: impl Into<String>, select: Option<String>, many: bool, nest: impl Nest) -> &mut Self
```

- `into` — destination field
- `from` — key on the parent
- `via` — related readonly or [CRUDRepository](./crud-repository.md). A CRUD repository is kept for deep writes
- `by` — key on the related row
- `select` — optional related field to copy
- `many` — collect a list
- `nest` — child-node callback, or `()`

**Returns** `&mut Self`

#### EnrichedCRUDRepository

```rust
pub struct EnrichedCRUDRepository
```

**Constructor**

##### new(inner)

Wraps an inner [CRUDRepository](./crud-repository.md). Uses the default strategy.

```rust
pub fn new(inner: Arc<dyn CRUDRepository>) -> Self
```

- `inner` — [CRUDRepository](./crud-repository.md)

**Returns** [EnrichedCRUDRepository](#deepcrudrepository)

##### with_strategy(inner, strategy)

Wraps an inner repository with a [ReadonlyEnrichmentStrategy](#readonlyenrichmentstrategy).

```rust
pub fn with_strategy(inner: Arc<dyn CRUDRepository>, strategy: Arc<dyn ReadonlyEnrichmentStrategy>) -> Self
```

- `inner` — [CRUDRepository](./crud-repository.md)
- `strategy` — [ReadonlyEnrichmentStrategy](#readonlyenrichmentstrategy)

**Returns** [EnrichedCRUDRepository](#deepcrudrepository)

**Methods**

##### add_enriched(into, from, via, by, select, many, nest)

Forwards to the inner [EnrichedReadonlyRepository](#enrichedreadonlyrepository). A CRUD `via` is kept so `many` children can be written.

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

- `via` — related readonly or [CRUDRepository](./crud-repository.md). A CRUD repository is kept for deep writes
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

Nest only on walkable objects (`many: true` or a whole related row). A `select` scalar is not walked. Depth is unbounded; each node applies, then children run on the related items.

`AddEnriched` stays fluent for siblings:

```
AddEnriched("children", "id", children, "parent_id", many: true,
    nest: node => node.Include(childTypes, "type_name", "type_id", "id", "name"));
```

## Examples

### Fill a lookup

::: example
```csharp csharp/Datasources.Tests/EnrichedReadonlyRepositoryTests.cs#L43-L47
```
```rust rust/datasources/src/tests/mod.rs#L1845-L1854
```
```typescript typescript/datasources/src/EnrichedReadonlyRepository.test.ts#L39-L51
```
:::
