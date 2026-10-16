# Drop Index Statement

`DROP INDEX` text, or a compiled `DROP INDEX` from a name and table.

## Classes

::: reference
### typescript
#### IDropIndexStatement

```typescript
interface IDropIndexStatement extends IStatement {}
```

#### DropIndexStatement

```typescript
class DropIndexStatement implements IDropIndexStatement
```

**Constructor**

##### DropIndexStatement(name, table)

Compiles a `DROP INDEX`.

```typescript
constructor(name: string, table: string)
```

- `name` — index name
- `table` — table name; MySQL and SQL Server render `DROP INDEX name ON table`

**Methods**

##### accept(visitor)

Dispatches to a statement visitor that renders command text.

```typescript
accept(visitor: IStatementVisitor): string
```

- `visitor` — [IStatementVisitor](./statements.md#istatementvisitor)

**Returns** `string`

### csharp
#### IDropIndexStatement

```csharp
public interface IDropIndexStatement : IStatement
```

#### DropIndexStatement

```csharp
public sealed record DropIndexStatement : IDropIndexStatement
```

**Constructor**

##### DropIndexStatement(name, table)

Compiles a `DROP INDEX`.

```csharp
public DropIndexStatement(string name, string table)
```

- `name` — index name
- `table` — table name; MySQL and SQL Server render `DROP INDEX name ON table`

**Methods**

##### Accept(visitor)

Dispatches to a statement visitor that renders command text.

```csharp
public string Accept(IStatementVisitor visitor)
```

- `visitor` — [IStatementVisitor](./statements.md#istatementvisitor)

**Returns** `string`

### rust
#### DropIndexStatement

```rust
pub struct DropIndexStatement
```

**Constructor**

##### new(name, table)

Compiles a `DROP INDEX`.

```rust
pub fn new(name: impl AsRef<str>, table: impl AsRef<str>) -> Self
```

- `name` — index name
- `table` — table name; MySQL and SQL Server render `DROP INDEX name ON table`

**Returns** [DropIndexStatement](#dropindexstatement)

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

Default text is `DROP INDEX IF EXISTS name`. MySQL and SQL Server render `DROP INDEX name ON table`. Oracle renders `DROP INDEX name`.

## Examples

### Compile a DROP INDEX

::: example
```csharp csharp/Datasources.Tests/StatementCompileTests.cs#CompilesADropIndex
```
```rust rust/datasources/src/tests/statement_compile.rs#compiles_a_drop_index
```
```typescript typescript/datasources/src/statements/StatementCompile.test.ts#compiles-a-drop-index
```

```output [After: drop index]
DROP INDEX IF EXISTS idx_orders_user
```
:::
