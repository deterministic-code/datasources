# Where

[IWhere](#iwhere) over an [IExpression](./expressions.md#iexpression) tree. A clause is an expression, not a statement. Placeholders are `$field` and `$field_0` for `In`.

## Classes

::: reference
### typescript
#### Where

```typescript
class Where implements IWhere
```

**Constructor**

##### Where(condition)

Builds a where clause from an [IExpression](./expressions.md#iexpression).

```typescript
constructor(condition: IExpression)
```

- `condition` — [IExpression](./expressions.md#iexpression)

##### Where(column, op, value)

Builds a comparison from a column, a [Cmp](./expressions.md#cmp) alias, and a literal.

```typescript
constructor(column: string, op: Comparison, value: Cell)
```

- `column` — column name
- `op` — [Cmp](./expressions.md#cmp) alias such as `eq` or `gt`
- `value` — [Cell](./flat-to-nested-mapper.md#cell)

**Properties**

##### condition

Predicate tree. Typically an [Operator](./expressions.md#operator).

```typescript
readonly condition: IExpression
```


##### parameters

Bound [Parameter](./command.md#parameter) list.

```typescript
get parameters(): Parameter[]
```

**Methods**

##### accept(visitor)

Dispatches to a condition visitor that renders predicate text.

```typescript
accept(visitor: IConditionVisitor): string
```

- `visitor` — [IConditionVisitor](#iconditionvisitor)

**Returns** `string`

##### and(column, op, value) / or(column, op, value)

Nests this clause with another comparison under [Operator.And](./expressions.md#operator) or `Operator.Or`.

```typescript
and(column: string, op: Comparison, value: Cell): Where
or(column: string, op: Comparison, value: Cell): Where
```

- `column` — column name
- `op` — [Cmp](./expressions.md#cmp) alias
- `value` — [Cell](./flat-to-nested-mapper.md#cell)

**Returns** [Where](#where)

##### map(column, value)

Rewrites identifiers through `column` and the literals beside them through `value`. The repository calls this so a clause uses stored column names. A field converter, when one is set, rewrites that property's literals.

```typescript
map(column: (name: string) => string, value: (name: string, cell: Cell) => Cell): Where
```

- `column` — property name to stored column
- `value` — property name and domain cell to stored cell

**Returns** [Where](#where)

#### ICondition

```typescript
interface ICondition
```

**Methods**

##### accept(visitor)

Dispatches to a condition visitor that renders predicate text.

```typescript
accept(visitor: IConditionVisitor): string
```

- `visitor` — [IConditionVisitor](#iconditionvisitor)

**Returns** `string`

#### IConditionVisitor

```typescript
interface IConditionVisitor
```

Visitor that turns a where into predicate text. [ConditionRenderer](#conditionrenderer) is the default.

**Methods**

##### visit(where)

```typescript
visit(where: Where): string
visit(where: WhereText): string
```

- `where` — [Where](#where) or [WhereText](#wheretext)

**Returns** `string`

#### ConditionRenderer

```typescript
class ConditionRenderer implements IConditionVisitor
```

Default [IConditionVisitor](#iconditionvisitor).

#### IWhere

```typescript
interface IWhere extends IExpression, ICondition {}
```

#### WhereText

```typescript
class WhereText implements IWhere
```

**Constructor**

##### WhereText(text)

Wraps predicate text.

```typescript
constructor(text: string)
```

- `text` — predicate text

**Methods**

##### accept(visitor)

Dispatches to a condition visitor that renders predicate text.

```typescript
accept(visitor: IConditionVisitor): string
```

- `visitor` — [IConditionVisitor](#iconditionvisitor)

**Returns** `string`

#### Scalar

```typescript
type Scalar = string | number | boolean | bigint | null
```

Cell value used by [Where](#where) and [Identifier](./expressions.md#identifier).

### csharp
#### Where

```csharp
public sealed record Where(IExpression Condition) : IWhere
```

**Constructor**

##### Where(condition)

Builds a where clause from an [IExpression](./expressions.md#iexpression).

```csharp
public Where(IExpression Condition)
```

- `condition` — [IExpression](./expressions.md#iexpression)

##### Where(column, op, value)

Builds a comparison from a column, a [Cmp](./expressions.md#cmp) alias, and a literal.

```csharp
public Where(string column, Comparison op, Value? value)
```

- `column` — column name
- `op` — [Cmp](./expressions.md#cmp) alias such as `eq` or `gt`
- `value` — cell value

**Properties**

##### Condition

Predicate tree. Typically an [Operator](./expressions.md#operator).

```csharp
IExpression Condition { get; }
```


##### Parameters

Bound [Parameter](./command.md#parameter) list.

```csharp
public IReadOnlyList<Parameter> Parameters { get; }
```

**Methods**

##### Accept(visitor)

Dispatches to a condition visitor that renders predicate text.

```csharp
public string Accept(IConditionVisitor visitor)
```

- `visitor` — [IConditionVisitor](#iconditionvisitor)

**Returns** `string`

##### And(column, op, value) / Or(column, op, value)

Nests this clause with another comparison under `Operator.And` or `Operator.Or`.

```csharp
public Where And(string column, Comparison op, Value? value)
public Where Or(string column, Comparison op, Value? value)
```

- `column` — column name
- `op` — [Cmp](./expressions.md#cmp) alias
- `value` — cell value

**Returns** [Where](#where)

##### Map(column, value)

Rewrites identifiers through `column` and the literals beside them through `value`. The repository calls this so a clause uses stored column names. A field converter, when one is set, rewrites that property's literals.

```csharp
public Where Map(Func<string, string> column, Func<string, Value, Value> value)
```

- `column` — property name to stored column
- `value` — property name and domain cell to stored cell

**Returns** [Where](#where)

#### ICondition

```csharp
public interface ICondition
```

**Methods**

##### Accept(visitor)

Dispatches to a condition visitor that renders predicate text.

```csharp
string Accept(IConditionVisitor visitor)
```

- `visitor` — [IConditionVisitor](#iconditionvisitor)

**Returns** `string`

#### IConditionVisitor

```csharp
public interface IConditionVisitor
```

Visitor that turns a where into predicate text. [ConditionRenderer](#conditionrenderer) is the default.

**Methods**

##### Visit(where)

```csharp
string Visit(Where where)
string Visit(WhereText where)
```

- `where` — [Where](#where) or [WhereText](#wheretext)

**Returns** `string`

#### ConditionRenderer

```csharp
public class ConditionRenderer : IConditionVisitor
```

Default [IConditionVisitor](#iconditionvisitor).

#### IWhere

```csharp
public interface IWhere : IExpression, ICondition
```

#### WhereText

```csharp
public sealed record WhereText : IWhere
```

**Constructor**

##### WhereText(text)

Wraps predicate text.

```csharp
public WhereText(string text)
```

- `text` — predicate text

**Methods**

##### Accept(visitor)

Dispatches to a condition visitor that renders predicate text.

```csharp
string Accept(IConditionVisitor visitor)
```

- `visitor` — [IConditionVisitor](#iconditionvisitor)

**Returns** `string`

### rust
#### Where

```rust
pub struct Where
```

**Constructor**

##### new(condition)

Builds a where clause from an [Expression](./expressions.md#expression).

```rust
pub fn new(condition: impl Into<Expression>) -> Self
```

- `condition` — [Expression](./expressions.md#expression) or [Operator](./expressions.md#operator)

**Returns** [Where](#where)

##### binary(column, op, value)

Builds a comparison from a column, a [Cmp](./expressions.md#cmp) alias, and a literal.

```rust
pub fn binary(column: &str, op: fn(&str, Value) -> Operator, value: impl Into<Value>) -> Self
```

- `column` — column name
- `op` — [Cmp](./expressions.md#cmp) alias such as `eq` or `gt`
- `value` — [Value](#value)

**Returns** [Where](#where)

##### and(column, op, value) / or(column, op, value)

Nests this clause with another comparison under `Operator::and` or `Operator::or`.

```rust
pub fn and(self, column: &str, op: fn(&str, Value) -> Operator, value: impl Into<Value>) -> Self
pub fn or(self, column: &str, op: fn(&str, Value) -> Operator, value: impl Into<Value>) -> Self
```

- `column` — column name
- `op` — [Cmp](./expressions.md#cmp) alias
- `value` — [Value](#value)

**Returns** [Where](#where)

##### map(column, value)

Rewrites identifiers through `column` and the literals beside them through `value`. The repository calls this so a clause uses stored column names. A field converter, when one is set, rewrites that property's literals.

```rust
pub fn map<C, V>(&self, column: C, value: V) -> Result<Self>
where
    C: Fn(&str) -> String,
    V: Fn(&str, &Value) -> Result<Value>,
```

- `column` — property name to stored column
- `value` — property name and domain cell to stored cell

**Returns** `Result<`[Where](#where)`>`

##### accept()

Dispatches through the renderer's condition visitor.

```rust
fn accept(&self, text: &StatementTextRenderer) -> String
```

- `text` — [StatementTextRenderer](./statements.md#statementtextrenderer)

**Returns** `String`

##### parameters()

Bound [Parameter](./command.md#parameter) list.

```rust
fn parameters(&self) -> Vec<Parameter>
```

**Returns** `Vec<`[Parameter](./command.md#parameter)`>`

#### ICondition

```rust
pub trait ICondition: Send + Sync
```

**Methods**

##### accept()

Dispatches through the renderer's condition visitor.

```rust
fn accept(&self, text: &StatementTextRenderer) -> String
```

- `text` — [StatementTextRenderer](./statements.md#statementtextrenderer)

**Returns** `String`

#### ConditionVisitor

```rust
pub trait ConditionVisitor: Send + Sync
```

Visitor that turns a where into predicate text. [ConditionRenderer](#conditionrenderer) is the default.

#### ConditionRenderer

```rust
pub struct ConditionRenderer
```

Default [ConditionVisitor](#conditionvisitor).

#### IWhere

```rust
pub trait IWhere: ICondition
```

#### WhereText

```rust
pub struct WhereText
```

**Constructor**

##### new(text)

Wraps predicate text.

```rust
pub fn new(text: impl Into<String>) -> Self
```

- `text` — predicate text

**Returns** [WhereText](#wheretext)

**Methods**

##### accept()

Dispatches through the renderer's condition visitor.

```rust
fn accept(&self, text: &StatementTextRenderer) -> String
```

- `text` — [StatementTextRenderer](./statements.md#statementtextrenderer)

**Returns** `String`
:::

## Remarks

Construct the predicate with [Operator](./expressions.md#operator) records, then wrap it: `new Where(new Operator.Equals("id", 9))`. A column, [Cmp](./expressions.md#cmp) alias, and literal is the same tree: `new Where("id", eq, 9)`. [SelectStatement](./select-statement.md) / [UpdateStatement](./update-statement.md) / [DeleteStatement](./delete-statement.md) embed {{StatementTextRenderer.Render|StatementTextRenderer.render|StatementTextRenderer::render}} of the where in the command SQL. Keep the [Where](#where) when the caller needs bound parameters. [ExpressionEvaluator](./expression-evaluator.md) interprets the same tree against a parameter map. Raw text [WhereText](#wheretext) is not evaluable.

## Examples

### Compare, groups, not, and in

::: example
```csharp csharp/Datasources.Tests/CoreTests.cs#StatementsAndBuilders
```
```rust rust/datasources/src/tests/mod.rs#published_mocks
```
```typescript typescript/datasources/src/builders/builders.test.ts#renders-compare-groups-and-not
```

```output [After: and]
id = $id AND age > $age
```

```output [After: in]
id IN ($id_0, $id_1)
```
:::

### Column, alias, literal

::: example
```csharp csharp/Datasources.Tests/ComparisonAliasTests.cs#RendersComparisonSymbols
```
```rust rust/datasources/src/tests/mod.rs#comparison_aliases
```
```typescript typescript/datasources/src/builders/builders.test.ts#creates-write-and-ddl-statements
```

```output [After: equals]
username = $username
```

```output [After: and]
username = $username AND is_enabled = $is_enabled
```
:::

### Attach to a query

::: example
```csharp csharp/Datasources.Tests/CoreTests.cs#StatementsAndBuilders
```
```rust rust/datasources/src/tests/mod.rs#statements_and_builders
```
```typescript typescript/datasources/src/builders/builders.test.ts#evaluates-compare-groups-not-and-in
```

```output [After: select]
SELECT id, name FROM users WHERE id = $id AND age > $age
```

```output:typescript [After: select]
SELECT id, name FROM users WHERE id = $id
```
:::
