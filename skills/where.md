<!-- Generated from docs/. Edit the markdown in docs/ and run npm run docs:skill. -->

# Where

[IWhere](#iwhere) over an [IExpression](expressions.md#iexpression) tree. A clause is an expression, not a statement. Placeholders are `$field` and `$field_0` for `In`.

## Classes

#### TypeScript

#### Where {#typescript-where}

```typescript
class Where implements IWhere
```

**Constructor**

| Name | Description |
| --- | --- |
| [Where(condition)](#typescript-where-where-condition) | Builds a where clause from an [IExpression](expressions.md#typescript-iexpression). |
| [Where(column, op, value)](#typescript-where-where-column-op-value) | Builds a comparison from a column, a [Cmp](expressions.md#typescript-cmp) alias, and a literal. |

**Properties**

| Name | Description |
| --- | --- |
| condition | Predicate tree. Typically an [Operator](expressions.md#typescript-operator). |
| parameters | Bound [Parameter](command.md#typescript-parameter) list. |

**Methods**

| Name | Description |
| --- | --- |
| [accept(visitor)](#typescript-where-accept-visitor) | Dispatches to a condition visitor that renders predicate text. |
| [and(column, op, value) / or(column, op, value)](#typescript-where-and-column-op-value-or-column-op-value) | Nests this clause with another comparison under [Operator.And](expressions.md#typescript-operator) or `Operator.Or`. |
| [map(column, value)](#typescript-where-map-column-value) | Rewrites identifiers through `column` and the literals beside them through `value`. The repository calls this so a clause uses stored column names. A field converter, when one is set, rewrites that property's literals. |

##### Where(condition) {#typescript-where-where-condition}

Builds a where clause from an [IExpression](expressions.md#typescript-iexpression).

**Definition**

```typescript
constructor(condition: IExpression)
```

**Parameters**

| Name | Description |
| --- | --- |
| `condition` | [IExpression](expressions.md#typescript-iexpression) |

##### Where(column, op, value) {#typescript-where-where-column-op-value}

Builds a comparison from a column, a [Cmp](expressions.md#typescript-cmp) alias, and a literal.

**Definition**

```typescript
constructor(column: string, op: Comparison, value: Cell)
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | column name |
| `op` | [Cmp](expressions.md#typescript-cmp) alias such as `eq` or `gt` |
| `value` | [Cell](mappings.md#typescript-cell) |

##### accept(visitor) {#typescript-where-accept-visitor}

Dispatches to a condition visitor that renders predicate text.

**Definition**

```typescript
accept(visitor: IConditionVisitor): string
```

**Parameters**

| Name | Description |
| --- | --- |
| `visitor` | [IConditionVisitor](#typescript-iconditionvisitor) |

**Returns**

`string`

##### and(column, op, value) / or(column, op, value) {#typescript-where-and-column-op-value-or-column-op-value}

Nests this clause with another comparison under [Operator.And](expressions.md#typescript-operator) or `Operator.Or`.

**Definition**

```typescript
and(column: string, op: Comparison, value: Cell): Where
or(column: string, op: Comparison, value: Cell): Where
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | column name |
| `op` | [Cmp](expressions.md#typescript-cmp) alias |
| `value` | [Cell](mappings.md#typescript-cell) |

**Returns**

[Where](#typescript-where)

##### map(column, value) {#typescript-where-map-column-value}

Rewrites identifiers through `column` and the literals beside them through `value`. The repository calls this so a clause uses stored column names. A field converter, when one is set, rewrites that property's literals.

**Definition**

```typescript
map(column: (name: string) => string, value: (name: string, cell: Cell) => Cell): Where
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | property name to stored column |
| `value` | property name and domain cell to stored cell |

**Returns**

[Where](#typescript-where)

#### ICondition {#typescript-icondition}

```typescript
interface ICondition
```

**Methods**

| Name | Description |
| --- | --- |
| [accept(visitor)](#typescript-icondition-accept-visitor) | Dispatches to a condition visitor that renders predicate text. |

##### accept(visitor) {#typescript-icondition-accept-visitor}

Dispatches to a condition visitor that renders predicate text.

**Definition**

```typescript
accept(visitor: IConditionVisitor): string
```

**Parameters**

| Name | Description |
| --- | --- |
| `visitor` | [IConditionVisitor](#typescript-iconditionvisitor) |

**Returns**

`string`

#### IConditionVisitor {#typescript-iconditionvisitor}

```typescript
interface IConditionVisitor
```

**Methods**

| Name | Description |
| --- | --- |
| [visit(where)](#typescript-iconditionvisitor-visit-where) |  |

##### visit(where) {#typescript-iconditionvisitor-visit-where}



**Definition**

```typescript
visit(where: Where): string
visit(where: WhereText): string
```

**Parameters**

| Name | Description |
| --- | --- |
| `where` | [Where](#typescript-where) or [WhereText](#typescript-wheretext) |

**Returns**

`string`

#### ConditionRenderer {#typescript-conditionrenderer}

```typescript
class ConditionRenderer implements IConditionVisitor
```

#### IWhere {#typescript-iwhere}

```typescript
interface IWhere extends IExpression, ICondition {}
```

#### WhereText {#typescript-wheretext}

```typescript
class WhereText implements IWhere
```

**Constructor**

| Name | Description |
| --- | --- |
| [WhereText(text)](#typescript-wheretext-wheretext-text) | Wraps predicate text. |

**Methods**

| Name | Description |
| --- | --- |
| [accept(visitor)](#typescript-wheretext-accept-visitor) | Dispatches to a condition visitor that renders predicate text. |

##### WhereText(text) {#typescript-wheretext-wheretext-text}

Wraps predicate text.

**Definition**

```typescript
constructor(text: string)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | predicate text |

##### accept(visitor) {#typescript-wheretext-accept-visitor}

Dispatches to a condition visitor that renders predicate text.

**Definition**

```typescript
accept(visitor: IConditionVisitor): string
```

**Parameters**

| Name | Description |
| --- | --- |
| `visitor` | [IConditionVisitor](#typescript-iconditionvisitor) |

**Returns**

`string`

#### Scalar {#typescript-scalar}

```typescript
type Scalar = string | number | boolean | bigint | null
```

#### C#

#### Where {#csharp-where}

```csharp
public sealed record Where(IExpression Condition) : IWhere
```

**Constructor**

| Name | Description |
| --- | --- |
| [Where(condition)](#csharp-where-where-condition) | Builds a where clause from an [IExpression](expressions.md#csharp-iexpression). |
| [Where(column, op, value)](#csharp-where-where-column-op-value) | Builds a comparison from a column, a [Cmp](expressions.md#csharp-cmp) alias, and a literal. |

**Properties**

| Name | Description |
| --- | --- |
| Condition | Predicate tree. Typically an [Operator](expressions.md#csharp-operator). |
| Parameters | Bound [Parameter](command.md#csharp-parameter) list. |

**Methods**

| Name | Description |
| --- | --- |
| [Accept(visitor)](#csharp-where-accept-visitor) | Dispatches to a condition visitor that renders predicate text. |
| [And(column, op, value) / Or(column, op, value)](#csharp-where-and-column-op-value-or-column-op-value) | Nests this clause with another comparison under `Operator.And` or `Operator.Or`. |
| [Map(column, value)](#csharp-where-map-column-value) | Rewrites identifiers through `column` and the literals beside them through `value`. The repository calls this so a clause uses stored column names. A field converter, when one is set, rewrites that property's literals. |

##### Where(condition) {#csharp-where-where-condition}

Builds a where clause from an [IExpression](expressions.md#csharp-iexpression).

**Definition**

```csharp
public Where(IExpression Condition)
```

**Parameters**

| Name | Description |
| --- | --- |
| `condition` | [IExpression](expressions.md#csharp-iexpression) |

##### Where(column, op, value) {#csharp-where-where-column-op-value}

Builds a comparison from a column, a [Cmp](expressions.md#csharp-cmp) alias, and a literal.

**Definition**

```csharp
public Where(string column, Comparison op, Value? value)
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | column name |
| `op` | [Cmp](expressions.md#csharp-cmp) alias such as `eq` or `gt` |
| `value` | cell value |

##### Accept(visitor) {#csharp-where-accept-visitor}

Dispatches to a condition visitor that renders predicate text.

**Definition**

```csharp
public string Accept(IConditionVisitor visitor)
```

**Parameters**

| Name | Description |
| --- | --- |
| `visitor` | [IConditionVisitor](#csharp-iconditionvisitor) |

**Returns**

`string`

##### And(column, op, value) / Or(column, op, value) {#csharp-where-and-column-op-value-or-column-op-value}

Nests this clause with another comparison under `Operator.And` or `Operator.Or`.

**Definition**

```csharp
public Where And(string column, Comparison op, Value? value)
public Where Or(string column, Comparison op, Value? value)
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | column name |
| `op` | [Cmp](expressions.md#csharp-cmp) alias |
| `value` | cell value |

**Returns**

[Where](#csharp-where)

##### Map(column, value) {#csharp-where-map-column-value}

Rewrites identifiers through `column` and the literals beside them through `value`. The repository calls this so a clause uses stored column names. A field converter, when one is set, rewrites that property's literals.

**Definition**

```csharp
public Where Map(Func<string, string> column, Func<string, Value, Value> value)
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | property name to stored column |
| `value` | property name and domain cell to stored cell |

**Returns**

[Where](#csharp-where)

#### ICondition {#csharp-icondition}

```csharp
public interface ICondition
```

**Methods**

| Name | Description |
| --- | --- |
| [Accept(visitor)](#csharp-icondition-accept-visitor) | Dispatches to a condition visitor that renders predicate text. |

##### Accept(visitor) {#csharp-icondition-accept-visitor}

Dispatches to a condition visitor that renders predicate text.

**Definition**

```csharp
string Accept(IConditionVisitor visitor)
```

**Parameters**

| Name | Description |
| --- | --- |
| `visitor` | [IConditionVisitor](#csharp-iconditionvisitor) |

**Returns**

`string`

#### IConditionVisitor {#csharp-iconditionvisitor}

```csharp
public interface IConditionVisitor
```

**Methods**

| Name | Description |
| --- | --- |
| [Visit(where)](#csharp-iconditionvisitor-visit-where) |  |

##### Visit(where) {#csharp-iconditionvisitor-visit-where}



**Definition**

```csharp
string Visit(Where where)
string Visit(WhereText where)
```

**Parameters**

| Name | Description |
| --- | --- |
| `where` | [Where](#csharp-where) or [WhereText](#csharp-wheretext) |

**Returns**

`string`

#### ConditionRenderer {#csharp-conditionrenderer}

```csharp
public class ConditionRenderer : IConditionVisitor
```

#### IWhere {#csharp-iwhere}

```csharp
public interface IWhere : IExpression, ICondition
```

#### WhereText {#csharp-wheretext}

```csharp
public sealed record WhereText : IWhere
```

**Constructor**

| Name | Description |
| --- | --- |
| [WhereText(text)](#csharp-wheretext-wheretext-text) | Wraps predicate text. |

**Methods**

| Name | Description |
| --- | --- |
| [Accept(visitor)](#csharp-wheretext-accept-visitor) | Dispatches to a condition visitor that renders predicate text. |

##### WhereText(text) {#csharp-wheretext-wheretext-text}

Wraps predicate text.

**Definition**

```csharp
public WhereText(string text)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | predicate text |

##### Accept(visitor) {#csharp-wheretext-accept-visitor}

Dispatches to a condition visitor that renders predicate text.

**Definition**

```csharp
string Accept(IConditionVisitor visitor)
```

**Parameters**

| Name | Description |
| --- | --- |
| `visitor` | [IConditionVisitor](#csharp-iconditionvisitor) |

**Returns**

`string`

#### Rust

#### Where {#rust-where}

```rust
pub struct Where
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(condition)](#rust-where-new-condition) | Builds a where clause from an [Expression](expressions.md#rust-expression). |
| [binary(column, op, value)](#rust-where-binary-column-op-value) | Builds a comparison from a column, a [Cmp](expressions.md#rust-cmp) alias, and a literal. |
| [and(column, op, value) / or(column, op, value)](#rust-where-and-column-op-value-or-column-op-value) | Nests this clause with another comparison under `Operator::and` or `Operator::or`. |
| [map(column, value)](#rust-where-map-column-value) | Rewrites identifiers through `column` and the literals beside them through `value`. The repository calls this so a clause uses stored column names. A field converter, when one is set, rewrites that property's literals. |
| [accept()](#rust-where-accept) | Dispatches through the renderer's condition visitor. |
| [parameters()](#rust-where-parameters) | Bound [Parameter](command.md#rust-parameter) list. |

##### new(condition) {#rust-where-new-condition}

Builds a where clause from an [Expression](expressions.md#rust-expression).

**Definition**

```rust
pub fn new(condition: impl Into<Expression>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `condition` | [Expression](expressions.md#rust-expression) or [Operator](expressions.md#rust-operator) |

**Returns**

[Where](#rust-where)

##### binary(column, op, value) {#rust-where-binary-column-op-value}

Builds a comparison from a column, a [Cmp](expressions.md#rust-cmp) alias, and a literal.

**Definition**

```rust
pub fn binary(column: &str, op: fn(&str, Value) -> Operator, value: impl Into<Value>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | column name |
| `op` | [Cmp](expressions.md#rust-cmp) alias such as `eq` or `gt` |
| `value` | [Value](#rust-value) |

**Returns**

[Where](#rust-where)

##### and(column, op, value) / or(column, op, value) {#rust-where-and-column-op-value-or-column-op-value}

Nests this clause with another comparison under `Operator::and` or `Operator::or`.

**Definition**

```rust
pub fn and(self, column: &str, op: fn(&str, Value) -> Operator, value: impl Into<Value>) -> Self
pub fn or(self, column: &str, op: fn(&str, Value) -> Operator, value: impl Into<Value>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | column name |
| `op` | [Cmp](expressions.md#rust-cmp) alias |
| `value` | [Value](#rust-value) |

**Returns**

[Where](#rust-where)

##### map(column, value) {#rust-where-map-column-value}

Rewrites identifiers through `column` and the literals beside them through `value`. The repository calls this so a clause uses stored column names. A field converter, when one is set, rewrites that property's literals.

**Definition**

```rust
pub fn map<C, V>(&self, column: C, value: V) -> Result<Self>
where
    C: Fn(&str) -> String,
    V: Fn(&str, &Value) -> Result<Value>,
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | property name to stored column |
| `value` | property name and domain cell to stored cell |

**Returns**

`Result<`[Where](#rust-where)`>`

##### accept() {#rust-where-accept}

Dispatches through the renderer's condition visitor.

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

##### parameters() {#rust-where-parameters}

Bound [Parameter](command.md#rust-parameter) list.

**Definition**

```rust
fn parameters(&self) -> Vec<Parameter>
```

**Returns**

`Vec<`[Parameter](command.md#rust-parameter)`>`

#### ICondition {#rust-icondition}

```rust
pub trait ICondition: Send + Sync
```

**Methods**

| Name | Description |
| --- | --- |
| [accept()](#rust-icondition-accept) | Dispatches through the renderer's condition visitor. |

##### accept() {#rust-icondition-accept}

Dispatches through the renderer's condition visitor.

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

#### ConditionVisitor {#rust-conditionvisitor}

```rust
pub trait ConditionVisitor: Send + Sync
```

#### ConditionRenderer {#rust-conditionrenderer}

```rust
pub struct ConditionRenderer
```

#### IWhere {#rust-iwhere}

```rust
pub trait IWhere: ICondition
```

#### WhereText {#rust-wheretext}

```rust
pub struct WhereText
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(text)](#rust-wheretext-new-text) | Wraps predicate text. |

**Methods**

| Name | Description |
| --- | --- |
| [accept()](#rust-wheretext-accept) | Dispatches through the renderer's condition visitor. |

##### new(text) {#rust-wheretext-new-text}

Wraps predicate text.

**Definition**

```rust
pub fn new(text: impl Into<String>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | predicate text |

**Returns**

[WhereText](#rust-wheretext)

##### accept() {#rust-wheretext-accept}

Dispatches through the renderer's condition visitor.

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

Construct the predicate with [Operator](expressions.md#operator) records, then wrap it: `new Where(new Operator.Equals("id", 9))`. A column, [Cmp](expressions.md#cmp) alias, and literal is the same tree: `new Where("id", eq, 9)`. [SelectStatement](select-statement.md) / [UpdateStatement](update-statement.md) / [DeleteStatement](delete-statement.md) embed `StatementTextRenderer.Render` / `StatementTextRenderer.render` / `StatementTextRenderer::render` of the where in the command SQL. Keep the [Where](#where) when the caller needs bound parameters. [ExpressionEvaluator](expressions.md) interprets the same tree against a parameter map. Raw text [WhereText](#wheretext) is not evaluable.

## Examples

### Compare, groups, not, and in

#### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/StatementCompileTests.cs#L186-L195)

```csharp
var where = new Where(new Operator.And(
    new Operator.Equals("id", 1),
    new Operator.GreaterThan("age", 18)));
var inIds = new Where(new Operator.In("id", [1, 2]));
```

**After: and**

```
id = $id AND age > $age
```

**After: in**

```
id IN ($id_0, $id_1)
```

#### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/statement_compile.rs#L238-L247)

```rust
let where_clause = Where::new(Operator::and(
    Operator::equals("id", 1_i64),
    Operator::greater_than("age", 18_i64),
));
let in_ids = Where::new(Operator::in_list("id", vec![Value::Int(1), Value::Int(2)]));
```

**After: and**

```
id = $id AND age > $age
```

**After: in**

```
id IN ($id_0, $id_1)
```

#### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/statements/StatementCompile.test.ts#L206-L213)

```typescript
const where = new Where(
  new Operator.And(new Operator.Equals("id", 1), new Operator.GreaterThan("age", 18)),
);
const inIds = new Where(new Operator.In("id", [1, 2]));
```

**After: and**

```
id = $id AND age > $age
```

**After: in**

```
id IN ($id_0, $id_1)
```

### Column, alias, literal

#### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/ComparisonAliasTests.cs#L11-L17)

```csharp
var equals = new Where("username", eq, "grace");
var notEquals = new Where("id", ne, 2);
var greater = new Where("n", gt, 0);
var less = new Where("n", lt, 0);
var greaterOrEqual = new Where("n", ge, 0);
var lessOrEqual = new Where("n", le, 0);
var both = equals.And("is_enabled", eq, 0);
```

**After: equals**

```
username = $username
```

**After: and**

```
username = $username AND is_enabled = $is_enabled
```

#### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L2137-L2143)

```rust
let equals = Where::binary("username", eq, "grace");
let not_equals = Where::binary("id", ne, 2_i64);
let greater = Where::binary("n", gt, 0);
let less = Where::binary("n", lt, 0);
let greater_or_equal = Where::binary("n", ge, 0);
let less_or_equal = Where::binary("n", le, 0);
let both = equals.clone().and("is_enabled", eq, 0);
```

**After: equals**

```
username = $username
```

**After: and**

```
username = $username AND is_enabled = $is_enabled
```

#### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/builders/builders.test.ts#L386-L403)

```typescript
const equals = new Where("username", eq, "grace");
const notEquals = new Where("id", ne, 2);
const greater = new Where("n", gt, 0);
const less = new Where("n", lt, 0);
const greaterOrEqual = new Where("n", ge, 0);
const lessOrEqual = new Where("n", le, 0);
const both = equals.and("is_enabled", eq, 0);
const either = equals.or("is_enabled", eq, 0);
```

**After: equals**

```
username = $username
```

**After: and**

```
username = $username AND is_enabled = $is_enabled
```

### Attach to a query

#### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/StatementCompileTests.cs#L206-L214)

```csharp
var where = new Where(new Operator.And(
    new Operator.Equals("id", 1),
    new Operator.GreaterThan("age", 18)));
var query = new SelectStatement(new() { Table = "users", Fields = ["id", "name"], Where = where });
```

**After: select**

```
SELECT id, name FROM users WHERE id = $id AND age > $age
```

#### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/statement_compile.rs#L258-L272)

```rust
let where_clause = Where::new(Operator::and(
    Operator::equals("id", 1_i64),
    Operator::greater_than("age", 18_i64),
));
let query = SelectStatement::new(SelectStatementOptions {
    where_clause: Some(&where_clause),
    ..SelectStatementOptions::new("users", &["id", "name"])
});
```

**After: select**

```
SELECT id, name FROM users WHERE id = $id AND age > $age
```

#### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/statements/StatementCompile.test.ts#L222-L230)

```typescript
const where = new Where(
  new Operator.And(new Operator.Equals("id", 1), new Operator.GreaterThan("age", 18)),
);
const query = new SelectStatement({ table: "users", fields: ["id", "name"], where });
```

**After: select**

```
SELECT id, name FROM users WHERE id = $id AND age > $age
```
