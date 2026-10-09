# DeleteEnrichmentStrategy

How a delete removes children and links. [DefaultWriteEnrichmentStrategy](./default-write-enrichment-strategy.md) calls these methods before the parent row is deleted. [CascadeDeleteEnrichmentStrategy](./cascade-delete-enrichment-strategy.md) is the default.

## Classes

::: reference
### typescript
#### DeleteEnrichmentStrategy

```typescript
abstract class DeleteEnrichmentStrategy<T>
```

**Methods**

##### deleteChildren(writer, enrichment, parent, deleteNested)

Removes the child `List` for one parent.

```typescript
abstract deleteChildren<TChild extends object>(writer: ICRUDRepository<TChild>, enrichment: Enrichment, parent: T, deleteNested: (rows: TChild[]) => Promise<void>): Promise<void>
```

- `writer` — child [ICRUDRepository](./crud-repository.md)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent entity
- `deleteNested` — deletes nested rows of the children before the children themselves

**Returns** `Promise<void>`

##### deleteLinked(writer, enrichment, parent)

Removes the junction rows for one parent.

```typescript
abstract deleteLinked<TChild extends object>(writer: ICRUDRepository<TChild>, enrichment: Enrichment, parent: T): Promise<void>
```

- `writer` — link [ICRUDRepository](./crud-repository.md)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent entity

**Returns** `Promise<void>`

##### load(writer, enrichment, parent)

Reads the child rows whose `RelatedKey` matches the parent `SourceProperty`.

```typescript
static load<TParent, TChild extends object>(writer: ICRUDRepository<TChild>, enrichment: Enrichment, parent: TParent): Promise<TChild[]>
```

- `writer` — child [ICRUDRepository](./crud-repository.md)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent entity

**Returns** `Promise<TChild[]>`

##### loadLinks(writer, column, key)

Reads junction rows whose `column` equals `key`.

```typescript
static loadLinks<TChild extends object>(writer: ICRUDRepository<TChild>, column: string, key: Scalar): Promise<TChild[]>
```

- `writer` — link [ICRUDRepository](./crud-repository.md)
- `column` — junction column
- `key` — parent key

**Returns** `Promise<TChild[]>`

### csharp
#### DeleteEnrichmentStrategy

```csharp
public abstract class DeleteEnrichmentStrategy<T>
```

**Methods**

##### DeleteChildren(writer, enrichment, parent, deleteNested)

Removes the child `List` for one parent.

```csharp
public abstract Task DeleteChildren<TChild>(ICRUDRepository<TChild> writer, Enrichment enrichment, T parent, Func<IReadOnlyList<TChild>, Task> deleteNested) where TChild : new()
```

- `writer` — child [ICRUDRepository](./crud-repository.md)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent entity
- `deleteNested` — deletes nested rows of the children before the children themselves

**Returns** `Task`

##### DeleteLinked(writer, enrichment, parent)

Removes the junction rows for one parent.

```csharp
public abstract Task DeleteLinked<TChild>(ICRUDRepository<TChild> writer, Enrichment enrichment, T parent) where TChild : new()
```

- `writer` — link [ICRUDRepository](./crud-repository.md)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent entity

**Returns** `Task`

### rust
#### DeleteEnrichmentStrategy

```rust
pub trait DeleteEnrichmentStrategy: Send + Sync
```

**Methods**

##### delete_children(writer, enrichment, parent, delete_nested)

Removes the child `List` for one parent.

```rust
async fn delete_children(&self, writer: Arc<dyn ICRUDRepository>, enrichment: &Enrichment, parent: &Row, delete_nested: &dyn DeleteNested) -> Result<()>
```

- `writer` — child [ICRUDRepository](./crud-repository.md)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent row
- `delete_nested` — deletes nested rows of the children before the children themselves

**Returns** `Result<()>`

##### delete_linked(writer, enrichment, parent)

Removes the junction rows for one parent.

```rust
async fn delete_linked(&self, writer: Arc<dyn ICRUDRepository>, enrichment: &Enrichment, parent: &Row) -> Result<()>
```

- `writer` — link [ICRUDRepository](./crud-repository.md)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent row

**Returns** `Result<()>`
:::

## Remarks

Pass a subclass as the `delete` argument of [DefaultWriteEnrichmentStrategy](./default-write-enrichment-strategy.md). A strategy that leaves {{DeleteChildren|deleteChildren|delete_children}} empty keeps the children when the parent is deleted. The default [CascadeDeleteEnrichmentStrategy](./cascade-delete-enrichment-strategy.md) loads existing children by foreign key and does not trust the payload lists.

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
