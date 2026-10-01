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

##### updateOwned(writer, enrichment, parent)

Loads existing children, deletes rows missing from the payload, updates matches, and creates the rest.

```typescript
updateOwned(writer: ICRUDRepository<object>, enrichment: Enrichment, parent: object): Promise<void>
```

- `writer` — child [ICRUDRepository](./crud-repository.md)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent object

**Returns** `Promise<void>`

##### updateLinked(writer, enrichment, parent)

Replaces the junction set for the parent key.

```typescript
updateLinked(writer: ICRUDRepository<object>, enrichment: Enrichment, parent: object): Promise<void>
```

- `writer` — link [ICRUDRepository](./crud-repository.md)
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

##### UpdateOwned(writer, enrichment, parent)

Loads existing children, deletes rows missing from the payload, updates matches, and creates the rest.

```csharp
public override Task UpdateOwned(IChildPersist writer, Enrichment enrichment, object parent)
```

- `writer` — child [IChildPersist](./update-enrichment-strategy.md#ichildpersist)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent object

**Returns** `Task`

##### UpdateLinked(writer, enrichment, parent)

Replaces the junction set for the parent key.

```csharp
public override Task UpdateLinked(IChildPersist writer, Enrichment enrichment, object parent)
```

- `writer` — link [IChildPersist](./update-enrichment-strategy.md#ichildpersist)
- `enrichment` — [Enrichment](./enrichment.md) with [Association](./enrichment.md#association)
- `parent` — parent object

**Returns** `Task`

### rust
#### DefaultUpdateEnrichmentStrategy

```rust
pub struct DefaultUpdateEnrichmentStrategy
```

**Methods**

##### update_owned(writer, enrichment, parent)

Loads existing children, deletes rows missing from the payload, updates matches, and creates the rest.

```rust
async fn update_owned(&self, writer: Arc<dyn ICRUDRepository>, enrichment: &Enrichment, parent: &mut Row) -> Result<()>
```

- `writer` — child [ICRUDRepository](./crud-repository.md)
- `enrichment` — [Enrichment](./enrichment.md)
- `parent` — parent row

**Returns** `Result<()>`

##### update_linked(writer, enrichment, parent)

Replaces the junction set for the parent key.

```rust
async fn update_linked(&self, writer: Arc<dyn ICRUDRepository>, enrichment: &Enrichment, parent: &mut Row) -> Result<()>
```

- `writer` — link [ICRUDRepository](./crud-repository.md)
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
