# LazyWriteEnrichmentStrategy

Nested write that skips a null list on create and update. Delete still removes the junction rows and owned children. The rest matches [DefaultWriteEnrichmentStrategy](./default-write-enrichment-strategy.md).

## Classes

::: reference
### typescript
#### LazyWriteEnrichmentStrategy

```typescript
class LazyWriteEnrichmentStrategy<T> extends DefaultWriteEnrichmentStrategy<T>
```

**Constructor**

##### new LazyWriteEnrichmentStrategy(deleteStrategy?, updateStrategy?, createStrategy?, useTransaction?)

Builds the lazy write strategy.

```typescript
constructor(deleteStrategy?: DeleteEnrichmentStrategy<T>, updateStrategy?: UpdateEnrichmentStrategy<T>, createStrategy?: CreateEnrichmentStrategy<T>, useTransaction?: boolean)
```

- `deleteStrategy` — [DeleteEnrichmentStrategy](./delete-enrichment-strategy.md). Omitted uses [CascadeDeleteEnrichmentStrategy](./cascade-delete-enrichment-strategy.md)
- `updateStrategy` — [UpdateEnrichmentStrategy](./update-enrichment-strategy.md). Omitted uses [DefaultUpdateEnrichmentStrategy](./default-update-enrichment-strategy.md)
- `createStrategy` — [CreateEnrichmentStrategy](./create-enrichment-strategy.md). Omitted uses [DefaultCreateEnrichmentStrategy](./default-create-enrichment-strategy.md)
- `useTransaction` — commit the parent write and nested persist together. Off by default

**Returns** [LazyWriteEnrichmentStrategy](#lazywriteenrichmentstrategy)

### csharp
#### LazyWriteEnrichmentStrategy

```csharp
public sealed class LazyWriteEnrichmentStrategy<T> : DefaultWriteEnrichmentStrategy<T>
```

**Constructor**

##### LazyWriteEnrichmentStrategy(delete, update, create, useTransaction)

Builds the lazy write strategy.

```csharp
public LazyWriteEnrichmentStrategy(DeleteEnrichmentStrategy<T>? delete = null, UpdateEnrichmentStrategy<T>? update = null, CreateEnrichmentStrategy<T>? create = null, bool useTransaction = false)
```

- `delete` — [DeleteEnrichmentStrategy](./delete-enrichment-strategy.md). Omitted uses [CascadeDeleteEnrichmentStrategy](./cascade-delete-enrichment-strategy.md)
- `update` — [UpdateEnrichmentStrategy](./update-enrichment-strategy.md). Omitted uses [DefaultUpdateEnrichmentStrategy](./default-update-enrichment-strategy.md)
- `create` — [CreateEnrichmentStrategy](./create-enrichment-strategy.md). Omitted uses [DefaultCreateEnrichmentStrategy](./default-create-enrichment-strategy.md)
- `useTransaction` — commit the parent write and nested persist together. Off by default

**Returns** [LazyWriteEnrichmentStrategy](#lazywriteenrichmentstrategy)`<T>`

### rust
#### LazyWriteEnrichmentStrategy

```rust
pub struct LazyWriteEnrichmentStrategy
```

**Constructor**

##### new(delete, update, create, use_transaction)

Builds a [DefaultWriteEnrichmentStrategy](./default-write-enrichment-strategy.md) that skips a null list on create and update.

```rust
pub fn new(delete: Option<Arc<dyn DeleteEnrichmentStrategy>>, update: Option<Arc<dyn UpdateEnrichmentStrategy>>, create: Option<Arc<dyn CreateEnrichmentStrategy>>, use_transaction: bool) -> DefaultWriteEnrichmentStrategy
```

- `delete` — [DeleteEnrichmentStrategy](./delete-enrichment-strategy.md). Omitted uses [CascadeDeleteEnrichmentStrategy](./cascade-delete-enrichment-strategy.md)
- `update` — [UpdateEnrichmentStrategy](./update-enrichment-strategy.md). Omitted uses [DefaultUpdateEnrichmentStrategy](./default-update-enrichment-strategy.md)
- `create` — [CreateEnrichmentStrategy](./create-enrichment-strategy.md). Omitted uses [DefaultCreateEnrichmentStrategy](./default-create-enrichment-strategy.md)
- `use_transaction` — commit the parent write and nested persist together

**Returns** [DefaultWriteEnrichmentStrategy](./default-write-enrichment-strategy.md)
:::

## Remarks

A null list on create or update leaves that association as it is. An empty list still clears it. Delete still removes junction rows and owned children. Set it as the repository `write` option.

::: prose
### rust
`new` returns [DefaultWriteEnrichmentStrategy](./default-write-enrichment-strategy.md).
:::

## Examples

### Skip unloaded lists

::: example
```csharp csharp/Providers/Sqlite.Tests/UserViewCRUDRepositoryTests.cs#LazyCreateAndUpdateSkipUnloadedCollectionsAndDeleteCascades
```
```rust rust/datasources/tests/user_view.rs#lazy_write_skips_unloaded_links
```
```typescript typescript/datasources/src/UserViewCRUDRepository.test.ts#lazy-write-skips-unloaded-links
```
:::
