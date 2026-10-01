# DefaultCreateEnrichmentStrategy

Default create fill. Stamps each child's `RelatedKey` from the parent `SourceProperty`, then inserts the payload children and junction rows.

## Classes

::: reference
### typescript
#### DefaultCreateEnrichmentStrategy

```typescript
class DefaultCreateEnrichmentStrategy<T> extends CreateEnrichmentStrategy<T>
```

**Constructor**

##### new DefaultCreateEnrichmentStrategy()

Builds the default create strategy.

```typescript
constructor()
```

**Returns** [DefaultCreateEnrichmentStrategy](#defaultcreateenrichmentstrategy)

**Methods**

##### createOwned(writer, enrichment, parent)

Stamps each child and creates the payload.

```typescript
createOwned(writer: ICRUDRepository<object>, enrichment: Enrichment, parent: object): Promise<void>
```

- `writer` — child [ICRUDRepository](./crud-repository.md)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent object

**Returns** `Promise<void>`

##### createLinked(writer, enrichment, parent)

Inserts one junction row per related id.

```typescript
createLinked(writer: ICRUDRepository<object>, enrichment: Enrichment, parent: object): Promise<void>
```

- `writer` — link [ICRUDRepository](./crud-repository.md)
- `enrichment` — [Enrichment](./enrichment.md) with [association](./enrichment.md#association)
- `parent` — parent object

**Returns** `Promise<void>`

### csharp
#### DefaultCreateEnrichmentStrategy

```csharp
public sealed class DefaultCreateEnrichmentStrategy<T> : CreateEnrichmentStrategy<T>
```

**Constructor**

##### DefaultCreateEnrichmentStrategy()

Builds the default create strategy.

```csharp
public DefaultCreateEnrichmentStrategy()
```

**Returns** [DefaultCreateEnrichmentStrategy](#defaultcreateenrichmentstrategy)`<T>`

**Methods**

##### CreateOwned(writer, enrichment, parent)

Stamps each child and creates the payload.

```csharp
public override Task CreateOwned(IChildPersist writer, Enrichment enrichment, object parent)
```

- `writer` — child [IChildPersist](./update-enrichment-strategy.md#ichildpersist)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent object

**Returns** `Task`

##### CreateLinked(writer, enrichment, parent)

Inserts one junction row per related id.

```csharp
public override Task CreateLinked(IChildPersist writer, Enrichment enrichment, object parent)
```

- `writer` — link [IChildPersist](./update-enrichment-strategy.md#ichildpersist)
- `enrichment` — [Enrichment](./enrichment.md) with [Association](./enrichment.md#association)
- `parent` — parent object

**Returns** `Task`

### rust
#### DefaultCreateEnrichmentStrategy

```rust
pub struct DefaultCreateEnrichmentStrategy
```

**Methods**

##### create_owned(writer, enrichment, parent)

Stamps each child and creates the payload.

```rust
async fn create_owned(&self, writer: Arc<dyn ICRUDRepository>, enrichment: &Enrichment, parent: &mut Row) -> Result<()>
```

- `writer` — child [ICRUDRepository](./crud-repository.md)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent row

**Returns** `Result<()>`

##### create_linked(writer, enrichment, parent)

Inserts one junction row per related id.

```rust
async fn create_linked(&self, writer: Arc<dyn ICRUDRepository>, enrichment: &Enrichment, parent: &mut Row) -> Result<()>
```

- `writer` — link [ICRUDRepository](./crud-repository.md)
- `enrichment` — [Enrichment](./enrichment.md) with [association](./enrichment.md#association)
- `parent` — parent row

**Returns** `Result<()>`
:::

## Remarks

This is the create strategy [DefaultWriteEnrichmentStrategy](./default-write-enrichment-strategy.md) uses when `create` is omitted. [CascadeDeleteUpdateEnrichmentStrategy](./cascade-delete-update-enrichment-strategy.md) reuses the same insert path after it clears existing children.
