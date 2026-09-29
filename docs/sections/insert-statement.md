# Insert Statement

`INSERT` text, or a compiled `INSERT` from a table and fields.

## Classes

::: reference
### typescript
#### IInsertStatement

```typescript
interface IInsertStatement extends IStatement {}
```

#### InsertStatement

```typescript
class InsertStatement implements IInsertStatement
```

**Constructor**

##### InsertStatement(text)

Wraps insert text.

```typescript
constructor(text: string)
```

- `text` — command text

##### InsertStatement(table, fields)

Compiles an `INSERT`.

```typescript
constructor(table: string, fields: Field[])
```

- `table` — table name
- `fields` — [Field](#field) columns and bound values

**Methods**

##### getText()

Command text.

```typescript
getText(): string
```

**Returns** `string`

#### Field

```typescript
interface Field
```

**Properties**

##### name

Column name.

```typescript
name: string
```

##### value

Bound [Scalar](./where-clause.md#scalar).

```typescript
value: Scalar
```

### csharp
#### IInsertStatement

```csharp
public interface IInsertStatement : IStatement
```

#### InsertStatement

```csharp
public sealed record InsertStatement : IInsertStatement
```

**Constructor**

##### InsertStatement(text)

Wraps insert text.

```csharp
public InsertStatement(string text)
```

- `text` — command text

##### InsertStatement(table, fields)

Compiles an `INSERT`.

```csharp
public InsertStatement(string table, IReadOnlyList<Parameter> fields)
```

- `table` — table name
- `fields` — [Parameter](./command.md#parameter) columns and bound values

**Methods**

##### GetText()

Command text.

```csharp
public string GetText()
```

**Returns** `string`

### rust
#### InsertStatement

```rust
pub struct InsertStatement
```

**Constructor**

##### text(text)

Wraps insert text.

```rust
pub fn text(text: impl Into<String>) -> Self
```

- `text` — command text

**Returns** [InsertStatement](#insertstatement)

##### new(table, fields)

Compiles an `INSERT`.

```rust
pub fn new(table: impl AsRef<str>, fields: &[Parameter]) -> Self
```

- `table` — table name
- `fields` — [Parameter](./command.md#parameter) columns and bound values

**Returns** [InsertStatement](#insertstatement)

**Methods**

##### get_text()

Command text.

```rust
fn get_text(&self) -> String
```

**Returns** `String`
:::

## Remarks

Compiled form is `INSERT INTO table (cols) VALUES ($cols)`. [Command](./command.md) uses {{GetText()|getText()|get_text()}} only.

## Examples

### Compile an INSERT

::: example
```csharp csharp/Datasources.Tests/CoreTests.cs#StatementsAndBuilders
```
```rust rust/datasources/src/tests/mod.rs#statements_and_builders
```
```typescript typescript/datasources/src/builders/builders.test.ts#creates-write-and-ddl-statements
```

```output [After: insert]
INSERT INTO users (id, name) VALUES ($id, $name)
```
:::
