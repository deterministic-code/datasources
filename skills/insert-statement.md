<!-- Generated from docs/. Edit the markdown in docs/ and run npm run docs:skill. -->

# Insert Statement

`INSERT` text, or a compiled `INSERT` from a table and fields.

## Classes

#### TypeScript

#### IInsertStatement {#typescript-iinsertstatement}

```typescript
interface IInsertStatement extends IStatement {}
```

#### InsertStatement {#typescript-insertstatement}

```typescript
class InsertStatement implements IInsertStatement
```

**Constructor**

| Name | Description |
| --- | --- |
| [InsertStatement(text)](#typescript-insertstatement-insertstatement-text) | Wraps insert text. |
| [InsertStatement(table, fields)](#typescript-insertstatement-insertstatement-table-fields) | Compiles an `INSERT`. |

**Methods**

| Name | Description |
| --- | --- |
| [accept(visitor)](#typescript-insertstatement-accept-visitor) | Dispatches to a statement visitor that renders command text. |

##### InsertStatement(text) {#typescript-insertstatement-insertstatement-text}

Wraps insert text.

**Definition**

```typescript
constructor(text: string)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |

##### InsertStatement(table, fields) {#typescript-insertstatement-insertstatement-table-fields}

Compiles an `INSERT`.

**Definition**

```typescript
constructor(table: string, fields: Field[])
```

**Parameters**

| Name | Description |
| --- | --- |
| `table` | table name |
| `fields` | [Field](#typescript-field) columns and bound values |

##### accept(visitor) {#typescript-insertstatement-accept-visitor}

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

#### Field {#typescript-field}

```typescript
interface Field
```

**Properties**

| Name | Description |
| --- | --- |
| name | Column name. |
| value | Bound [Scalar](where.md#typescript-scalar). |

#### C#

#### IInsertStatement {#csharp-iinsertstatement}

```csharp
public interface IInsertStatement : IStatement
```

#### InsertStatement {#csharp-insertstatement}

```csharp
public sealed record InsertStatement : IInsertStatement
```

**Constructor**

| Name | Description |
| --- | --- |
| [InsertStatement(text)](#csharp-insertstatement-insertstatement-text) | Wraps insert text. |
| [InsertStatement(table, fields)](#csharp-insertstatement-insertstatement-table-fields) | Compiles an `INSERT`. |

**Methods**

| Name | Description |
| --- | --- |
| [Accept(visitor)](#csharp-insertstatement-accept-visitor) | Dispatches to a statement visitor that renders command text. |

##### InsertStatement(text) {#csharp-insertstatement-insertstatement-text}

Wraps insert text.

**Definition**

```csharp
public InsertStatement(string text)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |

##### InsertStatement(table, fields) {#csharp-insertstatement-insertstatement-table-fields}

Compiles an `INSERT`.

**Definition**

```csharp
public InsertStatement(string table, IReadOnlyList<Parameter> fields)
```

**Parameters**

| Name | Description |
| --- | --- |
| `table` | table name |
| `fields` | [Parameter](command.md#csharp-parameter) columns and bound values |

##### Accept(visitor) {#csharp-insertstatement-accept-visitor}

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

#### InsertStatement {#rust-insertstatement}

```rust
pub struct InsertStatement
```

**Constructor**

| Name | Description |
| --- | --- |
| [text(text)](#rust-insertstatement-text-text) | Wraps insert text. |
| [new(table, fields)](#rust-insertstatement-new-table-fields) | Compiles an `INSERT`. |

**Methods**

| Name | Description |
| --- | --- |
| [accept()](#rust-insertstatement-accept) | Dispatches through the renderer's statement visitor. |

##### text(text) {#rust-insertstatement-text-text}

Wraps insert text.

**Definition**

```rust
pub fn text(text: impl Into<String>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |

**Returns**

[InsertStatement](#rust-insertstatement)

##### new(table, fields) {#rust-insertstatement-new-table-fields}

Compiles an `INSERT`.

**Definition**

```rust
pub fn new(table: impl AsRef<str>, fields: &[Parameter]) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `table` | table name |
| `fields` | [Parameter](command.md#rust-parameter) columns and bound values |

**Returns**

[InsertStatement](#rust-insertstatement)

##### accept() {#rust-insertstatement-accept}

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

## Remarks

Compiled form is `INSERT INTO table (cols) VALUES ($cols)`. [Command](command.md) uses `StatementTextRenderer.Render` / `StatementTextRenderer.render` / `StatementTextRenderer::render` only.

## Examples

### Compile an INSERT

#### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/StatementCompileTests.cs#L96-L102)

```csharp
var fields = new[] { new Parameter("id", 1), new Parameter("name", "a") };
var insert = new InsertStatement("users", fields);
```

**After: insert**

```
INSERT INTO users (id, name) VALUES ($id, $name)
```

#### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/statement_compile.rs#L122-L130)

```rust
let fields = [Parameter::new("id", 1_i64), Parameter::new("name", "a")];
let insert = InsertStatement::new("users", &fields);
```

**After: insert**

```
INSERT INTO users (id, name) VALUES ($id, $name)
```

#### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/statements/StatementCompile.test.ts#L106-L115)

```typescript
const fields = [
  { name: "id", value: 1 },
  { name: "name", value: "a" },
];
const insert = new InsertStatement("users", fields);
```

**After: insert**

```
INSERT INTO users (id, name) VALUES ($id, $name)
```
