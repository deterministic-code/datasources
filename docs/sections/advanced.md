# Advanced

Strategies that fill related fields on read and write nested rows or bulk lists. Repositories call these. A repository page covers the call. This group covers the strategy.

## Enrichment

- [IEnriched](./i-enriched.md) — registers a related fill
- [Enrichment](./enrichment.md) — one related fill
- [IReadonlyEnrichmentStrategy](./i-readonly-enrichment-strategy.md) — fills parents on read
- [DefaultReadonlyEnrichmentStrategy](./default-readonly-enrichment-strategy.md) — reads each related key, and caches lookups
- [HashsetOptimizedReadonlyEnrichmentStrategy](./hashset-optimized-readonly-enrichment-strategy.md) — one `IN` query for the key set
- [NestedToObjectEnrichmentStrategy](./nested-to-object-enrichment-strategy.md) — collapses join rows

## Write

- [IWriteEnrichmentStrategy](./i-write-enrichment-strategy.md) — writes nested rows with the parent
- [DefaultWriteEnrichmentStrategy](./default-write-enrichment-strategy.md) — cascade delete and replace-sync, with an optional transaction
- [LazyWriteEnrichmentStrategy](./lazy-write-enrichment-strategy.md) — skips a null list on create and update
- [UpdateEnrichmentStrategy](./update-enrichment-strategy.md) — how an update writes children and links
- [DefaultUpdateEnrichmentStrategy](./default-update-enrichment-strategy.md) — replace-syncs each many list
- [CascadeDeleteUpdateEnrichmentStrategy](./cascade-delete-update-enrichment-strategy.md) — deletes children, then inserts the payload
- [DeleteEnrichmentStrategy](./delete-enrichment-strategy.md) — how a delete removes children and links
- [CascadeDeleteEnrichmentStrategy](./cascade-delete-enrichment-strategy.md) — deletes nested rows, then children, then the parent

## Bulk write

- [IBulkWriteStrategy](./i-bulk-write-strategy.md) — writes a list through a [Bulk repository](./bulk-repository.md)
- [SingleCommandIBulkWriteStrategy](./single-command-i-bulk-write-strategy.md) — one statement per row
- [BatchCommandIBulkWriteStrategy](./batch-command-i-bulk-write-strategy.md) — joins the statements
- [StoredProcedureBulkWriteStrategy](./stored-procedure-bulk-write-strategy.md) — one JSON call to a stored procedure
