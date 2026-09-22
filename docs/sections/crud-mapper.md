# CRUDMapper

Collection of mappers for a CRUD repository. Entity and column mappers run when the repository is built. Per-statement mappers rewrite a finished query / insert / update / delete. [Rows](#crudmapper-rows) is the read-row mapper; [CRUDRepository.Mapper](./crud-repository.md) aliases it.

## Classes

::: reference
### typescript
#### CRUDMapper

```typescript
class CRUDMapper<T>
```

**Constructor**

##### CRUDMapper(options?)

Builds a CRUD mapper. Omitted fields default to identity name mappers and an identity [rows](#crudmapper-rows) mapper.

```typescript
constructor(options?: CRUDMapperOptions<T>)
```

- `options` — [CRUDMapperOptions](#crudmapperoptions)

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

Rewrites a finished [IQuery](./query-statement.md#iquery).

```typescript
readonly query?: IMapper<IQuery, IQuery>
```

##### insert

Rewrites a finished [IInsertStatement](./insert-statement.md#iinsertstatement).

```typescript
readonly insert?: IMapper<IInsertStatement, IInsertStatement>
```

##### update

Rewrites a finished [IUpdateStatement](./update-statement.md#iupdatestatement).

```typescript
readonly update?: IMapper<IUpdateStatement, IUpdateStatement>
```

##### delete

Rewrites a finished [IDeleteStatement](./delete-statement.md#ideletestatement).

```typescript
readonly delete?: IMapper<IDeleteStatement, IDeleteStatement>
```

##### return

Rewrites a finished [IReturnStatement](./return-statement.md#ireturnstatement).

```typescript
readonly return?: IMapper<IReturnStatement, IReturnStatement>
```

##### rows

Read-row [Mapper](./mappers.md).

```typescript
readonly rows: IMapper<T, T>
```

##### create

Maps `Response.result` onto a created item after insert. Default writes the value onto `id`.

```typescript
readonly create: IMapper<Scalar, T>
```

#### CRUDMapperOptions

```typescript
type CRUDMapperOptions<T>
```

Constructor bag for [CRUDMapper](#crudmapper).

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

Read-row [Mapper](./mappers.md).

```typescript
rows?: IMapper<T, T>
```

##### create

Maps `Response.result` onto a created item after insert.

```typescript
create?: IMapper<Scalar, T>
```

##### query

Rewrites a finished [IQuery](./query-statement.md#iquery).

```typescript
query?: IMapper<IQuery, IQuery>
```

##### insert

Rewrites a finished [IInsertStatement](./insert-statement.md#iinsertstatement).

```typescript
insert?: IMapper<IInsertStatement, IInsertStatement>
```

##### update

Rewrites a finished [IUpdateStatement](./update-statement.md#iupdatestatement).

```typescript
update?: IMapper<IUpdateStatement, IUpdateStatement>
```

##### delete

Rewrites a finished [IDeleteStatement](./delete-statement.md#ideletestatement).

```typescript
delete?: IMapper<IDeleteStatement, IDeleteStatement>
```

### csharp
#### CRUDMapper

```csharp
public sealed class CRUDMapper<T> where T : new()
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

Rewrites a finished [IQuery](./query-statement.md#iquery).

```csharp
IMapper<IQuery, IQuery>? Query { get; init; }
```

##### Insert

Rewrites a finished [IInsertStatement](./insert-statement.md#iinsertstatement).

```csharp
IMapper<IInsertStatement, IInsertStatement>? Insert { get; init; }
```

##### Update

Rewrites a finished [IUpdateStatement](./update-statement.md#iupdatestatement).

```csharp
IMapper<IUpdateStatement, IUpdateStatement>? Update { get; init; }
```

##### Delete

Rewrites a finished [IDeleteStatement](./delete-statement.md#ideletestatement).

```csharp
IMapper<IDeleteStatement, IDeleteStatement>? Delete { get; init; }
```

##### Return

Rewrites a finished [IReturnStatement](./return-statement.md#ireturnstatement).

```csharp
IMapper<IReturnStatement, IReturnStatement>? Return { get; init; }
```

##### Rows

Read-row [Mapper](./mappers.md).

```csharp
IMapper<T, T> Rows { get; init; }
```

##### Create

Maps `Response.Result` onto a created item after insert. Default writes the value onto `id` / `Id`.

```csharp
IMapper<Value, T> Create { get; init; }
```

### rust
#### CRUDMapper

```rust
pub struct CRUDMapper
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

##### return_

Rewrites finished return-call SQL.

```rust
pub return_: Option<Arc<TransformMapper<String, String>>>
```

##### rows

Read-row [TransformMapper](./transform-mapper.md).

```rust
pub rows: TransformMapper<Row, Row>
```

##### create

Maps `Response.result` onto a created row after insert. Default writes the value onto `id`.

```rust
pub create: TransformMapper<Value, Row>
```

**Methods**

##### identity(source, destination)

Identity CRUD mapper. Source and destination labels are unused.

```rust
pub fn identity(_source: impl Into<String>, _destination: impl Into<String>) -> Self
```

- `source` — unused source label
- `destination` — unused destination label

**Returns** [CRUDMapper](#crudmapper)

##### map_sql(mapper, text)

Applies an optional SQL-text mapper.

```rust
pub fn map_sql(mapper: &Option<Arc<TransformMapper<String, String>>>, text: String) -> String
```

- `mapper` — optional SQL-text [TransformMapper](./transform-mapper.md)
- `text` — finished SQL

**Returns** `String`
:::

## Remarks

Apply mapping in either or both places:

1. **Builder** — `RepositoryBuilder` maps `Entity` / `entity` and `Columns` / `columns` into `RepositoryStatements`.
2. **Per-statement** — `Query` / `Insert` / `Update` / `Delete` / `Return` rewrite the finished statement.

C# and TypeScript rewrite statement objects. Rust `query` / `insert` / `update` / `delete` / `return_` are [TransformMapper](./transform-mapper.md)`<String, String>` over SQL text via `CRUDMapper::map_sql`.

`Repository.Mapper` / `mapper` aliases `Rows` / `rows`. Insert always omits configured id columns. After execute, `Create` / `create` maps `Response.Result` onto the created item.

## Examples

### Rewrite a finished statement

::: example
```csharp csharp/Datasources.Tests/CRUDMapperTests.cs#L56-L81
```
```rust rust/datasources/src/tests/mod.rs#L1615-L1677
```
```typescript typescript/datasources/src/CRUDMapper.test.ts#L22-L52
```
:::

### Rewrite a return call

::: example
```csharp csharp/Datasources.Tests/CRUDMapperTests.cs#L84-L95
```
```rust rust/datasources/src/tests/mod.rs#L1285-L1294
```
```typescript typescript/datasources/src/CRUDMapper.test.ts#L54-L64
```
:::
