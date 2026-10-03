# Delete Statement

Compiled `DELETE` from a table and optional [IWhere](./where-clause.md#iwhere), or raw delete text.

## Classes

::: reference
### typescript
#### IDeleteStatement

```typescript
interface IDeleteStatement extends IStatement
```

**Properties**

##### where

Optional [IWhere](./where-clause.md#iwhere) used to build this delete.

```typescript
readonly where?: IWhere
```

#### DeleteStatement

```typescript
class DeleteStatement implements IDeleteStatement
```

**Constructor**

##### DeleteStatement(table, where?)

Compiles a `DELETE`.

```typescript
constructor(table: string, where?: IWhere)
```

- `table` — table name
- `where` — optional [IWhere](./where-clause.md#iwhere)

**Properties**

##### where

Optional [IWhere](./where-clause.md#iwhere) used to build this delete.

```typescript
readonly where?: IWhere
```

**Methods**

##### raw(text, where?)

Wraps delete text and an optional [IWhere](./where-clause.md#iwhere).

```typescript
static raw(text: string, where?: IWhere): DeleteStatement
```

- `text` — command text
- `where` — optional [IWhere](./where-clause.md#iwhere)

**Returns** [DeleteStatement](#deletestatement)

##### accept(visitor)

Dispatches to a statement visitor that renders command text.

```typescript
accept(visitor: IStatementVisitor): string
```

- `visitor` — [IStatementVisitor](./statements.md#istatementvisitor)

**Returns** `string`

### csharp
#### IDeleteStatement

```csharp
public interface IDeleteStatement : IStatement
```

**Properties**

##### Where

Optional [IWhere](./where-clause.md#iwhere) used to build this delete.

```csharp
IWhere? Where { get; }
```

#### DeleteStatement

```csharp
public sealed record DeleteStatement : IDeleteStatement
```

**Constructor**

##### DeleteStatement(table, where)

Compiles a `DELETE`.

```csharp
public DeleteStatement(string table, IWhere? where = null)
```

- `table` — table name
- `where` — optional [IWhere](./where-clause.md#iwhere)

**Properties**

##### Where

Optional [IWhere](./where-clause.md#iwhere) used to build this delete.

```csharp
IWhere? Where { get; }
```

**Methods**

##### Raw(text, where)

Wraps delete text and an optional [IWhere](./where-clause.md#iwhere).

```csharp
public static DeleteStatement Raw(string text, IWhere? where = null)
```

- `text` — command text
- `where` — optional [IWhere](./where-clause.md#iwhere)

**Returns** [DeleteStatement](#deletestatement)

##### Accept(visitor)

Dispatches to a statement visitor that renders command text.

```csharp
public string Accept(IStatementVisitor visitor)
```

- `visitor` — [IStatementVisitor](./statements.md#istatementvisitor)

**Returns** `string`

### rust
#### DeleteStatement

```rust
pub struct DeleteStatement
```

**Constructor**

##### new(table, where_clause)

Compiles a `DELETE`.

```rust
pub fn new(table: impl AsRef<str>, where_clause: Option<&dyn IWhere>) -> Self
```

- `table` — table name
- `where_clause` — optional [IWhere](./where-clause.md#iwhere)

**Returns** [DeleteStatement](#deletestatement)

##### raw(text, where_clause)

Wraps delete text and an optional [IWhere](./where-clause.md#iwhere).

```rust
pub fn raw(text: impl Into<String>, where_clause: Option<Box<dyn IWhere>>) -> Self
```

- `text` — command text
- `where_clause` — optional [IWhere](./where-clause.md#iwhere)

**Returns** [DeleteStatement](#deletestatement)

**Methods**

##### accept()

Dispatches through the renderer's statement visitor.

```rust
fn accept(&self, text: &StatementTextRenderer) -> String
```

- `text` — [StatementTextRenderer](./statements.md#statementtextrenderer)

**Returns** `String`

##### where_clause()

Optional [IWhere](./where-clause.md#iwhere) used to build this delete.

```rust
fn where_clause(&self) -> Option<&dyn IWhere>
```

**Returns** `Option<&dyn `[IWhere](./where-clause.md#iwhere)`>`
:::

## Remarks

Compiled form is `DELETE FROM table` plus `WHERE` when an [IWhere](./where-clause.md#iwhere) is present. {{Raw|raw|raw}} wraps command text without compiling. Keep the [Where](./where-clause.md) when the caller needs bound parameters.

## Examples

### Compile a DELETE

::: example
```csharp csharp/Datasources.Tests/StatementCompileTests.cs#CompilesADelete
```
```rust rust/datasources/src/tests/statement_compile.rs#compiles_a_delete
```
```typescript typescript/datasources/src/statements/StatementCompile.test.ts#compiles-a-delete
```

```output [After: delete]
DELETE FROM users WHERE id = $id AND age > $age
```
:::
