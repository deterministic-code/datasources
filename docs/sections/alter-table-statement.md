# Alter Table Statement

`ALTER TABLE` text, or a compiled `ALTER TABLE` with one action: add or drop a column, alter a column, rename a column or table, or add or drop a constraint.

## Classes

::: reference
### typescript
#### IAlterTableStatement

```typescript
interface IAlterTableStatement extends IStatement {}
```

#### AlterTableStatement

```typescript
class AlterTableStatement implements IAlterTableStatement
```

**Constructor**

##### AlterTableStatement(table, action)

Compiles an `ALTER TABLE`.

```typescript
constructor(table: string, action: AlterAction)
```

- `table` — table name
- `action` — [AlterAction](#alteraction); one action per statement

**Methods**

##### accept(visitor)

Dispatches to a statement visitor that renders command text.

```typescript
accept(visitor: IStatementVisitor): string
```

- `visitor` — [IStatementVisitor](./statements.md#istatementvisitor)

**Returns** `string`

#### AlterAction

```typescript
type AlterAction =
  | { kind: "addColumn"; column: Column }
  | { kind: "dropColumn"; name: string }
  | { kind: "alterColumn"; column: Column }
  | { kind: "renameColumn"; from: string; to: string }
  | { kind: "renameTable"; to: string }
  | { kind: "addConstraint"; constraint: ITableConstraint }
  | { kind: "dropConstraint"; name: string }
```

### csharp
#### IAlterTableStatement

```csharp
public interface IAlterTableStatement : IStatement
```

#### AlterTableStatement

```csharp
public sealed record AlterTableStatement : IAlterTableStatement
```

**Constructor**

##### AlterTableStatement(table, action)

Compiles an `ALTER TABLE`.

```csharp
public AlterTableStatement(string table, AlterTableStatement.IAction action)
```

- `table` — table name
- `action` — nested `AddColumn`, `DropColumn`, `AlterColumn`, `RenameColumn`, `RenameTable`, `AddConstraint`, or `DropConstraint`

**Methods**

##### Accept(visitor)

Dispatches to a statement visitor that renders command text.

```csharp
public string Accept(IStatementVisitor visitor)
```

- `visitor` — [IStatementVisitor](./statements.md#istatementvisitor)

**Returns** `string`

### rust
#### AlterTableStatement

```rust
pub struct AlterTableStatement
```

**Constructor**

##### new(table, action)

Compiles an `ALTER TABLE`.

```rust
pub fn new(table: impl AsRef<str>, action: AlterAction) -> Self
```

- `table` — table name
- `action` — [AlterAction](#alteraction)

**Returns** [AlterTableStatement](#altertablestatement)

**Methods**

##### accept()

Dispatches through the renderer's statement visitor.

```rust
fn accept(&self, text: &StatementTextRenderer) -> String
```

- `text` — [StatementTextRenderer](./statements.md#statementtextrenderer)

**Returns** `String`

#### AlterAction

```rust
pub enum AlterAction
```
:::

## Remarks

Default text is `ALTER TABLE t ADD COLUMN …`, `DROP COLUMN`, `ALTER COLUMN c TYPE …`, `RENAME COLUMN`, `RENAME TO`, `ADD CONSTRAINT`, and `DROP CONSTRAINT`. One statement is one SQL command. SQLite renders add, drop, and rename only; type, null, default, and constraint changes throw.

## Examples

### Compile an ALTER TABLE ADD COLUMN

::: example
```csharp csharp/Datasources.Tests/StatementCompileTests.cs#CompilesAnAlterTableAddColumn
```
```rust rust/datasources/src/tests/statement_compile.rs#compiles_an_alter_table_add_column
```
```typescript typescript/datasources/src/statements/StatementCompile.test.ts#compiles-an-alter-table-add-column
```

```output [After: alter add column]
ALTER TABLE users ADD COLUMN email TEXT NOT NULL
```
:::
