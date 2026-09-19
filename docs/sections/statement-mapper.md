# StatementMapper

Mapper for repository statements. `Map` / `ReverseMap` only touch `Rows`. Entity and column mappers run when the repository is built. Per-statement mappers rewrite a finished query / insert / update / delete.

## Classes

::: reference
### typescript
#### StatementMapper

```typescript
class StatementMapper<T> implements IMapper<T, T>
```

**Constructor**

##### StatementMapper(options?)

Builds a statement mapper. Omitted fields default to identity name mappers and an identity [rows](#statementmapper-rows) mapper.

```typescript
constructor(options?: StatementMapperOptions<T>)
```

- `options` — [StatementMapperOptions](#statementmapperoptions)

**Properties**

##### entity

Maps the table or entity name when the repository is built.

```typescript
readonly entity: IMapper<string, string>
```

##### columns

Maps column names when the repository is built.

```typescript
readonly columns: IMapper<string, string>
```

##### query

Rewrites a finished query.

```typescript
readonly query?: IMapper<IQuery, IQuery>
```

##### insert

Rewrites a finished insert.

```typescript
readonly insert?: IMapper<IInsertStatement, IInsertStatement>
```

##### update

Rewrites a finished update.

```typescript
readonly update?: IMapper<IUpdateStatement, IUpdateStatement>
```

##### delete

Rewrites a finished delete.

```typescript
readonly delete?: IMapper<IDeleteStatement, IDeleteStatement>
```

##### storedProcedure

Rewrites a finished stored-procedure statement.

```typescript
readonly storedProcedure?: IMapper<IStoredProcedureStatement, IStoredProcedureStatement>
```

##### rows

Get-row Mapper. [map](#statementmapper-map-value) / [reverseMap](#statementmapper-reversemap-value) delegate here.

```typescript
readonly rows: IMapper<T, T>
```

**Methods**

##### map(value)

Maps a row through [rows](#statementmapper-rows).

```typescript
map(value: T): T
```

- `value` — converted row

**Returns** `T`

##### reverseMap(value)

Reverse-maps a row through [rows](#statementmapper-rows).

```typescript
reverseMap(value: T): T
```

- `value` — shaped row

**Returns** `T`

#### StatementMapperOptions

```typescript
type StatementMapperOptions<T>
```

Constructor bag for [StatementMapper](#statementmapper).

**Properties**

##### entity

Maps the table or entity name.

```typescript
entity?: IMapper<string, string>
```

##### columns

Maps column names.

```typescript
columns?: IMapper<string, string>
```

##### rows

Get-row Mapper.

```typescript
rows?: IMapper<T, T>
```

##### query

Rewrites a finished query.

```typescript
query?: IMapper<IQuery, IQuery>
```

##### insert

Rewrites a finished insert.

```typescript
insert?: IMapper<IInsertStatement, IInsertStatement>
```

##### update

Rewrites a finished update.

```typescript
update?: IMapper<IUpdateStatement, IUpdateStatement>
```

##### delete

Rewrites a finished delete.

```typescript
delete?: IMapper<IDeleteStatement, IDeleteStatement>
```

##### storedProcedure

Rewrites a finished stored-procedure statement.

```typescript
storedProcedure?: IMapper<IStoredProcedureStatement, IStoredProcedureStatement>
```

### csharp
#### StatementMapper

```csharp
public sealed class StatementMapper<T> : IMapper<T, T> where T : new()
```

**Properties**

##### Entity

Maps the table or entity name when the repository is built.

```csharp
IMapper<string, string> Entity { get; init; }
```

##### Columns

Maps column names when the repository is built.

```csharp
IMapper<string, string> Columns { get; init; }
```

##### Query

Rewrites a finished query.

```csharp
IMapper<IQuery, IQuery>? Query { get; init; }
```

##### Insert

Rewrites a finished insert.

```csharp
IMapper<IInsertStatement, IInsertStatement>? Insert { get; init; }
```

##### Update

Rewrites a finished update.

```csharp
IMapper<IUpdateStatement, IUpdateStatement>? Update { get; init; }
```

##### Delete

Rewrites a finished delete.

```csharp
IMapper<IDeleteStatement, IDeleteStatement>? Delete { get; init; }
```

##### StoredProcedure

Rewrites a finished stored-procedure statement.

```csharp
IMapper<IStoredProcedureStatement, IStoredProcedureStatement>? StoredProcedure { get; init; }
```

##### Rows

Get-row Mapper. [Map](#statementmapper-map-value) / [ReverseMap](#statementmapper-reversemap-value) delegate here.

```csharp
IMapper<T, T> Rows { get; init; }
```

**Methods**

##### Map(value)

Maps a row through [Rows](#statementmapper-rows).

```csharp
T Map(T value)
```

- `value` — converted row

**Returns** `T`

##### ReverseMap(value)

Reverse-maps a row through [Rows](#statementmapper-rows).

```csharp
T ReverseMap(T value)
```

- `value` — shaped row

**Returns** `T`

### rust
#### StatementMapper

```rust
pub struct StatementMapper
```

**Properties**

##### entity

Maps the table or entity name when the repository is built.

```rust
pub entity: TransformMapper<String, String>
```

##### columns

Maps column names when the repository is built.

```rust
pub columns: TransformMapper<String, String>
```

##### query

Rewrites finished query SQL.

```rust
pub query: Option<Arc<TransformMapper<String, String>>>
```

##### insert

Rewrites finished insert SQL.

```rust
pub insert: Option<Arc<TransformMapper<String, String>>>
```

##### update

Rewrites finished update SQL.

```rust
pub update: Option<Arc<TransformMapper<String, String>>>
```

##### delete

Rewrites finished delete SQL.

```rust
pub delete: Option<Arc<TransformMapper<String, String>>>
```

##### stored_procedure

Rewrites finished stored-procedure SQL.

```rust
pub stored_procedure: Option<Arc<TransformMapper<String, String>>>
```

##### rows

Get-row mapper. [map](#statementmapper-map-value) / [reverse_map](#statementmapper-reverse_map-value) delegate here.

```rust
pub rows: TransformMapper<Row, Row>
```

**Methods**

##### identity(source, destination)

Identity statement mapper. Source and destination labels are unused.

```rust
pub fn identity(_source: impl Into<String>, _destination: impl Into<String>) -> Self
```

- `source` — unused source label
- `destination` — unused destination label

**Returns** [StatementMapper](#statementmapper)

##### map_sql(mapper, text)

Applies an optional SQL-text mapper.

```rust
pub fn map_sql(mapper: &Option<Arc<TransformMapper<String, String>>>, text: String) -> String
```

- `mapper` — optional SQL-text [TransformMapper](./transform-mapper.md)
- `text` — finished SQL

**Returns** `String`

##### map(value)

Maps a row through [rows](#statementmapper-rows).

```rust
fn map(&self, value: Row) -> Row
```

- `value` — converted row

**Returns** `Row`

##### reverse_map(value)

Reverse-maps a row through [rows](#statementmapper-rows).

```rust
fn reverse_map(&self, value: Row) -> Row
```

- `value` — shaped row

**Returns** `Row`
:::

## Remarks

Apply statement mapping in either or both places:

1. **Builder** — `RepositoryBuilder` maps `Entity` / `entity` and `Columns` / `columns` into `RepositoryStatements`.
2. **Per-statement** — `Query` / `Insert` / `Update` / `Delete` rewrite the finished statement.

C# and TypeScript statement mappers rewrite statement objects. Rust `query` / `insert` / `update` / `delete` / `stored_procedure` are [TransformMapper](./transform-mapper.md)`<String, String>` over SQL text via `StatementMapper::map_sql`.

`Repository.Mapper` / `mapper` aliases `Rows` / `rows`.

## Examples

### Rewrite a finished statement

::: example
```csharp csharp/Datasources.Tests/StatementMapperTests.cs#L55-L81
```
```rust rust/datasources/src/tests/mod.rs#L794-L804
```
```typescript typescript/datasources/src/mappers/StatementMapper.test.ts#L27-L59
```
:::
