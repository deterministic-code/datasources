# Delete Statement

Compiled `DELETE` from a table and optional [IWhere](./where-clause.md#iwhere), or raw delete text.

## Classes

::: reference
### typescript
#### IDeleteStatement

```typescript
interface IDeleteStatement extends IStatement
```

**Properties**

##### where

Optional [IWhere](./where-clause.md#iwhere) used to build this delete.

```typescript
readonly where?: IWhere
```

#### DeleteStatement

```typescript
class DeleteStatement implements IDeleteStatement
```

**Constructor**

##### DeleteStatement(table, where?)

Compiles a `DELETE`.

```typescript
constructor(table: string, where?: IWhere)
```

- `table` — table name
- `where` — optional [IWhere](./where-clause.md#iwhere)

**Properties**

##### where

Optional [IWhere](./where-clause.md#iwhere) used to build this delete.

```typescript
readonly where?: IWhere
```

**Methods**

##### raw(text, where?)

Wraps delete text and an optional [IWhere](./where-clause.md#iwhere).

```typescript
static raw(text: string, where?: IWhere): DeleteStatement
```

- `text` — command text
- `where` — optional [IWhere](./where-clause.md#iwhere)

**Returns** [DeleteStatement](#deletestatement)

##### getText()

Command text.

```typescript
getText(): string
```

**Returns** `string`

### csharp
#### IDeleteStatement

```csharp
public interface IDeleteStatement : IStatement
```

**Properties**

##### Where

Optional [IWhere](./where-clause.md#iwhere) used to build this delete.

```csharp
IWhere? Where { get; }
```

#### DeleteStatement

```csharp
public sealed record DeleteStatement : IDeleteStatement
```

**Constructor**

##### DeleteStatement(table, where)

Compiles a `DELETE`.

```csharp
public DeleteStatement(string table, IWhere? where = null)
```

- `table` — table name
- `where` — optional [IWhere](./where-clause.md#iwhere)

**Properties**

##### Where

Optional [IWhere](./where-clause.md#iwhere) used to build this delete.

```csharp
IWhere? Where { get; }
```

**Methods**

##### Raw(text, where)

Wraps delete text and an optional [IWhere](./where-clause.md#iwhere).

```csharp
public static DeleteStatement Raw(string text, IWhere? where = null)
```

- `text` — command text
- `where` — optional [IWhere](./where-clause.md#iwhere)

**Returns** [DeleteStatement](#deletestatement)

##### GetText()

Command text.

```csharp
public string GetText()
```

**Returns** `string`

### rust
#### DeleteStatement

```rust
pub struct DeleteStatement
```

**Constructor**

##### new(table, where_clause)

Compiles a `DELETE`.

```rust
pub fn new(table: impl AsRef<str>, where_clause: Option<&dyn Where>) -> Self
```

- `table` — table name
- `where_clause` — optional [Where](./where-clause.md#where)

**Returns** [DeleteStatement](#deletestatement)

##### raw(text, where_clause)

Wraps delete text and an optional [Where](./where-clause.md#where).

```rust
pub fn raw(text: impl Into<String>, where_clause: Option<Box<dyn Where>>) -> Self
```

- `text` — command text
- `where_clause` — optional [Where](./where-clause.md#where)

**Returns** [DeleteStatement](#deletestatement)

**Methods**

##### get_text()

Command text.

```rust
fn get_text(&self) -> String
```

**Returns** `String`

##### where_clause()

Optional [Where](./where-clause.md#where) used to build this delete.

```rust
fn where_clause(&self) -> Option<&dyn Where>
```

**Returns** `Option<&dyn `[Where](./where-clause.md#where)`>`
:::

## Remarks

Compiled form is `DELETE FROM table` plus `WHERE` when an [IWhere](./where-clause.md#iwhere) is present. `Raw` / `raw` wraps command text without compiling. Keep the [WhereClause](./where-clause.md) when the caller needs bound parameters.

## Examples

### Compile a DELETE

::: example
```csharp csharp/Datasources.Tests/CoreTests.cs#L158-L160
```
```rust rust/datasources/src/tests/mod.rs#L937-L942
```
```typescript typescript/datasources/src/builders/builders.test.ts#L224-L226
```

```output [After: delete]
DELETE FROM users WHERE id = $id AND age > $age
```

```output:typescript [After: delete]
DELETE FROM users WHERE id = $id
```
:::
