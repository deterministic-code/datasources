# UpdateEnrichmentStrategy

How an update writes owned children and links. [DefaultWriteEnrichmentStrategy](./default-write-enrichment-strategy.md) calls these methods. [DefaultUpdateEnrichmentStrategy](./default-update-enrichment-strategy.md) replace-syncs. [CascadeDeleteUpdateEnrichmentStrategy](./cascade-delete-update-enrichment-strategy.md) deletes, then inserts.

## Classes

::: reference
### typescript
#### UpdateEnrichmentStrategy

```typescript
abstract class UpdateEnrichmentStrategy<T>
```

**Methods**

##### updateOwned(writer, enrichment, parent)

Writes the owned `List` for one parent.

```typescript
abstract updateOwned(writer: ICRUDRepository<object>, enrichment: Enrichment, parent: T): Promise<void>
```

- `writer` — child [ICRUDRepository](./crud-repository.md)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent entity

**Returns** `Promise<void>`

##### updateLinked(writer, enrichment, parent)

Writes the junction rows for one parent.

```typescript
abstract updateLinked(writer: ICRUDRepository<object>, enrichment: Enrichment, parent: T): Promise<void>
```

- `writer` — link [ICRUDRepository](./crud-repository.md)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent entity

**Returns** `Promise<void>`

### csharp
#### UpdateEnrichmentStrategy

```csharp
public abstract class UpdateEnrichmentStrategy<T>
```

**Methods**

##### UpdateOwned(writer, enrichment, parent)

Writes the owned `List` for one parent.

```csharp
public abstract Task UpdateOwned<TChild>(ICRUDRepository<TChild> writer, Enrichment enrichment, T parent) where TChild : new()
```

- `writer` — child [ICRUDRepository](./crud-repository.md)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent entity

**Returns** `Task`

##### UpdateLinked(writer, enrichment, parent)

Writes the junction rows for one parent.

```csharp
public abstract Task UpdateLinked<TChild>(ICRUDRepository<TChild> writer, Enrichment enrichment, T parent) where TChild : new()
```

- `writer` — link [ICRUDRepository](./crud-repository.md)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent entity

**Returns** `Task`

### rust
#### UpdateEnrichmentStrategy

```rust
pub trait UpdateEnrichmentStrategy: Send + Sync
```

**Methods**

##### update_owned(writer, enrichment, parent)

Writes the owned `List` for one parent.

```rust
async fn update_owned(&self, writer: Arc<dyn ICRUDRepository>, enrichment: &Enrichment, parent: &mut Row) -> Result<()>
```

- `writer` — child [ICRUDRepository](./crud-repository.md)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent row

**Returns** `Result<()>`

##### update_linked(writer, enrichment, parent)

Writes the junction rows for one parent.

```rust
async fn update_linked(&self, writer: Arc<dyn ICRUDRepository>, enrichment: &Enrichment, parent: &mut Row) -> Result<()>
```

- `writer` — link [ICRUDRepository](./crud-repository.md)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent row

**Returns** `Result<()>`
:::

## Remarks

A subclass implements the two update methods. Pass a subclass to [DefaultWriteEnrichmentStrategy](./default-write-enrichment-strategy.md). Create uses [CreateEnrichmentStrategy](./create-enrichment-strategy.md).

## Examples

### Replace children

::: example
```csharp csharp/Datasources.Tests/CRUDRepositoryTests.cs#CascadeUpdateDeletesChildrenAndRecreates
```
```rust rust/datasources/tests/enriched.rs#cascade_update_deletes_children_and_recreates
```
```typescript typescript/datasources/src/CRUDRepository.test.ts#cascade-update-deletes-children-and-recreates
```
:::
