# IEnriched

Registers a related fill on a repository. Fluent for siblings. A related repository fills its own fields. Does not write.

## Classes

::: reference
### typescript
#### IEnriched

```typescript
interface IEnriched<T>
```

**Methods**

##### addEnriched(enrichment, via, strategy?)

Adds a related fill. Fluent for siblings. A related repository fills its own fields. Does not write.

```typescript
addEnriched<TRelated extends object>(enrichment: Enrichment, via: IReadonlyRepository<TRelated>, strategy?: IReadonlyEnrichmentStrategy<T & object>): this
```

- `enrichment` — [Enrichment](./enrichment.md)
- `via` — related [IReadonlyRepository](./readonly-repository.md)
- `strategy` — optional [IReadonlyEnrichmentStrategy](./i-readonly-enrichment-strategy.md) for this edge. Omitted uses [DefaultReadonlyEnrichmentStrategy](./default-readonly-enrichment-strategy.md)

**Returns** `this`

### csharp
#### IEnriched

```csharp
public interface IEnriched<T>
```

**Methods**

##### AddEnriched(enrichment, via, strategy)

Adds a related fill. Fluent for siblings. A related repository fills its own fields. Does not write.

```csharp
IEnriched<T> AddEnriched<TRelated>(Enrichment enrichment, IReadonlyRepository<TRelated> via, IReadonlyEnrichmentStrategy<T>? strategy = null)
```

- `enrichment` — [Enrichment](./enrichment.md)
- `via` — related [IReadonlyRepository](./readonly-repository.md)
- `strategy` — optional [IReadonlyEnrichmentStrategy](./i-readonly-enrichment-strategy.md) for this edge. Omitted uses [DefaultReadonlyEnrichmentStrategy](./default-readonly-enrichment-strategy.md)

**Returns** [IEnriched](#ienriched)`<T>`

### rust
#### ReadonlyRepository

```rust
pub struct ReadonlyRepository
```

**Methods**

##### add_enriched(enrichment, via, strategy)

Adds a related fill. A related repository fills its own fields. Does not write.

```rust
pub fn add_enriched(&mut self, enrichment: Enrichment, via: impl EnrichmentVia, strategy: Option<Arc<dyn ReadonlyEnrichmentStrategy>>) -> &mut Self
```

- `enrichment` — [Enrichment](./enrichment.md)
- `via` — related [IReadonlyRepository](./readonly-repository.md). The readonly path keeps the read repository only
- `strategy` — optional [ReadonlyEnrichmentStrategy](./i-readonly-enrichment-strategy.md) for this edge. Omitted uses [DefaultReadonlyEnrichmentStrategy](./default-readonly-enrichment-strategy.md)

**Returns** `&mut Self`
:::

## Remarks

Register the child repository's own fills on that repository:

```
childrenRepo.AddEnriched(new("type_name", "type_id", "id", "name"), childTypesRepo);
parentsRepo.AddEnriched(new("children", "id", "parent_id", Many: true), childrenRepo);
```

Rust has no separate trait. {{AddEnriched|addEnriched|add_enriched}} is a method on [ReadonlyRepository](./readonly-repository.md) and [CRUDRepository](./crud-repository.md). With no enrichments, {{Read|read|read}} stays streaming. With enrichments, the read is collected, filled, then yielded. Writes, including `many` children and associations, are [CRUDRepository](./crud-repository.md).

## Examples

### Register a fill

::: example
```csharp csharp/Datasources.Tests/EnrichedReadonlyRepositoryTests.cs#ReadFillsLookupSelectAndLeavesMissingDefault
```
```rust rust/datasources/tests/enriched.rs#enriched_fills_lookup_select_and_leaves_missing_default
```
```typescript typescript/datasources/src/EnrichedReadonlyRepository.test.ts#fills-lookup-select-and-leaves-missing-default
```
:::
