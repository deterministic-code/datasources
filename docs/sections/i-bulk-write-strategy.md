# IBulkWriteStrategy

Writes a list of rows for a [BulkCRUDRepository](./bulk-repository.md). Create, update, and delete each receive the items and the repository. The default is [SingleCommandIBulkWriteStrategy](./single-command-i-bulk-write-strategy.md).

## Classes

::: reference
### typescript
#### IBulkWriteStrategy

```typescript
interface IBulkWriteStrategy<T extends object>
```

**Methods**

##### create(items, repo)

Inserts `items`.

```typescript
create(items: T[], repo: BulkCRUDRepository<T>): Promise<Response>
```

- `items` — rows to insert
- `repo` — [BulkCRUDRepository](./bulk-repository.md)

**Returns** [Response](./command.md#response)

##### update(items, byWhere, repo)

Updates `items`.

```typescript
update(items: T[], byWhere: IWhere | undefined, repo: BulkCRUDRepository<T>): Promise<Response>
```

- `items` — rows to update
- `byWhere` — optional [IWhere](./where-clause.md) applied with each item
- `repo` — [BulkCRUDRepository](./bulk-repository.md)

**Returns** [Response](./command.md#response)

##### delete(items, byWhere, repo)

Deletes `items`.

```typescript
delete(items: T[], byWhere: IWhere | undefined, repo: BulkCRUDRepository<T>): Promise<Response>
```

- `items` — rows to delete
- `byWhere` — optional [IWhere](./where-clause.md) applied with each item
- `repo` — [BulkCRUDRepository](./bulk-repository.md)

**Returns** [Response](./command.md#response)

### csharp
#### IBulkWriteStrategy

```csharp
public interface IBulkWriteStrategy<T> where T : new()
```

**Methods**

##### Create(items, repo)

Inserts `items`.

```csharp
Task<Response> Create(IReadOnlyList<T> items, BulkCRUDRepository<T> repo)
```

- `items` — rows to insert
- `repo` — [BulkCRUDRepository](./bulk-repository.md)

**Returns** [Response](./command.md#response)

##### Update(items, byWhere, repo)

Updates `items`.

```csharp
Task<Response> Update(IReadOnlyList<T> items, IWhere? byWhere, BulkCRUDRepository<T> repo)
```

- `items` — rows to update
- `byWhere` — optional [IWhere](./where-clause.md) applied with each item
- `repo` — [BulkCRUDRepository](./bulk-repository.md)

**Returns** [Response](./command.md#response)

##### Delete(items, byWhere, repo)

Deletes `items`.

```csharp
Task<Response> Delete(IReadOnlyList<T> items, IWhere? byWhere, BulkCRUDRepository<T> repo)
```

- `items` — rows to delete
- `byWhere` — optional [IWhere](./where-clause.md) applied with each item
- `repo` — [BulkCRUDRepository](./bulk-repository.md)

**Returns** [Response](./command.md#response)

### rust
#### BulkWriteStrategy

```rust
pub trait BulkWriteStrategy: Send + Sync
```

**Methods**

##### create(items, repo)

Inserts `items`.

```rust
async fn create(&self, items: &mut [Row], repo: &BulkCRUDRepository) -> Result<Response>
```

- `items` — rows to insert
- `repo` — [BulkCRUDRepository](./bulk-repository.md)

**Returns** `Result<Response>`

##### update(items, by_where, repo)

Updates `items`.

```rust
async fn update(&self, items: &[Row], by_where: Option<&dyn IWhere>, repo: &BulkCRUDRepository) -> Result<Response>
```

- `items` — rows to update
- `by_where` — optional [IWhere](./where-clause.md) applied with each item
- `repo` — [BulkCRUDRepository](./bulk-repository.md)

**Returns** `Result<Response>`

##### delete(items, by_where, repo)

Deletes `items`.

```rust
async fn delete(&self, items: &[Row], by_where: Option<&dyn IWhere>, repo: &BulkCRUDRepository) -> Result<Response>
```

- `items` — rows to delete
- `by_where` — optional [IWhere](./where-clause.md) applied with each item
- `repo` — [BulkCRUDRepository](./bulk-repository.md)

**Returns** `Result<Response>`
:::

## Remarks

Pass the strategy to the bulk repository constructor. [SingleCommandIBulkWriteStrategy](./single-command-i-bulk-write-strategy.md) runs one statement per row. [BatchCommandIBulkWriteStrategy](./batch-command-i-bulk-write-strategy.md) joins those statements. [StoredProcedureBulkWriteStrategy](./stored-procedure-bulk-write-strategy.md) sends one JSON payload. An empty list does not execute. Nested persist still follows the repository write strategy.

## Examples

### One statement per row

::: example
```csharp csharp/Datasources.Tests/BulkWriteStrategyTests.cs#SingleCommandIBulkWriteStrategyWritesOneStatementPerRow
```
```rust rust/datasources/src/tests/bulk_write_strategy.rs#single_command_i_bulk_write_strategy_writes_one_statement_per_row
```
```typescript typescript/datasources/src/BulkWriteStrategy.test.ts#single-commands-writes-one-statement-per-row
```
:::
