<!-- Generated from docs/. Edit the markdown in docs/ and run npm run docs:skill. -->

# Bulk repository

[CRUDRepository](crud-repository.md) plus list writes. Each item still runs the same deep create / update / delete. `Create` / `createBulk` / `create_bulk` returns the items with ids set. Update and delete responses sum `rowsAffected`.

## Classes

#### TypeScript

#### BulkCRUDRepository {#typescript-bulkcrudrepository}

```typescript
class BulkCRUDRepository<T extends object> extends CRUDRepository<T>
```

**Constructor**

| Name | Description |
| --- | --- |
| [BulkCRUDRepository(inner)](#typescript-bulkcrudrepository-bulkcrudrepository-inner) | Wraps an inner [ICRUDRepository](crud-repository.md). |

**Methods**

| Name | Description |
| --- | --- |
| [createBulk(items)](#typescript-bulkcrudrepository-createbulk-items) | Creates each item, including nested `List` children. Returns the items with ids set, including composite keys. |
| [updateBulk(items, byWhere?)](#typescript-bulkcrudrepository-updatebulk-items-bywhere) | Updates each item, then replace-syncs nested `List` lists. |
| [deleteBulk(items, byWhere?)](#typescript-bulkcrudrepository-deletebulk-items-bywhere) | Deletes nested `List` rows for each item, then the parents. |

##### BulkCRUDRepository(inner) {#typescript-bulkcrudrepository-bulkcrudrepository-inner}

Wraps an inner [ICRUDRepository](crud-repository.md).

**Definition**

```typescript
constructor(inner: ICRUDRepository<T>)
```

**Parameters**

| Name | Description |
| --- | --- |
| `inner` | [ICRUDRepository](crud-repository.md) |

##### createBulk(items) {#typescript-bulkcrudrepository-createbulk-items}

Creates each item, including nested `List` children. Returns the items with ids set, including composite keys.

**Definition**

```typescript
createBulk(items: T[]): Promise<T[]>
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | parents to insert |

**Returns**

`Promise<T[]>`

##### updateBulk(items, byWhere?) {#typescript-bulkcrudrepository-updatebulk-items-bywhere}

Updates each item, then replace-syncs nested `List` lists.

**Definition**

```typescript
updateBulk(items: T[], byWhere?: IWhere): Promise<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | parents to update |
| `byWhere` | optional predicate applied to each item |

**Returns**

`Promise<Response>`

##### deleteBulk(items, byWhere?) {#typescript-bulkcrudrepository-deletebulk-items-bywhere}

Deletes nested `List` rows for each item, then the parents.

**Definition**

```typescript
deleteBulk(items: T[], byWhere?: IWhere): Promise<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | parents to delete |
| `byWhere` | optional predicate applied to each item |

**Returns**

`Promise<Response>`

#### C#

#### BulkCRUDRepository {#csharp-bulkcrudrepository}

```csharp
public class BulkCRUDRepository<T> : CRUDRepository<T>
```

**Constructor**

| Name | Description |
| --- | --- |
| [BulkCRUDRepository(inner)](#csharp-bulkcrudrepository-bulkcrudrepository-inner) | Wraps an inner [ICRUDRepository](crud-repository.md). |

**Methods**

| Name | Description |
| --- | --- |
| [Create(items)](#csharp-bulkcrudrepository-create-items) | Creates each item, including nested `List` children. Returns the items with ids set, including composite keys. |
| [Update(items, byWhere)](#csharp-bulkcrudrepository-update-items-bywhere) | Updates each item, then replace-syncs nested `List` lists. |
| [Delete(items, byWhere)](#csharp-bulkcrudrepository-delete-items-bywhere) | Deletes nested `List` rows for each item, then the parents. |

##### BulkCRUDRepository(inner) {#csharp-bulkcrudrepository-bulkcrudrepository-inner}

Wraps an inner [ICRUDRepository](crud-repository.md).

**Definition**

```csharp
public BulkCRUDRepository(ICRUDRepository<T> inner)
```

**Parameters**

| Name | Description |
| --- | --- |
| `inner` | [ICRUDRepository](crud-repository.md) |

##### Create(items) {#csharp-bulkcrudrepository-create-items}

Creates each item, including nested `List` children. Returns the items with ids set, including composite keys.

**Definition**

```csharp
public Task<IReadOnlyList<T>> Create(IReadOnlyList<T> items)
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | parents to insert |

**Returns**

`Task<IReadOnlyList<T>>`

##### Update(items, byWhere) {#csharp-bulkcrudrepository-update-items-bywhere}

Updates each item, then replace-syncs nested `List` lists.

**Definition**

```csharp
public Task<Response> Update(IReadOnlyList<T> items, IWhere? byWhere = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | parents to update |
| `byWhere` | optional predicate applied to each item |

**Returns**

`Task<Response>`

##### Delete(items, byWhere) {#csharp-bulkcrudrepository-delete-items-bywhere}

Deletes nested `List` rows for each item, then the parents.

**Definition**

```csharp
public Task<Response> Delete(IReadOnlyList<T> items, IWhere? byWhere = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | parents to delete |
| `byWhere` | optional predicate applied to each item |

**Returns**

`Task<Response>`

#### Rust

#### BulkCRUDRepository {#rust-bulkcrudrepository}

```rust
pub struct BulkCRUDRepository
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(inner)](#rust-bulkcrudrepository-new-inner) | Wraps an inner [CRUDRepository](crud-repository.md). |

**Methods**

| Name | Description |
| --- | --- |
| [add_enriched(enrichment, via, strategy)](#rust-bulkcrudrepository-add_enriched-enrichment-via-strategy) | Forwards to [CRUDRepository.add_enriched](crud-repository.md#rust-add_enrichedenrichment-via-strategy). A CRUD `via` is kept so `List` children can be written. |
| [create_bulk(items)](#rust-bulkcrudrepository-create_bulk-items) | Creates each item, including nested `List` children. Returns the items with ids set, including composite keys. |
| [update_bulk(items, by_where)](#rust-bulkcrudrepository-update_bulk-items-by_where) | Updates each item, then replace-syncs nested `List` lists. |
| [delete_bulk(items, by_where)](#rust-bulkcrudrepository-delete_bulk-items-by_where) | Deletes nested `List` rows for each item, then the parents. |

##### new(inner) {#rust-bulkcrudrepository-new-inner}

Wraps an inner [CRUDRepository](crud-repository.md).

**Definition**

```rust
pub fn new(inner: Arc<dyn CRUDRepository>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `inner` | [CRUDRepository](crud-repository.md) |

**Returns**

[BulkCRUDRepository](#rust-bulkcrudrepository)

##### add_enriched(enrichment, via, strategy) {#rust-bulkcrudrepository-add_enriched-enrichment-via-strategy}

Forwards to [CRUDRepository.add_enriched](crud-repository.md#rust-add_enrichedenrichment-via-strategy). A CRUD `via` is kept so `List` children can be written.

**Definition**

```rust
pub fn add_enriched(&mut self, enrichment: Enrichment, via: impl EnrichmentVia, strategy: Option<Arc<dyn ReadonlyEnrichmentStrategy>>) -> &mut Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `enrichment` | [Enrichment](enrichment.md) |
| `via` | related repository. A CRUD repository is kept so `List` children can be written |
| `strategy` | optional read strategy for this edge. Omitted uses the default strategy |

**Returns**

`&mut Self`

##### create_bulk(items) {#rust-bulkcrudrepository-create_bulk-items}

Creates each item, including nested `List` children. Returns the items with ids set, including composite keys.

**Definition**

```rust
pub async fn create_bulk(&self, items: &mut [Row]) -> Result<Vec<Row>>
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | parents to insert |

**Returns**

`Result<Vec<`[Row](repositories.md)`>>`

##### update_bulk(items, by_where) {#rust-bulkcrudrepository-update_bulk-items-by_where}

Updates each item, then replace-syncs nested `List` lists.

**Definition**

```rust
pub async fn update_bulk(&self, items: &[Row], by_where: Option<&dyn IWhere>) -> Result<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | parents to update |
| `by_where` | optional predicate applied to each item |

**Returns**

`Result<Response>`

##### delete_bulk(items, by_where) {#rust-bulkcrudrepository-delete_bulk-items-by_where}

Deletes nested `List` rows for each item, then the parents.

**Definition**

```rust
pub async fn delete_bulk(&self, items: &[Row], by_where: Option<&dyn IWhere>) -> Result<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | parents to delete |
| `by_where` | optional predicate applied to each item |

**Returns**

`Result<Response>`

## Remarks

> **TypeScript**
>
> Bulk writes use `createBulk`, `updateBulk`, and `deleteBulk`.

> **C#**
>
> List writes are `Create(IReadOnlyList<T>)`, `Update`, and `Delete`.

> **Rust**
>
> Bulk writes use `create_bulk`, `update_bulk`, and `delete_bulk`.

Each item still runs the same deep persist as [CRUDRepository](crud-repository.md). The list write goes through [IBulkWriteStrategy](docs/sections/i-bulk-write-strategy.md). The default is [SingleCommandIBulkWriteStrategy](docs/sections/single-command-i-bulk-write-strategy.md). When the write strategy's `useTransaction` is on, a list create, update, or delete is one transaction.

## Examples

### Create parents and children

#### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/BulkCRUDRepositoryTests.cs#L30-L37)

```csharp
var connection = await Setup();
var parentsRepo = new BulkCRUDRepository<Parent>(connection);
var childrenRepo = new BulkCRUDRepository<Child>(connection);
var ids = await Repo(parentsRepo, childrenRepo).Create(
[
    new Parent { children = [new Child { label = "a" }] },
```

#### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/tests/enriched.rs#L904-L942)

```rust
let connection = setup(&mut []).await;
let mut options = CRUDRepositoryOptions::default();
options.entity = "Parent".into();
let mut parents = BulkCRUDRepository::new(connection.clone(), options).unwrap();
let mut child_options = CRUDRepositoryOptions::default();
child_options.entity = "Child".into();
let children = Arc::new(BulkCRUDRepository::new(connection, child_options).unwrap());
parents.add_enriched(
    Enrichment::new("children", "id", "parent_id", None, datasources::EnrichmentPropertyType::List),
    children.clone(),
    None,
);
let ids = parents
    .create_bulk(&mut [
        record(&[(
            "children",
            encode_rows(&[record(&[("label", Value::from("a"))])]),
        )]),
        record(&[(
            "children",
            encode_rows(&[
                record(&[("label", Value::from("b"))]),
                record(&[("label", Value::from("c"))]),
            ]),
        )]),
    ])
    .await
    .unwrap();
let stored_parents = parents.read(None, None, None).await.unwrap();
```

#### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/BulkCRUDRepository.test.ts#L33-L39)

```typescript
const connection = await setup();
const parentsRepo = new BulkCRUDRepository(Parent, connection);
const childrenRepo = new BulkCRUDRepository(Child, connection);
const ids = await relate(parentsRepo, childrenRepo).createBulk([
  Object.assign(new Parent(), { children: [Object.assign(new Child(), { label: "a" })] }),
  Object.assign(new Parent(), {
```
