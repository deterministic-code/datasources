# Update Statement

`UPDATE` text, or a compiled `UPDATE` from a table, fields, and optional [IWhere](./where-clause.md#iwhere).

## Classes

::: reference
### typescript
#### IUpdateStatement

```typescript
interface IUpdateStatement extends IStatement
```

**Properties**

##### where

Optional [IWhere](./where-clause.md#iwhere) used to build this update.

```typescript
readonly where?: IWhere
```

#### UpdateStatement

```typescript
class UpdateStatement implements IUpdateStatement
```

**Constructor**

##### UpdateStatement(text, where?)

Wraps update text and an optional [IWhere](./where-clause.md#iwhere).

```typescript
constructor(text: string, where?: IWhere)
```

- `text` — command text
- `where` — optional [IWhere](./where-clause.md#iwhere)

##### UpdateStatement(table, fields, where?)

Compiles an `UPDATE`.

```typescript
constructor(table: string, fields: Field[], where?: IWhere)
```

- `table` — table name
- `fields` — [Field](./insert-statement.md#field) columns and bound values
- `where` — optional [IWhere](./where-clause.md#iwhere)

**Properties**

##### where

Optional [IWhere](./where-clause.md#iwhere) used to build this update.

```typescript
readonly where?: IWhere
```

**Methods**

##### accept(visitor)

Dispatches to a statement visitor that renders command text.

```typescript
accept(visitor: IStatementVisitor): string
```

- `visitor` — [IStatementVisitor](./statements.md#istatementvisitor)

**Returns** `string`

### csharp
#### IUpdateStatement

```csharp
public interface IUpdateStatement : IStatement
```

**Properties**

##### Where

Optional [IWhere](./where-clause.md#iwhere) used to build this update.

```csharp
IWhere? Where { get; }
```

#### UpdateStatement

```csharp
public sealed record UpdateStatement : IUpdateStatement
```

**Constructor**

##### UpdateStatement(text, where)

Wraps update text and an optional [IWhere](./where-clause.md#iwhere).

```csharp
public UpdateStatement(string text, IWhere? where = null)
```

- `text` — command text
- `where` — optional [IWhere](./where-clause.md#iwhere)

##### UpdateStatement(table, fields, where)

Compiles an `UPDATE`.

```csharp
public UpdateStatement(string table, IReadOnlyList<Parameter> fields, IWhere? where = null)
```

- `table` — table name
- `fields` — [Parameter](./command.md#parameter) columns and bound values
- `where` — optional [IWhere](./where-clause.md#iwhere)

**Properties**

##### Where

Optional [IWhere](./where-clause.md#iwhere) used to build this update.

```csharp
IWhere? Where { get; }
```

**Methods**

##### Accept(visitor)

Dispatches to a statement visitor that renders command text.

```csharp
public string Accept(IStatementVisitor visitor)
```

- `visitor` — [IStatementVisitor](./statements.md#istatementvisitor)

**Returns** `string`

### rust
#### UpdateStatement

```rust
pub struct UpdateStatement
```

**Constructor**

##### text(text, where_clause)

Wraps update text and an optional [IWhere](./where-clause.md#iwhere).

```rust
pub fn text(text: impl Into<String>, where_clause: Option<Box<dyn IWhere>>) -> Self
```

- `text` — command text
- `where_clause` — optional [IWhere](./where-clause.md#iwhere)

**Returns** [UpdateStatement](#updatestatement)

##### new(table, fields, where_clause)

Compiles an `UPDATE`.

```rust
pub fn new(table: impl AsRef<str>, fields: &[Parameter], where_clause: Option<&dyn IWhere>) -> Self
```

- `table` — table name
- `fields` — [Parameter](./command.md#parameter) columns and bound values
- `where_clause` — optional [IWhere](./where-clause.md#iwhere)

**Returns** [UpdateStatement](#updatestatement)

**Methods**

##### accept()

Dispatches through the renderer's statement visitor.

```rust
fn accept(&self, text: &StatementTextRenderer) -> String
```

- `text` — [StatementTextRenderer](./statements.md#statementtextrenderer)

**Returns** `String`

##### where_clause()

Optional [IWhere](./where-clause.md#iwhere) used to build this update.

```rust
fn where_clause(&self) -> Option<&dyn IWhere>
```

**Returns** `Option<&dyn `[IWhere](./where-clause.md#iwhere)`>`
:::

## Remarks

Compiled form is `UPDATE table SET col = $col` plus `WHERE` when an [IWhere](./where-clause.md#iwhere) is present. Keep the [Where](./where-clause.md) when the caller needs bound parameters.

## Examples

### Compile an UPDATE

::: example
```csharp csharp/Datasources.Tests/CoreTests.cs#StatementsAndBuilders
```
```rust rust/datasources/src/tests/mod.rs#statements_and_builders
```
```typescript typescript/datasources/src/builders/builders.test.ts#creates-write-and-ddl-statements
```

```output [After: update]
UPDATE users SET id = $id, name = $name WHERE id = $id AND age > $age
```

```output:typescript [After: update]
UPDATE users SET id = $id, name = $name WHERE id = $id
```
:::
