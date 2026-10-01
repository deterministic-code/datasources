# CascadeDeleteEnrichmentStrategy

Default delete. Deletes nested `List` rows first, then children, then the parent delete continues. Loads existing children by foreign key. Does not trust payload lists. Junction rows for a link are deleted and the related rows stay.

## Classes

::: reference
### typescript
#### CascadeDeleteEnrichmentStrategy

```typescript
class CascadeDeleteEnrichmentStrategy<T> extends DeleteEnrichmentStrategy<T>
```

**Constructor**

##### new CascadeDeleteEnrichmentStrategy()

Builds the cascade delete strategy.

```typescript
constructor()
```

**Returns** [CascadeDeleteEnrichmentStrategy](#cascadedeleteenrichmentstrategy)

**Methods**

##### deleteOwned(entry, enrichment, parent, deleteNested)

Loads children by foreign key, deletes their nested rows, then deletes the children.

```typescript
deleteOwned(entry: StoredEnrichment, enrichment: Enrichment, parent: object, deleteNested: (rows: object[]) => Promise<void>): Promise<void>
```

- `entry` — [StoredEnrichment](./update-enrichment-strategy.md#storedenrichment)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent object
- `deleteNested` — deletes nested rows of the children before the children themselves

**Returns** `Promise<void>`

##### deleteLinked(entry, enrichment, parent)

Deletes junction rows for the parent key. Related rows stay.

```typescript
deleteLinked(entry: StoredEnrichment, enrichment: Enrichment, parent: object): Promise<void>
```

- `entry` — [StoredEnrichment](./update-enrichment-strategy.md#storedenrichment)
- `enrichment` — [Enrichment](./enrichment.md) with [association](./enrichment.md#association)
- `parent` — parent object

**Returns** `Promise<void>`

### csharp
#### CascadeDeleteEnrichmentStrategy

```csharp
public sealed class CascadeDeleteEnrichmentStrategy<T> : DeleteEnrichmentStrategy<T>
```

**Constructor**

##### CascadeDeleteEnrichmentStrategy()

Builds the cascade delete strategy.

```csharp
public CascadeDeleteEnrichmentStrategy()
```

**Returns** [CascadeDeleteEnrichmentStrategy](#cascadedeleteenrichmentstrategy)`<T>`

**Methods**

##### DeleteOwned(entry, enrichment, parent, deleteNested)

Loads children by foreign key, deletes their nested rows, then deletes the children.

```csharp
public override Task DeleteOwned(StoredEnrichment entry, Enrichment enrichment, object parent, Func<IReadOnlyList<object>, Task> deleteNested)
```

- `entry` — [StoredEnrichment](./update-enrichment-strategy.md#storedenrichment)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent object
- `deleteNested` — deletes nested rows of the children before the children themselves

**Returns** `Task`

##### DeleteLinked(entry, enrichment, parent)

Deletes junction rows for the parent key. Related rows stay.

```csharp
public override Task DeleteLinked(StoredEnrichment entry, Enrichment enrichment, object parent)
```

- `entry` — [StoredEnrichment](./update-enrichment-strategy.md#storedenrichment)
- `enrichment` — [Enrichment](./enrichment.md) with [Association](./enrichment.md#association)
- `parent` — parent object

**Returns** `Task`

### rust
#### CascadeDeleteEnrichmentStrategy

```rust
pub struct CascadeDeleteEnrichmentStrategy
```

**Methods**

##### delete_owned(entry, enrichment, parent, delete_nested)

Loads children by foreign key, deletes their nested rows, then deletes the children.

```rust
async fn delete_owned(&self, entry: &StoredEnrichment, enrichment: &Enrichment, parent: &Row, delete_nested: &dyn DeleteNested) -> Result<()>
```

- `entry` — [StoredEnrichment](./update-enrichment-strategy.md#storedenrichment)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent row
- `delete_nested` — deletes nested rows of the children before the children themselves

**Returns** `Result<()>`

##### delete_linked(entry, enrichment, parent)

Deletes junction rows for the parent key. Related rows stay.

```rust
async fn delete_linked(&self, entry: &StoredEnrichment, enrichment: &Enrichment, parent: &Row) -> Result<()>
```

- `entry` — [StoredEnrichment](./update-enrichment-strategy.md#storedenrichment)
- `enrichment` — [Enrichment](./enrichment.md) with [association](./enrichment.md#association)
- `parent` — parent row

**Returns** `Result<()>`
:::

## Remarks

[DefaultWriteEnrichmentStrategy](./default-write-enrichment-strategy.md) uses this when `delete` is omitted. The parent row is deleted by the repository after {{Persist|persist|persist}} returns.

## Examples

### Cascade delete

::: example
```csharp csharp/Datasources.Tests/CRUDRepositoryTests.cs#DeleteCascadesChildrenThenParent
```
```rust rust/datasources/tests/enriched.rs#deep_delete_cascades_children_then_parent
```
```typescript typescript/datasources/src/CRUDRepository.test.ts#delete-cascades-children-then-parent
```
:::
