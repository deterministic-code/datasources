# StatementEvaluator

Interprets an [IExpression](./expressions.md) against a name→value map. [Where.GetText](./where-clause.md) materializes SQL; this type evaluates.

## Classes

::: reference
### typescript
#### StatementEvaluator

```typescript
class StatementEvaluator
```

**Methods**

##### evaluate(expression, parameters?)

Resolves identifiers from `parameters`. A where condition returns a boolean.

```typescript
evaluate(expression: IExpression, parameters?: Record<string, Scalar>): Scalar
```

- `expression` — [IExpression](./expressions.md#iexpression)
- `parameters` — row / name→value context

**Returns** [Scalar](./where-clause.md#scalar)

### csharp
#### StatementEvaluator

```csharp
public sealed class StatementEvaluator
```

**Methods**

##### Evaluate(expression, parameters)

Resolves identifiers from `parameters`. A where condition returns a [Value](./command.md) bool.

```csharp
public Value Evaluate(IExpression expression, IReadOnlyDictionary<string, Value>? parameters)
```

- `expression` — [IExpression](./expressions.md#iexpression)
- `parameters` — row / name→value context

**Returns** `Value`

### rust
#### StatementEvaluator

```rust
pub struct StatementEvaluator
```

**Methods**

##### evaluate(expression, parameters)

Resolves identifiers from `parameters`. A where condition returns `Value::Bool`.

```rust
pub fn evaluate(&self, expression: &Expression, parameters: &Row) -> Value
```

- `expression` — [Expression](./expressions.md#expression)
- `parameters` — row / name→value context

**Returns** [Value](./command.md)
:::

## Remarks

Parameters here are the row, not the SQL bind list on `IStatement.Parameters`. A raw text [WhereText](./where-clause.md#wheretext) is not evaluable.

## Examples

### Match and miss

::: example
```csharp csharp/Datasources.Tests/CoreTests.cs#L244-L245
```
```rust rust/datasources/src/tests/mod.rs#L1166-L1170
```
```typescript typescript/datasources/src/builders/builders.test.ts#L100-L101
```
:::
