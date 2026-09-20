# WhereBuilder

[IWhere](./statements.md#iwhere) built from compare, group, not, and in factories. Placeholders are `$field` and `$field_0` for `In`.

## Classes

::: reference
### typescript
#### WhereBuilder

```typescript
class WhereBuilder implements IWhere
```

**Properties**

##### parameters

Bound [Parameter](./command.md#parameter) list.

```typescript
get parameters(): Parameter[]
```

**Methods**

##### equals(field, value)

`field = $field`.

```typescript
static equals(field: string, value: Scalar): WhereBuilder
```

- `field` — column name
- `value` — [Scalar](#scalar)

**Returns** [WhereBuilder](#wherebuilder)

##### ne(field, value)

`field <> $field`.

```typescript
static ne(field: string, value: Scalar): WhereBuilder
```

- `field` — column name
- `value` — [Scalar](#scalar)

**Returns** [WhereBuilder](#wherebuilder)

##### greaterThan(field, value)

`field > $field`.

```typescript
static greaterThan(field: string, value: Scalar): WhereBuilder
```

- `field` — column name
- `value` — [Scalar](#scalar)

**Returns** [WhereBuilder](#wherebuilder)

##### lt(field, value)

`field < $field`.

```typescript
static lt(field: string, value: Scalar): WhereBuilder
```

- `field` — column name
- `value` — [Scalar](#scalar)

**Returns** [WhereBuilder](#wherebuilder)

##### gte(field, value)

`field >= $field`.

```typescript
static gte(field: string, value: Scalar): WhereBuilder
```

- `field` — column name
- `value` — [Scalar](#scalar)

**Returns** [WhereBuilder](#wherebuilder)

##### lte(field, value)

`field <= $field`.

```typescript
static lte(field: string, value: Scalar): WhereBuilder
```

- `field` — column name
- `value` — [Scalar](#scalar)

**Returns** [WhereBuilder](#wherebuilder)

##### and(...children)

Joins children with `AND`. Empty is `1 = 1`.

```typescript
static and(...children: WhereBuilder[]): WhereBuilder
```

- `children` — [WhereBuilder](#wherebuilder) predicates

**Returns** [WhereBuilder](#wherebuilder)

##### or(...children)

Joins children with `OR`. Empty is `1 = 0`.

```typescript
static or(...children: WhereBuilder[]): WhereBuilder
```

- `children` — [WhereBuilder](#wherebuilder) predicates

**Returns** [WhereBuilder](#wherebuilder)

##### not(child)

`NOT (child)`.

```typescript
static not(child: WhereBuilder): WhereBuilder
```

- `child` — [WhereBuilder](#wherebuilder)

**Returns** [WhereBuilder](#wherebuilder)

##### in(field, values)

`field IN ($field_0, …)`. Empty is `1 = 0`.

```typescript
static in(field: string, values: Scalar[]): WhereBuilder
```

- `field` — column name
- `values` — [Scalar](#scalar) list

**Returns** [WhereBuilder](#wherebuilder)

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

Cell value used by [WhereBuilder](#wherebuilder) and [Field](./statement-builder.md#field).

### csharp
#### WhereBuilder

```csharp
public sealed class WhereBuilder : IWhere
```

**Properties**

##### Parameters

Bound [Parameter](./command.md#parameter) list.

```csharp
public IReadOnlyList<Parameter> Parameters { get; }
```

**Methods**

##### Equals(field, value)

`field = $field`.

```csharp
public static WhereBuilder Equals(string field, Value? value)
```

- `field` — column name
- `value` — cell value; omitted is null

**Returns** [WhereBuilder](#wherebuilder)

##### Ne(field, value)

`field <> $field`.

```csharp
public static WhereBuilder Ne(string field, Value? value)
```

- `field` — column name
- `value` — cell value; omitted is null

**Returns** [WhereBuilder](#wherebuilder)

##### GreaterThan(field, value)

`field > $field`.

```csharp
public static WhereBuilder GreaterThan(string field, Value? value)
```

- `field` — column name
- `value` — cell value; omitted is null

**Returns** [WhereBuilder](#wherebuilder)

##### Lt(field, value)

`field < $field`.

```csharp
public static WhereBuilder Lt(string field, Value? value)
```

- `field` — column name
- `value` — cell value; omitted is null

**Returns** [WhereBuilder](#wherebuilder)

##### Gte(field, value)

`field >= $field`.

```csharp
public static WhereBuilder Gte(string field, Value? value)
```

- `field` — column name
- `value` — cell value; omitted is null

**Returns** [WhereBuilder](#wherebuilder)

##### Lte(field, value)

`field <= $field`.

```csharp
public static WhereBuilder Lte(string field, Value? value)
```

- `field` — column name
- `value` — cell value; omitted is null

**Returns** [WhereBuilder](#wherebuilder)

##### And(children)

Joins children with `AND`. Empty is `1 = 1`.

```csharp
public static WhereBuilder And(params WhereBuilder[] children)
```

- `children` — [WhereBuilder](#wherebuilder) predicates

**Returns** [WhereBuilder](#wherebuilder)

##### Or(children)

Joins children with `OR`. Empty is `1 = 0`.

```csharp
public static WhereBuilder Or(params WhereBuilder[] children)
```

- `children` — [WhereBuilder](#wherebuilder) predicates

**Returns** [WhereBuilder](#wherebuilder)

##### Not(child)

`NOT (child)`.

```csharp
public static WhereBuilder Not(WhereBuilder child)
```

- `child` — [WhereBuilder](#wherebuilder)

**Returns** [WhereBuilder](#wherebuilder)

##### In(field, values)

`field IN ($field_0, …)`. Empty is `1 = 0`.

```csharp
public static WhereBuilder In(string field, IReadOnlyList<Value> values)
```

- `field` — column name
- `values` — cell values

**Returns** [WhereBuilder](#wherebuilder)

##### GetText()

Predicate text.

```csharp
public string GetText()
```

**Returns** `string`

### rust
#### WhereBuilder

```rust
pub struct WhereBuilder
```

**Methods**

##### equals(field, value)

`field = $field`.

```rust
pub fn equals(field: impl Into<String>, value: impl Into<Value>) -> Self
```

- `field` — column name
- `value` — cell value

**Returns** [WhereBuilder](#wherebuilder)

##### ne(field, value)

`field <> $field`.

```rust
pub fn ne(field: impl Into<String>, value: impl Into<Value>) -> Self
```

- `field` — column name
- `value` — cell value

**Returns** [WhereBuilder](#wherebuilder)

##### greater_than(field, value)

`field > $field`.

```rust
pub fn greater_than(field: impl Into<String>, value: impl Into<Value>) -> Self
```

- `field` — column name
- `value` — cell value

**Returns** [WhereBuilder](#wherebuilder)

##### lt(field, value)

`field < $field`.

```rust
pub fn lt(field: impl Into<String>, value: impl Into<Value>) -> Self
```

- `field` — column name
- `value` — cell value

**Returns** [WhereBuilder](#wherebuilder)

##### gte(field, value)

`field >= $field`.

```rust
pub fn gte(field: impl Into<String>, value: impl Into<Value>) -> Self
```

- `field` — column name
- `value` — cell value

**Returns** [WhereBuilder](#wherebuilder)

##### lte(field, value)

`field <= $field`.

```rust
pub fn lte(field: impl Into<String>, value: impl Into<Value>) -> Self
```

- `field` — column name
- `value` — cell value

**Returns** [WhereBuilder](#wherebuilder)

##### and(children)

Joins children with `AND`. Empty is `1 = 1`.

```rust
pub fn and(children: Vec<WhereBuilder>) -> Self
```

- `children` — [WhereBuilder](#wherebuilder) predicates

**Returns** [WhereBuilder](#wherebuilder)

##### or(children)

Joins children with `OR`. Empty is `1 = 0`.

```rust
pub fn or(children: Vec<WhereBuilder>) -> Self
```

- `children` — [WhereBuilder](#wherebuilder) predicates

**Returns** [WhereBuilder](#wherebuilder)

##### not(child)

`NOT (child)`.

```rust
pub fn not(child: WhereBuilder) -> Self
```

- `child` — [WhereBuilder](#wherebuilder)

**Returns** [WhereBuilder](#wherebuilder)

##### in_list(field, values)

`field IN ($field_0, …)`. Empty is `1 = 0`.

```rust
pub fn in_list(field: impl Into<String>, values: Vec<Value>) -> Self
```

- `field` — column name
- `values` — cell values

**Returns** [WhereBuilder](#wherebuilder)

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

[StatementBuilder](./statement-builder.md) inlines predicate text into query / update / delete SQL. Keep the [WhereBuilder](#wherebuilder) when the caller needs bound parameters.

## Examples

### Compare, groups, not, and in

::: example
```csharp csharp/Datasources.Tests/CoreTests.cs#L53-L54
```
```rust rust/datasources/src/tests/mod.rs#L655-L658
```
```typescript typescript/datasources/src/builders/builders.test.ts#L27-L30
```

```output [After: and]
id = $id AND age > $age
```

```output [After: in]
id IN ($id_0, $id_1)
```
:::
