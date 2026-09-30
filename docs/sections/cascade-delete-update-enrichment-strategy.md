# CascadeDeleteUpdateEnrichmentStrategy

Update that deletes every existing child, then inserts the payload as new rows. Recreated children get new ids. Links are replaced the same way. Related lookup rows stay.

## Classes

::: reference
### typescript
#### CascadeDeleteUpdateEnrichmentStrategy

```typescript
class CascadeDeleteUpdateEnrichmentStrategy<T> extends UpdateEnrichmentStrategy<T>
```

**Constructor**

##### new CascadeDeleteUpdateEnrichmentStrategy()

Builds the delete-then-insert update strategy.

```typescript
constructor()
```

**Returns** [CascadeDeleteUpdateEnrichmentStrategy](#cascadedeleteupdateenrichmentstrategy)

**Methods**

##### updateOwned(entry, enrichment, parent)

Deletes every existing child for the parent key, then inserts the payload.

```typescript
updateOwned(entry: StoredEnrichment, enrichment: Enrichment, parent: object): Promise<void>
```

- `entry` — [StoredEnrichment](./update-enrichment-strategy.md#storedenrichment)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent object

**Returns** `Promise<void>`

##### updateLinked(entry, enrichment, parent)

Deletes every junction row for the parent key, then inserts the payload ids.

```typescript
updateLinked(entry: StoredEnrichment, enrichment: Enrichment, parent: object): Promise<void>
```

- `entry` — [StoredEnrichment](./update-enrichment-strategy.md#storedenrichment)
- `enrichment` — [Enrichment](./enrichment.md) with [association](./enrichment.md#association)
- `parent` — parent object

**Returns** `Promise<void>`

### csharp
#### CascadeDeleteUpdateEnrichmentStrategy

```csharp
public sealed class CascadeDeleteUpdateEnrichmentStrategy<T> : UpdateEnrichmentStrategy<T>
```

**Constructor**

##### CascadeDeleteUpdateEnrichmentStrategy()

Builds the delete-then-insert update strategy.

```csharp
public CascadeDeleteUpdateEnrichmentStrategy()
```

**Returns** [CascadeDeleteUpdateEnrichmentStrategy](#cascadedeleteupdateenrichmentstrategy)`<T>`

**Methods**

##### UpdateOwned(entry, enrichment, parent)

Deletes every existing child for the parent key, then inserts the payload.

```csharp
public override Task UpdateOwned(StoredEnrichment entry, Enrichment enrichment, object parent)
```

- `entry` — [StoredEnrichment](./update-enrichment-strategy.md#storedenrichment)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent object

**Returns** `Task`

##### UpdateLinked(entry, enrichment, parent)

Deletes every junction row for the parent key, then inserts the payload ids.

```csharp
public override Task UpdateLinked(StoredEnrichment entry, Enrichment enrichment, object parent)
```

- `entry` — [StoredEnrichment](./update-enrichment-strategy.md#storedenrichment)
- `enrichment` — [Enrichment](./enrichment.md) with [Association](./enrichment.md#association)
- `parent` — parent object

**Returns** `Task`

### rust
#### CascadeDeleteUpdateEnrichmentStrategy

```rust
pub struct CascadeDeleteUpdateEnrichmentStrategy
```

**Methods**

##### update_owned(entry, enrichment, parent)

Deletes every existing child for the parent key, then inserts the payload.

```rust
async fn update_owned(&self, entry: &StoredEnrichment, enrichment: &Enrichment, parent: &mut Row) -> Result<()>
```

- `entry` — [StoredEnrichment](./update-enrichment-strategy.md#storedenrichment)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent row

**Returns** `Result<()>`

##### update_linked(entry, enrichment, parent)

Deletes every junction row for the parent key, then inserts the payload ids.

```rust
async fn update_linked(&self, entry: &StoredEnrichment, enrichment: &Enrichment, parent: &mut Row) -> Result<()>
```

- `entry` — [StoredEnrichment](./update-enrichment-strategy.md#storedenrichment)
- `enrichment` — [Enrichment](./enrichment.md) with [association](./enrichment.md#association)
- `parent` — parent row

**Returns** `Result<()>`
:::

## Remarks

Pass it as the `update` argument of [DefaultWriteEnrichmentStrategy](./default-write-enrichment-strategy.md). Existing child ids are not reused. [DefaultUpdateEnrichmentStrategy](./default-update-enrichment-strategy.md) keeps matching ids.

## Examples

### Recreate children

::: example
```csharp csharp/Datasources.Tests/CRUDRepositoryTests.cs#CascadeUpdateDeletesChildrenAndRecreates
```
```rust rust/datasources/tests/enriched.rs#cascade_update_deletes_children_and_recreates
```
```typescript typescript/datasources/src/CRUDRepository.test.ts#cascade-update-deletes-children-and-recreates
```
:::
