# IBulkWriteStrategy

Writes a list of rows for a [BulkCRUDRepository](./bulk-repository.md). Create, update, and delete each receive the items. The repository sets `Repo` after construction. The default is [SingleCommandIBulkWriteStrategy](./single-command-i-bulk-write-strategy.md).

## Classes

::: reference
### typescript
#### IBulkWriteStrategy

```typescript
interface IBulkWriteStrategy<T extends object>
```

**Properties**

##### repo

[CRUDRepository](./crud-repository.md) the orchestrator assigns after `base(...)`.

```typescript
repo: CRUDRepository<T>
```

**Methods**

##### attach(command, options)

Receives the repository command and options. [SingleCommandIBulkWriteStrategy](./single-command-i-bulk-write-strategy.md) ignores them.

```typescript
attach(command: ICommand, options: CRUDRepositoryOptions<T>): void
```

- `command` — middleware-wrapped [ICommand](./command.md)
- `options` — [CRUDRepositoryOptions](./crud-repository.md)

##### create(items)

Inserts `items`.

```typescript
create(items: T[]): Promise<Response>
```

- `items` — rows to insert

**Returns** [Response](./command.md#response)

##### update(items, byWhere)

Updates `items`.

```typescript
update(items: T[], byWhere: IWhere | undefined): Promise<Response>
```

- `items` — rows to update
- `byWhere` — optional [IWhere](./where-clause.md) applied with each item

**Returns** [Response](./command.md#response)

##### delete(items, byWhere)

Deletes `items`.

```typescript
delete(items: T[], byWhere: IWhere | undefined): Promise<Response>
```

- `items` — rows to delete
- `byWhere` — optional [IWhere](./where-clause.md) applied with each item

**Returns** [Response](./command.md#response)

### csharp
#### IBulkWriteStrategy

```csharp
public interface IBulkWriteStrategy<T> where T : new()
```

**Properties**

##### Repo

[CRUDRepository](./crud-repository.md) the orchestrator assigns after `base(...)`.

```csharp
CRUDRepository<T> Repo { get; set; }
```

**Methods**

##### Attach(command, options)

Receives the repository command and options. [SingleCommandIBulkWriteStrategy](./single-command-i-bulk-write-strategy.md) ignores them.

```csharp
void Attach(ICommand command, CRUDRepositoryOptions<T> options)
```

- `command` — middleware-wrapped [ICommand](./command.md)
- `options` — [CRUDRepositoryOptions](./crud-repository.md)

##### Create(items)

Inserts `items`.

```csharp
Task<Response> Create(IReadOnlyList<T> items)
```

- `items` — rows to insert

**Returns** [Response](./command.md#response)

##### Update(items, byWhere)

Updates `items`.

```csharp
Task<Response> Update(IReadOnlyList<T> items, IWhere? byWhere)
```

- `items` — rows to update
- `byWhere` — optional [IWhere](./where-clause.md) applied with each item

**Returns** [Response](./command.md#response)

##### Delete(items, byWhere)

Deletes `items`.

```csharp
Task<Response> Delete(IReadOnlyList<T> items, IWhere? byWhere)
```

- `items` — rows to delete
- `byWhere` — optional [IWhere](./where-clause.md) applied with each item

**Returns** [Response](./command.md#response)

### rust
#### BulkWriteStrategy

```rust
pub trait BulkWriteStrategy: Send + Sync
```

**Methods**

##### set_repo(repo)

Stores the [CRUDRepository](./crud-repository.md). [SingleCommandIBulkWriteStrategy](./single-command-i-bulk-write-strategy.md) uses it for one-row writes.

```rust
fn set_repo(&self, repo: Arc<dyn ICRUDRepository>)
```

- `repo` — [CRUDRepository](./crud-repository.md)

##### attach(command, options, connection)

Receives the repository command, options, and connection. [SingleCommandIBulkWriteStrategy](./single-command-i-bulk-write-strategy.md) ignores them.

```rust
fn attach(&self, command: Arc<dyn Command>, options: CRUDRepositoryOptions, connection: Arc<dyn Connection>)
```

- `command` — middleware-wrapped command
- `options` — [CRUDRepositoryOptions](./crud-repository.md)
- `connection` — used to build statements (Rust `Command` has no connection)

##### create(items)

Inserts `items`.

```rust
async fn create(&self, items: &mut [Row]) -> Result<Response>
```

- `items` — rows to insert

**Returns** `Result<Response>`

##### update(items, by_where)

Updates `items`.

```rust
async fn update(&self, items: &[Row], by_where: Option<&dyn IWhere>) -> Result<Response>
```

- `items` — rows to update
- `by_where` — optional [IWhere](./where-clause.md) applied with each item

**Returns** `Result<Response>`

##### delete(items, by_where)

Deletes `items`.

```rust
async fn delete(&self, items: &[Row], by_where: Option<&dyn IWhere>) -> Result<Response>
```

- `items` — rows to delete
- `by_where` — optional [IWhere](./where-clause.md) applied with each item

**Returns** `Result<Response>`
:::

## Remarks

Pass the strategy to the bulk repository constructor. The orchestrator assigns `Repo` and calls `Attach` on the strategy interface. [BatchCommandIBulkWriteStrategy](./batch-command-i-bulk-write-strategy.md) and [StoredProcedureBulkWriteStrategy](./stored-procedure-bulk-write-strategy.md) keep `Command` and `Options` from that call. [SingleCommandIBulkWriteStrategy](./single-command-i-bulk-write-strategy.md) runs one statement per row. [BatchCommandIBulkWriteStrategy](./batch-command-i-bulk-write-strategy.md) joins those statements. [StoredProcedureBulkWriteStrategy](./stored-procedure-bulk-write-strategy.md) sends one JSON payload. An empty list does not execute. Nested persist still follows the repository write strategy.

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
