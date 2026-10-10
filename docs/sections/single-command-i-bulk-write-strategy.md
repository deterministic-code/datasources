# SingleCommandIBulkWriteStrategy

Bulk write that runs one insert, update, or delete per row. This is the bulk repository default. It uses `Repo` only.

## Classes

::: reference
### typescript
#### SingleCommandIBulkWriteStrategy

```typescript
class SingleCommandIBulkWriteStrategy<T extends object> implements IBulkWriteStrategy<T>
```

**Constructor**

##### new SingleCommandIBulkWriteStrategy()

Builds the per-row bulk strategy.

```typescript
constructor()
```

**Returns** [SingleCommandIBulkWriteStrategy](#singlecommandibulkwritestrategy)

**Properties**

##### repo

[CRUDRepository](./crud-repository.md) assigned by [BulkCRUDRepository](./bulk-repository.md).

```typescript
repo: CRUDRepository<T>
```

**Methods**

##### create(items)

Inserts each item with its own statement. The result is the last insert.

```typescript
create(items: T[]): Promise<Response>
```

- `items` — rows to insert

**Returns** [Response](./command.md#response)

##### update(items, byWhere)

Updates each item with its own statement. Rows affected are summed.

```typescript
update(items: T[], byWhere: IWhere | undefined): Promise<Response>
```

- `items` — rows to update
- `byWhere` — optional [IWhere](./where-clause.md) applied with each item

**Returns** [Response](./command.md#response)

##### delete(items, byWhere)

Deletes each item with its own statement. Rows affected are summed.

```typescript
delete(items: T[], byWhere: IWhere | undefined): Promise<Response>
```

- `items` — rows to delete
- `byWhere` — optional [IWhere](./where-clause.md) applied with each item

**Returns** [Response](./command.md#response)

### csharp
#### SingleCommandIBulkWriteStrategy

```csharp
public sealed class SingleCommandIBulkWriteStrategy<T> : IBulkWriteStrategy<T> where T : new()
```

**Constructor**

##### SingleCommandIBulkWriteStrategy()

Builds the per-row bulk strategy.

```csharp
public SingleCommandIBulkWriteStrategy()
```

**Returns** [SingleCommandIBulkWriteStrategy](#singlecommandibulkwritestrategy)`<T>`

**Properties**

##### Repo

[CRUDRepository](./crud-repository.md) assigned by [BulkCRUDRepository](./bulk-repository.md).

```csharp
public CRUDRepository<T> Repo { get; set; }
```

**Methods**

##### Create(items)

Inserts each item with its own statement. The result is the last insert.

```csharp
public Task<Response> Create(IReadOnlyList<T> items)
```

- `items` — rows to insert

**Returns** [Response](./command.md#response)

##### Update(items, byWhere)

Updates each item with its own statement. Rows affected are summed.

```csharp
public Task<Response> Update(IReadOnlyList<T> items, IWhere? byWhere)
```

- `items` — rows to update
- `byWhere` — optional [IWhere](./where-clause.md) applied with each item

**Returns** [Response](./command.md#response)

##### Delete(items, byWhere)

Deletes each item with its own statement. Rows affected are summed.

```csharp
public Task<Response> Delete(IReadOnlyList<T> items, IWhere? byWhere)
```

- `items` — rows to delete
- `byWhere` — optional [IWhere](./where-clause.md) applied with each item

**Returns** [Response](./command.md#response)

### rust
#### SingleCommandIBulkWriteStrategy

```rust
pub struct SingleCommandIBulkWriteStrategy
```

**Methods**

##### create(items)

Inserts each item with its own statement. The result is the last insert.

```rust
async fn create(&self, items: &mut [Row]) -> Result<Response>
```

- `items` — rows to insert

**Returns** `Result<Response>`

##### update(items, by_where)

Updates each item with its own statement. Rows affected are summed.

```rust
async fn update(&self, items: &[Row], by_where: Option<&dyn IWhere>) -> Result<Response>
```

- `items` — rows to update
- `by_where` — optional [IWhere](./where-clause.md) applied with each item

**Returns** `Result<Response>`

##### delete(items, by_where)

Deletes each item with its own statement. Rows affected are summed.

```rust
async fn delete(&self, items: &[Row], by_where: Option<&dyn IWhere>) -> Result<Response>
```

- `items` — rows to delete
- `by_where` — optional [IWhere](./where-clause.md) applied with each item

**Returns** `Result<Response>`
:::

## Remarks

Each row still runs the repository's nested persist. An empty list returns no result and does not execute.

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
