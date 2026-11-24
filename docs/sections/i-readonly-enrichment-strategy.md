# IReadonlyEnrichmentStrategy

Fills each parent from a related repository. [IEnriched](./i-enriched.md) uses one strategy per edge. Omitted uses [LoadLevelOrderReadonlyEnrichmentStrategy](./load-level-order-readonly-enrichment-strategy.md).

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
apply<TRelated extends object>(via: IReadonlyRepository<TRelated>, enrichment: Enrichment, parents: T[], sources?: (Row | undefined)[]): Promise<void>
```

- `via` — related [IReadonlyRepository](./readonly-repository.md)
- `enrichment` — [Enrichment](./enrichment.md)
- `parents` — parents to fill
- `sources` — optional source rows aligned with `parents`

**Returns** `Promise<void>`

##### applyLinked(via, link, enrichment, parents, sources?)

Fills each parent by reading the link table, then the related repository.

```typescript
applyLinked<TRelated extends object, TLink extends object>(via: IReadonlyRepository<TRelated>, link: IReadonlyRepository<TLink>, enrichment: Enrichment, parents: T[], sources?: (Row | undefined)[]): Promise<void>
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

Fills each parent from the related repository. The default dispatches to `apply_object`, `apply_list`, or `apply_dictionary`.

```rust
async fn apply(&self, via: Arc<dyn IReadonlyRepository>, enrichment: &Enrichment, parents: &mut Vec<Row>, sources: Option<&[Option<Row>]>) -> Result<()>
```

- `via` — related [IReadonlyRepository](./readonly-repository.md)
- `enrichment` — [Enrichment](./enrichment.md)
- `parents` — parents to fill
- `sources` — optional source rows aligned with `parents`

**Returns** `Result<()>`

##### apply_object(via, enrichment, parents, sources)

Fills a scalar related value. The default loads distinct keys with a chunked `IN` query.

```rust
async fn apply_object(&self, via: Arc<dyn IReadonlyRepository>, enrichment: &Enrichment, parents: &mut Vec<Row>, sources: Option<&[Option<Row>]>) -> Result<()>
```

- `via` — related [IReadonlyRepository](./readonly-repository.md)
- `enrichment` — [Enrichment](./enrichment.md)
- `parents` — parents to fill
- `sources` — optional source rows aligned with `parents`

**Returns** `Result<()>`

##### apply_list(via, enrichment, parents, sources)

Fills a list. The default loads distinct keys with a chunked `IN` query.

```rust
async fn apply_list(&self, via: Arc<dyn IReadonlyRepository>, enrichment: &Enrichment, parents: &mut Vec<Row>, sources: Option<&[Option<Row>]>) -> Result<()>
```

- `via` — related [IReadonlyRepository](./readonly-repository.md)
- `enrichment` — [Enrichment](./enrichment.md)
- `parents` — parents to fill
- `sources` — optional source rows aligned with `parents`

**Returns** `Result<()>`

##### apply_dictionary(via, enrichment, parents, sources)

Fills a dictionary. The default loads distinct keys with a chunked `IN` query.

```rust
async fn apply_dictionary(&self, via: Arc<dyn IReadonlyRepository>, enrichment: &Enrichment, parents: &mut Vec<Row>, sources: Option<&[Option<Row>]>) -> Result<()>
```

- `via` — related [IReadonlyRepository](./readonly-repository.md)
- `enrichment` — [Enrichment](./enrichment.md)
- `parents` — parents to fill
- `sources` — optional source rows aligned with `parents`

**Returns** `Result<()>`

##### linked_batch_size()

When set, linked fills use that key batch size for two `IN` reads. Omitted keeps the per-parent default. The trait stays dyn-compatible; C# and TypeScript override {{ApplyLinked|applyLinked|apply_linked}} instead.

```rust
fn linked_batch_size(&self) -> Option<usize>
```

**Returns** `Option<usize>`

##### fill_batch_size()

Maximum keys per `IN` list for lookups, lists, and dictionaries. Default is 500.

```rust
fn fill_batch_size(&self) -> usize
```

**Returns** `usize`
:::

## Remarks

Pass the strategy to [IEnriched](./i-enriched.md) for one edge. The repository calls {{Apply|apply|apply}} after it reads the parents. {{ApplyLinked|applyLinked|apply_linked}} runs for an enrichment that has an [association](./enrichment.md#association). A `List` is walked. A `RelatedProperty` scalar is not. Lookups, lists, and dictionaries use chunked `IN` reads. Linked fills stay per-parent unless [LoadLevelOrderReadonlyEnrichmentStrategy](./load-level-order-readonly-enrichment-strategy.md) is chosen. Rust `fill_batch_size` and `linked_batch_size` set those chunk sizes.

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
