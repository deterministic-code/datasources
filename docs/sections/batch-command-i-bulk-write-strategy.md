# BatchCommandIBulkWriteStrategy

Bulk write that joins one statement per row into a single batch, separated by `;`. Parameters are renamed so names do not collide.

## Classes

::: reference
### typescript
#### BatchCommandIBulkWriteStrategy

```typescript
class BatchCommandIBulkWriteStrategy<T extends object> implements IBulkWriteStrategy<T>
```

**Constructor**

##### new BatchCommandIBulkWriteStrategy()

Builds the batched statement strategy.

```typescript
constructor()
```

**Returns** [BatchCommandIBulkWriteStrategy](#batchcommandibulkwritestrategy)

**Methods**

##### create(items, repo)

Sends every insert as one batched command.

```typescript
create(items: T[], repo: BulkCRUDRepository<T>): Promise<Response>
```

- `items` — rows to insert
- `repo` — [BulkCRUDRepository](./bulk-repository.md)

**Returns** [Response](./command.md#response)

##### update(items, byWhere, repo)

Sends every update as one batched command.

```typescript
update(items: T[], byWhere: IWhere | undefined, repo: BulkCRUDRepository<T>): Promise<Response>
```

- `items` — rows to update
- `byWhere` — optional [IWhere](./where-clause.md) applied with each item
- `repo` — [BulkCRUDRepository](./bulk-repository.md)

**Returns** [Response](./command.md#response)

##### delete(items, byWhere, repo)

Sends every delete as one batched command.

```typescript
delete(items: T[], byWhere: IWhere | undefined, repo: BulkCRUDRepository<T>): Promise<Response>
```

- `items` — rows to delete
- `byWhere` — optional [IWhere](./where-clause.md) applied with each item
- `repo` — [BulkCRUDRepository](./bulk-repository.md)

**Returns** [Response](./command.md#response)

### csharp
#### BatchCommandIBulkWriteStrategy

```csharp
public sealed class BatchCommandIBulkWriteStrategy<T> : IBulkWriteStrategy<T> where T : new()
```

**Constructor**

##### BatchCommandIBulkWriteStrategy()

Builds the batched statement strategy.

```csharp
public BatchCommandIBulkWriteStrategy()
```

**Returns** [BatchCommandIBulkWriteStrategy](#batchcommandibulkwritestrategy)`<T>`

**Methods**

##### Create(items, repo)

Sends every insert as one batched command.

```csharp
public Task<Response> Create(IReadOnlyList<T> items, BulkCRUDRepository<T> repo)
```

- `items` — rows to insert
- `repo` — [BulkCRUDRepository](./bulk-repository.md)

**Returns** [Response](./command.md#response)

##### Update(items, byWhere, repo)

Sends every update as one batched command.

```csharp
public Task<Response> Update(IReadOnlyList<T> items, IWhere? byWhere, BulkCRUDRepository<T> repo)
```

- `items` — rows to update
- `byWhere` — optional [IWhere](./where-clause.md) applied with each item
- `repo` — [BulkCRUDRepository](./bulk-repository.md)

**Returns** [Response](./command.md#response)

##### Delete(items, byWhere, repo)

Sends every delete as one batched command.

```csharp
public Task<Response> Delete(IReadOnlyList<T> items, IWhere? byWhere, BulkCRUDRepository<T> repo)
```

- `items` — rows to delete
- `byWhere` — optional [IWhere](./where-clause.md) applied with each item
- `repo` — [BulkCRUDRepository](./bulk-repository.md)

**Returns** [Response](./command.md#response)

### rust
#### BatchCommandIBulkWriteStrategy

```rust
pub struct BatchCommandIBulkWriteStrategy
```

**Methods**

##### create(items, repo)

Sends every insert as one batched command.

```rust
async fn create(&self, items: &mut [Row], repo: &BulkCRUDRepository) -> Result<Response>
```

- `items` — rows to insert
- `repo` — [BulkCRUDRepository](./bulk-repository.md)

**Returns** `Result<Response>`

##### update(items, by_where, repo)

Sends every update as one batched command.

```rust
async fn update(&self, items: &[Row], by_where: Option<&dyn IWhere>, repo: &BulkCRUDRepository) -> Result<Response>
```

- `items` — rows to update
- `by_where` — optional [IWhere](./where-clause.md) applied with each item
- `repo` — [BulkCRUDRepository](./bulk-repository.md)

**Returns** `Result<Response>`

##### delete(items, by_where, repo)

Sends every delete as one batched command.

```rust
async fn delete(&self, items: &[Row], by_where: Option<&dyn IWhere>, repo: &BulkCRUDRepository) -> Result<Response>
```

- `items` — rows to delete
- `by_where` — optional [IWhere](./where-clause.md) applied with each item
- `repo` — [BulkCRUDRepository](./bulk-repository.md)

**Returns** `Result<Response>`
:::

## Remarks

An empty list does not execute. The provider must accept several statements in one command. Nested persist still runs through the repository write strategy, separate from this batch.

## Examples

### Join statements

::: example
```csharp csharp/Datasources.Tests/BulkWriteStrategyTests.cs#BatchCommandIBulkWriteStrategyJoinsStatements
```
```rust rust/datasources/src/tests/bulk_write_strategy.rs#batch_command_i_bulk_write_strategy_joins_statements
```
```typescript typescript/datasources/src/BulkWriteStrategy.test.ts#batch-commands-joins-statements
```
:::
