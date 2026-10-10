# BatchCommandIBulkWriteStrategy

Bulk write that joins one statement per row into a single batch, separated by `;`. Parameters are renamed so names do not collide. The orchestrator assigns `Repo`, `Command`, and `Options`.

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

**Properties**

##### repo

```typescript
repo: CRUDRepository<T>
```

##### command

Middleware-wrapped command used to `execute` the joined script.

```typescript
command: ICommand
```

##### options

Table name, ids, and converters used to build INSERT, UPDATE, and DELETE.

```typescript
options: CRUDRepositoryOptions<T>
```

**Methods**

##### create(items)

Sends every insert as one batched command.

```typescript
create(items: T[]): Promise<Response>
```

- `items` — rows to insert

**Returns** [Response](./command.md#response)

##### update(items, byWhere)

Sends every update as one batched command.

```typescript
update(items: T[], byWhere: IWhere | undefined): Promise<Response>
```

- `items` — rows to update
- `byWhere` — optional [IWhere](./where-clause.md) applied with each item

**Returns** [Response](./command.md#response)

##### delete(items, byWhere)

Sends every delete as one batched command.

```typescript
delete(items: T[], byWhere: IWhere | undefined): Promise<Response>
```

- `items` — rows to delete
- `byWhere` — optional [IWhere](./where-clause.md) applied with each item

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

**Properties**

##### Repo

```csharp
public CRUDRepository<T> Repo { get; set; }
```

##### Command

```csharp
public ICommand Command { get; set; }
```

##### Options

```csharp
public CRUDRepositoryOptions<T> Options { get; set; }
```

**Methods**

##### Create(items)

Sends every insert as one batched command.

```csharp
public Task<Response> Create(IReadOnlyList<T> items)
```

- `items` — rows to insert

**Returns** [Response](./command.md#response)

##### Update(items, byWhere)

Sends every update as one batched command.

```csharp
public Task<Response> Update(IReadOnlyList<T> items, IWhere? byWhere)
```

- `items` — rows to update
- `byWhere` — optional [IWhere](./where-clause.md) applied with each item

**Returns** [Response](./command.md#response)

##### Delete(items, byWhere)

Sends every delete as one batched command.

```csharp
public Task<Response> Delete(IReadOnlyList<T> items, IWhere? byWhere)
```

- `items` — rows to delete
- `byWhere` — optional [IWhere](./where-clause.md) applied with each item

**Returns** [Response](./command.md#response)

### rust
#### BatchCommandIBulkWriteStrategy

```rust
pub struct BatchCommandIBulkWriteStrategy
```

**Methods**

##### create(items)

Sends every insert as one batched command.

```rust
async fn create(&self, items: &mut [Row]) -> Result<Response>
```

- `items` — rows to insert

**Returns** `Result<Response>`

##### update(items, by_where)

Sends every update as one batched command.

```rust
async fn update(&self, items: &[Row], by_where: Option<&dyn IWhere>) -> Result<Response>
```

- `items` — rows to update
- `by_where` — optional [IWhere](./where-clause.md) applied with each item

**Returns** `Result<Response>`

##### delete(items, by_where)

Sends every delete as one batched command.

```rust
async fn delete(&self, items: &[Row], by_where: Option<&dyn IWhere>) -> Result<Response>
```

- `items` — rows to delete
- `by_where` — optional [IWhere](./where-clause.md) applied with each item

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
