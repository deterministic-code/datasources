# DeleteEnrichmentStrategy

How a delete removes owned children and links. [DefaultWriteEnrichmentStrategy](./default-write-enrichment-strategy.md) calls these methods before the parent row is deleted. [CascadeDeleteEnrichmentStrategy](./cascade-delete-enrichment-strategy.md) is the default.

## Classes

::: reference
### typescript
#### DeleteEnrichmentStrategy

```typescript
abstract class DeleteEnrichmentStrategy<T>
```

**Methods**

##### deleteOwned(entry, enrichment, parent, deleteNested)

Removes the owned `many` list for one parent.

```typescript
abstract deleteOwned(entry: StoredEnrichment, enrichment: Enrichment, parent: object, deleteNested: (rows: object[]) => Promise<void>): Promise<void>
```

- `entry` — [StoredEnrichment](./update-enrichment-strategy.md#storedenrichment)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent object
- `deleteNested` — deletes nested rows of the children before the children themselves

**Returns** `Promise<void>`

##### deleteLinked(entry, enrichment, parent)

Removes the junction rows for one parent.

```typescript
abstract deleteLinked(entry: StoredEnrichment, enrichment: Enrichment, parent: object): Promise<void>
```

- `entry` — [StoredEnrichment](./update-enrichment-strategy.md#storedenrichment)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent object

**Returns** `Promise<void>`

##### load(writer, enrichment, parent)

Reads the owned rows whose `by` matches the parent `from`.

```typescript
static load(writer: ICRUDRepository<object>, enrichment: Enrichment, parent: object): Promise<object[]>
```

- `writer` — child [ICRUDRepository](./crud-repository.md)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent object

**Returns** `Promise<object[]>`

##### loadLinks(writer, column, key)

Reads junction rows whose `column` equals `key`.

```typescript
static loadLinks(writer: ICRUDRepository<object>, column: string, key: Scalar): Promise<object[]>
```

- `writer` — link [ICRUDRepository](./crud-repository.md)
- `column` — junction column
- `key` — parent key

**Returns** `Promise<object[]>`

### csharp
#### DeleteEnrichmentStrategy

```csharp
public abstract class DeleteEnrichmentStrategy<T>
```

**Methods**

##### DeleteOwned(entry, enrichment, parent, deleteNested)

Removes the owned `many` list for one parent.

```csharp
public abstract Task DeleteOwned(StoredEnrichment entry, Enrichment enrichment, object parent, Func<IReadOnlyList<object>, Task> deleteNested)
```

- `entry` — [StoredEnrichment](./update-enrichment-strategy.md#storedenrichment)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent object
- `deleteNested` — deletes nested rows of the children before the children themselves

**Returns** `Task`

##### DeleteLinked(entry, enrichment, parent)

Removes the junction rows for one parent.

```csharp
public abstract Task DeleteLinked(StoredEnrichment entry, Enrichment enrichment, object parent)
```

- `entry` — [StoredEnrichment](./update-enrichment-strategy.md#storedenrichment)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent object

**Returns** `Task`

### rust
#### DeleteEnrichmentStrategy

```rust
pub trait DeleteEnrichmentStrategy: Send + Sync
```

**Methods**

##### delete_owned(entry, enrichment, parent, delete_nested)

Removes the owned `many` list for one parent.

```rust
async fn delete_owned(&self, entry: &StoredEnrichment, enrichment: &Enrichment, parent: &Row, delete_nested: &dyn DeleteNested) -> Result<()>
```

- `entry` — [StoredEnrichment](./update-enrichment-strategy.md#storedenrichment)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent row
- `delete_nested` — deletes nested rows of the children before the children themselves

**Returns** `Result<()>`

##### delete_linked(entry, enrichment, parent)

Removes the junction rows for one parent.

```rust
async fn delete_linked(&self, entry: &StoredEnrichment, enrichment: &Enrichment, parent: &Row) -> Result<()>
```

- `entry` — [StoredEnrichment](./update-enrichment-strategy.md#storedenrichment)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent row

**Returns** `Result<()>`
:::

## Remarks

Pass a subclass as the `delete` argument of [DefaultWriteEnrichmentStrategy](./default-write-enrichment-strategy.md). A strategy that leaves {{DeleteOwned|deleteOwned|delete_owned}} empty keeps the children when the parent is deleted. The default [CascadeDeleteEnrichmentStrategy](./cascade-delete-enrichment-strategy.md) loads existing children by foreign key and does not trust the payload lists.

## Examples

### Skip child deletes

::: example
```csharp csharp/Datasources.Tests/CRUDRepositoryTests.cs#DeleteStrategyCanSkipChildren
```
```rust rust/datasources/src/delete_enrichment_strategy.rs#delete_strategy_can_skip_children
```
```typescript typescript/datasources/src/CRUDRepository.test.ts#delete-strategy-can-skip-children
```
:::
