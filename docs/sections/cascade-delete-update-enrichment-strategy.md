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

##### updateOwned(writer, enrichment, parent)

Deletes every existing child for the parent key, then inserts the payload.

```typescript
updateOwned(writer: ICRUDRepository<object>, enrichment: Enrichment, parent: T): Promise<void>
```

- `writer` — child [ICRUDRepository](./crud-repository.md)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent entity

**Returns** `Promise<void>`

##### updateLinked(writer, enrichment, parent)

Deletes every junction row for the parent key, then inserts the payload ids.

```typescript
updateLinked(writer: ICRUDRepository<object>, enrichment: Enrichment, parent: T): Promise<void>
```

- `writer` — link [ICRUDRepository](./crud-repository.md)
- `enrichment` — [Enrichment](./enrichment.md) with [association](./enrichment.md#association)
- `parent` — parent entity

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

##### UpdateOwned(writer, enrichment, parent)

Deletes every existing child for the parent key, then inserts the payload.

```csharp
public override Task UpdateOwned<TChild>(ICRUDRepository<TChild> writer, Enrichment enrichment, T parent)
```

- `writer` — child [ICRUDRepository](./crud-repository.md)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent entity

**Returns** `Task`

##### UpdateLinked(writer, enrichment, parent)

Deletes every junction row for the parent key, then inserts the payload ids.

```csharp
public override Task UpdateLinked<TChild>(ICRUDRepository<TChild> writer, Enrichment enrichment, T parent)
```

- `writer` — link [ICRUDRepository](./crud-repository.md)
- `enrichment` — [Enrichment](./enrichment.md) with [Association](./enrichment.md#association)
- `parent` — parent entity

**Returns** `Task`

### rust
#### CascadeDeleteUpdateEnrichmentStrategy

```rust
pub struct CascadeDeleteUpdateEnrichmentStrategy
```

**Methods**

##### update_owned(writer, enrichment, parent)

Deletes every existing child for the parent key, then inserts the payload.

```rust
async fn update_owned(&self, writer: Arc<dyn ICRUDRepository>, enrichment: &Enrichment, parent: &mut Row) -> Result<()>
```

- `writer` — child [ICRUDRepository](./crud-repository.md)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent row

**Returns** `Result<()>`

##### update_linked(writer, enrichment, parent)

Deletes every junction row for the parent key, then inserts the payload ids.

```rust
async fn update_linked(&self, writer: Arc<dyn ICRUDRepository>, enrichment: &Enrichment, parent: &mut Row) -> Result<()>
```

- `writer` — link [ICRUDRepository](./crud-repository.md)
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
