# Where Clause

[IWhere](./statements.md#iwhere) over an [IExpression](./expressions.md#iexpression) tree. A clause is an expression, not a statement. Placeholders are `$field` and `$field_0` for `In`.

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

#### Scalar

```typescript
type Scalar = string | number | boolean | bigint | null
```

Cell value used by [WhereClause](#whereclause) and [Field](./statements.md#field).

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
:::

## Remarks

Construct the predicate with [Operator](./expressions.md#operator) records, then wrap it: `new WhereClause(new Operator.Equals("id", 9))`. [QueryStatement](./statements.md#querystatement) / [UpdateStatement](./statements.md#updatestatement) / [DeleteStatement](./statements.md#deletestatement) embed `WhereClause.GetText()` in the command SQL. Keep the [WhereClause](#whereclause) when the caller needs bound parameters. [StatementEvaluator](./statement-evaluator.md) interprets the same tree against a parameter map.

## Examples

### Compare, groups, not, and in

::: example
```csharp csharp/Datasources.Tests/CoreTests.cs#L53-L55
```
```rust rust/datasources/src/tests/mod.rs#L654-L658
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
