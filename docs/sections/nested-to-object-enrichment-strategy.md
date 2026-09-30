# NestedToObjectEnrichmentStrategy

Read fill that collapses join rows into objects. It ignores the related repository and maps `sources` with a [TypeMap](./flat-to-nested-mapper.md#typemap).

## Classes

::: reference
### typescript
#### NestedToObjectEnrichmentStrategy

```typescript
class NestedToObjectEnrichmentStrategy<T extends object> extends DefaultReadonlyEnrichmentStrategy<T>
```

**Constructor**

##### new NestedToObjectEnrichmentStrategy(typeMap)

Builds a collapse strategy from a [TypeMap](./flat-to-nested-mapper.md#typemap).

```typescript
constructor(typeMap: TypeMap<T>)
```

- `typeMap` — [TypeMap](./flat-to-nested-mapper.md#typemap) used to collapse join rows

**Returns** [NestedToObjectEnrichmentStrategy](#nestedtoobjectenrichmentstrategy)

**Methods**

##### apply(via, enrichment, parents, sources?)

Replaces `parents` with the objects collapsed from `sources`.

```typescript
apply<TRelated extends object>(via: IReadonlyRepository<TRelated>, enrichment: Enrichment, parents: object[], sources?: (Row | undefined)[]): Promise<void>
```

- `via` — unused
- `enrichment` — unused
- `parents` — replaced with the collapsed objects
- `sources` — join rows aligned with the read

**Returns** `Promise<void>`

### csharp
#### NestedToObjectEnrichmentStrategy

```csharp
public sealed class NestedToObjectEnrichmentStrategy<T> : DefaultReadonlyEnrichmentStrategy<T> where T : new()
```

**Constructor**

##### NestedToObjectEnrichmentStrategy(typeMap)

Builds a collapse strategy from a [TypeMap](./flat-to-nested-mapper.md#typemap).

```csharp
public NestedToObjectEnrichmentStrategy(TypeMap typeMap)
```

- `typeMap` — [TypeMap](./flat-to-nested-mapper.md#typemap) used to collapse join rows

**Returns** [NestedToObjectEnrichmentStrategy](#nestedtoobjectenrichmentstrategy)`<T>`

**Methods**

##### Apply(via, enrichment, parents, sources)

Replaces `parents` with the objects collapsed from `sources`.

```csharp
public override Task Apply<TRelated>(IReadonlyRepository<TRelated> via, Enrichment enrichment, IList<object> parents, IReadOnlyList<Row?>? sources = null)
```

- `via` — unused
- `enrichment` — unused
- `parents` — replaced with the collapsed objects
- `sources` — join rows aligned with the read

**Returns** `Task`

### rust
#### NestedToObjectEnrichmentStrategy

```rust
pub struct NestedToObjectEnrichmentStrategy
```

**Constructor**

##### new(type_map)

Builds a collapse strategy from a [TypeMap](./flat-to-nested-mapper.md#typemap).

```rust
pub fn new(type_map: TypeMap) -> Self
```

- `type_map` — [TypeMap](./flat-to-nested-mapper.md#typemap) used to collapse join rows

**Returns** [NestedToObjectEnrichmentStrategy](#nestedtoobjectenrichmentstrategy)

**Methods**

##### apply(via, enrichment, parents, sources)

Replaces `parents` with the objects collapsed from `sources`.

```rust
async fn apply(&self, via: Arc<dyn IReadonlyRepository>, enrichment: &Enrichment, parents: &mut Vec<Row>, sources: Option<&[Option<Row>]>) -> Result<()>
```

- `via` — unused
- `enrichment` — unused
- `parents` — replaced with the collapsed objects
- `sources` — join rows aligned with the read

**Returns** `Result<()>`
:::

## Remarks

This is the read-side collapse. [FlatToNestedMapper](./flat-to-nested-mapper.md) is the mapper. Pass this strategy to [IEnriched](./i-enriched.md) when the repository read is already a join. The related repository is not queried.

## Examples

### Collapse join rows

::: example
```csharp csharp/Datasources.Tests/EnrichedReadonlyRepositoryTests.cs#FlatToNestedCollapsesJoinRows
```
```rust rust/providers/sqlite/tests/readonly_repository.rs#fills_roles_from_a_join
```
```typescript typescript/datasources/src/EnrichedReadonlyRepository.test.ts#collapses-join-rows
```
:::
