# Drop Trigger Statement

`DROP TRIGGER` text, or a compiled `DROP TRIGGER` from a name and table.

## Classes

::: reference
### typescript
#### IDropTriggerStatement

```typescript
interface IDropTriggerStatement extends IStatement {}
```

#### DropTriggerStatement

```typescript
class DropTriggerStatement implements IDropTriggerStatement
```

**Constructor**

##### DropTriggerStatement(name, table)

Compiles a `DROP TRIGGER`.

```typescript
constructor(name: string, table: string)
```

- `name` — trigger name
- `table` — table name; Postgres renders `DROP TRIGGER name ON table`

**Methods**

##### accept(visitor)

Dispatches to a statement visitor that renders command text.

```typescript
accept(visitor: IStatementVisitor): string
```

- `visitor` — [IStatementVisitor](./statements.md#istatementvisitor)

**Returns** `string`

### csharp
#### IDropTriggerStatement

```csharp
public interface IDropTriggerStatement : IStatement
```

#### DropTriggerStatement

```csharp
public sealed record DropTriggerStatement : IDropTriggerStatement
```

**Constructor**

##### DropTriggerStatement(name, table)

Compiles a `DROP TRIGGER`.

```csharp
public DropTriggerStatement(string name, string table)
```

- `name` — trigger name
- `table` — table name; Postgres renders `DROP TRIGGER name ON table`

**Methods**

##### Accept(visitor)

Dispatches to a statement visitor that renders command text.

```csharp
public string Accept(IStatementVisitor visitor)
```

- `visitor` — [IStatementVisitor](./statements.md#istatementvisitor)

**Returns** `string`

### rust
#### DropTriggerStatement

```rust
pub struct DropTriggerStatement
```

**Constructor**

##### new(name, table)

Compiles a `DROP TRIGGER`.

```rust
pub fn new(name: impl AsRef<str>, table: impl AsRef<str>) -> Self
```

- `name` — trigger name
- `table` — table name; Postgres renders `DROP TRIGGER name ON table`

**Returns** [DropTriggerStatement](#droptriggerstatement)

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

Default text is `DROP TRIGGER IF EXISTS name`. Postgres renders `DROP TRIGGER IF EXISTS name ON table`. Oracle renders `DROP TRIGGER name`.

## Examples

### Compile a DROP TRIGGER

::: example
```csharp csharp/Datasources.Tests/StatementCompileTests.cs#CompilesADropTrigger
```
```rust rust/datasources/src/tests/statement_compile.rs#compiles_a_drop_trigger
```
```typescript typescript/datasources/src/statements/StatementCompile.test.ts#compiles-a-drop-trigger
```

```output [After: drop trigger]
DROP TRIGGER IF EXISTS trg_users_updated
```
:::
