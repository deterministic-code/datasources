# IReadonlyEnrichmentStrategy

Fills each parent from a related repository. [IEnriched](./i-enriched.md) uses one strategy per edge. Omitted uses [DefaultReadonlyEnrichmentStrategy](./default-readonly-enrichment-strategy.md).

## Classes

::: reference
### typescript
#### IReadonlyEnrichmentStrategy

```typescript
interface IReadonlyEnrichmentStrategy<T extends object>
```

**Methods**

##### apply(via, enrichment, parents, sources?)

Fills each parent from the related repository.

```typescript
apply<TRelated extends object>(via: IReadonlyRepository<TRelated>, enrichment: Enrichment, parents: object[], sources?: (Row | undefined)[]): Promise<void>
```

- `via` — related [IReadonlyRepository](./readonly-repository.md)
- `enrichment` — [Enrichment](./enrichment.md)
- `parents` — parents to fill
- `sources` — optional source rows aligned with `parents`

**Returns** `Promise<void>`

##### applyLinked(via, link, enrichment, parents, sources?)

Fills each parent by reading the link table, then the related repository.

```typescript
applyLinked<TRelated extends object, TLink extends object>(via: IReadonlyRepository<TRelated>, link: IReadonlyRepository<TLink>, enrichment: Enrichment, parents: object[], sources?: (Row | undefined)[]): Promise<void>
```

- `via` — related [IReadonlyRepository](./readonly-repository.md)
- `link` — junction [IReadonlyRepository](./readonly-repository.md)
- `enrichment` — [Enrichment](./enrichment.md)
- `parents` — parents to fill
- `sources` — optional source rows aligned with `parents`

**Returns** `Promise<void>`

### csharp
#### IReadonlyEnrichmentStrategy

```csharp
public interface IReadonlyEnrichmentStrategy<T>
```

**Methods**

##### Apply(via, enrichment, parents, sources)

Fills each parent from the related repository.

```csharp
Task Apply<TRelated>(IReadonlyRepository<TRelated> via, Enrichment enrichment, IList<object> parents, IReadOnlyList<Row?>? sources = null)
```

- `via` — related [IReadonlyRepository](./readonly-repository.md)
- `enrichment` — [Enrichment](./enrichment.md)
- `parents` — parents to fill
- `sources` — optional source rows aligned with `parents`

**Returns** `Task`

##### ApplyLinked(via, link, enrichment, parents, sources)

Fills each parent by reading the link table, then the related repository.

```csharp
Task ApplyLinked<TRelated, TLink>(IReadonlyRepository<TRelated> via, IReadonlyRepository<TLink> link, Enrichment enrichment, IList<object> parents, IReadOnlyList<Row?>? sources = null)
```

- `via` — related [IReadonlyRepository](./readonly-repository.md)
- `link` — junction [IReadonlyRepository](./readonly-repository.md)
- `enrichment` — [Enrichment](./enrichment.md)
- `parents` — parents to fill
- `sources` — optional source rows aligned with `parents`

**Returns** `Task`

### rust
#### ReadonlyEnrichmentStrategy

```rust
pub trait ReadonlyEnrichmentStrategy: Send + Sync
```

**Methods**

##### apply(via, enrichment, parents, sources)

Fills each parent from the related repository. The default dispatches to `apply_object` or `apply_list`.

```rust
async fn apply(&self, via: Arc<dyn IReadonlyRepository>, enrichment: &Enrichment, parents: &mut Vec<Row>, sources: Option<&[Option<Row>]>) -> Result<()>
```

- `via` — related [IReadonlyRepository](./readonly-repository.md)
- `enrichment` — [Enrichment](./enrichment.md)
- `parents` — parents to fill
- `sources` — optional source rows aligned with `parents`

**Returns** `Result<()>`

##### apply_object(via, enrichment, parents, sources)

Fills a scalar related value. The default reads one related row per distinct key.

```rust
async fn apply_object(&self, via: Arc<dyn IReadonlyRepository>, enrichment: &Enrichment, parents: &mut Vec<Row>, sources: Option<&[Option<Row>]>) -> Result<()>
```

- `via` — related [IReadonlyRepository](./readonly-repository.md)
- `enrichment` — [Enrichment](./enrichment.md)
- `parents` — parents to fill
- `sources` — optional source rows aligned with `parents`

**Returns** `Result<()>`

##### apply_list(via, enrichment, parents, sources)

Fills a list. The default reads the related rows for each parent key.

```rust
async fn apply_list(&self, via: Arc<dyn IReadonlyRepository>, enrichment: &Enrichment, parents: &mut Vec<Row>, sources: Option<&[Option<Row>]>) -> Result<()>
```

- `via` — related [IReadonlyRepository](./readonly-repository.md)
- `enrichment` — [Enrichment](./enrichment.md)
- `parents` — parents to fill
- `sources` — optional source rows aligned with `parents`

**Returns** `Result<()>`

##### apply_linked(via, link, enrichment, parents, sources)

Fills each parent by reading the link table, then the related repository.

```rust
async fn apply_linked(&self, via: Arc<dyn IReadonlyRepository>, link: Arc<dyn IReadonlyRepository>, enrichment: &Enrichment, parents: &mut Vec<Row>, sources: Option<&[Option<Row>]>) -> Result<()>
```

- `via` — related [IReadonlyRepository](./readonly-repository.md)
- `link` — junction [IReadonlyRepository](./readonly-repository.md)
- `enrichment` — [Enrichment](./enrichment.md)
- `parents` — parents to fill
- `sources` — optional source rows aligned with `parents`

**Returns** `Result<()>`
:::

## Remarks

Pass the strategy to [IEnriched](./i-enriched.md) for one edge. The repository calls {{Apply|apply|apply}} after it reads the parents. {{ApplyLinked|applyLinked|apply_linked}} runs for an enrichment that has an [association](./enrichment.md#association). A `List` is walked. A `RelatedProperty` scalar is not. C# and TypeScript override the object and list fills on [DefaultReadonlyEnrichmentStrategy](./default-readonly-enrichment-strategy.md). Rust overrides `apply_object` and `apply_list` on this trait.

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
