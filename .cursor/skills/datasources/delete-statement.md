<!-- Generated from docs/. Edit the markdown in docs/ and run npm run docs:skill. -->

# Delete Statement

Compiled `DELETE` from a table and optional [IWhere](where.md#iwhere), or raw delete text.

## Classes

#### TypeScript

#### IDeleteStatement {#typescript-ideletestatement}

```typescript
interface IDeleteStatement extends IStatement
```

**Properties**

| Name | Description |
| --- | --- |
| where | Optional [IWhere](where.md#typescript-iwhere) used to build this delete. |

#### DeleteStatement {#typescript-deletestatement}

```typescript
class DeleteStatement implements IDeleteStatement
```

**Constructor**

| Name | Description |
| --- | --- |
| [DeleteStatement(table, where?)](#typescript-deletestatement-deletestatement-table-where) | Compiles a `DELETE`. |

**Properties**

| Name | Description |
| --- | --- |
| where | Optional [IWhere](where.md#typescript-iwhere) used to build this delete. |

**Methods**

| Name | Description |
| --- | --- |
| [raw(text, where?)](#typescript-deletestatement-raw-text-where) | Wraps delete text and an optional [IWhere](where.md#typescript-iwhere). |
| [accept(visitor)](#typescript-deletestatement-accept-visitor) | Dispatches to a statement visitor that renders command text. |

##### DeleteStatement(table, where?) {#typescript-deletestatement-deletestatement-table-where}

Compiles a `DELETE`.

**Definition**

```typescript
constructor(table: string, where?: IWhere)
```

**Parameters**

| Name | Description |
| --- | --- |
| `table` | table name |
| `where` | optional [IWhere](where.md#typescript-iwhere) |

##### raw(text, where?) {#typescript-deletestatement-raw-text-where}

Wraps delete text and an optional [IWhere](where.md#typescript-iwhere).

**Definition**

```typescript
static raw(text: string, where?: IWhere): DeleteStatement
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |
| `where` | optional [IWhere](where.md#typescript-iwhere) |

**Returns**

[DeleteStatement](#typescript-deletestatement)

##### accept(visitor) {#typescript-deletestatement-accept-visitor}

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

#### IDeleteStatement {#csharp-ideletestatement}

```csharp
public interface IDeleteStatement : IStatement
```

**Properties**

| Name | Description |
| --- | --- |
| Where | Optional [IWhere](where.md#csharp-iwhere) used to build this delete. |

#### DeleteStatement {#csharp-deletestatement}

```csharp
public sealed record DeleteStatement : IDeleteStatement
```

**Constructor**

| Name | Description |
| --- | --- |
| [DeleteStatement(table, where)](#csharp-deletestatement-deletestatement-table-where) | Compiles a `DELETE`. |

**Properties**

| Name | Description |
| --- | --- |
| Where | Optional [IWhere](where.md#csharp-iwhere) used to build this delete. |

**Methods**

| Name | Description |
| --- | --- |
| [Raw(text, where)](#csharp-deletestatement-raw-text-where) | Wraps delete text and an optional [IWhere](where.md#csharp-iwhere). |
| [Accept(visitor)](#csharp-deletestatement-accept-visitor) | Dispatches to a statement visitor that renders command text. |

##### DeleteStatement(table, where) {#csharp-deletestatement-deletestatement-table-where}

Compiles a `DELETE`.

**Definition**

```csharp
public DeleteStatement(string table, IWhere? where = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `table` | table name |
| `where` | optional [IWhere](where.md#csharp-iwhere) |

##### Raw(text, where) {#csharp-deletestatement-raw-text-where}

Wraps delete text and an optional [IWhere](where.md#csharp-iwhere).

**Definition**

```csharp
public static DeleteStatement Raw(string text, IWhere? where = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |
| `where` | optional [IWhere](where.md#csharp-iwhere) |

**Returns**

[DeleteStatement](#csharp-deletestatement)

##### Accept(visitor) {#csharp-deletestatement-accept-visitor}

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

#### DeleteStatement {#rust-deletestatement}

```rust
pub struct DeleteStatement
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(table, where_clause)](#rust-deletestatement-new-table-where_clause) | Compiles a `DELETE`. |
| [raw(text, where_clause)](#rust-deletestatement-raw-text-where_clause) | Wraps delete text and an optional [IWhere](where.md#rust-iwhere). |

**Methods**

| Name | Description |
| --- | --- |
| [accept()](#rust-deletestatement-accept) | Dispatches through the renderer's statement visitor. |
| [where_clause()](#rust-deletestatement-where_clause) | Optional [IWhere](where.md#rust-iwhere) used to build this delete. |

##### new(table, where_clause) {#rust-deletestatement-new-table-where_clause}

Compiles a `DELETE`.

**Definition**

```rust
pub fn new(table: impl AsRef<str>, where_clause: Option<&dyn IWhere>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `table` | table name |
| `where_clause` | optional [IWhere](where.md#rust-iwhere) |

**Returns**

[DeleteStatement](#rust-deletestatement)

##### raw(text, where_clause) {#rust-deletestatement-raw-text-where_clause}

Wraps delete text and an optional [IWhere](where.md#rust-iwhere).

**Definition**

```rust
pub fn raw(text: impl Into<String>, where_clause: Option<Box<dyn IWhere>>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |
| `where_clause` | optional [IWhere](where.md#rust-iwhere) |

**Returns**

[DeleteStatement](#rust-deletestatement)

##### accept() {#rust-deletestatement-accept}

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

##### where_clause() {#rust-deletestatement-where_clause}

Optional [IWhere](where.md#rust-iwhere) used to build this delete.

**Definition**

```rust
fn where_clause(&self) -> Option<&dyn IWhere>
```

**Returns**

`Option<&dyn `[IWhere](where.md#rust-iwhere)`>`

## Remarks

Compiled form is `DELETE FROM table` plus `WHERE` when an [IWhere](where.md#iwhere) is present. `Raw` / `raw` / `raw` wraps command text without compiling. Keep the [Where](where.md) when the caller needs bound parameters.

## Examples

### Compile a DELETE

#### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/StatementCompileTests.cs#L116-L125)

```csharp
var where = new Where(new Operator.And(
    new Operator.Equals("id", 1),
    new Operator.GreaterThan("age", 18)));
var delete = new DeleteStatement("users", where);
```

**After: delete**

```
DELETE FROM users WHERE id = $id AND age > $age
```

#### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/statement_compile.rs#L147-L159)

```rust
let where_clause = Where::new(Operator::and(
    Operator::equals("id", 1_i64),
    Operator::greater_than("age", 18_i64),
));
let delete = DeleteStatement::new("users", Some(&where_clause));
```

**After: delete**

```
DELETE FROM users WHERE id = $id AND age > $age
```

#### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/statements/StatementCompile.test.ts#L132-L141)

```typescript
const where = new Where(
  new Operator.And(new Operator.Equals("id", 1), new Operator.GreaterThan("age", 18)),
);
const del = new DeleteStatement("users", where);
```

**After: delete**

```
DELETE FROM users WHERE id = $id AND age > $age
```
