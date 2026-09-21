# Create Table Statement

`CREATE TABLE` text, or a compiled `CREATE TABLE` from a table and columns.

## Classes

::: reference
### typescript
#### ICreateTableStatement

```typescript
interface ICreateTableStatement extends IStatement {}
```

#### CreateTableStatement

```typescript
class CreateTableStatement implements ICreateTableStatement
```

**Constructor**

##### CreateTableStatement(text)

Wraps create-table text.

```typescript
constructor(text: string)
```

- `text` — command text

##### CreateTableStatement(table, columns)

Compiles a `CREATE TABLE`.

```typescript
constructor(table: string, columns: Column[])
```

- `table` — table name
- `columns` — [Column](#column) names and types

**Methods**

##### getText()

Command text.

```typescript
getText(): string
```

**Returns** `string`

#### Column

```typescript
interface Column
```

**Properties**

##### name

Column name.

```typescript
name: string
```

##### type

SQL type name.

```typescript
type: string
```

### csharp
#### ICreateTableStatement

```csharp
public interface ICreateTableStatement : IStatement
```

#### CreateTableStatement

```csharp
public sealed record CreateTableStatement : ICreateTableStatement
```

**Constructor**

##### CreateTableStatement(text)

Wraps create-table text.

```csharp
public CreateTableStatement(string text)
```

- `text` — command text

##### CreateTableStatement(table, columns)

Compiles a `CREATE TABLE`.

```csharp
public CreateTableStatement(string table, IReadOnlyList<Column> columns)
```

- `table` — table name
- `columns` — [Column](#column) names and types

**Methods**

##### GetText()

Command text.

```csharp
public string GetText()
```

**Returns** `string`

#### Column

```csharp
public sealed record Column(string Name, string Type)
```

**Properties**

##### Name

Column name.

```csharp
string Name { get; }
```

##### Type

SQL type name.

```csharp
string Type { get; }
```

### rust
#### CreateTableStatement

```rust
pub struct CreateTableStatement
```

**Constructor**

##### text(text)

Wraps create-table text.

```rust
pub fn text(text: impl Into<String>) -> Self
```

- `text` — command text

**Returns** [CreateTableStatement](#createtablestatement)

##### new(table, columns)

Compiles a `CREATE TABLE`.

```rust
pub fn new(table: impl AsRef<str>, columns: &[Column]) -> Self
```

- `table` — table name
- `columns` — [Column](#column) names and types

**Returns** [CreateTableStatement](#createtablestatement)

**Methods**

##### get_text()

Command text.

```rust
fn get_text(&self) -> String
```

**Returns** `String`

#### Column

```rust
pub struct Column
```

**Constructor**

##### new(name, type_name)

Column name and SQL type.

```rust
pub fn new(name: impl Into<String>, type_name: impl Into<String>) -> Self
```

- `name` — column name
- `type_name` — SQL type name

**Returns** [Column](#column)
:::

## Remarks

Compiled form is `CREATE TABLE table (name TYPE, …)`. [Command](./command.md) uses `GetText()` / `getText()` / `get_text()` only.

## Examples

### Compile a CREATE TABLE

::: example
```csharp csharp/Datasources.Tests/CoreTests.cs#L162-L163
```
```rust rust/datasources/src/tests/mod.rs#L947-L954
```
```typescript typescript/datasources/src/builders/builders.test.ts#L230-L234
```

```output [After: create table]
CREATE TABLE users (id INTEGER, name TEXT)
```
:::
