# Expressions

AST nodes for [WhereClause](./where-clause.md). `Operator` records hold operands; `GetText()` maps them to SQL.

## Classes

::: reference
### typescript
#### IExpression

```typescript
interface IExpression {}
```

#### Operator

```typescript
abstract class Operator implements IExpression
```

Compare, group, not, and in nodes. Field constructors take a column and [Scalar](./where-clause.md#scalar): `new Operator.Equals("id", 9)`.

##### Operator.Equals / NotEquals / GreaterThan / LessThan / GreaterThanOrEqual / LessThanOrEqual

```typescript
new Operator.Equals(field: string, value: Scalar)
new Operator.Equals(left: IExpression, right: IExpression)
```

##### Operator.And / Or

```typescript
new Operator.And(left: IExpression, right: IExpression)
```

##### Operator.In

```typescript
new Operator.In(field: string, values: Scalar[])
new Operator.In(left: IExpression, right: IExpression)
```

##### Operator.Not

```typescript
new Operator.Not(argument: IExpression)
```

#### Identifier

```typescript
class Identifier implements IExpression
```

**Constructor**

##### Identifier(name)

```typescript
constructor(name: string)
```

- `name` — column name

**Properties**

##### name

```typescript
readonly name: string
```

#### Literal

```typescript
class Literal implements IExpression
```

**Constructor**

##### Literal(value)

```typescript
constructor(value: Scalar)
```

- `value` — [Scalar](./where-clause.md#scalar)

**Properties**

##### value

```typescript
readonly value: Scalar
```

#### ArrayExpression

```typescript
class ArrayExpression implements IExpression
```

**Constructor**

##### ArrayExpression(elements)

```typescript
constructor(elements: IExpression[])
```

- `elements` — [IExpression](#iexpression) list

**Properties**

##### elements

```typescript
readonly elements: IExpression[]
```

### csharp
#### IExpression

```csharp
public interface IExpression
```

#### Operator

```csharp
namespace Datasources.Operator
```

Compare, group, not, and in types. Field constructors take a column and value: `new Operator.Equals("id", 9)`.

```csharp
public sealed class Equals : IExpression
public Equals(string field, Value? value)

public sealed record And(IExpression Left, IExpression Right) : IExpression
public sealed record Or(IExpression Left, IExpression Right) : IExpression
public sealed record In(IExpression Left, IExpression Right) : IExpression
public In(string field, IReadOnlyList<Value> values)

public sealed record Not(IExpression Argument) : IExpression
```

Same shape for `NotEquals`, `GreaterThan`, `LessThan`, `GreaterThanOrEqual`, and `LessThanOrEqual`.

#### Identifier

```csharp
public sealed record Identifier(string Name) : IExpression
```

**Constructor**

##### Identifier(name)

```csharp
public Identifier(string Name)
```

- `name` — column name

**Properties**

##### Name

```csharp
string Name { get; }
```

#### Literal

```csharp
public sealed record Literal(Value Value) : IExpression
```

**Constructor**

##### Literal(value)

```csharp
public Literal(Value Value)
```

- `value` — cell value

**Properties**

##### Value

```csharp
Value Value { get; }
```

#### ArrayExpression

```csharp
public sealed record ArrayExpression(IReadOnlyList<IExpression> Elements) : IExpression
```

**Constructor**

##### ArrayExpression(elements)

```csharp
public ArrayExpression(IReadOnlyList<IExpression> Elements)
```

- `elements` — [IExpression](#iexpression) list

**Properties**

##### Elements

```csharp
IReadOnlyList<IExpression> Elements { get; }
```

### rust
#### Expression

```rust
pub enum Expression
```

#### Operator

```rust
pub enum Operator
```

Compare, group, not, and in variants hold operands. Field constructors: `Operator::equals("id", 9)`.

```rust
Equals { left: Box<Expression>, right: Box<Expression> }
And { left: Box<Expression>, right: Box<Expression> }
Or { left: Box<Expression>, right: Box<Expression> }
In { left: Box<Expression>, right: Box<Expression> }
Not { argument: Box<Expression> }
```

```rust
pub fn equals(field: impl Into<String>, value: impl Into<Value>) -> Self
pub fn and(left: impl Into<Expression>, right: impl Into<Expression>) -> Self
pub fn not(argument: impl Into<Expression>) -> Self
pub fn in_list(field: impl Into<String>, values: Vec<Value>) -> Self
```

Same field constructors for `not_equals`, `greater_than`, `less_than`, `greater_than_or_equal`, and `less_than_or_equal`.

#### Identifier

```rust
pub struct Identifier
```

**Constructor**

##### new(name)

```rust
pub fn new(name: impl Into<String>) -> Self
```

- `name` — column name

**Returns** [Identifier](#identifier)

#### Literal

```rust
pub struct Literal
```

**Constructor**

##### new(value)

```rust
pub fn new(value: impl Into<Value>) -> Self
```

- `value` — cell value

**Returns** [Literal](#literal)

#### ArrayExpression

```rust
pub struct ArrayExpression
```

**Constructor**

##### new(elements)

```rust
pub fn new(elements: Vec<Expression>) -> Self
```

- `elements` — [Expression](#expression) list

**Returns** [ArrayExpression](#arrayexpression)
:::

## Remarks

A comparison is `Operator.Equals(Identifier, Literal)`. `And` / `Or` nest operator records. `Not` holds one argument. `In` uses an `ArrayExpression` of literals.

## Examples

### Compare tree

::: example
```csharp csharp/Datasources.Tests/CoreTests.cs#L78-L81
```
```rust rust/datasources/src/tests/mod.rs#L704-L719
```
```typescript typescript/datasources/src/builders/builders.test.ts#L63-L73
```
:::
