# Create Index Statement

`CREATE INDEX` text, or a compiled `CREATE [UNIQUE] INDEX` from a name, table, and columns.

## Classes

::: reference
### typescript
#### ICreateIndexStatement

```typescript
interface ICreateIndexStatement extends IStatement {}
```

#### CreateIndexStatement

```typescript
class CreateIndexStatement implements ICreateIndexStatement
```

**Constructor**

##### CreateIndexStatement(text)

Wraps create-index text.

```typescript
constructor(text: string)
```

- `text` — command text

##### CreateIndexStatement(name, table, columns, unique?)

Compiles a `CREATE INDEX`.

```typescript
constructor(name: string, table: string, columns: string[], unique?: boolean)
```

- `name` — index name
- `table` — table name
- `columns` — indexed columns
- `unique` — `CREATE UNIQUE INDEX` when true

**Methods**

##### accept(visitor)

Dispatches to a statement visitor that renders command text.

```typescript
accept(visitor: IStatementVisitor): string
```

- `visitor` — [IStatementVisitor](./statements.md#istatementvisitor)

**Returns** `string`

### csharp
#### ICreateIndexStatement

```csharp
public interface ICreateIndexStatement : IStatement
```

#### CreateIndexStatement

```csharp
public sealed record CreateIndexStatement : ICreateIndexStatement
```

**Constructor**

##### CreateIndexStatement(text)

Wraps create-index text.

```csharp
public CreateIndexStatement(string text)
```

- `text` — command text

##### CreateIndexStatement(name, table, columns, unique?)

Compiles a `CREATE INDEX`.

```csharp
public CreateIndexStatement(string name, string table, IReadOnlyList<string> columns, bool unique = false)
```

- `name` — index name
- `table` — table name
- `columns` — indexed columns
- `unique` — `CREATE UNIQUE INDEX` when true

**Methods**

##### Accept(visitor)

Dispatches to a statement visitor that renders command text.

```csharp
public string Accept(IStatementVisitor visitor)
```

- `visitor` — [IStatementVisitor](./statements.md#istatementvisitor)

**Returns** `string`

### rust
#### CreateIndexStatement

```rust
pub struct CreateIndexStatement
```

**Constructor**

##### text(text)

Wraps create-index text.

```rust
pub fn text(text: impl Into<String>) -> Self
```

- `text` — command text

**Returns** [CreateIndexStatement](#createindexstatement)

##### new(name, table, columns, unique)

Compiles a `CREATE INDEX`.

```rust
pub fn new(name: impl AsRef<str>, table: impl AsRef<str>, columns: &[&str], unique: bool) -> Self
```

- `name` — index name
- `table` — table name
- `columns` — indexed columns
- `unique` — `CREATE UNIQUE INDEX` when true

**Returns** [CreateIndexStatement](#createindexstatement)

**Methods**

##### accept()

Dispatches through the renderer's statement visitor.

```rust
fn accept(&self, text: &StatementTextRenderer) -> String
```

- `text` — [StatementTextRenderer](./statements.md#statementtextrenderer)

**Returns** `String`
:::

## Remarks

Compiled form is `CREATE INDEX name ON table (cols)` or `CREATE UNIQUE INDEX name ON table (cols)`. Table and column names are interpolated as given. [Command](./command.md) uses {{StatementTextRenderer.Render|StatementTextRenderer.render|StatementTextRenderer::render}} only.

## Examples

### Compile a CREATE INDEX

::: example
```csharp csharp/Datasources.Tests/StatementCompileTests.cs#CompilesACreateIndex
```
```rust rust/datasources/src/tests/statement_compile.rs#compiles_a_create_index
```
```typescript typescript/datasources/src/statements/StatementCompile.test.ts#compiles-a-create-index
```

```output [After: create index]
CREATE INDEX idx_orders_user ON orders (user_id)
```
:::

### Compile a CREATE UNIQUE INDEX

::: example
```csharp csharp/Datasources.Tests/StatementCompileTests.cs#CompilesACreateUniqueIndex
```
```rust rust/datasources/src/tests/statement_compile.rs#compiles_a_create_unique_index
```
```typescript typescript/datasources/src/statements/StatementCompile.test.ts#compiles-a-create-unique-index
```

```output [After: create unique index]
CREATE UNIQUE INDEX uq_orders_code ON orders (code)
```
:::
