# StoredProcedureBulkWriteStrategy

Bulk write that sends the rows as one JSON `payload` argument to a stored procedure. Create, update, and delete each name their own procedure.

## Classes

::: reference
### typescript
#### StoredProcedureBulkWriteStrategy

```typescript
class StoredProcedureBulkWriteStrategy<T extends object> implements IBulkWriteStrategy<T>
```

**Constructor**

##### new StoredProcedureBulkWriteStrategy(createName, updateName, deleteName)

Builds the procedure strategy.

```typescript
constructor(createName: string, updateName: string, deleteName: string)
```

- `createName` — procedure for insert
- `updateName` — procedure for update
- `deleteName` — procedure for delete

**Returns** [StoredProcedureBulkWriteStrategy](#storedprocedurebulkwritestrategy)

**Methods**

##### create(items, repo)

Calls the create procedure with a JSON `payload` of the rows.

```typescript
create(items: T[], repo: BulkCRUDRepository<T>): Promise<Response>
```

- `items` — rows to insert
- `repo` — [BulkCRUDRepository](./bulk-repository.md)

**Returns** [Response](./command.md#response)

##### update(items, byWhere, repo)

Calls the update procedure with a JSON `payload` of the rows.

```typescript
update(items: T[], byWhere: IWhere | undefined, repo: BulkCRUDRepository<T>): Promise<Response>
```

- `items` — rows to update
- `byWhere` — unused
- `repo` — [BulkCRUDRepository](./bulk-repository.md)

**Returns** [Response](./command.md#response)

##### delete(items, byWhere, repo)

Calls the delete procedure with a JSON `payload` of the rows.

```typescript
delete(items: T[], byWhere: IWhere | undefined, repo: BulkCRUDRepository<T>): Promise<Response>
```

- `items` — rows to delete
- `byWhere` — unused
- `repo` — [BulkCRUDRepository](./bulk-repository.md)

**Returns** [Response](./command.md#response)

### csharp
#### StoredProcedureBulkWriteStrategy

```csharp
public sealed class StoredProcedureBulkWriteStrategy<T> : IBulkWriteStrategy<T> where T : new()
```

**Constructor**

##### StoredProcedureBulkWriteStrategy(create, update, delete)

Builds the procedure strategy.

```csharp
public StoredProcedureBulkWriteStrategy(string create, string update, string delete)
```

- `create` — procedure for insert
- `update` — procedure for update
- `delete` — procedure for delete

**Returns** [StoredProcedureBulkWriteStrategy](#storedprocedurebulkwritestrategy)`<T>`

**Methods**

##### Create(items, repo)

Calls the create procedure with a JSON `payload` of the rows.

```csharp
public Task<Response> Create(IReadOnlyList<T> items, BulkCRUDRepository<T> repo)
```

- `items` — rows to insert
- `repo` — [BulkCRUDRepository](./bulk-repository.md)

**Returns** [Response](./command.md#response)

##### Update(items, byWhere, repo)

Calls the update procedure with a JSON `payload` of the rows.

```csharp
public Task<Response> Update(IReadOnlyList<T> items, IWhere? byWhere, BulkCRUDRepository<T> repo)
```

- `items` — rows to update
- `byWhere` — unused
- `repo` — [BulkCRUDRepository](./bulk-repository.md)

**Returns** [Response](./command.md#response)

##### Delete(items, byWhere, repo)

Calls the delete procedure with a JSON `payload` of the rows.

```csharp
public Task<Response> Delete(IReadOnlyList<T> items, IWhere? byWhere, BulkCRUDRepository<T> repo)
```

- `items` — rows to delete
- `byWhere` — unused
- `repo` — [BulkCRUDRepository](./bulk-repository.md)

**Returns** [Response](./command.md#response)

### rust
#### StoredProcedureBulkWriteStrategy

```rust
pub struct StoredProcedureBulkWriteStrategy
```

**Constructor**

##### new(create, update, delete)

Builds the procedure strategy.

```rust
pub fn new(create: impl Into<String>, update: impl Into<String>, delete: impl Into<String>) -> Self
```

- `create` — procedure for insert
- `update` — procedure for update
- `delete` — procedure for delete

**Returns** [StoredProcedureBulkWriteStrategy](#storedprocedurebulkwritestrategy)

**Methods**

##### create(items, repo)

Calls the create procedure with a JSON `payload` of the rows.

```rust
async fn create(&self, items: &mut [Row], repo: &BulkCRUDRepository) -> Result<Response>
```

- `items` — rows to insert
- `repo` — [BulkCRUDRepository](./bulk-repository.md)

**Returns** `Result<Response>`

##### update(items, by_where, repo)

Calls the update procedure with a JSON `payload` of the rows.

```rust
async fn update(&self, items: &[Row], by_where: Option<&dyn IWhere>, repo: &BulkCRUDRepository) -> Result<Response>
```

- `items` — rows to update
- `by_where` — unused
- `repo` — [BulkCRUDRepository](./bulk-repository.md)

**Returns** `Result<Response>`

##### delete(items, by_where, repo)

Calls the delete procedure with a JSON `payload` of the rows.

```rust
async fn delete(&self, items: &[Row], by_where: Option<&dyn IWhere>, repo: &BulkCRUDRepository) -> Result<Response>
```

- `items` — rows to delete
- `by_where` — unused
- `repo` — [BulkCRUDRepository](./bulk-repository.md)

**Returns** `Result<Response>`
:::

## Remarks

The payload is a JSON array of objects, one object per row, with column names in order. An empty list does not execute. The procedure is an [ExecuteStoredProcedureStatement](./execute-stored-procedure-statement.md) with a single `payload` parameter.

## Examples

### One JSON call

::: example
```csharp csharp/Datasources.Tests/BulkWriteStrategyTests.cs#StoredProcedureBulkWriteStrategySendsOneJsonCall
```
```rust rust/datasources/src/tests/bulk_write_strategy.rs#stored_procedure_bulk_write_strategy_sends_one_json_call
```
```typescript typescript/datasources/src/BulkWriteStrategy.test.ts#stored-procedure-commands-sends-one-json-call
```
:::
