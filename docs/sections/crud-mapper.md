# CRUDMapperOptions

Collection of mappers for a CRUD repository. Entity and column mappers run when the repository is built. Per-statement mappers rewrite a finished query / insert / update / delete. [Rows](#crudmapperoptions-rows) is the read-row mapper; [CRUDRepository.Mapper](./crud-repository.md) aliases it.

## Classes

::: reference
### typescript
#### CRUDMapperOptions

```typescript
class CRUDMapperOptions<T>
```

**Constructor**

##### CRUDMapperOptions(options?)

Builds a CRUD mapper. Omitted fields default to identity name mappers and an identity [rows](#crudmapperoptions-rows) mapper.

```typescript
constructor(options?: {
  entity?: IMapper<string, string>;
  columns?: IMapper<string, string>;
  rows?: IMapper<T, T>;
  query?: IMapper<IQuery, IQuery>;
  insert?: IMapper<IInsertStatement, IInsertStatement>;
  update?: IMapper<IUpdateStatement, IUpdateStatement>;
  delete?: IMapper<IDeleteStatement, IDeleteStatement>;
  return?: IMapper<IReturnStatement, IReturnStatement>;
  create?: IMapper<Scalar, T>;
})
```

- `options` — optional field overrides

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

### csharp
#### CRUDMapperOptions

```csharp
public sealed class CRUDMapperOptions<T> where T : new()
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
#### CRUDMapperOptions

```rust
pub struct CRUDMapperOptions
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

**Returns** [CRUDMapperOptions](#crudmapperoptions)

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

C# and TypeScript rewrite statement objects. Rust `query` / `insert` / `update` / `delete` / `return_` are [TransformMapper](./transform-mapper.md)`<String, String>` over SQL text via `CRUDMapperOptions::map_sql`.

`Repository.Mapper` / `mapper` aliases `Rows` / `rows`. Insert always omits configured id columns. After execute, `Create` / `create` maps `Response.Result` onto the created item.

## Examples

### Rewrite a finished statement

::: example
```csharp csharp/Datasources.Tests/CRUDMapperOptionsTests.cs#L56-L81
```
```rust rust/datasources/src/tests/mod.rs#L1701-L1763
```
```typescript typescript/datasources/src/CRUDMapperOptions.test.ts#L22-L52
```
:::

### Rewrite a return call

::: example
```csharp csharp/Datasources.Tests/CRUDMapperOptionsTests.cs#L84-L95
```
```rust rust/datasources/src/tests/mod.rs#L1383-L1392
```
```typescript typescript/datasources/src/CRUDMapperOptions.test.ts#L54-L64
```
:::
