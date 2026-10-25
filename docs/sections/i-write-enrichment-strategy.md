# IWriteEnrichmentStrategy

Writes nested rows with a parent create, update, or delete. [CRUDRepository](./crud-repository.md) calls {{Persist|persist|persist}} after the parent write when the repository has enrichments.

## Classes

::: reference
### typescript
#### IWriteEnrichmentStrategy

```typescript
interface IWriteEnrichmentStrategy<T>
```

**Properties**

##### useTransaction

When true, the parent write and nested persist commit together.

```typescript
readonly useTransaction: boolean
```

**Methods**

##### persist(tree, items, mode)

Writes each nested edge on `tree` for `items`.

```typescript
persist(tree: IRepositoryTree, items: T[], mode: WriteMode): Promise<void>
```

- `tree` — [IRepositoryTree](./readonly-repository.md#irepositorytree)
- `items` — parents to persist
- `mode` — [WriteMode](#writemode)

**Returns** `Promise<void>`

#### WriteMode

```typescript
type WriteMode = "create" | "update" | "delete"
```

Create, update, or delete.

### csharp
#### IWriteEnrichmentStrategy

```csharp
public interface IWriteEnrichmentStrategy<T>
```

**Properties**

##### UseTransaction

When true, the parent write and nested persist commit together. The interface default is false.

```csharp
bool UseTransaction { get; }
```

**Methods**

##### Persist(tree, items, mode)

Writes each nested edge on `tree` for `items`.

```csharp
Task Persist(IRepositoryTree tree, IList<T> items, WriteMode mode)
```

- `tree` — [IRepositoryTree](./readonly-repository.md#irepositorytree)
- `items` — parents to persist
- `mode` — [WriteMode](#writemode)

**Returns** `Task`

#### WriteMode

```csharp
public enum WriteMode
```

**Properties**

##### Create

Write children and links after the parent insert.

```csharp
Create
```

##### Update

Replace-sync or recreate children and links after the parent update.

```csharp
Update
```

##### Delete

Remove children and links before the parent delete.

```csharp
Delete
```

### rust
#### WriteEnrichmentStrategy

```rust
pub trait WriteEnrichmentStrategy: Send + Sync
```

**Methods**

##### use_transaction()

When true, the parent write and nested persist commit together. The default is false.

```rust
fn use_transaction(&self) -> bool
```

**Returns** `bool`

##### persist(tree, items, mode)

Writes each nested edge on `tree` for `items`.

```rust
async fn persist(&self, tree: &RepositoryTree, items: &mut [Row], mode: WriteMode) -> Result<()>
```

- `tree` — [RepositoryTree](./readonly-repository.md#repositorytree)
- `items` — parents to persist
- `mode` — [WriteMode](#writemode)

**Returns** `Result<()>`

#### WriteMode

```rust
pub enum WriteMode
```

**Properties**

##### Create

Write children and links after the parent insert.

```rust
Create
```

##### Update

Replace-sync or recreate children and links after the parent update.

```rust
Update
```

##### Delete

Remove children and links before the parent delete.

```rust
Delete
```
:::

## Remarks

Set this as the repository {{WriteEnrichment|writeEnrichment|write_enrichment}} option. [DefaultWriteEnrichmentStrategy](./default-write-enrichment-strategy.md) is the default. Lookups are filled on read and are not written. A `List` and a link are written. A bulk list uses one transaction for the whole list when {{UseTransaction|useTransaction|use_transaction}} is on.

## Examples

### Roll back a bulk update

::: example
```csharp csharp/Datasources.Tests/BulkCRUDRepositoryTests.cs#BatchUpdateRollsBackWhenTransactionIsOn
```
```rust rust/datasources/tests/enriched.rs#bulk_update_rolls_back_when_transaction_is_on
```
```typescript typescript/datasources/src/BulkCRUDRepository.test.ts#rolls-back-a-bulk-update-when-usetransaction-is-on
```
:::
