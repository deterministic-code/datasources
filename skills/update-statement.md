<!-- Generated from docs/. Edit the markdown in docs/ and run npm run docs:skill. -->

# Update Statement

`UPDATE` text, or a compiled `UPDATE` from a table, fields, and optional [IWhere](where.md#iwhere).

## Classes

#### TypeScript

#### IUpdateStatement {#typescript-iupdatestatement}

```typescript
interface IUpdateStatement extends IStatement
```

**Properties**

| Name | Description |
| --- | --- |
| where | Optional [IWhere](where.md#typescript-iwhere) used to build this update. |

#### UpdateStatement {#typescript-updatestatement}

```typescript
class UpdateStatement implements IUpdateStatement
```

**Constructor**

| Name | Description |
| --- | --- |
| [UpdateStatement(text, where?)](#typescript-updatestatement-updatestatement-text-where) | Wraps update text and an optional [IWhere](where.md#typescript-iwhere). |
| [UpdateStatement(table, fields, where?)](#typescript-updatestatement-updatestatement-table-fields-where) | Compiles an `UPDATE`. |

**Properties**

| Name | Description |
| --- | --- |
| where | Optional [IWhere](where.md#typescript-iwhere) used to build this update. |

**Methods**

| Name | Description |
| --- | --- |
| [accept(visitor)](#typescript-updatestatement-accept-visitor) | Dispatches to a statement visitor that renders command text. |

##### UpdateStatement(text, where?) {#typescript-updatestatement-updatestatement-text-where}

Wraps update text and an optional [IWhere](where.md#typescript-iwhere).

**Definition**

```typescript
constructor(text: string, where?: IWhere)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |
| `where` | optional [IWhere](where.md#typescript-iwhere) |

##### UpdateStatement(table, fields, where?) {#typescript-updatestatement-updatestatement-table-fields-where}

Compiles an `UPDATE`.

**Definition**

```typescript
constructor(table: string, fields: Field[], where?: IWhere)
```

**Parameters**

| Name | Description |
| --- | --- |
| `table` | table name |
| `fields` | [Field](insert-statement.md#typescript-field) columns and bound values |
| `where` | optional [IWhere](where.md#typescript-iwhere) |

##### accept(visitor) {#typescript-updatestatement-accept-visitor}

Dispatches to a statement visitor that renders command text.

**Definition**

```typescript
accept(visitor: IStatementVisitor): string
```

**Parameters**

| Name | Description |
| --- | --- |
| `visitor` | [IStatementVisitor](statements.md#typescript-istatementvisitor) |

**Returns**

`string`

#### C#

#### IUpdateStatement {#csharp-iupdatestatement}

```csharp
public interface IUpdateStatement : IStatement
```

**Properties**

| Name | Description |
| --- | --- |
| Where | Optional [IWhere](where.md#csharp-iwhere) used to build this update. |

#### UpdateStatement {#csharp-updatestatement}

```csharp
public sealed record UpdateStatement : IUpdateStatement
```

**Constructor**

| Name | Description |
| --- | --- |
| [UpdateStatement(text, where)](#csharp-updatestatement-updatestatement-text-where) | Wraps update text and an optional [IWhere](where.md#csharp-iwhere). |
| [UpdateStatement(table, fields, where)](#csharp-updatestatement-updatestatement-table-fields-where) | Compiles an `UPDATE`. |

**Properties**

| Name | Description |
| --- | --- |
| Where | Optional [IWhere](where.md#csharp-iwhere) used to build this update. |

**Methods**

| Name | Description |
| --- | --- |
| [Accept(visitor)](#csharp-updatestatement-accept-visitor) | Dispatches to a statement visitor that renders command text. |

##### UpdateStatement(text, where) {#csharp-updatestatement-updatestatement-text-where}

Wraps update text and an optional [IWhere](where.md#csharp-iwhere).

**Definition**

```csharp
public UpdateStatement(string text, IWhere? where = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |
| `where` | optional [IWhere](where.md#csharp-iwhere) |

##### UpdateStatement(table, fields, where) {#csharp-updatestatement-updatestatement-table-fields-where}

Compiles an `UPDATE`.

**Definition**

```csharp
public UpdateStatement(string table, IReadOnlyList<Parameter> fields, IWhere? where = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `table` | table name |
| `fields` | [Parameter](command.md#csharp-parameter) columns and bound values |
| `where` | optional [IWhere](where.md#csharp-iwhere) |

##### Accept(visitor) {#csharp-updatestatement-accept-visitor}

Dispatches to a statement visitor that renders command text.

**Definition**

```csharp
public string Accept(IStatementVisitor visitor)
```

**Parameters**

| Name | Description |
| --- | --- |
| `visitor` | [IStatementVisitor](statements.md#csharp-istatementvisitor) |

**Returns**

`string`

#### Rust

#### UpdateStatement {#rust-updatestatement}

```rust
pub struct UpdateStatement
```

**Constructor**

| Name | Description |
| --- | --- |
| [text(text, where_clause)](#rust-updatestatement-text-text-where_clause) | Wraps update text and an optional [IWhere](where.md#rust-iwhere). |
| [new(table, fields, where_clause)](#rust-updatestatement-new-table-fields-where_clause) | Compiles an `UPDATE`. |

**Methods**

| Name | Description |
| --- | --- |
| [accept()](#rust-updatestatement-accept) | Dispatches through the renderer's statement visitor. |
| [where_clause()](#rust-updatestatement-where_clause) | Optional [IWhere](where.md#rust-iwhere) used to build this update. |

##### text(text, where_clause) {#rust-updatestatement-text-text-where_clause}

Wraps update text and an optional [IWhere](where.md#rust-iwhere).

**Definition**

```rust
pub fn text(text: impl Into<String>, where_clause: Option<Box<dyn IWhere>>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |
| `where_clause` | optional [IWhere](where.md#rust-iwhere) |

**Returns**

[UpdateStatement](#rust-updatestatement)

##### new(table, fields, where_clause) {#rust-updatestatement-new-table-fields-where_clause}

Compiles an `UPDATE`.

**Definition**

```rust
pub fn new(table: impl AsRef<str>, fields: &[Parameter], where_clause: Option<&dyn IWhere>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `table` | table name |
| `fields` | [Parameter](command.md#rust-parameter) columns and bound values |
| `where_clause` | optional [IWhere](where.md#rust-iwhere) |

**Returns**

[UpdateStatement](#rust-updatestatement)

##### accept() {#rust-updatestatement-accept}

Dispatches through the renderer's statement visitor.

**Definition**

```rust
fn accept(&self, text: &StatementTextRenderer) -> String
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | [StatementTextRenderer](statements.md#rust-statementtextrenderer) |

**Returns**

`String`

##### where_clause() {#rust-updatestatement-where_clause}

Optional [IWhere](where.md#rust-iwhere) used to build this update.

**Definition**

```rust
fn where_clause(&self) -> Option<&dyn IWhere>
```

**Returns**

`Option<&dyn `[IWhere](where.md#rust-iwhere)`>`

## Remarks

Compiled form is `UPDATE table SET col = $col` plus `WHERE` when an [IWhere](where.md#iwhere) is present. Keep the [Where](where.md) when the caller needs bound parameters.

## Examples

### Compile an UPDATE

#### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/StatementCompileTests.cs#L104-L114)

```csharp
var fields = new[] { new Parameter("id", 1), new Parameter("name", "a") };
var where = new Where(new Operator.And(
    new Operator.Equals("id", 1),
    new Operator.GreaterThan("age", 18)));
var update = new UpdateStatement("users", fields, where);
```

**After: update**

```
UPDATE users SET id = $id, name = $name WHERE id = $id AND age > $age
```

#### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/statement_compile.rs#L132-L145)

```rust
let fields = [Parameter::new("id", 1_i64), Parameter::new("name", "a")];
let where_clause = Where::new(Operator::and(
    Operator::equals("id", 1_i64),
    Operator::greater_than("age", 18_i64),
));
let update = UpdateStatement::new("users", &fields, Some(&where_clause));
```

**After: update**

```
UPDATE users SET id = $id, name = $name WHERE id = $id AND age > $age
```

#### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/statements/StatementCompile.test.ts#L117-L130)

```typescript
const fields = [
  { name: "id", value: 1 },
  { name: "name", value: "a" },
];
const where = new Where(
  new Operator.And(new Operator.Equals("id", 1), new Operator.GreaterThan("age", 18)),
);
const update = new UpdateStatement("users", fields, where);
```

**After: update**

```
UPDATE users SET id = $id, name = $name WHERE id = $id AND age > $age
```
