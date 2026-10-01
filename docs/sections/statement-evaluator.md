# StatementEvaluator

Evaluates an [IStatement](./statements.md#istatement). Expression trees use [ExpressionEvaluator](./expression-evaluator.md).

## Classes

::: reference
### typescript
#### StatementEvaluator

```typescript
class StatementEvaluator
```

**Methods**

##### evaluate(statement)

Takes a statement. Body is empty until statement-level evaluation is defined.

```typescript
evaluate(statement: IStatement): void
```

- `statement` — [IStatement](./statements.md#istatement)

### csharp
#### StatementEvaluator

```csharp
public sealed class StatementEvaluator
```

**Methods**

##### Evaluate(statement)

Takes a statement. Body is empty until statement-level evaluation is defined.

```csharp
public void Evaluate(IStatement statement)
```

- `statement` — [IStatement](./statements.md#istatement)

### rust
#### StatementEvaluator

```rust
pub struct StatementEvaluator
```

**Methods**

##### evaluate(statement)

Takes a statement. Body is empty until statement-level evaluation is defined.

```rust
pub fn evaluate(&self, statement: &dyn Statement)
```

- `statement` — [Statement](./statements.md#statement)
:::

## Remarks

For predicates and operators against a row map, use [ExpressionEvaluator](./expression-evaluator.md).
