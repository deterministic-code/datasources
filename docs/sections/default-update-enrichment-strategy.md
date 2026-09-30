# DefaultUpdateEnrichmentStrategy

Replace-sync for an owned list and a link. Matching children update. Missing children delete. New children insert. A link is replaced the same way: junction rows that are gone are deleted, and new related ids are inserted.

## Classes

::: reference
### typescript
#### DefaultUpdateEnrichmentStrategy

```typescript
class DefaultUpdateEnrichmentStrategy<T> extends UpdateEnrichmentStrategy<T>
```

**Constructor**

##### new DefaultUpdateEnrichmentStrategy()

Builds the replace-sync update strategy.

```typescript
constructor()
```

**Returns** [DefaultUpdateEnrichmentStrategy](#defaultupdateenrichmentstrategy)

**Methods**

##### updateOwned(entry, enrichment, parent)

Loads existing children, deletes rows missing from the payload, updates matches, and creates the rest.

```typescript
updateOwned(entry: StoredEnrichment, enrichment: Enrichment, parent: object): Promise<void>
```

- `entry` — [StoredEnrichment](./update-enrichment-strategy.md#storedenrichment)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent object

**Returns** `Promise<void>`

##### updateLinked(entry, enrichment, parent)

Replaces the junction set for the parent key.

```typescript
updateLinked(entry: StoredEnrichment, enrichment: Enrichment, parent: object): Promise<void>
```

- `entry` — [StoredEnrichment](./update-enrichment-strategy.md#storedenrichment)
- `enrichment` — [Enrichment](./enrichment.md) with [association](./enrichment.md#association)
- `parent` — parent object

**Returns** `Promise<void>`

### csharp
#### DefaultUpdateEnrichmentStrategy

```csharp
public sealed class DefaultUpdateEnrichmentStrategy<T> : UpdateEnrichmentStrategy<T>
```

**Constructor**

##### DefaultUpdateEnrichmentStrategy()

Builds the replace-sync update strategy.

```csharp
public DefaultUpdateEnrichmentStrategy()
```

**Returns** [DefaultUpdateEnrichmentStrategy](#defaultupdateenrichmentstrategy)`<T>`

**Methods**

##### UpdateOwned(entry, enrichment, parent)

Loads existing children, deletes rows missing from the payload, updates matches, and creates the rest.

```csharp
public override Task UpdateOwned(StoredEnrichment entry, Enrichment enrichment, object parent)
```

- `entry` — [StoredEnrichment](./update-enrichment-strategy.md#storedenrichment)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent object

**Returns** `Task`

##### UpdateLinked(entry, enrichment, parent)

Replaces the junction set for the parent key.

```csharp
public override Task UpdateLinked(StoredEnrichment entry, Enrichment enrichment, object parent)
```

- `entry` — [StoredEnrichment](./update-enrichment-strategy.md#storedenrichment)
- `enrichment` — [Enrichment](./enrichment.md) with [Association](./enrichment.md#association)
- `parent` — parent object

**Returns** `Task`

### rust
#### DefaultUpdateEnrichmentStrategy

```rust
pub struct DefaultUpdateEnrichmentStrategy
```

**Methods**

##### update_owned(entry, enrichment, parent)

Loads existing children, deletes rows missing from the payload, updates matches, and creates the rest.

```rust
async fn update_owned(&self, entry: &StoredEnrichment, enrichment: &Enrichment, parent: &mut Row) -> Result<()>
```

- `entry` — [StoredEnrichment](./update-enrichment-strategy.md#storedenrichment)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent row

**Returns** `Result<()>`

##### update_linked(entry, enrichment, parent)

Replaces the junction set for the parent key.

```rust
async fn update_linked(&self, entry: &StoredEnrichment, enrichment: &Enrichment, parent: &mut Row) -> Result<()>
```

- `entry` — [StoredEnrichment](./update-enrichment-strategy.md#storedenrichment)
- `enrichment` — [Enrichment](./enrichment.md) with [association](./enrichment.md#association)
- `parent` — parent row

**Returns** `Result<()>`
:::

## Remarks

This is the update strategy [DefaultWriteEnrichmentStrategy](./default-write-enrichment-strategy.md) uses when `update` is omitted. Identity comes from the child repository's id columns. A child with no id is created. Related rows that are only lookups stay.

## Examples

### Replace-sync children

::: example
```csharp csharp/Datasources.Tests/CRUDRepositoryTests.cs#UpdateReplaceSyncsChildren
```
```rust rust/datasources/tests/enriched.rs#deep_update_replace_syncs_children
```
```typescript typescript/datasources/src/CRUDRepository.test.ts#update-replace-syncs-children
```
:::
