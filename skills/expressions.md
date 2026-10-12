<!-- Generated from docs/. Edit the markdown in docs/ and run npm run docs:skill. -->

# Expressions

AST nodes for [Where](where.md). `Operator` records hold operands; [StatementTextRenderer](statements.md#statementtextrenderer) maps them to SQL.

## Classes

#### TypeScript

#### IExpression {#typescript-iexpression}

```typescript
interface IExpression {}
```

#### Operator {#typescript-operator}

```typescript
abstract class Operator implements IExpression
```

#### Cmp {#typescript-cmp}

```typescript
function eq(column: string, value: Cell): IExpression
```

**Methods**

| Name | Description |
| --- | --- |
| [eq / ne / gt / lt / ge / le](#typescript-cmp-eq-ne-gt-lt-ge-le) | `eq` is [Operator.Equals](#typescript-operator). `ne` is not-equals. `gt` / `lt` are greater-than and less-than. `ge` / `le` are the or-equal forms. |

##### eq / ne / gt / lt / ge / le {#typescript-cmp-eq-ne-gt-lt-ge-le}

`eq` is [Operator.Equals](#typescript-operator). `ne` is not-equals. `gt` / `lt` are greater-than and less-than. `ge` / `le` are the or-equal forms.

**Definition**

```typescript
function eq(column: string, value: Cell): IExpression
function ne(column: string, value: Cell): IExpression
function gt(column: string, value: Cell): IExpression
function lt(column: string, value: Cell): IExpression
function ge(column: string, value: Cell): IExpression
function le(column: string, value: Cell): IExpression
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | column name |
| `value` | [Cell](mappings.md#typescript-cell) |

**Returns**

[IExpression](#typescript-iexpression)

#### Identifier {#typescript-identifier}

```typescript
class Identifier implements IExpression
```

**Constructor**

| Name | Description |
| --- | --- |
| [Identifier(name)](#typescript-identifier-identifier-name) |  |

**Properties**

| Name | Description |
| --- | --- |
| name |  |

##### Identifier(name) {#typescript-identifier-identifier-name}



**Definition**

```typescript
constructor(name: string)
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | column name |

#### Literal {#typescript-literal}

```typescript
class Literal implements IExpression
```

**Constructor**

| Name | Description |
| --- | --- |
| [Literal(value)](#typescript-literal-literal-value) |  |

**Properties**

| Name | Description |
| --- | --- |
| value |  |

##### Literal(value) {#typescript-literal-literal-value}



**Definition**

```typescript
constructor(value: Cell)
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | [Cell](mappings.md#typescript-cell) |

#### ArrayExpression {#typescript-arrayexpression}

```typescript
class ArrayExpression implements IExpression
```

**Constructor**

| Name | Description |
| --- | --- |
| [ArrayExpression(elements)](#typescript-arrayexpression-arrayexpression-elements) |  |

**Properties**

| Name | Description |
| --- | --- |
| elements |  |

##### ArrayExpression(elements) {#typescript-arrayexpression-arrayexpression-elements}



**Definition**

```typescript
constructor(elements: IExpression[])
```

**Parameters**

| Name | Description |
| --- | --- |
| `elements` | [IExpression](#typescript-iexpression) list |

#### C#

#### IExpression {#csharp-iexpression}

```csharp
public interface IExpression
```

#### Operator {#csharp-operator}

```csharp
namespace Datasources.Operator
```

#### Cmp {#csharp-cmp}

```csharp
public static class Cmp
public delegate IExpression Comparison(string column, Value? value)
```

**Methods**

| Name | Description |
| --- | --- |
| [eq / ne / gt / lt / ge / le](#csharp-cmp-eq-ne-gt-lt-ge-le) | `eq` is `Operator.Equals`. `ne` is not-equals. `gt` / `lt` are greater-than and less-than. `ge` / `le` are the or-equal forms. |

##### eq / ne / gt / lt / ge / le {#csharp-cmp-eq-ne-gt-lt-ge-le}

`eq` is `Operator.Equals`. `ne` is not-equals. `gt` / `lt` are greater-than and less-than. `ge` / `le` are the or-equal forms.

**Definition**

```csharp
public static IExpression eq(string column, Value? value)
public static IExpression ne(string column, Value? value)
public static IExpression gt(string column, Value? value)
public static IExpression lt(string column, Value? value)
public static IExpression ge(string column, Value? value)
public static IExpression le(string column, Value? value)
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | column name |
| `value` | cell value |

**Returns**

[IExpression](#csharp-iexpression)

#### Identifier {#csharp-identifier}

```csharp
public sealed record Identifier(string Name) : IExpression
```

**Constructor**

| Name | Description |
| --- | --- |
| [Identifier(name)](#csharp-identifier-identifier-name) |  |

**Properties**

| Name | Description |
| --- | --- |
| Name |  |

##### Identifier(name) {#csharp-identifier-identifier-name}



**Definition**

```csharp
public Identifier(string Name)
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | column name |

#### Literal {#csharp-literal}

```csharp
public sealed record Literal(Value Value) : IExpression
```

**Constructor**

| Name | Description |
| --- | --- |
| [Literal(value)](#csharp-literal-literal-value) |  |

**Properties**

| Name | Description |
| --- | --- |
| Value |  |

##### Literal(value) {#csharp-literal-literal-value}



**Definition**

```csharp
public Literal(Value Value)
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | cell value |

#### ArrayExpression {#csharp-arrayexpression}

```csharp
public sealed record ArrayExpression(IReadOnlyList<IExpression> Elements) : IExpression
```

**Constructor**

| Name | Description |
| --- | --- |
| [ArrayExpression(elements)](#csharp-arrayexpression-arrayexpression-elements) |  |

**Properties**

| Name | Description |
| --- | --- |
| Elements |  |

##### ArrayExpression(elements) {#csharp-arrayexpression-arrayexpression-elements}



**Definition**

```csharp
public ArrayExpression(IReadOnlyList<IExpression> Elements)
```

**Parameters**

| Name | Description |
| --- | --- |
| `elements` | [IExpression](#csharp-iexpression) list |

#### Rust

#### Expression {#rust-expression}

```rust
pub enum Expression
```

#### Operator {#rust-operator}

```rust
pub enum Operator
```

#### Cmp {#rust-cmp}

```rust
pub fn eq(column: &str, value: Value) -> Operator
pub fn ne(column: &str, value: Value) -> Operator
pub fn gt(column: &str, value: Value) -> Operator
pub fn lt(column: &str, value: Value) -> Operator
pub fn ge(column: &str, value: Value) -> Operator
pub fn le(column: &str, value: Value) -> Operator
```

**Methods**

| Name | Description |
| --- | --- |
| [eq / ne / gt / lt / ge / le](#rust-cmp-eq-ne-gt-lt-ge-le) | `eq` calls `Operator::equals`. `ne` is not-equals. `gt` / `lt` are greater-than and less-than. `ge` / `le` are the or-equal forms. |

##### eq / ne / gt / lt / ge / le {#rust-cmp-eq-ne-gt-lt-ge-le}

`eq` calls `Operator::equals`. `ne` is not-equals. `gt` / `lt` are greater-than and less-than. `ge` / `le` are the or-equal forms.

**Definition**

```rust
pub fn eq(column: &str, value: Value) -> Operator
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | column name |
| `value` | [Value](where.md#rust-value) |

**Returns**

[Operator](#rust-operator)

#### Identifier {#rust-identifier}

```rust
pub struct Identifier
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(name)](#rust-identifier-new-name) |  |

##### new(name) {#rust-identifier-new-name}



**Definition**

```rust
pub fn new(name: impl Into<String>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | column name |

**Returns**

[Identifier](#rust-identifier)

#### Literal {#rust-literal}

```rust
pub struct Literal
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(value)](#rust-literal-new-value) |  |

##### new(value) {#rust-literal-new-value}



**Definition**

```rust
pub fn new(value: impl Into<Value>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | cell value |

**Returns**

[Literal](#rust-literal)

#### ArrayExpression {#rust-arrayexpression}

```rust
pub struct ArrayExpression
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(elements)](#rust-arrayexpression-new-elements) |  |

##### new(elements) {#rust-arrayexpression-new-elements}



**Definition**

```rust
pub fn new(elements: Vec<Expression>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `elements` | [Expression](#rust-expression) list |

**Returns**

[ArrayExpression](#rust-arrayexpression)

## Remarks

A comparison is `Operator.Equals(Identifier, Literal)`. `eq`, `ne`, `gt`, `lt`, `ge`, and `le` are aliases of those constructors. `And` / `Or` nest operator records. `Not` holds one argument. `In` uses an `ArrayExpression` of literals.

## Examples

### Compare tree

#### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/StatementCompileTests.cs#L197-L204)

```csharp
var where = new Where(new Operator.And(
    new Operator.Equals("id", 1),
    new Operator.GreaterThan("age", 18)));
```

#### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/statement_compile.rs#L249-L256)

```rust
let where_clause = Where::new(Operator::and(
    Operator::equals("id", 1_i64),
    Operator::greater_than("age", 18_i64),
));
```

#### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/statements/StatementCompile.test.ts#L215-L220)

```typescript
const where = new Where(
  new Operator.And(new Operator.Equals("id", 1), new Operator.GreaterThan("age", 18)),
);
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

## ExpressionEvaluator

Interprets an [IExpression](#) against a name→value map. [StatementTextRenderer](statements.md#statementtextrenderer) materializes SQL; this type evaluates.

### Classes

##### TypeScript

##### ExpressionEvaluator {#typescript-expressionevaluator}

```typescript
class ExpressionEvaluator
```

**Methods**

| Name | Description |
| --- | --- |
| [evaluate(expression, parameters?)](#typescript-expressionevaluator-evaluate-expression-parameters) | Resolves identifiers from `parameters`. A where condition returns a boolean. |

###### evaluate(expression, parameters?) {#typescript-expressionevaluator-evaluate-expression-parameters}

Resolves identifiers from `parameters`. A where condition returns a boolean.

**Definition**

```typescript
evaluate(expression: IExpression, parameters?: Record<string, Scalar>): Scalar
```

**Parameters**

| Name | Description |
| --- | --- |
| `expression` | [IExpression](#typescript-iexpression) |
| `parameters` | row / name→value context |

**Returns**

[Scalar](where.md#typescript-scalar)

##### C#

##### ExpressionEvaluator {#csharp-expressionevaluator}

```csharp
public sealed class ExpressionEvaluator
```

**Methods**

| Name | Description |
| --- | --- |
| [Evaluate(expression, parameters)](#csharp-expressionevaluator-evaluate-expression-parameters) | Resolves identifiers from `parameters`. A where condition returns a [Value](command.md) bool. |

###### Evaluate(expression, parameters) {#csharp-expressionevaluator-evaluate-expression-parameters}

Resolves identifiers from `parameters`. A where condition returns a [Value](command.md) bool.

**Definition**

```csharp
public Value Evaluate(IExpression expression, IReadOnlyDictionary<string, Value>? parameters)
```

**Parameters**

| Name | Description |
| --- | --- |
| `expression` | [IExpression](#csharp-iexpression) |
| `parameters` | row / name→value context |

**Returns**

`Value`

##### Rust

##### ExpressionEvaluator {#rust-expressionevaluator}

```rust
pub struct ExpressionEvaluator
```

**Methods**

| Name | Description |
| --- | --- |
| [evaluate(expression, parameters)](#rust-expressionevaluator-evaluate-expression-parameters) | Resolves identifiers from `parameters`. A where condition returns `Value::Bool`. |

###### evaluate(expression, parameters) {#rust-expressionevaluator-evaluate-expression-parameters}

Resolves identifiers from `parameters`. A where condition returns `Value::Bool`.

**Definition**

```rust
pub fn evaluate(&self, expression: &Expression, parameters: &Row) -> Value
```

**Parameters**

| Name | Description |
| --- | --- |
| `expression` | [Expression](#rust-expression) |
| `parameters` | row / name→value context |

**Returns**

[Value](command.md)

### Remarks

Parameters here are the row, not the SQL bind list on `IStatement.Parameters`. A raw text [WhereText](where.md#wheretext) is not evaluable.

### Examples

#### Match and miss

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/CoreTests.cs#L273-L352)

```csharp
var evaluator = new ExpressionEvaluator();
IReadOnlyDictionary<string, Value> row = new Dictionary<string, Value> { ["id"] = 1, ["age"] = 21, ["name"] = "a" };
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L1412-L1597)

```rust
let evaluator = ExpressionEvaluator;
let row = record(&[
    ("id", Value::Int(1)),
    ("age", Value::Int(21)),
    ("name", Value::Text("a".into())),
]);
let yes = Value::Bool(true);
let no = Value::Bool(false);
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/builders/builders.test.ts#L110-L188)

```typescript

```

## StatementEvaluator

Evaluates an [IStatement](statements.md#istatement). Expression trees use [ExpressionEvaluator](#).

### Classes

##### TypeScript

##### StatementEvaluator {#typescript-statementevaluator}

```typescript
class StatementEvaluator
```

**Methods**

| Name | Description |
| --- | --- |
| [evaluate(statement)](#typescript-statementevaluator-evaluate-statement) | Takes a statement. Body is empty until statement-level evaluation is defined. |

###### evaluate(statement) {#typescript-statementevaluator-evaluate-statement}

Takes a statement. Body is empty until statement-level evaluation is defined.

**Definition**

```typescript
evaluate(statement: IStatement): void
```

**Parameters**

| Name | Description |
| --- | --- |
| `statement` | [IStatement](statements.md#typescript-istatement) |

##### C#

##### StatementEvaluator {#csharp-statementevaluator}

```csharp
public sealed class StatementEvaluator
```

**Methods**

| Name | Description |
| --- | --- |
| [Evaluate(statement)](#csharp-statementevaluator-evaluate-statement) | Takes a statement. Body is empty until statement-level evaluation is defined. |

###### Evaluate(statement) {#csharp-statementevaluator-evaluate-statement}

Takes a statement. Body is empty until statement-level evaluation is defined.

**Definition**

```csharp
public void Evaluate(IStatement statement)
```

**Parameters**

| Name | Description |
| --- | --- |
| `statement` | [IStatement](statements.md#csharp-istatement) |

##### Rust

##### StatementEvaluator {#rust-statementevaluator}

```rust
pub struct StatementEvaluator
```

**Methods**

| Name | Description |
| --- | --- |
| [evaluate(statement)](#rust-statementevaluator-evaluate-statement) | Takes a statement. Body is empty until statement-level evaluation is defined. |

###### evaluate(statement) {#rust-statementevaluator-evaluate-statement}

Takes a statement. Body is empty until statement-level evaluation is defined.

**Definition**

```rust
pub fn evaluate(&self, statement: &dyn Statement)
```

**Parameters**

| Name | Description |
| --- | --- |
| `statement` | [Statement](statements.md#rust-statement) |

### Remarks

For predicates and operators against a row map, use [ExpressionEvaluator](#).
