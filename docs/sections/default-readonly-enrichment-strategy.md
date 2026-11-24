# DefaultReadonlyEnrichmentStrategy

Default read fill. Lookups, lists, and dictionaries load distinct parent keys with a chunked `IN` query. A link reads the junction, then each related row.

## Classes

::: reference
### typescript
#### DefaultReadonlyEnrichmentStrategy

```typescript
class DefaultReadonlyEnrichmentStrategy<T extends object> implements IReadonlyEnrichmentStrategy<T>
```

**Constructor**

##### new DefaultReadonlyEnrichmentStrategy()

Builds the default read strategy.

```typescript
constructor()
```

**Returns** [DefaultReadonlyEnrichmentStrategy](#defaultreadonlyenrichmentstrategy)

**Methods**

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

Reads the junction for each parent key, then each related row.

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
#### DefaultReadonlyEnrichmentStrategy

```csharp
public class DefaultReadonlyEnrichmentStrategy<T> : IReadonlyEnrichmentStrategy<T> where T : new()
```

**Constructor**

##### DefaultReadonlyEnrichmentStrategy()

Builds the default read strategy.

```csharp
public DefaultReadonlyEnrichmentStrategy()
```

**Returns** [DefaultReadonlyEnrichmentStrategy](#defaultreadonlyenrichmentstrategy)`<T>`

**Methods**

##### Apply(via, enrichment, parents, sources)

Fills lookups, lists, and dictionaries from chunked `IN` reads of the related repository.

```csharp
public virtual Task Apply<TRelated>(IReadonlyRepository<TRelated> via, Enrichment enrichment, IList<object> parents, IReadOnlyList<Row?>? sources = null)
```

- `via` — related [IReadonlyRepository](./readonly-repository.md)
- `enrichment` — [Enrichment](./enrichment.md)
- `parents` — parents to fill
- `sources` — optional source rows aligned with `parents`

**Returns** `Task`

##### ApplyLinked(via, link, enrichment, parents, sources)

Reads the junction for each parent key, then each related row.

```csharp
public virtual Task ApplyLinked<TRelated, TLink>(IReadonlyRepository<TRelated> via, IReadonlyRepository<TLink> link, Enrichment enrichment, IList<object> parents, IReadOnlyList<Row?>? sources = null)
```

- `via` — related [IReadonlyRepository](./readonly-repository.md)
- `link` — junction [IReadonlyRepository](./readonly-repository.md)
- `enrichment` — [Enrichment](./enrichment.md) with [Association](./enrichment.md#association)
- `parents` — parents to fill
- `sources` — optional source rows aligned with `parents`

**Returns** `Task`

### rust
#### DefaultReadonlyEnrichmentStrategy

```rust
pub struct DefaultReadonlyEnrichmentStrategy
```

Uses the default methods on [ReadonlyEnrichmentStrategy](./i-readonly-enrichment-strategy.md).

**Methods**

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

##### apply_linked(via, link, enrichment, parents, sources)

Reads the junction for each parent key, then each related row.

```rust
async fn apply_linked(&self, via: Arc<dyn IReadonlyRepository>, link: Arc<dyn IReadonlyRepository>, enrichment: &Enrichment, parents: &mut Vec<Row>, sources: Option<&[Option<Row>]>) -> Result<()>
```

- `via` — related [IReadonlyRepository](./readonly-repository.md)
- `link` — junction [IReadonlyRepository](./readonly-repository.md)
- `enrichment` — [Enrichment](./enrichment.md) with [association](./enrichment.md#association)
- `parents` — parents to fill
- `sources` — optional source rows aligned with `parents`

**Returns** `Result<()>`
:::

## Remarks

[IEnriched](./i-enriched.md) uses [LoadLevelOrderReadonlyEnrichmentStrategy](./load-level-order-readonly-enrichment-strategy.md) when the call omits a strategy. Pass this type to keep linked fills per-parent. Lookups, lists, and dictionaries still use chunked `IN` reads. Zero keys issue zero queries. A missing lookup is skipped. A missing list or dictionary is empty. [HashsetOptimizedReadonlyEnrichmentStrategy](./hashset-optimized-readonly-enrichment-strategy.md) batches lookups and lists without chunking.

## Examples

### Fill a lookup

::: example
```csharp csharp/Datasources.Tests/ReadonlyRepositoryTests.cs#ReadFillsLookupSelectAndLeavesMissingDefault
```
```rust rust/datasources/tests/enriched.rs#enriched_fills_lookup_select_and_leaves_missing_default
```
```typescript typescript/datasources/src/ReadonlyRepository.test.ts#fills-lookup-select-and-leaves-missing-default
```
:::
