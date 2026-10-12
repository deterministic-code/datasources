<!-- Generated from docs/. Edit the markdown in docs/ and run npm run docs:skill. -->

# Readonly repository

Runs readonly select commands and can enrich those datasource objects. Writes are [CRUD repository](crud-repository.md).

## Classes

#### TypeScript

#### IReadonlyRepository {#typescript-ireadonlyrepository}

```typescript
interface IReadonlyRepository<T = unknown> extends IRepository<T>
```

**Methods**

| Name | Description |
| --- | --- |
| [readSingle(query?)](#typescript-ireadonlyrepository-readsingle-query) | Returns the first item, or `undefined` when the result is empty. |
| [read(query?)](#typescript-ireadonlyrepository-read-query) | Yields every item. |

##### readSingle(query?) {#typescript-ireadonlyrepository-readsingle-query}

Returns the first item, or `undefined` when the result is empty.

**Definition**

```typescript
readSingle(query?: ISelectStatement | IWhere | IExecuteStoredProcedureStatement): Promise<T | undefined>
```

**Parameters**

| Name | Description |
| --- | --- |
| `query` | optional query, where, or stored procedure; omitted uses the default select |

**Returns**

`Promise<T | undefined>`

##### read(query?) {#typescript-ireadonlyrepository-read-query}

Yields every item.

**Definition**

```typescript
read(query?: ISelectStatement | IWhere | IExecuteStoredProcedureStatement): AsyncIterable<T>
```

**Parameters**

| Name | Description |
| --- | --- |
| `query` | optional query, where, or stored procedure; omitted uses the default select |

**Returns**

`AsyncIterable<T>`

#### ReadonlyRepository {#typescript-readonlyrepository}

```typescript
abstract class ReadonlyRepository<T> implements IReadonlyRepository<T>
```

**Constructor**

| Name | Description |
| --- | --- |
| [ReadonlyRepository(type)](#typescript-readonlyrepository-readonlyrepository-type) | Stores the item constructor. |
| [ReadonlyRepository(inner, strategy?)](#typescript-readonlyrepository-readonlyrepository-inner-strategy) | Wraps an inner repository. `read` reads that repository, then fills related fields. Omitted strategy uses the default. See [IEnriched](enrichment.md). |

**Properties**

| Name | Description |
| --- | --- |
| type | Item constructor the repository exposes. |

**Methods**

| Name | Description |
| --- | --- |
| [read(query?)](#typescript-readonlyrepository-read-query) | Yields every item. Table repositories convert then map; subclasses may override. |
| [readSingle(query?)](#typescript-readonlyrepository-readsingle-query) | Returns the first item from [read](#typescript-readonlyrepository-read-query), or `undefined`. |

##### ReadonlyRepository(type) {#typescript-readonlyrepository-readonlyrepository-type}

Stores the item constructor.

**Definition**

```typescript
constructor(type: Constructable<T>)
```

**Parameters**

| Name | Description |
| --- | --- |
| `type` | item constructor ([IRepository.type](repositories.md#typescript-irepository-type)) |

##### ReadonlyRepository(inner, strategy?) {#typescript-readonlyrepository-readonlyrepository-inner-strategy}

Wraps an inner repository. `read` reads that repository, then fills related fields. Omitted strategy uses the default. See [IEnriched](enrichment.md).

**Definition**

```typescript
constructor(inner: IReadonlyRepository<T>, strategy?: IReadonlyEnrichmentStrategy<T>)
```

**Parameters**

| Name | Description |
| --- | --- |
| `inner` | [IReadonlyRepository](#typescript-ireadonlyrepository) |
| `strategy` | optional enrichment strategy |

##### read(query?) {#typescript-readonlyrepository-read-query}

Yields every item. Table repositories convert then map; subclasses may override.

**Definition**

```typescript
abstract read(query?: ISelectStatement | IWhere | IExecuteStoredProcedureStatement): AsyncIterable<T>
```

**Parameters**

| Name | Description |
| --- | --- |
| `query` | optional query, where, or stored procedure; omitted uses the default select |

**Returns**

`AsyncIterable<T>`

##### readSingle(query?) {#typescript-readonlyrepository-readsingle-query}

Returns the first item from [read](#typescript-readonlyrepository-read-query), or `undefined`.

**Definition**

```typescript
readSingle(query?: ISelectStatement | IWhere | IExecuteStoredProcedureStatement): Promise<T | undefined>
```

**Parameters**

| Name | Description |
| --- | --- |
| `query` | optional query, where, or stored procedure; omitted uses the default select |

**Returns**

`Promise<T | undefined>`

#### ReadonlyRepositoryMappingOptions {#typescript-readonlyrepositorymappingoptions}

```typescript
type ReadonlyRepositoryMappingOptions<T = Cells>
```

**Properties**

| Name | Description |
| --- | --- |
| entity | Table or entity name. |
| entityMapper | Maps the table name when the repository is built. Default is [IdentityMapper](mappings.md). |
| propertyMappings | Maps property names to stored columns. Default is [IdentityMapper](mappings.md). The repository maps where identifiers and write parameters. Read reverse-maps each result key before the row converter. |
| rowConverter | `Cells` to `Cells`. Omitted leaves each cell unchanged. The object step is [MetaDataRowTypeConverter](type-converters.md). |
| objectConverter | `Cells` to `T`, after `rowConverter`. Omitted assigns the row onto `T`. |
| type | Item constructor. Supplies the default entity name and column list. |
| rowTypeConverters | Provider converters keyed by `bool`, `datetime`, `uuid`, `decimal`, and `binary`, merged into the row converter. The driver applies its converters when it binds a parameter. A field converter reverse-converts that property in the repository. |
| readColumns | SELECT column names. When omitted, public fields of `type` are used. |

#### IRepositoryTree {#typescript-irepositorytree}

```typescript
interface IRepositoryTree
```

**Properties**

| Name | Description |
| --- | --- |
| parent | Root [IRepositoryTreeNode](#typescript-irepositorytreenode). |

#### IRepositoryTreeNode {#typescript-irepositorytreenode}

```typescript
interface IRepositoryTreeNode
```

**Properties**

| Name | Description |
| --- | --- |
| value | Related repository. |
| propertyName | Destination property name. |
| parent | Parent node, if any. |
| items | Child nodes. |
| enrichment | [Enrichment](enrichment.md) for this node. |

**Methods**

| Name | Description |
| --- | --- |
| [include(via, destinationProperty, sourceProperty, relatedKey, relatedProperty?, type?)](#typescript-irepositorytreenode-include-via-destinationproperty-sourceproperty-relatedkey-relatedproperty-type) | Adds a child node. |
| [enrichFrom(parents)](#typescript-irepositorytreenode-enrichfrom-parents) | Fills `parents` from this node. |

##### include(via, destinationProperty, sourceProperty, relatedKey, relatedProperty?, type?) {#typescript-irepositorytreenode-include-via-destinationproperty-sourceproperty-relatedkey-relatedproperty-type}

Adds a child node.

**Definition**

```typescript
include<TRelated extends object>(via: IReadonlyRepository<TRelated>, destinationProperty: string, sourceProperty: string, relatedKey: string, relatedProperty?: string, type?: EnrichmentPropertyType): IRepositoryTreeNode
```

**Parameters**

| Name | Description |
| --- | --- |
| `via` | related [IReadonlyRepository](#) |
| `destinationProperty` | parent property that gets filled |
| `sourceProperty` | parent key |
| `relatedKey` | key on the related row |
| `relatedProperty` | optional related property to copy |
| `type` | [EnrichmentPropertyType](enrichment.md#typescript-enrichmentpropertytype) |

**Returns**

[IRepositoryTreeNode](#typescript-irepositorytreenode)

##### enrichFrom(parents) {#typescript-irepositorytreenode-enrichfrom-parents}

Fills `parents` from this node.

**Definition**

```typescript
enrichFrom(parents: object[]): Promise<void>
```

**Parameters**

| Name | Description |
| --- | --- |
| `parents` | parent objects |

#### C#

#### IReadonlyRepository {#csharp-ireadonlyrepository}

```csharp
public interface IReadonlyRepository<T> : IRepository
```

**Methods**

| Name | Description |
| --- | --- |
| [ReadSingle(query)](#csharp-ireadonlyrepository-readsingle-query) | Returns the first item, or `default` when the result is empty. |
| [Read(query)](#csharp-ireadonlyrepository-read-query) | Yields every item. |

##### ReadSingle(query) {#csharp-ireadonlyrepository-readsingle-query}

Returns the first item, or `default` when the result is empty.

**Definition**

```csharp
Task<T?> ReadSingle(IStatement? query = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `query` | optional query, where, or stored-procedure statement; omitted uses the default select |

**Returns**

`Task<T?>`

##### Read(query) {#csharp-ireadonlyrepository-read-query}

Yields every item.

**Definition**

```csharp
IAsyncEnumerable<T> Read(IStatement? query = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `query` | optional query, where, or stored-procedure statement; omitted uses the default select |

**Returns**

`IAsyncEnumerable<T>`

#### ReadonlyRepository {#csharp-readonlyrepository}

```csharp
public class ReadonlyRepository<T> : IReadonlyRepository<T>, IEnriched<T>
```

**Constructor**

| Name | Description |
| --- | --- |
| [ReadonlyRepository(inner, strategy)](#csharp-readonlyrepository-readonlyrepository-inner-strategy) | Wraps an inner repository. `Read` reads that repository, then fills related fields. Omitted strategy uses the default. See [IEnriched](enrichment.md). |

**Properties**

| Name | Description |
| --- | --- |
| Type | Item CLR type. `typeof(T)`. |

**Methods**

| Name | Description |
| --- | --- |
| [Read(query)](#csharp-readonlyrepository-read-query) | Yields every item. Table repositories convert then map; subclasses may override. |
| [ReadSingle(query)](#csharp-readonlyrepository-readsingle-query) | Returns the first item from [Read](#csharp-readonlyrepository-read-query), or `default`. |

##### ReadonlyRepository(inner, strategy) {#csharp-readonlyrepository-readonlyrepository-inner-strategy}

Wraps an inner repository. `Read` reads that repository, then fills related fields. Omitted strategy uses the default. See [IEnriched](enrichment.md).

**Definition**

```csharp
public ReadonlyRepository(IReadonlyRepository<T> inner, IReadonlyEnrichmentStrategy<T>? strategy = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `inner` | [IReadonlyRepository](#csharp-ireadonlyrepository) |
| `strategy` | optional enrichment strategy |

##### Read(query) {#csharp-readonlyrepository-read-query}

Yields every item. Table repositories convert then map; subclasses may override.

**Definition**

```csharp
public virtual IAsyncEnumerable<T> Read(IStatement? query = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `query` | optional query, where, or stored-procedure statement; omitted uses the default select |

**Returns**

`IAsyncEnumerable<T>`

##### ReadSingle(query) {#csharp-readonlyrepository-readsingle-query}

Returns the first item from [Read](#csharp-readonlyrepository-read-query), or `default`.

**Definition**

```csharp
public virtual Task<T?> ReadSingle(IStatement? query = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `query` | optional query, where, or stored-procedure statement; omitted uses the default select |

**Returns**

`Task<T?>`

#### ReadonlyRepositoryMappingOptions {#csharp-readonlyrepositorymappingoptions}

```csharp
public class ReadonlyRepositoryMappingOptions<T> where T : new()
```

**Properties**

| Name | Description |
| --- | --- |
| Entity | Table or entity name. When omitted, `typeof(T).Name` is used. |
| EntityMapper | Maps the table name when the repository is built. Default is [IdentityMapper](mappings.md). |
| PropertyMappings | Maps property names to stored columns. Default is [IdentityMapper](mappings.md). The repository maps where identifiers and write parameters. Read reverse-maps each result key before the row converter. |
| RowConverter | `Row` to `Row`. Omitted leaves each cell unchanged. |
| ObjectConverter | `Row` to `T`, after `RowConverter`. Omitted assigns public properties. A handwritten converter assigns each column and does not reflect. |
| RowTypeConverters | Provider converters keyed by `bool`, `datetime`, `uuid`, `decimal`, and `binary`. The driver applies its converters when it binds a parameter. A field converter on the object converter reverse-converts that property in the repository. |
| ReadColumns | SELECT column names. When omitted, public properties of `T` are used. |

#### IRepositoryTree {#csharp-irepositorytree}

```csharp
public interface IRepositoryTree
```

**Properties**

| Name | Description |
| --- | --- |
| Parent | Root [IRepositoryTreeNode](#csharp-irepositorytreenode). |

#### IRepositoryTreeNode {#csharp-irepositorytreenode}

```csharp
public interface IRepositoryTreeNode
```

**Properties**

| Name | Description |
| --- | --- |
| Value | Related repository. |
| PropertyName | Destination property name. |
| Parent | Parent node, if any. |
| Items | Child nodes. |

#### Rust

#### IReadonlyRepository {#rust-ireadonlyrepository}

```rust
pub trait IReadonlyRepository: IRepository
```

**Methods**

| Name | Description |
| --- | --- |
| [read_single(query)](#rust-ireadonlyrepository-read_single-query) | Returns the first row, or `None` when the result is empty. |
| [read(query)](#rust-ireadonlyrepository-read-query) | Returns every row. |

##### read_single(query) {#rust-ireadonlyrepository-read_single-query}

Returns the first row, or `None` when the result is empty.

**Definition**

```rust
async fn read_single(&self, query: Option<&dyn ReadInput>) -> Result<Option<Row>>
```

**Parameters**

| Name | Description |
| --- | --- |
| `query` | optional query, where, or stored procedure; omitted uses the default select |

**Returns**

`Result<Option<Row>>`

##### read(query) {#rust-ireadonlyrepository-read-query}

Returns every row.

**Definition**

```rust
async fn read(&self, query: Option<&dyn ReadInput>) -> Result<Vec<Row>>
```

**Parameters**

| Name | Description |
| --- | --- |
| `query` | optional query, where, or stored procedure; omitted uses the default select |

**Returns**

`Result<Vec<Row>>`

#### ReadonlyRepository {#rust-readonlyrepository}

```rust
pub struct ReadonlyRepository
```

**Constructor**

| Name | Description |
| --- | --- |
| [from_repository(inner)](#rust-readonlyrepository-from_repository-inner) | Wraps an inner repository. `read` reads that repository, then fills related fields. Uses the default strategy. See [IEnriched](enrichment.md). |
| [with_strategy(inner, strategy)](#rust-readonlyrepository-with_strategy-inner-strategy) | Wraps an inner repository with an enrichment strategy. |

##### from_repository(inner) {#rust-readonlyrepository-from_repository-inner}

Wraps an inner repository. `read` reads that repository, then fills related fields. Uses the default strategy. See [IEnriched](enrichment.md).

**Definition**

```rust
pub fn from_repository(inner: Arc<dyn IReadonlyRepository>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `inner` | [IReadonlyRepository](#rust-ireadonlyrepository) |

**Returns**

[ReadonlyRepository](#rust-readonlyrepository)

##### with_strategy(inner, strategy) {#rust-readonlyrepository-with_strategy-inner-strategy}

Wraps an inner repository with an enrichment strategy.

**Definition**

```rust
pub fn with_strategy(inner: Arc<dyn IReadonlyRepository>, strategy: Arc<dyn ReadonlyEnrichmentStrategy>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `inner` | [IReadonlyRepository](#rust-ireadonlyrepository) |
| `strategy` | [ReadonlyEnrichmentStrategy](docs/sections/i-readonly-enrichment-strategy.md) |

**Returns**

[ReadonlyRepository](#rust-readonlyrepository)

#### ReadonlyRepositoryMappingOptions {#rust-readonlyrepositorymappingoptions}

```rust
pub struct ReadonlyRepositoryMappingOptions
```

**Properties**

| Name | Description |
| --- | --- |
| entity | Table or entity name. Default `"items"`. |
| entity_mapper | Maps the table name when the repository is built. Default is [IdentityMapper](mappings.md). |
| property_mappings | Maps property names to stored columns. Default is [IdentityMapper](mappings.md). The repository maps where identifiers and write parameters. Read reverse-maps each result key before the row converter. |
| row_converter | `Row` to `Row` cell step. `None` leaves each cell unchanged. The object step is [MetaDataRowTypeConverter](type-converters.md). |
| object_converter | `Row` to `Row`, after `row_converter`. Omitted copies every cell. A handwritten converter assigns each column. |
| row_type_converters | Provider converters keyed by `bool`, `datetime`, `uuid`, `decimal`, and `binary`, merged into the row converter. The driver applies its converters when it binds a parameter. A field converter reverse-converts that property in the repository. |

#### RepositoryTree {#rust-repositorytree}

```rust
pub struct RepositoryTree
```

**Properties**

| Name | Description |
| --- | --- |
| parent | Root [RepositoryTreeNode](#rust-repositorytreenode). |

#### RepositoryTreeNode {#rust-repositorytreenode}

```rust
pub struct RepositoryTreeNode
```

**Properties**

| Name | Description |
| --- | --- |
| value | Related [ReadonlyRepository](#). |
| property_name | Destination field name. |
| items | Child nodes. |
| enrichment | [Enrichment](enrichment.md) for this node. |

**Methods**

| Name | Description |
| --- | --- |
| [include(via, destination_property, source_property, related_key, related_property, type)](#rust-repositorytreenode-include-via-destination_property-source_property-related_key-related_property-type) | Adds a child node. |

##### include(via, destination_property, source_property, related_key, related_property, type) {#rust-repositorytreenode-include-via-destination_property-source_property-related_key-related_property-type}

Adds a child node.

**Definition**

```rust
pub fn include(&mut self, via: impl EnrichmentVia, destination_property: impl Into<String>, source_property: impl Into<String>, related_key: impl Into<String>, related_property: Option<String>, r#type: EnrichmentPropertyType) -> &mut Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `via` | related [IReadonlyRepository](#rust-ireadonlyrepository) |
| `destination_property` | parent field that gets filled |
| `source_property` | parent key |
| `related_key` | key on the related row |
| `related_property` | optional related field to copy |
| `type` | [EnrichmentPropertyType](enrichment.md#rust-enrichmentpropertytype) |

**Returns**

`&mut Self`

## Remarks

`AddEnriched` / `addEnriched` / `add_enriched` on [IEnriched](enrichment.md) registers a read fill. With no enrichments, `Read` / `read` / `read` stays streaming. With enrichments, the read is collected, filled, then yielded. A `List` is walked. A `RelatedProperty` scalar is not. Depth follows each related repository's own enrichments. Writes, including `List` children and associations, are [CRUDRepository](crud-repository.md).

`AddEnriched` stays fluent for siblings. Register the child repository's own fills on that repository:

```
childrenRepo.AddEnriched(new("type_name", "type_id", "id", "name"), childTypesRepo);
parentsRepo.AddEnriched(new("children", "id", "parent_id", Type: EnrichmentPropertyType.List), childrenRepo);
```

Repository read is convert, then map. See [Type Converters](type-converters.md) and [Mappers](mappings.md).

`read` and `read_single` accept a query, a where, or an [ExecuteStoredProcedureStatement](other-statements.md). Result rows still convert, then map. Procedure argument names are not rewritten as columns.

## Examples

### Simple readonly

#### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#L49-L72)

```csharp
var ds = Datasource.Default;
await Execute(connection, "INSERT INTO users (id, username, is_enabled) VALUES (1, 'ada', 1)");
await Execute(connection, "INSERT INTO users (id, username, is_enabled) VALUES (2, 'grace', 0)");
await Execute(connection, "INSERT INTO users (id, username, is_enabled) VALUES (3, 'bea', 1)");
var repo = ds.CreateReadonlyRepository<User>(connection, new ReadonlyRepositoryMappingOptions<User>
{
    EntityMapper = SnakeThenPlural(),
    PropertyMappings = Snake(),
});

var byId = await repo.ReadSingle(new Where("Id", eq, 1));
var row = await repo.Read(new Where("Username", eq, "grace").And("IsEnabled", eq, 0)).SingleAsync();
```

**After: by id**

```
{
  id: 1,
  username: 'ada',
  is_enabled: true
}
```

**After: disabled**

```
[
  {
    id: 2,
    username: 'grace',
    is_enabled: false
  }
]
```

#### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/readonly_repository.rs#L68-L113)

```rust
let ds = default().unwrap();
execute_sql(
    &connection,
    "INSERT INTO users (id, username, is_enabled) VALUES (1, 'ada', 1)",
)
.await;
execute_sql(
    &connection,
    "INSERT INTO users (id, username, is_enabled) VALUES (2, 'grace', 0)",
)
.await;
execute_sql(
    &connection,
    "INSERT INTO users (id, username, is_enabled) VALUES (3, 'bea', 1)",
)
.await;
let mut options = create_readonly_repository_mapping_options::<User>();
options.entity_mapper = Some(std::sync::Arc::new(snake_then_plural()));
options.property_mappings = Some(std::sync::Arc::new(snake()));
let repo = ds.create_readonly_repository::<User>(Some(connection), Some(options));
let by_id = repo
    .read_single(Some(&Where::binary("Id", eq, 1_i64)), None, None)
    .await
    .unwrap()
    .unwrap();
let row = repo
    .read(
        Some(&Where::binary("Username", eq, "grace").and("IsEnabled", eq, 0_i64)),
        None, None
    )
    .await
    .unwrap()
    .into_iter()
    .next()
    .unwrap();
```

**After: by id**

```
{
  id: 1,
  username: 'ada',
  is_enabled: 1
}
```

**After: disabled**

```
[
  {
    id: 2,
    username: 'grace',
    is_enabled: 0
  }
]
```

#### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/readonly.repository.integration.test.ts#L76-L94)

```typescript
const ds = Datasource.default;
await executeSql(connection, "INSERT INTO users (id, username, is_enabled) VALUES (1, 'ada', 1)");
await executeSql(connection, "INSERT INTO users (id, username, is_enabled) VALUES (2, 'grace', 0)");
await executeSql(connection, "INSERT INTO users (id, username, is_enabled) VALUES (3, 'bea', 1)");
const repo = ds.createReadonlyRepository(User, connection, {
  entityMapper: snakeThenPlural(),
  propertyMappings: snake(),
});
const byId = await repo.readSingle(new Where("Id", eq, 1));
const [row] = await Array.fromAsync(repo.read(new Where("Username", eq, "grace").and("IsEnabled", eq, 0)));
```

**After: by id**

```
{
  id: 1,
  username: 'ada',
  is_enabled: 1
}
```

**After: disabled**

```
[
  {
    id: 2,
    username: 'grace',
    is_enabled: 0
  }
]
```

Schema: [minimal-users](docs/examples.md#minimal-users)

```sql
CREATE TABLE users (id int, username string NOT NULL, is_enabled bool NOT NULL, CONSTRAINT pk_users PRIMARY KEY (id))
```

### Map an object explicitly

A handwritten object converter assigns each column. `ObjectConverter` / `objectConverter` / `object_converter` replaces [MetaDataRowTypeConverter](type-converters.md), so the read does not reflect.

#### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#L74-L89) [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#L91-L115)

```csharp
sealed class UserConverter : TypeConverter<User, Row>
{
    public override User Convert(Row from) => new()
    {
        Id = (long)from["Id"].ToObject()!,
        Username = (string)from["Username"].ToObject()!,
        IsEnabled = (bool)from["IsEnabled"].ToObject()!
    };

    public override Row ReverseConvert(User to) => new()
    {
        ["Id"] = to.Id,
        ["Username"] = to.Username,
        ["IsEnabled"] = to.IsEnabled
    };
}

var driver = new SqliteDriver();
var ds = new Datasource("sqlite", driver, new Dictionary<string, string?> { ["DATASOURCE_URL"] = "sqlite:///:memory:" });
await Execute(connection, """
    INSERT INTO users (id, username, is_enabled) VALUES
        (1, 'ada', 1),
        (2, 'grace', 0);
    """);
var usersRepo = ds.CreateReadonlyRepository<User>(connection, new ReadonlyRepositoryMappingOptions<User>
{
    EntityMapper = SnakeThenPlural(),
    PropertyMappings = Snake(),
    ObjectConverter = new UserConverter()
});

var user = await usersRepo.ReadSingle(new Where("Id", eq, 1));
```

**After: by id**

```
{
  id: 1,
  username: 'ada',
  is_enabled: true
}
```

#### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/readonly_repository.rs#L117-L139) [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/readonly_repository.rs#L141-L169)

```rust
impl TypeConverter<Row, Row> for UserConverter {
    fn convert(&self, from: &Row) -> Result<Row> {
        let mut row = from.clone();
        let enabled = match row.get("IsEnabled") {
            Some(Value::Bool(flag)) => Value::Bool(*flag),
            Some(value) => value.clone(),
            None => Value::Null,
        };
        row.insert("IsEnabled".into(), enabled);
        Ok(row)
    }

    fn reverse_convert(&self, to: &Row) -> Result<Row> {
        let mut row = to.clone();
        let enabled = match row.get("IsEnabled") {
            Some(Value::Bool(flag)) => Value::Int(if *flag { 1 } else { 0 }),
            Some(value) => value.clone(),
            None => Value::Null,
        };
        row.insert("IsEnabled".into(), enabled);
        Ok(row)
    }
}

let built = datasource();
let ds: &dyn Datasource = &*built;
execute_sql(
    &connection,
    "
    INSERT INTO users (id, username, is_enabled) VALUES
        (1, 'ada', 1),
        (2, 'grace', 0);
    ",
)
.await;
let mut options = create_readonly_repository_mapping_options::<User>();
options.entity_mapper = Some(std::sync::Arc::new(snake_then_plural()));
options.property_mappings = Some(std::sync::Arc::new(snake()));
options.object_converter = Some(Box::new(UserConverter));
let users_repo = ds.create_readonly_repository::<User>(Some(connection), Some(options));
let user = users_repo
    .read_single(Some(&Where::binary("Id", eq, 1_i64)), None, None)
    .await
    .unwrap()
    .unwrap();
```

**After: by id**

```
{
  id: 1,
  username: 'ada',
  is_enabled: true
}
```

#### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/readonly.repository.integration.test.ts#L44-L58) [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/readonly.repository.integration.test.ts#L96-L114)

```typescript
class UserConverter extends TypeConverter<User, Cells> {
  convert = (from: Cells): User => {
    const user = new User();
    user.Id = from.Id as number;
    user.Username = from.Username as string;
    user.IsEnabled = from.IsEnabled === true;
    return user;
  };

  reverseConvert = (to: User): Cells => ({
    Id: to.Id,
    Username: to.Username,
    IsEnabled: to.IsEnabled ? 1 : 0,
  });
}

const ds = new Datasource("sqlite", new SqliteDriver(), { DATASOURCE_URL: "sqlite:///:memory:" });
await executeSql(connection, `
  INSERT INTO users (id, username, is_enabled) VALUES
    (1, 'ada', 1),
    (2, 'grace', 0);
`);
const usersRepo = ds.createReadonlyRepository(User, connection, {
  entityMapper: snakeThenPlural(),
  propertyMappings: snake(),
  objectConverter: new UserConverter(),
});
const user = await usersRepo.readSingle(new Where("Id", eq, 1));
```

**After: by id**

```
{
  id: 1,
  username: 'ada',
  is_enabled: true
}
```

Schema: [minimal-users](docs/examples.md#minimal-users)

```sql
CREATE TABLE users (id int, username string NOT NULL, is_enabled bool NOT NULL, CONSTRAINT pk_users PRIMARY KEY (id))
```

### Enriched readonly

`UsersView` maps onto `user`. `UserType` stays on the row as `user_type_id` until enrichment replaces it with the type name. The lookup repository's entity is `UserType`, and [ChangeCasingMapper](mappings.md) maps that name, plus `Id` and `Name`, to `user_type`, `id`, and `name`.

#### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#L331-L374)

```csharp
var ds = new Datasource("sqlite", new SqliteDriver(), new Dictionary<string, string?> { ["DATASOURCE_URL"] = "sqlite:///:memory:" });
await Execute(connection, $"""
    INSERT INTO user_type (id, name) VALUES
        (1, 'admin'),
        (2, 'user'),
        (3, 'service');
    INSERT INTO user (id, username, is_enabled, created, user_type_id) VALUES
        ('{Ada}', 'ada', 1, '2020-01-01T00:00:00.000Z', 1),
        ('{Grace}', 'grace', 1, '2024-06-01T00:00:00.000Z', 2),
        ('{Lin}', 'lin', 0, '2024-06-01T00:00:00.000Z', 3);
    """);
var casing = new ChangeCasingMapper(Casing.Pascal, Casing.Snake);
var usersRepo = (ReadonlyRepository<UsersView>)ds.CreateReadonlyRepository<UsersView>(connection, new ReadonlyRepositoryMappingOptions<UsersView>
{
    EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["UsersView"] = "user" }),
    PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
    {
        ["Key"] = "id",
        ["Login"] = "username",
        ["Active"] = "is_enabled",
        ["Opened"] = "created",
        ["UserType"] = "user_type_id",
    }),
});
var typesRepo = ds.CreateReadonlyRepository<UserType>(connection, new ReadonlyRepositoryMappingOptions<UserType>
{
    EntityMapper = casing,
    PropertyMappings = casing,
});
usersRepo.AddEnriched(new("UserType", "UserType", "Id", "Name"), typesRepo);
var usersView = await usersRepo.ReadSingle(new Where("Key", eq, Ada));
```

**User view**

```
{
  Key: '11111111-1111-1111-1111-111111111111',
  Login: 'ada',
  Active: true,
  Opened: '2020-01-01T00:00:00.000Z',
  UserType: 'admin'
}
```

#### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/readonly_repository.rs#L287-L359)

```rust
let built = datasource();
let ds: &dyn Datasource = &*built;
execute_sql(
    &connection,
    &format!(
        "
    INSERT INTO user_type (id, name) VALUES
        (1, 'admin'),
        (2, 'user'),
        (3, 'service');
    INSERT INTO user (id, username, is_enabled, created, user_type_id) VALUES
        ('{ADA}', 'ada', 1, '2020-01-01T00:00:00.000Z', 1),
        ('{GRACE}', 'grace', 1, '2024-06-01T00:00:00.000Z', 2),
        ('{LIN}', 'lin', 0, '2024-06-01T00:00:00.000Z', 3);
    "
    ),
)
.await;
let mut users_options = create_readonly_repository_mapping_options::<UserAccount>();
users_options.entity = "user".into();
users_options.property_mappings = Some(std::sync::Arc::new(DictionaryMapper::new([
    ("Key".to_string(), "id".to_string()),
    ("Login".to_string(), "username".to_string()),
    ("Active".to_string(), "is_enabled".to_string()),
    ("Opened".to_string(), "created".to_string()),
    ("UserTypeId".to_string(), "user_type_id".to_string()),
])));
let mut type_options = create_readonly_repository_mapping_options::<UserType>();
type_options.entity = "UserType".into();
type_options.entity_mapper =
    Some(std::sync::Arc::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake)));
type_options.property_mappings =
    Some(std::sync::Arc::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake)));
let mut users_repo =
    ReadonlyRepository::new(connection.clone(), users_options).unwrap();
let types =
    ds.create_readonly_repository::<UserType>(Some(connection), Some(type_options));
users_repo.add_enriched(
    datasources::Enrichment::new(
        "UserType",
        "UserTypeId",
        "Id",
        Some("Name".into()),
        datasources::EnrichmentPropertyType::Object,
    ),
    types,
    None,
);
let account = users_repo
    .read_single(
        Some(&Where::binary("Key", eq, Uuid::parse_str(ADA).unwrap())),
        None, None
)
    .await
    .unwrap()
    .unwrap();
let mut users_view = account.clone();
users_view.remove("UserTypeId");
```

**User view**

```
{
  Key: '11111111-1111-1111-1111-111111111111',
  Login: 'ada',
  Active: 1,
  Opened: '2020-01-01T00:00:00.000Z',
  UserType: 'admin'
}
```

#### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/readonly.repository.integration.test.ts#L168-L205)

```typescript
const ds = new Datasource("sqlite", new SqliteDriver(), { DATASOURCE_URL: "sqlite:///:memory:" });
await executeSql(connection, `
  INSERT INTO user_type (id, name) VALUES
    (1, 'admin'),
    (2, 'user'),
    (3, 'service');
  INSERT INTO user (id, username, is_enabled, created, user_type_id) VALUES
    ('${ada}', 'ada', 1, '2020-01-01T00:00:00.000Z', 1),
    ('${grace}', 'grace', 1, '2024-06-01T00:00:00.000Z', 2),
    ('${lin}', 'lin', 0, '2024-06-01T00:00:00.000Z', 3);
`);
const casing = new ChangeCasingMapper("pascalCase", "snakeCase");
const usersRepo = ds.createReadonlyRepository(UsersView, connection, {
  entityMapper: new DictionaryMapper([["UsersView", "user"]]),
  propertyMappings: new DictionaryMapper([
    ["Key", "id"],
    ["Login", "username"],
    ["Active", "is_enabled"],
    ["Opened", "created"],
    ["UserType", "user_type_id"],
  ]),
}) as ReadonlyRepository<UsersView>;
const typesRepo = ds.createReadonlyRepository(UserType, connection, {
  entityMapper: casing,
  propertyMappings: casing,
});
usersRepo.addEnriched({ destinationProperty: "UserType", sourceProperty: "UserType", relatedKey: "Id", relatedProperty: "Name" }, typesRepo);
const usersView = await usersRepo.readSingle(new Where("Key", eq, ada));
```

**User view**

```
{
  Key: '11111111-1111-1111-1111-111111111111',
  Login: 'ada',
  Active: 1,
  Opened: '2020-01-01T00:00:00.000Z',
  UserType: 'admin'
}
```

Schema: [user-type-lookup](docs/examples.md#user-type-lookup)

```sql
CREATE TABLE user_type (id int, name string NOT NULL, CONSTRAINT pk_user_type PRIMARY KEY (id));
CREATE TABLE user (id uuid, username string NOT NULL, is_enabled bool NOT NULL, created datetime NOT NULL, user_type_id int NOT NULL, CONSTRAINT pk_user PRIMARY KEY (id))
```

### Read a user by guid, username, and created

#### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#L117-L156)

```csharp
var ds = new Datasource("sqlite", new SqliteDriver(), new Dictionary<string, string?> { ["DATASOURCE_URL"] = "sqlite:///:memory:" });
await Execute(connection, $"""
    INSERT INTO user (id, username, is_enabled, created) VALUES
        ('{Ada}', 'ada', 1, '2020-01-01T00:00:00.000Z'),
        ('{Grace}', 'grace', 1, '2024-06-01T00:00:00.000Z'),
        ('{Lin}', 'lin', 0, '2024-06-01T00:00:00.000Z');
    """);
var repo = ds.CreateReadonlyRepository<UserAccount>(connection, new ReadonlyRepositoryMappingOptions<UserAccount>
{
    EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["UserAccount"] = "user" }),
    PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
    {
        ["Key"] = "id",
        ["Login"] = "username",
        ["Active"] = "is_enabled",
        ["Opened"] = "created",
    }),
});

var opened = new DateTime(2024, 1, 1, 0, 0, 0, DateTimeKind.Utc);
var byId = await repo.ReadSingle(new Where("Key", eq, Ada));
var byLogin = new List<UserAccount>();
await foreach (var row in repo.Read(new Where("Login", eq, "ada").And("Active", eq, true)))
    byLogin.Add(row);
var later = new List<UserAccount>();
await foreach (var row in repo.Read(new Where("Active", eq, true).And("Opened", gt, opened)))
    later.Add(row);
```

**After: by id**

```
{
  Key: '11111111-1111-1111-1111-111111111111',
  Login: 'ada',
  Active: true,
  Opened: '2020-01-01T00:00:00.000Z'
}
```

**After: by login**

```
[
  {
    Key: '11111111-1111-1111-1111-111111111111',
    Login: 'ada',
    Active: true,
    Opened: '2020-01-01T00:00:00.000Z'
  }
]
```

**After: later**

```
[
  {
    Key: '22222222-2222-2222-2222-222222222222',
    Login: 'grace',
    Active: true,
    Opened: '2024-06-01T00:00:00.000Z'
  }
]
```

#### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/readonly_repository.rs#L171-L235)

```rust
let built = datasource();
let ds: &dyn Datasource = &*built;
execute_sql(
    &connection,
    &format!(
        "
    INSERT INTO user (id, username, is_enabled, created) VALUES
        ('{ADA}', 'ada', 1, '2020-01-01T00:00:00.000Z'),
        ('{GRACE}', 'grace', 1, '2024-06-01T00:00:00.000Z'),
        ('{LIN}', 'lin', 0, '2024-06-01T00:00:00.000Z');
    "
    ),
)
.await;
let mut options = create_readonly_repository_mapping_options::<UserAccount>();
options.entity_mapper = Some(std::sync::Arc::new(DictionaryMapper::new([(
    "UserAccount".to_string(),
    "user".to_string(),
)])));
options.property_mappings = Some(std::sync::Arc::new(DictionaryMapper::new([
    ("Key".to_string(), "id".to_string()),
    ("Login".to_string(), "username".to_string()),
    ("Active".to_string(), "is_enabled".to_string()),
    ("Opened".to_string(), "created".to_string()),
])));
let repo = ds.create_readonly_repository::<UserAccount>(Some(connection), Some(options));
let opened = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
let by_id = repo
    .read_single(
        Some(&Where::binary("Key", eq, Uuid::parse_str(ADA).unwrap())),
        None, None
    )
    .await
    .unwrap()
    .unwrap();
let by_login = repo
    .read(
        Some(&Where::binary("Login", eq, "ada").and("Active", eq, true)),
        None, None
    )
    .await
    .unwrap();
let later = repo
    .read(
        Some(&Where::binary("Active", eq, true).and("Opened", gt, opened)),
        None, None
    )
    .await
    .unwrap();
```

**After: by id**

```
{
  Key: '11111111-1111-1111-1111-111111111111',
  Login: 'ada',
  Active: 1,
  Opened: '2020-01-01T00:00:00.000Z'
}
```

**After: by login**

```
[
  {
    Key: '11111111-1111-1111-1111-111111111111',
    Login: 'ada',
    Active: 1,
    Opened: '2020-01-01T00:00:00.000Z'
  }
]
```

**After: later**

```
[
  {
    Key: '22222222-2222-2222-2222-222222222222',
    Login: 'grace',
    Active: 1,
    Opened: '2024-06-01T00:00:00.000Z'
  }
]
```

#### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/readonly.repository.integration.test.ts#L116-L151)

```typescript
const ds = new Datasource("sqlite", new SqliteDriver(), { DATASOURCE_URL: "sqlite:///:memory:" });
await executeSql(connection, `
  INSERT INTO user (id, username, is_enabled, created) VALUES
    ('${ada}', 'ada', 1, '2020-01-01T00:00:00.000Z'),
    ('${grace}', 'grace', 1, '2024-06-01T00:00:00.000Z'),
    ('${lin}', 'lin', 0, '2024-06-01T00:00:00.000Z');
`);
const repo = ds.createReadonlyRepository(UserAccount, connection, {
  entityMapper: new DictionaryMapper([["UserAccount", "user"]]),
  propertyMappings: new DictionaryMapper([
    ["Key", "id"],
    ["Login", "username"],
    ["Active", "is_enabled"],
    ["Opened", "created"],
  ]),
});
const opened = new Date("2024-01-01T00:00:00.000Z");
const byId = await repo.readSingle(new Where("Key", eq, ada));
const byLogin: UserAccount[] = [];
for await (const row of repo.read(new Where("Login", eq, "ada").and("Active", eq, true)))
  byLogin.push(row);
const later: UserAccount[] = [];
for await (const row of repo.read(new Where("Active", eq, true).and("Opened", gt, opened)))
  later.push(row);
```

**After: by id**

```
{
  Key: '11111111-1111-1111-1111-111111111111',
  Login: 'ada',
  Active: 1,
  Opened: '2020-01-01T00:00:00.000Z'
}
```

**After: by login**

```
[
  {
    Key: '11111111-1111-1111-1111-111111111111',
    Login: 'ada',
    Active: 1,
    Opened: '2020-01-01T00:00:00.000Z'
  }
]
```

**After: later**

```
[
  {
    Key: '22222222-2222-2222-2222-222222222222',
    Login: 'grace',
    Active: 1,
    Opened: '2024-06-01T00:00:00.000Z'
  }
]
```

Schema: [user-account](docs/examples.md#user-account)

```sql
CREATE TABLE user (id uuid, username string NOT NULL, is_enabled bool NOT NULL, created datetime NOT NULL, CONSTRAINT pk_user PRIMARY KEY (id))
```

### Call a stored procedure

#### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/RepositoryTests.cs#L107-L116)

```csharp
Parameter[] parameters = [new("id", 9)];
var call = new ExecuteStoredProcedureStatement("get_user", parameters);
var rows = new List<UserRow>();
await foreach (var row in repo.Read(call, parameters))
    rows.Add(row);
```

**After: rows**

```
[
  { id: 1, name: 'a' }
]
```

#### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L1526-L1542)

```rust
        &row
    ),
    no
);
```

**After: rows**

```
[
  { id: 1, name: 'a' }
]
```

#### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/repository.test.ts#L100-L107)

```typescript
command.reader = new MockReader([{ id: 1, name: "a" }]);
const parameters = [{ name: "id", value: 9 }];
const call = new ExecuteStoredProcedureStatement("get_user", parameters);
const rows: Cells[] = [];
for await (const row of repo.read(call, parameters)) rows.push(row);
```

**After: rows**

```
[
  { id: 1, name: 'a' }
]
```

### Read and read single

#### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/RepositoryTests.cs#L69-L76)

```csharp
await foreach (var row in repo.Read())
    rows.Add(row);
Assert.Single(rows);
```

**After: read**

```
[
  { id: 1, name: 'a' }
]
```

**After: read single**

```
{ id: 1, name: 'a' }
```

#### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L1563-L1569)

```rust

```

**After: read**

```
[
  { id: 1, name: 'a' }
]
```

**After: read single**

```
{ id: 1, name: 'a' }
```

#### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/repository.test.ts#L61-L71)

```typescript
const { repo, command } = create();
repo.addMiddleware({ beforeCommand() {}, afterCommand() {} });
command.reader = new MockReader([
  { id: 1, name: "a" },
]);
const rows: Cells[] = [];
for await (const row of repo.read()) rows.push(row);
```

**After: read**

```
[
  { id: 1, name: 'a' }
]
```

**After: read single**

```
{ id: 1, name: 'a' }
```

### Fill a lookup

#### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/ReadonlyRepositoryTests.cs#L72-L88)

```csharp
var person = new Row { ["name"] = "person" };
var gone = new Row { ["name"] = "gone" };
var typesConnection = await Setup(person, gone);
await typesConnection.CreateCommand().Execute("delete", [new Parameter("id", gone["id"])]);
var connection = await Setup(
    new Row { ["type_id"] = person["id"] },
    new Row { ["type_id"] = gone["id"] });
var parentsRepo = Open<Parent>(connection);
var typesRepo = Open<UserType>(typesConnection);
parentsRepo.AddEnriched(new("TypeName", "Type", "Id", "Name"), typesRepo);
var items = await parentsRepo.Read().ToListAsync();
```

#### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/tests/enriched.rs#L151-L157)

```rust
let mut parents_repo = mapped(connection, "Parent");
let types_repo = Arc::new(mapped(types_connection, "UserType"));
parents_repo.add_enriched(
    Enrichment::new("TypeName", "Type", "Id", Some("Name".into()), datasources::EnrichmentPropertyType::Object),
    types_repo,
    None,
);
```

#### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/ReadonlyRepository.test.ts#L94-L106)

```typescript
const person: Seed = { name: "person" };
const gone: Seed = { name: "gone" };
const typesConnection = await setup([person, gone]);
await remove(typesConnection, gone);
const connection = await setup([{ type_id: person.id }, { type_id: gone.id }]);
const parentsRepo = repo(Parent, connection);
const typesRepo = repo(UserType, typesConnection);
parentsRepo.addEnriched({ destinationProperty: "TypeName", sourceProperty: "Type", relatedKey: "Id", relatedProperty: "Name" }, typesRepo);
const items = await collectAsync(parentsRepo.read());
```

### Fill roles from one join

The repository type is `UsersView`. The select reads `User`, joined to `UserType`, `UserRole`, and `Role`. Nested enrichment groups each user's `Roles`.

#### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#L463-L545)

```csharp
var ds = new Datasource("sqlite", new SqliteDriver(), new Dictionary<string, string?> { ["DATASOURCE_URL"] = "sqlite:///:memory:" });
await Execute(connection, Seed());
var usersRepo = new ReadonlyRepository<UsersView>(connection, new ReadonlyRepositoryMappingOptions<UsersView>
{
    Entity = "User",
    EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string>
    {
        ["User"] = "user",
        ["UserType"] = "user_type",
        ["UserRole"] = "user_roles",
        ["Role"] = "roles",
    }),
    PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
    {
        ["Key"] = "id",
        ["Login"] = "username",
        ["Active"] = "is_enabled",
        ["Opened"] = "created",
        ["UserType"] = "user_type.name",
        ["UserTypeId"] = "user_type_id",
        ["UserId"] = "user_id",
        ["RoleId"] = "role_id",
        ["Name"] = "roles.name",
        ["Id"] = "id",
    }),
});
var joined = new SelectStatement(new()
{
    Table = "User",
    Fields =
    [
        "User.Key",
        "User.Login",
        "User.Active",
        "User.Opened",
        "User.UserType",
        "UserRole.UserId",
        "UserRole.RoleId",
        "Role.Name",
    ],
    Joins =
    [
        new Join(JoinKind.Left, "UserType", "UserType.Id", "User.UserTypeId"),
        new Join(JoinKind.Left, "UserRole", "UserRole.UserId", "User.Key"),
        new Join(JoinKind.Left, "Role", "Role.Id", "UserRole.RoleId"),
    ],
});
usersRepo.AddEnriched(
    new("Roles", "Key", "UserId", Type: EnrichmentPropertyType.List),
    usersRepo,
    new NestedToObjectEnrichmentStrategy<UsersView>(new TypeMap
    {
        Key = "Key",
        Fields = new Dictionary<string, TypeMapField>
        {
            ["Key"] = "Key",
            ["Login"] = "Login",
            ["Active"] = "Active",
            ["Opened"] = "Opened",
            ["UserType"] = "UserType",
            ["Roles"] = new TypeMapGroup
            {
                Key = "RoleId",
                Fields = new Dictionary<string, TypeMapField>
                {
                    ["RoleId"] = "RoleId",
                    ["UserId"] = "UserId",
                    ["Name"] = "Name",
                }
            }
        }
    }));
var items = new List<UsersView>();
await foreach (var item in usersRepo.Read(joined))
    items.Add(item);
var ada = Assert.Single(items, item => item.Login == "ada");
var grace = Assert.Single(items, item => item.Login == "grace");
var lin = Assert.Single(items, item => item.Login == "lin");
```

**Users with roles**

```
[
  {
    Key: '11111111-1111-1111-1111-111111111111',
    Login: 'ada',
    Active: true,
    Opened: '2020-01-01T00:00:00.000Z',
    UserType: 'admin',
    Roles: [
      { UserId: '11111111-1111-1111-1111-111111111111', Name: 'admin' },
      { UserId: '11111111-1111-1111-1111-111111111111', Name: 'editor' }
    ]
  },
  {
    Key: '22222222-2222-2222-2222-222222222222',
    Login: 'grace',
    Active: true,
    Opened: '2024-06-01T00:00:00.000Z',
    UserType: 'user',
    Roles: [
      { UserId: '22222222-2222-2222-2222-222222222222', Name: 'viewer' }
    ]
  },
  {
    Key: '33333333-3333-3333-3333-333333333333',
    Login: 'lin',
    Active: false,
    Opened: '2024-06-01T00:00:00.000Z',
    UserType: 'service',
    Roles: []
  }
]
```

#### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/readonly_repository.rs#L533-L642)

```rust
let built = datasource();
let ds: &dyn Datasource = &*built;
execute_sql(&connection, &seed()).await;
let mut options = ReadonlyRepositoryMappingOptions::default();
options.entity = "User".into();
options.entity_mapper = Some(std::sync::Arc::new(DictionaryMapper::new([
    ("User".into(), "user".into()),
    ("UserType".into(), "user_type".into()),
    ("UserRole".into(), "user_roles".into()),
    ("Role".into(), "roles".into()),
])));
options.property_mappings = Some(std::sync::Arc::new(DictionaryMapper::new([
    ("Key".into(), "id".into()),
    ("Login".into(), "username".into()),
    ("Active".into(), "is_enabled".into()),
    ("Opened".into(), "created".into()),
    ("UserType".into(), "user_type.name".into()),
    ("UserTypeId".into(), "user_type_id".into()),
    ("UserId".into(), "user_id".into()),
    ("RoleId".into(), "role_id".into()),
    ("Name".into(), "roles.name".into()),
    ("Id".into(), "id".into()),
])));
let mut users_repo = ReadonlyRepository::new(connection.clone(), options).unwrap();
users_repo.add_enriched(
    datasources::Enrichment::new("Roles", "Key", "UserId", None, datasources::EnrichmentPropertyType::List),
    Arc::new(ReadonlyRepository::memory(vec![])),
    Some(Arc::new(NestedToObjectEnrichmentStrategy::new(
        TypeMap::new(
            "Key",
            vec![
                ("Key".into(), TypeMapField::column("Key")),
                ("Login".into(), TypeMapField::column("Login")),
                ("Active".into(), TypeMapField::column("Active")),
                ("Opened".into(), TypeMapField::column("Opened")),
                ("UserType".into(), TypeMapField::column("UserType")),
                (
                    "Roles".into(),
                    TypeMapField::Group(TypeMapGroup::new(
                        "RoleId",
                        vec![
                            ("RoleId".into(), TypeMapField::column("RoleId")),
                            ("UserId".into(), TypeMapField::column("UserId")),
                            ("Name".into(), TypeMapField::column("Name")),
                        ],
                    )),
                ),
            ],
        ),
    ))),
);
let joined = joined_users();
let items = users_repo.read(Some(&joined), None, None).await.unwrap();
let view = |login: &str| {
    items
        .iter()
        .find(|item| item.get("Login") == Some(&Value::from(login)))
        .unwrap()
        .clone()
};
let ada = view("ada");
let grace = view("grace");
let lin = view("lin");
```

**Users with roles**

```
[
  {
    Key: '11111111-1111-1111-1111-111111111111',
    Login: 'ada',
    Active: true,
    Opened: '2020-01-01T00:00:00.000Z',
    UserType: 'admin',
    Roles: 'Name=admin,UserId=11111111-1111-1111-1111-111111111111|Name=editor,UserId=11111111-1111-1111-1111-111111111111'
  },
  {
    Key: '22222222-2222-2222-2222-222222222222',
    Login: 'grace',
    Active: true,
    Opened: '2024-06-01T00:00:00.000Z',
    UserType: 'user',
    Roles: 'Name=viewer,UserId=22222222-2222-2222-2222-222222222222'
  },
  {
    Key: '33333333-3333-3333-3333-333333333333',
    Login: 'lin',
    Active: false,
    Opened: '2024-06-01T00:00:00.000Z',
    UserType: 'service',
    Roles: ''
  }
]
```

#### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/users-view-roles.repository.integration.test.ts#L107-L165)

```typescript
const usersRepo = new ReadonlyRepository(connection, {
  type: UsersView,
  entity: "User",
  entityMapper: new DictionaryMapper([
    ["User", "user"],
    ["UserType", "user_type"],
    ["UserRole", "user_roles"],
    ["Role", "roles"],
  ]),
  propertyMappings: new DictionaryMapper([
    ["Key", "id"],
    ["Login", "username"],
    ["Active", "is_enabled"],
    ["Opened", "created"],
    ["UserType", "user_type.name"],
    ["UserTypeId", "user_type_id"],
    ["UserId", "user_id"],
    ["RoleId", "role_id"],
    ["Name", "roles.name"],
    ["Id", "id"],
  ]),
});
const joined = new SelectStatement({
  table: "User",
  fields: [
    "User.Key",
    "User.Login",
    "User.Active",
    "User.Opened",
    "User.UserType",
    "UserRole.UserId",
    "UserRole.RoleId",
    "Role.Name",
  ],
  joins: [
    Join.left("UserType", "UserType.Id", "User.UserTypeId"),
    Join.left("UserRole", "UserRole.UserId", "User.Key"),
    Join.left("Role", "Role.Id", "UserRole.RoleId"),
  ],
});
usersRepo.addEnriched(
  { destinationProperty: "Roles", sourceProperty: "Key", relatedKey: "UserId", type: "list" },
  usersRepo,
  new NestedToObjectEnrichmentStrategy<UsersView>({
    key: "Key",
    fields: {
      Key: "Key",
      Login: "Login",
      Active: "Active",
      Opened: "Opened",
      UserType: "UserType",
      Roles: {
        key: "RoleId",
        fields: { RoleId: "RoleId", UserId: "UserId", Name: "Name" },
      },
    },
  }),
);
const items: UsersView[] = [];
```

**Users with roles**

```
[
  {
    Key: '11111111-1111-1111-1111-111111111111',
    Login: 'ada',
    Active: true,
    Opened: '2020-01-01T00:00:00.000Z',
    UserType: 'admin',
    Roles: [
      { RoleId: 1, UserId: '11111111-1111-1111-1111-111111111111', Name: 'admin' },
      { RoleId: 2, UserId: '11111111-1111-1111-1111-111111111111', Name: 'editor' }
    ]
  },
  {
    Key: '22222222-2222-2222-2222-222222222222',
    Login: 'grace',
    Active: true,
    Opened: '2024-06-01T00:00:00.000Z',
    UserType: 'user',
    Roles: [
      { RoleId: 3, UserId: '22222222-2222-2222-2222-222222222222', Name: 'viewer' }
    ]
  },
  {
    Key: '33333333-3333-3333-3333-333333333333',
    Login: 'lin',
    Active: false,
    Opened: '2024-06-01T00:00:00.000Z',
    UserType: 'service',
    Roles: []
  }
]
```

Schema: [user-roles-read](docs/examples.md#user-roles-read)

```sql
CREATE TABLE user_type (id int, name string NOT NULL, CONSTRAINT pk_user_type PRIMARY KEY (id));
CREATE TABLE user (id uuid, username string NOT NULL, is_enabled bool NOT NULL, created datetime NOT NULL, user_type_id int NOT NULL, CONSTRAINT pk_user PRIMARY KEY (id));
CREATE TABLE roles (id int, name string NOT NULL, CONSTRAINT pk_roles PRIMARY KEY (id));
CREATE TABLE user_roles (user_id uuid NOT NULL, role_id int NOT NULL)
```

### Read with where clause and custom flat to nested custom object mapping options

The repository reads `Parent` with nested `Children`. Constructor [PropertyMappings](#propertymappings) stay in effect when a later read passes only an [EntityMapper](repositories.md#entitymapper) override. A normal lookup enrichment read runs first; the join read then maps entity names per call. The join select names entity types (`Parent`, `Child`). [NestedToObjectEnrichmentStrategy](docs/sections/nested-to-object-enrichment-strategy.md) collapses flat join rows using a [TypeMap](mappings.md#typemap).

#### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/ReadWithWhereClauseAndCustomFlatToNestedObjectMappingOptionsTests.cs#L46-L108)

```csharp
var connection = await Setup();
var typesRepo = TypesRepo(connection);
var type = await typesRepo.Create(new UserType { Name = "person" });
var parentsRepo = ParentsRepo(connection);
parentsRepo.AddEnriched(new("TypeName", "Type", "Id", "Name"), typesRepo);
await parentsRepo.Create(new Parent { Type = new UserType { Id = type.Id } });
var enriched = await parentsRepo.ReadSingle();

var row1 = new Row { ["Id"] = 1L, ["TypeName"] = "person", ["ChildId"] = 10L, ["Label"] = "a" };
var row2 = new Row { ["Id"] = 1L, ["TypeName"] = "person", ["ChildId"] = 11L, ["Label"] = "b" };
var mockConnection = new MockConnection { Provider = "memory" };
var command = new MockCommand(mockConnection);
mockConnection.Command = command;
var joinRepo = ParentsRepo(mockConnection);
joinRepo.AddEnriched(new("TypeName", "Type", "Id", "Name"), typesRepo);
joinRepo.AddEnriched(
    new("Children", "Id", "ParentId", Type: EnrichmentPropertyType.List),
    joinRepo,
    new NestedToObjectEnrichmentStrategy<Parent>(new TypeMap
    {
        Key = "Id",
        Fields = new Dictionary<string, TypeMapField>
        {
            ["Id"] = "Id",
            ["TypeName"] = "TypeName",
            ["Children"] = new TypeMapGroup
            {
                Key = "ChildId",
                Fields = new Dictionary<string, TypeMapField>
                {
                    ["Id"] = "ChildId",
                    ["Label"] = "Label",
                },
            },
        },
    }));
var join = new SelectStatement(new()
{
    Table = "Parent",
    Fields =
    [
        "Parent.Id AS Id",
        "Parent.TypeName AS TypeName",
        "Child.Id AS ChildId",
        "Child.Label AS Label",
    ],
    Joins = [Join.Left("Child", "Parent.Id", "Child.ParentId")],
    Where = new Where(new Operator.Equals("Parent.Id", 1)),
});
command.Reader = new MockReader<Row>([row1, row2]);
var items = await joinRepo.Read(
    join,
    null,
    new ReadonlyRepositoryMappingOptions<Parent> { EntityMapper = JoinEntity }).ToListAsync();
```

**Parent from join**

```
[
  {
    Id: 1,
    TypeName: 'person',
    Children: [
      { Id: 10, Label: 'a' },
      { Id: 11, Label: 'b' }
    ]
  }
]
```

#### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/tests/read_with_where_clause_and_custom_flat_to_nested_object_mapping_options.rs#L60-L170)

```rust
let connection = MemoryDriver.connection(None);
connection.open().await.unwrap();
let types = Arc::new(types_repo(connection.clone()));
let mut created_type = record(&[("Name", Value::from("person"))]);
let created_type = types.create(&mut created_type).await.unwrap();
let mut parents = parents_repo(connection);
parents.add_enriched(
    Enrichment::new(
        "TypeName",
        "Type",
        "Id",
        Some("Name".into()),
        EnrichmentPropertyType::Object,
    ),
    as_readonly(types.clone()),
    None,
);
let mut parent = record(&[("Type", created_type.get("Id").unwrap().clone())]);
parents.create(&mut parent).await.unwrap();
let enriched = parents.read_single(None, None, None).await.unwrap();

let command = Arc::new(MockCommand::new());
command.reader_rows.lock().unwrap().extend([
    record(&[
        ("Id", Value::Int(1)),
        ("TypeName", Value::from("person")),
        ("ChildId", Value::Int(10)),
        ("Label", Value::from("a")),
    ]),
    record(&[
        ("Id", Value::Int(1)),
        ("TypeName", Value::from("person")),
        ("ChildId", Value::Int(11)),
        ("Label", Value::from("b")),
    ]),
]);
let mut join_repo = parents_repo(MockConnection::with_command_provider(
    command,
    "memory",
));
join_repo.add_enriched(
    Enrichment::new(
        "TypeName",
        "Type",
        "Id",
        Some("Name".into()),
        EnrichmentPropertyType::Object,
    ),
    as_readonly(types),
    None,
);
join_repo.add_enriched(
    Enrichment::new(
        "Children",
        "Id",
        "ParentId",
        None,
        EnrichmentPropertyType::List,
    ),
    Arc::new(CRUDRepository::memory(vec![])),
    Some(Arc::new(NestedToObjectEnrichmentStrategy::new(TypeMap::new(
        "Id",
        vec![
            ("Id".into(), TypeMapField::column("Id")),
            ("TypeName".into(), TypeMapField::column("TypeName")),
            (
                "Children".into(),
                TypeMapField::Group(TypeMapGroup::new(
                    "ChildId",
                    vec![
                        ("Id".into(), TypeMapField::column("ChildId")),
                        ("Label".into(), TypeMapField::column("Label")),
                    ],
                )),
            ),
        ],
    )))),
);
let where_clause = Where::new(Operator::equals("Parent.Id", Value::Int(1)));
let join = SelectStatement::new(SelectStatementOptions {
    joins: &[Join::left("Child", "Parent.Id", "Child.ParentId")],
    where_clause: Some(&where_clause),
    ..SelectStatementOptions::new(
        "Parent",
        &[
            "Parent.Id AS Id",
            "Parent.TypeName AS TypeName",
            "Child.Id AS ChildId",
            "Child.Label AS Label",
        ],
    )
});
let mut options = ReadonlyRepositoryMappingOptions::default();
options.entity_mapper = Some(Arc::new(join_entity()));
let items = join_repo
    .read(Some(&join), None, Some(&options))
    .await
    .unwrap();
```

**Parent from join**

```
[
  {
    Id: 1,
    TypeName: 'person',
    Children: [
      { Id: 10, Label: 'a' },
      { Id: 11, Label: 'b' }
    ]
  }
]
```

#### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/readWithWhereClauseAndCustomFlatToNestedObjectMappingOptions.test.ts#L18-L125)

```typescript
class UserType {
  Id = 0;
  Name = "";
}

class Parent {
  Id = 0;
  Type = new UserType();
  TypeName = "";
  Children: Child[] = [];
}

class Child {
  Id = 0;
  ParentId = 0;
  Label = "";
}

type Seed = Record<string, Scalar>;

const snake = new ChangeCasingMapper("pascalCase", "snakeCase");
const references = new DictionaryMapper<string>([["Type", "type_id"]]);
const columns: IMapper<string, string> = new TransformMapper(
  (name) => {
    const mapped = references.map(name);
    return mapped !== name ? mapped : snake.map(name);
  },
  (name) => {
    const from = references.reverseMap(name);
    return from !== name ? from : snake.reverseMap(name);
  },
);

const joinEntity = new DictionaryMapper([
  ["Parent", "parent"],
  ["Child", "child"],
]);

const setup = async (): Promise<IConnection> => {
  const connection = new MemoryDriver().connection();
  await connection.open();
  return connection;
};

const typesRepo = (connection: IConnection) =>
  new CRUDRepository(connection, { type: UserType, ids: ["Id"], propertyMappings: columns });

const parentsRepo = (connection: IConnection) =>
  new CRUDRepository(connection, { type: Parent, ids: ["Id"], propertyMappings: columns });

const connection = await setup();
const types = typesRepo(connection);
const type = await types.create(Object.assign(new UserType(), { Name: "person" }));
const parents = parentsRepo(connection);
parents.addEnriched(
  { destinationProperty: "TypeName", sourceProperty: "Type", relatedKey: "Id", relatedProperty: "Name" },
  types,
);
await parents.create(Object.assign(new Parent(), { Type: Object.assign(new UserType(), { Id: type.Id }) }));
const enriched = await parents.readSingle();

const row1: Seed = { Id: 1, TypeName: "person", ChildId: 10, Label: "a" };
const row2: Seed = { Id: 1, TypeName: "person", ChildId: 11, Label: "b" };
const mockConnection = new MockConnection({ provider: "memory" });
const command = new MockCommand(mockConnection);
mockConnection.command = command;
const joinParents = parentsRepo(mockConnection);
joinParents.addEnriched(
  { destinationProperty: "TypeName", sourceProperty: "Type", relatedKey: "Id", relatedProperty: "Name" },
  types,
);
joinParents.addEnriched(
  { destinationProperty: "Children", sourceProperty: "Id", relatedKey: "ParentId", type: "list" },
  joinParents,
  new NestedToObjectEnrichmentStrategy<Parent>({
    key: "Id",
    fields: {
      Id: "Id",
      TypeName: "TypeName",
      Children: {
        key: "ChildId",
        fields: { Id: "ChildId", Label: "Label" },
      },
    },
  }),
);
const join = new SelectStatement({
  table: "Parent",
  fields: [
    "Parent.Id AS Id",
    "Parent.TypeName AS TypeName",
    "Child.Id AS ChildId",
    "Child.Label AS Label",
  ],
  joins: [Join.left("Child", "Parent.Id", "Child.ParentId")],
  where: new Where(new Operator.Equals("Parent.Id", 1)),
});
command.reader = new MockReader([row1, row2]);
const items = await collectAsync(
  joinParents.read(join, undefined, { entityMapper: joinEntity }),
);
```

**Parent from join**

```
[
  {
    Id: 1,
    TypeName: 'person',
    Children: [
      { Id: 10, Label: 'a' },
      { Id: 11, Label: 'b' }
    ]
  }
]
```
