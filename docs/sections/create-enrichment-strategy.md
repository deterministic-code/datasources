# CreateEnrichmentStrategy

How a create writes owned children and links. [DefaultWriteEnrichmentStrategy](./default-write-enrichment-strategy.md) calls these methods after the parent row is created. [DefaultCreateEnrichmentStrategy](./default-create-enrichment-strategy.md) is the default.

## Classes

::: reference
### typescript
#### CreateEnrichmentStrategy

```typescript
abstract class CreateEnrichmentStrategy<T>
```

**Methods**

##### createOwned(writer, enrichment, parent)

Creates the owned `List` for one parent.

```typescript
abstract createOwned(writer: ICRUDRepository<object>, enrichment: Enrichment, parent: T): Promise<void>
```

- `writer` — child [ICRUDRepository](./crud-repository.md)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent entity

**Returns** `Promise<void>`

##### createLinked(writer, enrichment, parent)

Creates the junction rows for one parent.

```typescript
abstract createLinked(writer: ICRUDRepository<object>, enrichment: Enrichment, parent: T): Promise<void>
```

- `writer` — link [ICRUDRepository](./crud-repository.md)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent entity

**Returns** `Promise<void>`

### csharp
#### CreateEnrichmentStrategy

```csharp
public abstract class CreateEnrichmentStrategy<T>
```

**Methods**

##### CreateOwned(writer, enrichment, parent)

Creates the owned `List` for one parent.

```csharp
public abstract Task CreateOwned<TChild>(ICRUDRepository<TChild> writer, Enrichment enrichment, T parent) where TChild : new()
```

- `writer` — child [ICRUDRepository](./crud-repository.md)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent entity

**Returns** `Task`

##### CreateLinked(writer, enrichment, parent)

Creates the junction rows for one parent.

```csharp
public abstract Task CreateLinked<TChild>(ICRUDRepository<TChild> writer, Enrichment enrichment, T parent) where TChild : new()
```

- `writer` — link [ICRUDRepository](./crud-repository.md)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent entity

**Returns** `Task`

### rust
#### CreateEnrichmentStrategy

```rust
pub trait CreateEnrichmentStrategy: Send + Sync
```

**Methods**

##### create_owned(writer, enrichment, parent)

Creates the owned `List` for one parent.

```rust
async fn create_owned(&self, writer: Arc<dyn ICRUDRepository>, enrichment: &Enrichment, parent: &mut Row) -> Result<()>
```

- `writer` — child [ICRUDRepository](./crud-repository.md)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent row

**Returns** `Result<()>`

##### create_linked(writer, enrichment, parent)

Creates the junction rows for one parent.

```rust
async fn create_linked(&self, writer: Arc<dyn ICRUDRepository>, enrichment: &Enrichment, parent: &mut Row) -> Result<()>
```

- `writer` — link [ICRUDRepository](./crud-repository.md)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent row

**Returns** `Result<()>`
:::

## Remarks

Pass a subclass as the `create` argument of [DefaultWriteEnrichmentStrategy](./default-write-enrichment-strategy.md). The default [DefaultCreateEnrichmentStrategy](./default-create-enrichment-strategy.md) stamps each child's `RelatedKey` from the parent `SourceProperty`, then inserts children and links.
