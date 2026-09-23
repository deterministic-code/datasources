# Where Clause

[IWhere](#iwhere) over an [IExpression](./expressions.md#iexpression) tree. A clause is an expression, not a statement. Placeholders are `$field` and `$field_0` for `In`.

## Classes

::: reference
### typescript
#### WhereClause

```typescript
class WhereClause implements IWhere
```

**Constructor**

##### WhereClause(condition)

Builds a where clause from an [IExpression](./expressions.md#iexpression).

```typescript
constructor(condition: IExpression)
```

- `condition` — [IExpression](./expressions.md#iexpression)

##### WhereClause(column, op, value)

Builds a comparison from a column, a [Cmp](./expressions.md#cmp) alias, and a literal.

```typescript
constructor(column: string, op: Comparison, value: Scalar)
```

- `column` — column name
- `op` — [Cmp](./expressions.md#cmp) alias such as `eq` or `gt`
- `value` — [Scalar](#scalar)

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

##### getText()

Predicate text.

```typescript
getText(): string
```

**Returns** `string`

##### and(column, op, value) / or(column, op, value)

Nests this clause with another comparison under [Operator.And](./expressions.md#operator) or `Operator.Or`.

```typescript
and(column: string, op: Comparison, value: Scalar): WhereClause
or(column: string, op: Comparison, value: Scalar): WhereClause
```

- `column` — column name
- `op` — [Cmp](./expressions.md#cmp) alias
- `value` — [Scalar](#scalar)

**Returns** [WhereClause](#whereclause)

#### IWhere

```typescript
interface IWhere extends IExpression, IText {}
```

#### Where

```typescript
class Where implements IWhere
```

**Constructor**

##### Where(text)

Wraps predicate text.

```typescript
constructor(text: string)
```

- `text` — predicate text

**Methods**

##### getText()

Predicate text.

```typescript
getText(): string
```

**Returns** `string`

#### Scalar

```typescript
type Scalar = string | number | boolean | bigint | null
```

Cell value used by [WhereClause](#whereclause) and [Identifier](./expressions.md#identifier).

### csharp
#### WhereClause

```csharp
public sealed record WhereClause(IExpression Condition) : IWhere
```

**Constructor**

##### WhereClause(condition)

Builds a where clause from an [IExpression](./expressions.md#iexpression).

```csharp
public WhereClause(IExpression Condition)
```

- `condition` — [IExpression](./expressions.md#iexpression)

##### WhereClause(column, op, value)

Builds a comparison from a column, a [Cmp](./expressions.md#cmp) alias, and a literal.

```csharp
public WhereClause(string column, Comparison op, Value? value)
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

##### GetText()

Predicate text.

```csharp
public string GetText()
```

**Returns** `string`

##### And(column, op, value) / Or(column, op, value)

Nests this clause with another comparison under `Operator.And` or `Operator.Or`.

```csharp
public WhereClause And(string column, Comparison op, Value? value)
public WhereClause Or(string column, Comparison op, Value? value)
```

- `column` — column name
- `op` — [Cmp](./expressions.md#cmp) alias
- `value` — cell value

**Returns** [WhereClause](#whereclause)

#### IWhere

```csharp
public interface IWhere : IExpression, IText
```

#### Where

```csharp
public sealed class Where : IWhere
```

**Constructor**

##### Where(text)

Wraps predicate text.

```csharp
public Where(string text)
```

- `text` — predicate text

**Methods**

##### GetText()

Predicate text.

```csharp
string GetText()
```

**Returns** `string`

### rust
#### WhereClause

```rust
pub struct WhereClause
```

**Constructor**

##### new(condition)

Builds a where clause from an [Expression](./expressions.md#expression).

```rust
pub fn new(condition: impl Into<Expression>) -> Self
```

- `condition` — [Expression](./expressions.md#expression) or [Operator](./expressions.md#operator)

**Returns** [WhereClause](#whereclause)

##### binary(column, op, value)

Builds a comparison from a column, a [Cmp](./expressions.md#cmp) alias, and a literal.

```rust
pub fn binary(column: &str, op: fn(&str, Value) -> Operator, value: impl Into<Value>) -> Self
```

- `column` — column name
- `op` — [Cmp](./expressions.md#cmp) alias such as `eq` or `gt`
- `value` — [Value](#value)

**Returns** [WhereClause](#whereclause)

##### and(column, op, value) / or(column, op, value)

Nests this clause with another comparison under `Operator::and` or `Operator::or`.

```rust
pub fn and(self, column: &str, op: fn(&str, Value) -> Operator, value: impl Into<Value>) -> Self
pub fn or(self, column: &str, op: fn(&str, Value) -> Operator, value: impl Into<Value>) -> Self
```

- `column` — column name
- `op` — [Cmp](./expressions.md#cmp) alias
- `value` — [Value](#value)

**Returns** [WhereClause](#whereclause)

##### get_text()

Predicate text.

```rust
fn get_text(&self) -> String
```

**Returns** `String`

##### parameters()

Bound [Parameter](./command.md#parameter) list.

```rust
fn parameters(&self) -> Vec<Parameter>
```

**Returns** `Vec<`[Parameter](./command.md#parameter)`>`

#### Where

```rust
pub trait Where: Text {}
```

#### TextWhere

```rust
pub struct TextWhere
```

**Constructor**

##### new(text)

Wraps predicate text.

```rust
pub fn new(text: impl Into<String>) -> Self
```

- `text` — predicate text

**Returns** [TextWhere](#textwhere)

**Methods**

##### get_text()

Predicate text.

```rust
fn get_text(&self) -> String
```

**Returns** `String`
:::

## Remarks

Construct the predicate with [Operator](./expressions.md#operator) records, then wrap it: `new WhereClause(new Operator.Equals("id", 9))`. A column, [Cmp](./expressions.md#cmp) alias, and literal is the same tree: `new WhereClause("id", eq, 9)`. [QueryStatement](./query-statement.md) / [UpdateStatement](./update-statement.md) / [DeleteStatement](./delete-statement.md) embed `WhereClause.GetText()` in the command SQL. Keep the [WhereClause](#whereclause) when the caller needs bound parameters. [StatementEvaluator](./statement-evaluator.md) interprets the same tree against a parameter map. Raw text [Where](#where) is not evaluable.

## Examples

### Compare, groups, not, and in

::: example
```csharp csharp/Datasources.Tests/CoreTests.cs#L53-L55
```
```rust rust/datasources/src/tests/mod.rs#L659-L663
```
```typescript typescript/datasources/src/builders/builders.test.ts#L31-L33
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
```csharp csharp/Datasources.Tests/ComparisonAliasTests.cs#L11-L17
```
```rust rust/datasources/src/tests/mod.rs#L2490-L2496
```
```typescript typescript/datasources/src/builders/builders.test.ts#L350-L356
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
```csharp csharp/Datasources.Tests/CoreTests.cs#L127-L130
```
```rust rust/datasources/src/tests/mod.rs#L929-L935
```
```typescript typescript/datasources/src/builders/builders.test.ts#L175-L180
```

```output [After: select]
SELECT id, name FROM users WHERE id = $id AND age > $age
```

```output:typescript [After: select]
SELECT id, name FROM users WHERE id = $id
```
:::
