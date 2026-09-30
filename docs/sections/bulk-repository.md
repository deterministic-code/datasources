# Bulk repository

[CRUDRepository](./crud-repository.md) plus list writes. Each item still runs the same deep create / update / delete. Responses sum `rowsAffected`; `result` is the last item.

## Classes

::: reference
### typescript
#### BulkCRUDRepository

```typescript
class BulkCRUDRepository<T extends object> extends CRUDRepository<T>
```

**Constructor**

##### BulkCRUDRepository(inner)

Wraps an inner [ICRUDRepository](./crud-repository.md).

```typescript
constructor(inner: ICRUDRepository<T>)
```

- `inner` — [ICRUDRepository](./crud-repository.md)

**Methods**

##### createBulk(items)

Creates each item, including nested `many` children.

```typescript
createBulk(items: T[]): Promise<Response>
```

- `items` — parents to insert

**Returns** `Promise<Response>`

##### updateBulk(items, byWhere?)

Updates each item, then replace-syncs nested `many` lists.

```typescript
updateBulk(items: T[], byWhere?: IWhere): Promise<Response>
```

- `items` — parents to update
- `byWhere` — optional predicate applied to each item

**Returns** `Promise<Response>`

##### deleteBulk(items, byWhere?)

Deletes nested `many` rows for each item, then the parents.

```typescript
deleteBulk(items: T[], byWhere?: IWhere): Promise<Response>
```

- `items` — parents to delete
- `byWhere` — optional predicate applied to each item

**Returns** `Promise<Response>`

### csharp
#### BulkCRUDRepository

```csharp
public class BulkCRUDRepository<T> : CRUDRepository<T>
```

**Constructor**

##### BulkCRUDRepository(inner)

Wraps an inner [ICRUDRepository](./crud-repository.md).

```csharp
public BulkCRUDRepository(ICRUDRepository<T> inner)
```

- `inner` — [ICRUDRepository](./crud-repository.md)

**Methods**

##### Create(items)

Creates each item, including nested `many` children.

```csharp
public Task<Response> Create(IReadOnlyList<T> items)
```

- `items` — parents to insert

**Returns** `Task<Response>`

##### Update(items, byWhere)

Updates each item, then replace-syncs nested `many` lists.

```csharp
public Task<Response> Update(IReadOnlyList<T> items, IWhere? byWhere = null)
```

- `items` — parents to update
- `byWhere` — optional predicate applied to each item

**Returns** `Task<Response>`

##### Delete(items, byWhere)

Deletes nested `many` rows for each item, then the parents.

```csharp
public Task<Response> Delete(IReadOnlyList<T> items, IWhere? byWhere = null)
```

- `items` — parents to delete
- `byWhere` — optional predicate applied to each item

**Returns** `Task<Response>`

##### BulkCreate(items)

Alias of [Create](#bulkcrudrepository-create-items).

```csharp
public Task<Response> BulkCreate(IReadOnlyList<T> items)
```

- `items` — parents to insert

**Returns** `Task<Response>`

##### BulkUpdate(items, byWhere)

Alias of [Update](#bulkcrudrepository-update-items-bywhere).

```csharp
public Task<Response> BulkUpdate(IReadOnlyList<T> items, IWhere? byWhere = null)
```

- `items` — parents to update
- `byWhere` — optional predicate applied to each item

**Returns** `Task<Response>`

##### BulkDelete(items, byWhere)

Alias of [Delete](#bulkcrudrepository-delete-items-bywhere).

```csharp
public Task<Response> BulkDelete(IReadOnlyList<T> items, IWhere? byWhere = null)
```

- `items` — parents to delete
- `byWhere` — optional predicate applied to each item

**Returns** `Task<Response>`

### rust
#### BulkCRUDRepository

```rust
pub struct BulkCRUDRepository
```

**Constructor**

##### new(inner)

Wraps an inner [CRUDRepository](./crud-repository.md).

```rust
pub fn new(inner: Arc<dyn CRUDRepository>) -> Self
```

- `inner` — [CRUDRepository](./crud-repository.md)

**Returns** [BulkCRUDRepository](#bulkcrudrepository)

**Methods**

##### add_enriched(enrichment, via, strategy)

Forwards to [CRUDRepository.add_enriched](./crud-repository.md#add_enrichedenrichment-via-strategy). A CRUD `via` is kept so `many` children can be written.

```rust
pub fn add_enriched(&mut self, enrichment: Enrichment, via: impl EnrichmentVia, strategy: Option<Arc<dyn ReadonlyEnrichmentStrategy>>) -> &mut Self
```

- `enrichment` — [Enrichment](./enrichment.md)
- `via` — related repository. A CRUD repository is kept so `many` children can be written
- `strategy` — optional read strategy for this edge. Omitted uses the default strategy

**Returns** `&mut Self`

##### create_bulk(items)

Creates each item, including nested `many` children.

```rust
pub async fn create_bulk(&self, items: &mut [Row]) -> Result<Response>
```

- `items` — parents to insert

**Returns** `Result<Response>`

##### update_bulk(items, by_where)

Updates each item, then replace-syncs nested `many` lists.

```rust
pub async fn update_bulk(&self, items: &[Row], by_where: Option<&dyn IWhere>) -> Result<Response>
```

- `items` — parents to update
- `by_where` — optional predicate applied to each item

**Returns** `Result<Response>`

##### delete_bulk(items, by_where)

Deletes nested `many` rows for each item, then the parents.

```rust
pub async fn delete_bulk(&self, items: &[Row], by_where: Option<&dyn IWhere>) -> Result<Response>
```

- `items` — parents to delete
- `by_where` — optional predicate applied to each item

**Returns** `Result<Response>`
:::

## Remarks

::: prose
### csharp
List overloads are `Create(IReadOnlyList<T>)`, with `Bulk*` aliases.
### typescript
Bulk writes use `createBulk` only.
### rust
Bulk writes use `create_bulk` only.
:::

Each item still runs the same deep persist as [CRUDRepository](./crud-repository.md). The list write goes through [IBulkWriteStrategy](./i-bulk-write-strategy.md). The default is [SingleCommandIBulkWriteStrategy](./single-command-i-bulk-write-strategy.md). When the write strategy's `useTransaction` is on, a list create, update, or delete is one transaction.

## Examples

### Create parents and children

::: example
```csharp csharp/Datasources.Tests/BulkCRUDRepositoryTests.cs#CreateWritesParentsAndChildren
```
```rust rust/datasources/src/tests/mod.rs#table_repository_maps_flat_row_into_contacts
```
```typescript typescript/datasources/src/BulkCRUDRepository.test.ts#creates-parents-and-children
```
:::
