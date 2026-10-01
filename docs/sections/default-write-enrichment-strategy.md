# DefaultWriteEnrichmentStrategy

Default nested write. Create uses [DefaultCreateEnrichmentStrategy](./default-create-enrichment-strategy.md). Update uses [DefaultUpdateEnrichmentStrategy](./default-update-enrichment-strategy.md). Delete uses [CascadeDeleteEnrichmentStrategy](./cascade-delete-enrichment-strategy.md). Each can be replaced.

## Classes

::: reference
### typescript
#### DefaultWriteEnrichmentStrategy

```typescript
class DefaultWriteEnrichmentStrategy<T> implements IWriteEnrichmentStrategy<T>
```

**Constructor**

##### new DefaultWriteEnrichmentStrategy(deleteStrategy?, updateStrategy?, createStrategy?, useTransaction?)

Builds the default write strategy.

```typescript
constructor(deleteStrategy?: DeleteEnrichmentStrategy<T>, updateStrategy?: UpdateEnrichmentStrategy<T>, createStrategy?: CreateEnrichmentStrategy<T>, useTransaction?: boolean)
```

- `deleteStrategy` — [DeleteEnrichmentStrategy](./delete-enrichment-strategy.md). Omitted uses [CascadeDeleteEnrichmentStrategy](./cascade-delete-enrichment-strategy.md)
- `updateStrategy` — [UpdateEnrichmentStrategy](./update-enrichment-strategy.md). Omitted uses [DefaultUpdateEnrichmentStrategy](./default-update-enrichment-strategy.md)
- `createStrategy` — [CreateEnrichmentStrategy](./create-enrichment-strategy.md). Omitted uses [DefaultCreateEnrichmentStrategy](./default-create-enrichment-strategy.md)
- `useTransaction` — commit the parent write and nested persist together. Off by default

**Returns** [DefaultWriteEnrichmentStrategy](#defaultwriteenrichmentstrategy)

**Properties**

##### useTransaction

When true, the parent write and nested persist commit together.

```typescript
readonly useTransaction: boolean
```

**Methods**

##### persist(tree, items, mode)

Walks each enrichment and writes children or links for `mode`.

```typescript
persist(tree: IRepositoryTree, items: T[], mode: WriteMode): Promise<void>
```

- `tree` — [IRepositoryTree](./readonly-repository.md#irepositorytree)
- `items` — parents to persist
- `mode` — [WriteMode](./i-write-enrichment-strategy.md#writemode)

**Returns** `Promise<void>`

### csharp
#### DefaultWriteEnrichmentStrategy

```csharp
public class DefaultWriteEnrichmentStrategy<T> : IWriteEnrichmentStrategy<T>
```

**Constructor**

##### DefaultWriteEnrichmentStrategy(delete, update, create, useTransaction)

Builds the default write strategy.

```csharp
public DefaultWriteEnrichmentStrategy(DeleteEnrichmentStrategy<T>? delete = null, UpdateEnrichmentStrategy<T>? update = null, CreateEnrichmentStrategy<T>? create = null, bool useTransaction = false)
```

- `delete` — [DeleteEnrichmentStrategy](./delete-enrichment-strategy.md). Omitted uses [CascadeDeleteEnrichmentStrategy](./cascade-delete-enrichment-strategy.md)
- `update` — [UpdateEnrichmentStrategy](./update-enrichment-strategy.md). Omitted uses [DefaultUpdateEnrichmentStrategy](./default-update-enrichment-strategy.md)
- `create` — [CreateEnrichmentStrategy](./create-enrichment-strategy.md). Omitted uses [DefaultCreateEnrichmentStrategy](./default-create-enrichment-strategy.md)
- `useTransaction` — commit the parent write and nested persist together. Off by default

**Returns** [DefaultWriteEnrichmentStrategy](#defaultwriteenrichmentstrategy)`<T>`

**Properties**

##### UseTransaction

When true, the parent write and nested persist commit together.

```csharp
public bool UseTransaction { get; }
```

**Methods**

##### Persist(tree, items, mode)

Walks each enrichment and writes children or links for `mode`.

```csharp
public Task Persist(IRepositoryTree tree, IList<T> items, WriteMode mode)
```

- `tree` — [IRepositoryTree](./readonly-repository.md#irepositorytree)
- `items` — parents to persist
- `mode` — [WriteMode](./i-write-enrichment-strategy.md#writemode)

**Returns** `Task`

### rust
#### DefaultWriteEnrichmentStrategy

```rust
pub struct DefaultWriteEnrichmentStrategy
```

**Constructor**

##### new(delete, update, create, use_transaction)

Builds the default write strategy.

```rust
pub fn new(delete: Option<Arc<dyn DeleteEnrichmentStrategy>>, update: Option<Arc<dyn UpdateEnrichmentStrategy>>, create: Option<Arc<dyn CreateEnrichmentStrategy>>, use_transaction: bool) -> Self
```

- `delete` — [DeleteEnrichmentStrategy](./delete-enrichment-strategy.md). Omitted uses [CascadeDeleteEnrichmentStrategy](./cascade-delete-enrichment-strategy.md)
- `update` — [UpdateEnrichmentStrategy](./update-enrichment-strategy.md). Omitted uses [DefaultUpdateEnrichmentStrategy](./default-update-enrichment-strategy.md)
- `create` — [CreateEnrichmentStrategy](./create-enrichment-strategy.md). Omitted uses [DefaultCreateEnrichmentStrategy](./default-create-enrichment-strategy.md)
- `use_transaction` — commit the parent write and nested persist together

**Returns** [DefaultWriteEnrichmentStrategy](#defaultwriteenrichmentstrategy)

##### lazy(delete, update, create, use_transaction)

Builds the same strategy and skips a null list on create and update. [LazyWriteEnrichmentStrategy](./lazy-write-enrichment-strategy.md) returns this.

```rust
pub fn lazy(delete: Option<Arc<dyn DeleteEnrichmentStrategy>>, update: Option<Arc<dyn UpdateEnrichmentStrategy>>, create: Option<Arc<dyn CreateEnrichmentStrategy>>, use_transaction: bool) -> Self
```

- `delete` — [DeleteEnrichmentStrategy](./delete-enrichment-strategy.md). Omitted uses [CascadeDeleteEnrichmentStrategy](./cascade-delete-enrichment-strategy.md)
- `update` — [UpdateEnrichmentStrategy](./update-enrichment-strategy.md). Omitted uses [DefaultUpdateEnrichmentStrategy](./default-update-enrichment-strategy.md)
- `create` — [CreateEnrichmentStrategy](./create-enrichment-strategy.md). Omitted uses [DefaultCreateEnrichmentStrategy](./default-create-enrichment-strategy.md)
- `use_transaction` — commit the parent write and nested persist together

**Returns** [DefaultWriteEnrichmentStrategy](#defaultwriteenrichmentstrategy)

**Methods**

##### use_transaction()

When true, the parent write and nested persist commit together.

```rust
fn use_transaction(&self) -> bool
```

**Returns** `bool`

##### persist(tree, items, mode)

Walks each enrichment and writes children or links for `mode`.

```rust
async fn persist(&self, tree: &RepositoryTree, items: &mut [Row], mode: WriteMode) -> Result<()>
```

- `tree` — [RepositoryTree](./readonly-repository.md#repositorytree)
- `items` — parents to persist
- `mode` — [WriteMode](./i-write-enrichment-strategy.md#writemode)

**Returns** `Result<()>`
:::

## Remarks

Set it on the repository `write` option. Create, update, and delete each delegate to the strategies passed here. A null or empty list clears a link when this strategy is eager. [LazyWriteEnrichmentStrategy](./lazy-write-enrichment-strategy.md) skips that null list. Nested writes join the transaction when they use the same connection.

## Examples

### Roll back a bulk update

::: example
```csharp csharp/Datasources.Tests/BulkCRUDRepositoryTests.cs#BatchUpdateRollsBackWhenTransactionIsOn
```
```rust rust/datasources/tests/enriched.rs#bulk_update_rolls_back_when_transaction_is_on
```
```typescript typescript/datasources/src/BulkCRUDRepository.test.ts#rolls-back-a-bulk-update-when-usetransaction-is-on
```
:::
