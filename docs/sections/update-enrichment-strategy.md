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
public abstract Task UpdateOwned(IChildPersist writer, Enrichment enrichment, T parent)
```

- `writer` — child [IChildPersist](#ichildpersist)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent entity

**Returns** `Task`

##### UpdateLinked(writer, enrichment, parent)

Writes the junction rows for one parent.

```csharp
public abstract Task UpdateLinked(IChildPersist writer, Enrichment enrichment, T parent)
```

- `writer` — link [IChildPersist](#ichildpersist)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent entity

**Returns** `Task`

#### IChildPersist

```csharp
public interface IChildPersist
```

Erased CRUD writer for an owned list or junction.

**Methods**

##### Create(item)

Creates one child or link row.

```csharp
Task<object> Create(object item)
```

- `item` — row to create

**Returns** `Task<object>`

##### Update(item, byWhere)

Updates one child or link row.

```csharp
Task<Response> Update(object item, IWhere? byWhere = null)
```

- `item` — row to update
- `byWhere` — optional predicate

**Returns** `Task` of `Response`

##### Delete(item, byWhere)

Deletes one child or link row.

```csharp
Task<Response> Delete(object item, IWhere? byWhere = null)
```

- `item` — row to delete
- `byWhere` — optional predicate

**Returns** `Task` of `Response`

##### Read(where)

Reads child or link rows.

```csharp
IAsyncEnumerable<object> Read(IWhere where)
```

- `where` — filter

**Returns** `IAsyncEnumerable<object>`

##### New()

Builds an empty child or link row.

```csharp
object New()
```

**Returns** `object`

**Properties**

##### Ids

Id property names on the child or link type.

```csharp
IReadOnlyList<string> Ids { get; }
```

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
