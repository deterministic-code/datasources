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

##### add_enriched(into, from, via, by, select, many, nest)

Forwards to the inner [CRUDRepository](./crud-repository.md). A CRUD `via` is kept so `many` children can be written.

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
pub async fn update_bulk(&self, items: &[Row], by_where: Option<&dyn Where>) -> Result<Response>
```

- `items` — parents to update
- `by_where` — optional predicate applied to each item

**Returns** `Result<Response>`

##### delete_bulk(items, by_where)

Deletes nested `many` rows for each item, then the parents.

```rust
pub async fn delete_bulk(&self, items: &[Row], by_where: Option<&dyn Where>) -> Result<Response>
```

- `items` — parents to delete
- `by_where` — optional predicate applied to each item

**Returns** `Result<Response>`
:::

## Remarks

C# exposes list overloads (`Create(IReadOnlyList<T>)`) and `Bulk*` aliases. TypeScript and Rust use `createBulk` / `create_bulk` only.

Each item still runs the same deep persist as [CRUDRepository](./crud-repository.md).

## Examples

### Create parents and children

::: example
```csharp csharp/Datasources.Tests/BulkCRUDRepositoryTests.cs#L23-L30
```
```rust rust/datasources/src/tests/mod.rs#L1862-L1879
```
```typescript typescript/datasources/src/BulkCRUDRepository.test.ts#L29-L37
```
:::
