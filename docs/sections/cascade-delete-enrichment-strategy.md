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

##### deleteOwned(writer, enrichment, parent, deleteNested)

Loads children by foreign key, deletes their nested rows, then deletes the children.

```typescript
deleteOwned(writer: ICRUDRepository<object>, enrichment: Enrichment, parent: T, deleteNested: (rows: object[]) => Promise<void>): Promise<void>
```

- `writer` — child [ICRUDRepository](./crud-repository.md)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent entity
- `deleteNested` — deletes nested rows of the children before the children themselves

**Returns** `Promise<void>`

##### deleteLinked(writer, enrichment, parent)

Deletes junction rows for the parent key. Related rows stay.

```typescript
deleteLinked(writer: ICRUDRepository<object>, enrichment: Enrichment, parent: T): Promise<void>
```

- `writer` — link [ICRUDRepository](./crud-repository.md)
- `enrichment` — [Enrichment](./enrichment.md) with [association](./enrichment.md#association)
- `parent` — parent entity

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

##### DeleteOwned(writer, enrichment, parent, deleteNested)

Loads children by foreign key, deletes their nested rows, then deletes the children.

```csharp
public override Task DeleteOwned(IChildPersist writer, Enrichment enrichment, T parent, Func<IReadOnlyList<object>, Task> deleteNested)
```

- `writer` — child [IChildPersist](./update-enrichment-strategy.md#ichildpersist)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent entity
- `deleteNested` — deletes nested rows of the children before the children themselves

**Returns** `Task`

##### DeleteLinked(writer, enrichment, parent)

Deletes junction rows for the parent key. Related rows stay.

```csharp
public override Task DeleteLinked(IChildPersist writer, Enrichment enrichment, T parent)
```

- `writer` — link [IChildPersist](./update-enrichment-strategy.md#ichildpersist)
- `enrichment` — [Enrichment](./enrichment.md) with [Association](./enrichment.md#association)
- `parent` — parent entity

**Returns** `Task`

### rust
#### CascadeDeleteEnrichmentStrategy

```rust
pub struct CascadeDeleteEnrichmentStrategy
```

**Methods**

##### delete_owned(writer, enrichment, parent, delete_nested)

Loads children by foreign key, deletes their nested rows, then deletes the children.

```rust
async fn delete_owned(&self, writer: Arc<dyn ICRUDRepository>, enrichment: &Enrichment, parent: &Row, delete_nested: &dyn DeleteNested) -> Result<()>
```

- `writer` — child [ICRUDRepository](./crud-repository.md)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent row
- `delete_nested` — deletes nested rows of the children before the children themselves

**Returns** `Result<()>`

##### delete_linked(writer, enrichment, parent)

Deletes junction rows for the parent key. Related rows stay.

```rust
async fn delete_linked(&self, writer: Arc<dyn ICRUDRepository>, enrichment: &Enrichment, parent: &Row) -> Result<()>
```

- `writer` — link [ICRUDRepository](./crud-repository.md)
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
