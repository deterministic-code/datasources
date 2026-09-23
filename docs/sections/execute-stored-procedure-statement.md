# Execute Stored Procedure Statement

`CALL` text, or a compiled `CALL` from a name and [Parameter](./command.md#parameter) list.

## Classes

::: reference
### typescript
#### IExecuteStoredProcedureStatement

```typescript
interface IExecuteStoredProcedureStatement extends IStatement {}
```

#### ExecuteStoredProcedureStatement

```typescript
class ExecuteStoredProcedureStatement implements IExecuteStoredProcedureStatement
```

**Constructor**

##### ExecuteStoredProcedureStatement(text)

Wraps stored-procedure call text.

```typescript
constructor(text: string)
```

- `text` — command text

##### ExecuteStoredProcedureStatement(name, parameters)

Compiles a `CALL`. Empty [Parameter](./command.md#parameter) list is `CALL name()`. [returnValue](./command.md#parameterdirection) parameters are omitted from the argument list.

```typescript
constructor(name: string, parameters: Parameter[])
```

- `name` — procedure name
- `parameters` — bound values (empty for `CALL name()`)

**Methods**

##### getText()

Command text.

```typescript
getText(): string
```

**Returns** `string`

### csharp
#### IExecuteStoredProcedureStatement

```csharp
public interface IExecuteStoredProcedureStatement : IStatement
```

#### ExecuteStoredProcedureStatement

```csharp
public sealed record ExecuteStoredProcedureStatement : IExecuteStoredProcedureStatement
```

**Constructor**

##### ExecuteStoredProcedureStatement(text)

Wraps stored-procedure call text.

```csharp
public ExecuteStoredProcedureStatement(string text)
```

- `text` — command text

##### ExecuteStoredProcedureStatement(name, parameters)

Compiles a `CALL`. Empty [Parameter](./command.md#parameter) list is `CALL name()`. [ReturnValue](./command.md#parameterdirection) parameters are omitted from the argument list.

```csharp
public ExecuteStoredProcedureStatement(string name, IReadOnlyList<Parameter> parameters)
```

- `name` — procedure name
- `parameters` — bound values (empty for `CALL name()`)

**Methods**

##### GetText()

Command text.

```csharp
public string GetText()
```

**Returns** `string`

### rust
#### ExecuteStoredProcedureStatement

```rust
pub struct ExecuteStoredProcedureStatement
```

**Constructor**

##### text(text)

Wraps stored-procedure call text.

```rust
pub fn text(text: impl Into<String>) -> Self
```

- `text` — command text

**Returns** [ExecuteStoredProcedureStatement](#executestoredprocedurestatement)

##### new(name, parameters)

Compiles a `CALL`. Empty [Parameter](./command.md#parameter) list is `CALL name()`. [ReturnValue](./command.md#parameterdirection) parameters are omitted from the argument list.

```rust
pub fn new(name: impl AsRef<str>, parameters: &[Parameter]) -> Self
```

- `name` — procedure name
- `parameters` — bound values (empty for `CALL name()`)

**Returns** [ExecuteStoredProcedureStatement](#executestoredprocedurestatement)

**Methods**

##### get_text()

Command text.

```rust
fn get_text(&self) -> String
```

**Returns** `String`
:::

## Remarks

Compiled form is `CALL name($p)` or `CALL name()`. [ReturnValue](./command.md#parameterdirection) parameters are binds only and stay out of the argument list.

## Examples

### Compile a CALL

::: example
```csharp csharp/Datasources.Tests/CoreTests.cs#L214-L215
```
```rust rust/datasources/src/tests/mod.rs#L1089-L1090
```
```typescript typescript/datasources/src/builders/builders.test.ts#L305-L306
```

```output [After: call]
CALL save($id, $name)
```
:::
