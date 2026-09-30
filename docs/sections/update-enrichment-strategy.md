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

##### updateOwned(entry, enrichment, parent)

Writes the owned `many` list for one parent.

```typescript
abstract updateOwned(entry: StoredEnrichment, enrichment: Enrichment, parent: object): Promise<void>
```

- `entry` — [StoredEnrichment](#storedenrichment)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent object

**Returns** `Promise<void>`

##### updateLinked(entry, enrichment, parent)

Writes the junction rows for one parent.

```typescript
abstract updateLinked(entry: StoredEnrichment, enrichment: Enrichment, parent: object): Promise<void>
```

- `entry` — [StoredEnrichment](#storedenrichment)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent object

**Returns** `Promise<void>`

##### createChildren(writer, enrichment, parent)

Stamps each child's `by` from the parent `from`, then creates the payload.

```typescript
createChildren(writer: ICRUDRepository<object>, enrichment: Enrichment, parent: object): Promise<void>
```

- `writer` — child [ICRUDRepository](./crud-repository.md)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent object

**Returns** `Promise<void>`

##### createLinks(writer, enrichment, parent)

Inserts one junction row per related id.

```typescript
createLinks(writer: ICRUDRepository<object>, enrichment: Enrichment, parent: object): Promise<void>
```

- `writer` — link [ICRUDRepository](./crud-repository.md)
- `enrichment` — [Enrichment](./enrichment.md) with [association](./enrichment.md#association)
- `parent` — parent object

**Returns** `Promise<void>`

#### StoredEnrichment

```typescript
class StoredEnrichment
```

One registered edge. Update and delete strategies receive it.

**Constructor**

##### new StoredEnrichment(enrichment, apply, persist?, link?)

Stores one edge.

```typescript
constructor(enrichment: Enrichment, apply: ApplyEnrichment, persist?: ICRUDRepository<object>, link?: ICRUDRepository<object>)
```

- `enrichment` — [Enrichment](./enrichment.md)
- `apply` — read fill for this edge
- `persist` — child [ICRUDRepository](./crud-repository.md) for an owned list
- `link` — junction [ICRUDRepository](./crud-repository.md)

**Returns** [StoredEnrichment](#storedenrichment)

**Properties**

##### enrichment

The registered [Enrichment](./enrichment.md).

```typescript
readonly enrichment: Enrichment
```

##### persist

Child repository for an owned list.

```typescript
persist?: ICRUDRepository<object>
```

##### link

Junction repository for an association.

```typescript
link?: ICRUDRepository<object>
```

### csharp
#### UpdateEnrichmentStrategy

```csharp
public abstract class UpdateEnrichmentStrategy<T>
```

**Methods**

##### UpdateOwned(entry, enrichment, parent)

Writes the owned `many` list for one parent.

```csharp
public abstract Task UpdateOwned(StoredEnrichment entry, Enrichment enrichment, object parent)
```

- `entry` — [StoredEnrichment](#storedenrichment)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent object

**Returns** `Task`

##### UpdateLinked(entry, enrichment, parent)

Writes the junction rows for one parent.

```csharp
public abstract Task UpdateLinked(StoredEnrichment entry, Enrichment enrichment, object parent)
```

- `entry` — [StoredEnrichment](#storedenrichment)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent object

**Returns** `Task`

#### StoredEnrichment

```csharp
public sealed class StoredEnrichment
```

One registered edge. Update and delete strategies receive it.

**Properties**

##### Enrichment

The registered [Enrichment](./enrichment.md).

```csharp
public Enrichment Enrichment { get; }
```

### rust
#### UpdateEnrichmentStrategy

```rust
pub trait UpdateEnrichmentStrategy: Send + Sync
```

**Methods**

##### update_owned(entry, enrichment, parent)

Writes the owned `many` list for one parent.

```rust
async fn update_owned(&self, entry: &StoredEnrichment, enrichment: &Enrichment, parent: &mut Row) -> Result<()>
```

- `entry` — [StoredEnrichment](#storedenrichment)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent row

**Returns** `Result<()>`

##### update_linked(entry, enrichment, parent)

Writes the junction rows for one parent.

```rust
async fn update_linked(&self, entry: &StoredEnrichment, enrichment: &Enrichment, parent: &mut Row) -> Result<()>
```

- `entry` — [StoredEnrichment](#storedenrichment)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent row

**Returns** `Result<()>`

#### StoredEnrichment

```rust
pub struct StoredEnrichment
```

One registered edge. Update and delete strategies receive it.

**Properties**

##### enrichment

The registered [Enrichment](./enrichment.md).

```rust
pub enrichment: Enrichment
```

##### via

Related read repository.

```rust
pub via: Arc<dyn IReadonlyRepository>
```

##### persist

Child repository for an owned list.

```rust
pub persist: Option<Arc<dyn ICRUDRepository>>
```

##### link

Junction repository for an association.

```rust
pub link: Option<Arc<dyn ICRUDRepository>>
```

##### strategy

Read strategy for this edge.

```rust
pub strategy: Arc<dyn ReadonlyEnrichmentStrategy>
```
:::

## Remarks

Create uses the same child and link inserts as an update of a new parent. A subclass implements the two update methods. TypeScript also exposes `createChildren` and `createLinks`. Pass a subclass to [DefaultWriteEnrichmentStrategy](./default-write-enrichment-strategy.md).

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
