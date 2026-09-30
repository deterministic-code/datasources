# HashsetOptimizedReadonlyEnrichmentStrategy

Read fill that loads every related row for the parent keys in one `IN` query, then assigns from that set. Lookups and lists both use that query.

## Classes

::: reference
### typescript
#### HashsetOptimizedReadonlyEnrichmentStrategy

```typescript
class HashsetOptimizedReadonlyEnrichmentStrategy<T extends object> extends DefaultReadonlyEnrichmentStrategy<T>
```

**Constructor**

##### new HashsetOptimizedReadonlyEnrichmentStrategy()

Builds the batched read strategy.

```typescript
constructor()
```

**Returns** [HashsetOptimizedReadonlyEnrichmentStrategy](#hashsetoptimizedreadonlyenrichmentstrategy)

**Methods**

##### apply(via, enrichment, parents, sources?)

Fills lookups and lists from one `IN` read of the related repository.

```typescript
apply<TRelated extends object>(via: IReadonlyRepository<TRelated>, enrichment: Enrichment, parents: object[], sources?: (Row | undefined)[]): Promise<void>
```

- `via` — related [IReadonlyRepository](./readonly-repository.md)
- `enrichment` — [Enrichment](./enrichment.md)
- `parents` — parents to fill
- `sources` — optional source rows aligned with `parents`

**Returns** `Promise<void>`

### csharp
#### HashsetOptimizedReadonlyEnrichmentStrategy

```csharp
public sealed class HashsetOptimizedReadonlyEnrichmentStrategy<T> : DefaultReadonlyEnrichmentStrategy<T> where T : new()
```

**Constructor**

##### HashsetOptimizedReadonlyEnrichmentStrategy()

Builds the batched read strategy.

```csharp
public HashsetOptimizedReadonlyEnrichmentStrategy()
```

**Returns** [HashsetOptimizedReadonlyEnrichmentStrategy](#hashsetoptimizedreadonlyenrichmentstrategy)`<T>`

**Methods**

##### Apply(via, enrichment, parents, sources)

Inherited from [DefaultReadonlyEnrichmentStrategy](./default-readonly-enrichment-strategy.md). Lookups and lists each use one `IN` read of the related repository.

```csharp
public virtual Task Apply<TRelated>(IReadonlyRepository<TRelated> via, Enrichment enrichment, IList<object> parents, IReadOnlyList<Row?>? sources = null)
```

- `via` — related [IReadonlyRepository](./readonly-repository.md)
- `enrichment` — [Enrichment](./enrichment.md)
- `parents` — parents to fill
- `sources` — optional source rows aligned with `parents`

**Returns** `Task`

### rust
#### HashsetOptimizedReadonlyEnrichmentStrategy

```rust
pub struct HashsetOptimizedReadonlyEnrichmentStrategy
```

**Methods**

##### apply(via, enrichment, parents, sources)

Fills lookups and lists from one `IN` read of the related repository.

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

Pass it as the strategy on [IEnriched](./i-enriched.md). Linked fills stay on [DefaultReadonlyEnrichmentStrategy](./default-readonly-enrichment-strategy.md). A parent with no key is skipped. A key with no related row leaves a lookup unset and a list empty.

## Examples

### Fill with one query

::: example
```csharp csharp/Datasources.Tests/EnrichedReadonlyRepositoryTests.cs#HashsetFillsLookupSelectAndLeavesMissingDefault
```
```rust rust/providers/sqlite/tests/readonly_repository.rs#fills_roles_with_a_hashset
```
```typescript typescript/datasources/src/EnrichedReadonlyRepository.test.ts#hashset-fills-lookup-select-and-leaves-missing-default
```
:::
