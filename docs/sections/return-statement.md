# Return Statement

Function-call text, or a compiled `SELECT name($p)` from a name and [Parameter](./command.md#parameter) list.

## Classes

::: reference
### typescript
#### IReturnStatement

```typescript
interface IReturnStatement extends IStatement {}
```

#### ReturnStatement

```typescript
class ReturnStatement implements IReturnStatement
```

**Constructor**

##### ReturnStatement(text)

Wraps function-call text.

```typescript
constructor(text: string)
```

- `text` — command text

##### ReturnStatement(name, parameters)

Compiles a `SELECT`. Empty [Parameter](./command.md#parameter) list is `SELECT name()`. [returnValue](./command.md#parameterdirection) parameters are omitted from the argument list.

```typescript
constructor(name: string, parameters: Parameter[])
```

- `name` — function name
- `parameters` — bound values (empty for `SELECT name()`)

**Methods**

##### getText()

Command text.

```typescript
getText(): string
```

**Returns** `string`

### csharp
#### IReturnStatement

```csharp
public interface IReturnStatement : IStatement
```

#### ReturnStatement

```csharp
public sealed record ReturnStatement : IReturnStatement
```

**Constructor**

##### ReturnStatement(text)

Wraps function-call text.

```csharp
public ReturnStatement(string text)
```

- `text` — command text

##### ReturnStatement(name, parameters)

Compiles a `SELECT`. Empty [Parameter](./command.md#parameter) list is `SELECT name()`. [ReturnValue](./command.md#parameterdirection) parameters are omitted from the argument list.

```csharp
public ReturnStatement(string name, IReadOnlyList<Parameter> parameters)
```

- `name` — function name
- `parameters` — bound values (empty for `SELECT name()`)

**Methods**

##### GetText()

Command text.

```csharp
public string GetText()
```

**Returns** `string`

### rust
#### ReturnStatement

```rust
pub struct ReturnStatement
```

**Constructor**

##### text(text)

Wraps function-call text.

```rust
pub fn text(text: impl Into<String>) -> Self
```

- `text` — command text

**Returns** [ReturnStatement](#returnstatement)

##### new(name, parameters)

Compiles a `SELECT`. Empty [Parameter](./command.md#parameter) list is `SELECT name()`. [ReturnValue](./command.md#parameterdirection) parameters are omitted from the argument list.

```rust
pub fn new(name: impl AsRef<str>, parameters: &[Parameter]) -> Self
```

- `name` — function name
- `parameters` — bound values (empty for `SELECT name()`)

**Returns** [ReturnStatement](#returnstatement)

**Methods**

##### get_text()

Command text.

```rust
fn get_text(&self) -> String
```

**Returns** `String`
:::

## Remarks

Compiled form is `SELECT name($p)` or `SELECT name()`. [ReturnValue](./command.md#parameterdirection) parameters are binds only and stay out of the argument list.

## Examples

### Compile a SELECT function call

::: example
```csharp csharp/Datasources.Tests/CoreTests.cs#L217-L218
```
```rust rust/datasources/src/tests/mod.rs#L1095-L1096
```
```typescript typescript/datasources/src/builders/builders.test.ts#L310-L311
```

```output [After: return]
SELECT add($id, $name)
```
:::
