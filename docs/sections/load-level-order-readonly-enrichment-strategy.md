# LoadLevelOrderReadonlyEnrichmentStrategy

Read fill that loads each enrichment level with one chunked `IN` query per edge, then assigns from that set. Lookups, lists, dictionaries, and links all use that query. Nested fills on the related repository run on the whole batched result, so a parent, child, and grandchild chain is one query per level.

## Classes

::: reference
### typescript
#### LoadLevelOrderReadonlyEnrichmentStrategy

```typescript
class LoadLevelOrderReadonlyEnrichmentStrategy<T extends object> extends DefaultReadonlyEnrichmentStrategy<T>
```

**Constructor**

##### new LoadLevelOrderReadonlyEnrichmentStrategy(batchSize?)

Builds the level-order read strategy. Default batch size is 500 keys.

```typescript
constructor(batchSize?: number)
```

- `batchSize` — optional; maximum keys per `IN` list

**Returns** [LoadLevelOrderReadonlyEnrichmentStrategy](#loadlevelorderreadonlyenrichmentstrategy)

**Methods**

##### withBatchSize(batchSize)

Returns a strategy that chunks keys into lists of this size.

```typescript
withBatchSize(batchSize: number): LoadLevelOrderReadonlyEnrichmentStrategy<T>
```

- `batchSize` — maximum keys per `IN` list

**Returns** [LoadLevelOrderReadonlyEnrichmentStrategy](#loadlevelorderreadonlyenrichmentstrategy)

##### apply(via, enrichment, parents, sources?)

Fills lookups, lists, and dictionaries from chunked `IN` reads of the related repository.

```typescript
apply<TRelated extends object>(via: IReadonlyRepository<TRelated>, enrichment: Enrichment, parents: T[], sources?: (Row | undefined)[]): Promise<void>
```

- `via` — related [IReadonlyRepository](./readonly-repository.md)
- `enrichment` — [Enrichment](./enrichment.md)
- `parents` — parents to fill
- `sources` — optional source rows aligned with `parents`

**Returns** `Promise<void>`

##### applyLinked(via, link, enrichment, parents, sources?)

Fills links with one `IN` read of the junction and one `IN` read of the related repository.

```typescript
applyLinked<TRelated extends object, TLink extends object>(via: IReadonlyRepository<TRelated>, link: IReadonlyRepository<TLink>, enrichment: Enrichment, parents: T[], sources?: (Row | undefined)[]): Promise<void>
```

- `via` — related [IReadonlyRepository](./readonly-repository.md)
- `link` — junction [IReadonlyRepository](./readonly-repository.md)
- `enrichment` — [Enrichment](./enrichment.md) with [association](./enrichment.md#association)
- `parents` — parents to fill
- `sources` — optional source rows aligned with `parents`

**Returns** `Promise<void>`

### csharp
#### LoadLevelOrderReadonlyEnrichmentStrategy

```csharp
public class LoadLevelOrderReadonlyEnrichmentStrategy<T> : DefaultReadonlyEnrichmentStrategy<T> where T : new()
```

**Constructor**

##### LoadLevelOrderReadonlyEnrichmentStrategy()

Builds the level-order read strategy. Default batch size is 500 keys.

```csharp
public LoadLevelOrderReadonlyEnrichmentStrategy()
```

**Returns** [LoadLevelOrderReadonlyEnrichmentStrategy](#loadlevelorderreadonlyenrichmentstrategy)`<T>`

##### LoadLevelOrderReadonlyEnrichmentStrategy(batchSize)

Builds the level-order read strategy with this key batch size.

```csharp
public LoadLevelOrderReadonlyEnrichmentStrategy(int batchSize)
```

- `batchSize` — maximum keys per `IN` list

**Returns** [LoadLevelOrderReadonlyEnrichmentStrategy](#loadlevelorderreadonlyenrichmentstrategy)`<T>`

**Methods**

##### WithBatchSize(batchSize)

Returns a strategy that chunks keys into lists of this size.

```csharp
public LoadLevelOrderReadonlyEnrichmentStrategy<T> WithBatchSize(int batchSize)
```

- `batchSize` — maximum keys per `IN` list

**Returns** [LoadLevelOrderReadonlyEnrichmentStrategy](#loadlevelorderreadonlyenrichmentstrategy)`<T>`

##### Apply(via, enrichment, parents, sources)

Inherited from [DefaultReadonlyEnrichmentStrategy](./default-readonly-enrichment-strategy.md). Lookups, lists, and dictionaries each use chunked `IN` reads of the related repository.

```csharp
public virtual Task Apply<TRelated>(IReadonlyRepository<TRelated> via, Enrichment enrichment, IList<object> parents, IReadOnlyList<Row?>? sources = null)
```

- `via` — related [IReadonlyRepository](./readonly-repository.md)
- `enrichment` — [Enrichment](./enrichment.md)
- `parents` — parents to fill
- `sources` — optional source rows aligned with `parents`

**Returns** `Task`

##### ApplyLinked(via, link, enrichment, parents, sources)

Fills links with one `IN` read of the junction and one `IN` read of the related repository.

```csharp
public override Task ApplyLinked<TRelated, TLink>(IReadonlyRepository<TRelated> via, IReadonlyRepository<TLink> link, Enrichment enrichment, IList<object> parents, IReadOnlyList<Row?>? sources = null)
```

- `via` — related [IReadonlyRepository](./readonly-repository.md)
- `link` — junction [IReadonlyRepository](./readonly-repository.md)
- `enrichment` — [Enrichment](./enrichment.md) with [association](./enrichment.md#association)
- `parents` — parents to fill
- `sources` — optional source rows aligned with `parents`

**Returns** `Task`

### rust
#### LoadLevelOrderReadonlyEnrichmentStrategy

```rust
pub struct LoadLevelOrderReadonlyEnrichmentStrategy
```

**Constructor**

##### new()

Builds the level-order read strategy. Default batch size is 500 keys.

```rust
pub fn new() -> Self
```

**Returns** [LoadLevelOrderReadonlyEnrichmentStrategy](#loadlevelorderreadonlyenrichmentstrategy)

**Methods**

##### with_batch_size(batch_size)

Returns a strategy that chunks keys into lists of this size.

```rust
pub fn with_batch_size(self, batch_size: usize) -> Self
```

- `batch_size` — maximum keys per `IN` list

**Returns** [LoadLevelOrderReadonlyEnrichmentStrategy](#loadlevelorderreadonlyenrichmentstrategy)

##### apply(via, enrichment, parents, sources)

Fills lookups, lists, and dictionaries from chunked `IN` reads of the related repository.

```rust
async fn apply(&self, via: Arc<dyn IReadonlyRepository>, enrichment: &Enrichment, parents: &mut Vec<Row>, sources: Option<&[Option<Row>]>) -> Result<()>
```

- `via` — related [IReadonlyRepository](./readonly-repository.md)
- `enrichment` — [Enrichment](./enrichment.md)
- `parents` — parents to fill
- `sources` — optional source rows aligned with `parents`

**Returns** `Result<()>`
:::

## Remarks

Pass it as the strategy on [IEnriched](./i-enriched.md). Zero keys issue zero queries. A missing lookup is skipped. A missing list, dictionary, or link is empty. Composite keys use `IN` on the first pair, then match every pair in memory. Order within a group is the order the database returns rows. [HashsetOptimizedReadonlyEnrichmentStrategy](./hashset-optimized-readonly-enrichment-strategy.md) batches only lookups and lists, and does not chunk. Tree views keep TreeReadonlyEnrichmentStrategy.

## Examples

### Fill each level in one query

::: example
```csharp csharp/Datasources.Tests/LoadLevelOrderReadonlyEnrichmentStrategyTests.cs#OneHundredParentsIsOneReadPerEnrichment
```
```rust rust/datasources/tests/load_level_order.rs#one_hundred_parents_is_one_read_per_enrichment
```
```typescript typescript/datasources/src/LoadLevelOrderReadonlyEnrichmentStrategy.test.ts#one-hundred-parents-is-one-read-per-enrichment
```
:::
