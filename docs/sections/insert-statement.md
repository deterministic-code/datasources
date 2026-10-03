# Insert Statement

`INSERT` text, or a compiled `INSERT` from a table and fields.

## Classes

::: reference
### typescript
#### IInsertStatement

```typescript
interface IInsertStatement extends IStatement {}
```

#### InsertStatement

```typescript
class InsertStatement implements IInsertStatement
```

**Constructor**

##### InsertStatement(text)

Wraps insert text.

```typescript
constructor(text: string)
```

- `text` — command text

##### InsertStatement(table, fields)

Compiles an `INSERT`.

```typescript
constructor(table: string, fields: Field[])
```

- `table` — table name
- `fields` — [Field](#field) columns and bound values

**Methods**

##### accept(visitor)

Dispatches to a statement visitor that renders command text.

```typescript
accept(visitor: IStatementVisitor): string
```

- `visitor` — [IStatementVisitor](./statements.md#istatementvisitor)

**Returns** `string`

#### Field

```typescript
interface Field
```

**Properties**

##### name

Column name.

```typescript
name: string
```

##### value

Bound [Scalar](./where-clause.md#scalar).

```typescript
value: Scalar
```

### csharp
#### IInsertStatement

```csharp
public interface IInsertStatement : IStatement
```

#### InsertStatement

```csharp
public sealed record InsertStatement : IInsertStatement
```

**Constructor**

##### InsertStatement(text)

Wraps insert text.

```csharp
public InsertStatement(string text)
```

- `text` — command text

##### InsertStatement(table, fields)

Compiles an `INSERT`.

```csharp
public InsertStatement(string table, IReadOnlyList<Parameter> fields)
```

- `table` — table name
- `fields` — [Parameter](./command.md#parameter) columns and bound values

**Methods**

##### Accept(visitor)

Dispatches to a statement visitor that renders command text.

```csharp
public string Accept(IStatementVisitor visitor)
```

- `visitor` — [IStatementVisitor](./statements.md#istatementvisitor)

**Returns** `string`

### rust
#### InsertStatement

```rust
pub struct InsertStatement
```

**Constructor**

##### text(text)

Wraps insert text.

```rust
pub fn text(text: impl Into<String>) -> Self
```

- `text` — command text

**Returns** [InsertStatement](#insertstatement)

##### new(table, fields)

Compiles an `INSERT`.

```rust
pub fn new(table: impl AsRef<str>, fields: &[Parameter]) -> Self
```

- `table` — table name
- `fields` — [Parameter](./command.md#parameter) columns and bound values

**Returns** [InsertStatement](#insertstatement)

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

Compiled form is `INSERT INTO table (cols) VALUES ($cols)`. [Command](./command.md) uses {{StatementTextRenderer.Render|StatementTextRenderer.render|StatementTextRenderer::render}} only.

## Examples

### Compile an INSERT

::: example
```csharp csharp/Datasources.Tests/StatementCompileTests.cs#CompilesAnInsert
```
```rust rust/datasources/src/tests/statement_compile.rs#compiles_an_insert
```
```typescript typescript/datasources/src/statements/StatementCompile.test.ts#compiles-an-insert
```

```output [After: insert]
INSERT INTO users (id, name) VALUES ($id, $name)
```
:::
