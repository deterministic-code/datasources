# Query Statement

`SELECT` text, or a compiled `SELECT` from a table, fields, optional [IWhere](./where-clause.md#iwhere), and joins.

## Classes

::: reference
### typescript
#### IQuery

```typescript
interface IQuery extends IStatement
```

**Properties**

##### where

Optional [IWhere](./where-clause.md#iwhere) used to build this query.

```typescript
readonly where?: IWhere
```

#### QueryStatement

```typescript
class QueryStatement implements IQuery
```

**Constructor**

##### QueryStatement(text, where?)

Wraps query text and an optional [IWhere](./where-clause.md#iwhere).

```typescript
constructor(text: string, where?: IWhere)
```

- `text` — command text
- `where` — optional [IWhere](./where-clause.md#iwhere)

##### QueryStatement(table, fields, where?, joins?)

Compiles a `SELECT` from a table, fields, optional [IWhere](./where-clause.md#iwhere), and joins.

```typescript
constructor(table: string, fields: string[], where?: IWhere, joins?: Join[])
```

- `table` — table name
- `fields` — selected columns (`*` when empty)
- `where` — optional [IWhere](./where-clause.md#iwhere)
- `joins` — optional [Join](#join) list

**Properties**

##### where

Optional [IWhere](./where-clause.md#iwhere) used to build this query.

```typescript
readonly where?: IWhere
```

**Methods**

##### getText()

Command text.

```typescript
getText(): string
```

**Returns** `string`

#### Join

```typescript
type Join = { kind: JoinKind; table: string; left: string; right: string }
```

**Methods**

##### inner(table, left, right)

Inner join.

```typescript
inner(table: string, left: string, right: string): Join
```

- `table` — joined table
- `left` — left column
- `right` — right column

**Returns** [Join](#join)

##### left(table, left, right)

Left join.

```typescript
left(table: string, left: string, right: string): Join
```

- `table` — joined table
- `left` — left column
- `right` — right column

**Returns** [Join](#join)

##### right(table, left, right)

Right join.

```typescript
right(table: string, left: string, right: string): Join
```

- `table` — joined table
- `left` — left column
- `right` — right column

**Returns** [Join](#join)

#### JoinKind

```typescript
type JoinKind = "inner" | "left" | "right"
```

### csharp
#### IQuery

```csharp
public interface IQuery : IStatement
```

**Properties**

##### Where

Optional [IWhere](./where-clause.md#iwhere) used to build this query.

```csharp
IWhere? Where { get; }
```

#### QueryStatement

```csharp
public sealed record QueryStatement : IQuery
```

**Constructor**

##### QueryStatement(text, where)

Wraps query text and an optional [IWhere](./where-clause.md#iwhere).

```csharp
public QueryStatement(string text, IWhere? where = null)
```

- `text` — command text
- `where` — optional [IWhere](./where-clause.md#iwhere)

##### QueryStatement(table, fields, where, joins)

Compiles a `SELECT` from a table, fields, optional [IWhere](./where-clause.md#iwhere), and joins.

```csharp
public QueryStatement(
    string table,
    IReadOnlyList<string> fields,
    IWhere? where = null,
    IReadOnlyList<Join>? joins = null)
```

- `table` — table name
- `fields` — selected columns (`*` when empty)
- `where` — optional [IWhere](./where-clause.md#iwhere)
- `joins` — optional [Join](#join) list

**Properties**

##### Where

Optional [IWhere](./where-clause.md#iwhere) used to build this query.

```csharp
IWhere? Where { get; }
```

**Methods**

##### GetText()

Command text.

```csharp
public string GetText()
```

**Returns** `string`

#### Join

```csharp
public sealed record Join(JoinKind Kind, string Table, string LeftColumn, string RightColumn)
```

**Methods**

##### Inner(table, left, right)

Inner join.

```csharp
public static Join Inner(string table, string left, string right)
```

- `table` — joined table
- `left` — left column
- `right` — right column

**Returns** [Join](#join)

##### Left(table, left, right)

Left join.

```csharp
public static Join Left(string table, string left, string right)
```

- `table` — joined table
- `left` — left column
- `right` — right column

**Returns** [Join](#join)

##### Right(table, left, right)

Right join.

```csharp
public static Join Right(string table, string left, string right)
```

- `table` — joined table
- `left` — left column
- `right` — right column

**Returns** [Join](#join)

#### JoinKind

```csharp
public enum JoinKind
```

### rust
#### Query

```rust
pub trait Query: Statement
```

**Methods**

##### where_clause()

Optional [Where](./where-clause.md#where) used to build this query.

```rust
fn where_clause(&self) -> Option<&dyn Where>
```

**Returns** `Option<&dyn `[Where](./where-clause.md#where)`>`

#### QueryStatement

```rust
pub struct QueryStatement
```

**Constructor**

##### text(text)

Wraps query text.

```rust
pub fn text(text: impl Into<String>) -> Self
```

- `text` — command text

**Returns** [QueryStatement](#querystatement)

##### with_where(text, where_clause)

Wraps query text and a [Where](./where-clause.md#where).

```rust
pub fn with_where(text: impl Into<String>, where_clause: impl Where + 'static) -> Self
```

- `text` — command text
- `where_clause` — [Where](./where-clause.md#where)

**Returns** [QueryStatement](#querystatement)

##### new(table, fields, where_clause, joins)

Compiles a `SELECT` from a table, fields, optional [Where](./where-clause.md#where), and joins.

```rust
pub fn new(
    table: impl AsRef<str>,
    fields: &[&str],
    where_clause: Option<&dyn Where>,
    joins: &[Join],
) -> Self
```

- `table` — table name
- `fields` — selected columns (`*` when empty)
- `where_clause` — optional [Where](./where-clause.md#where)
- `joins` — [Join](#join) list

**Returns** [QueryStatement](#querystatement)

**Methods**

##### get_text()

Command text.

```rust
fn get_text(&self) -> String
```

**Returns** `String`

##### where_clause()

Optional [Where](./where-clause.md#where) used to build this query.

```rust
fn where_clause(&self) -> Option<&dyn Where>
```

**Returns** `Option<&dyn `[Where](./where-clause.md#where)`>`

#### Join

```rust
pub struct Join
```

**Methods**

##### inner(table, left, right)

Inner join.

```rust
pub fn inner(table: impl Into<String>, left: impl Into<String>, right: impl Into<String>) -> Self
```

- `table` — joined table
- `left` — left column
- `right` — right column

**Returns** [Join](#join)

##### left(table, left, right)

Left join.

```rust
pub fn left(table: impl Into<String>, left: impl Into<String>, right: impl Into<String>) -> Self
```

- `table` — joined table
- `left` — left column
- `right` — right column

**Returns** [Join](#join)

##### right(table, left, right)

Right join.

```rust
pub fn right(table: impl Into<String>, left: impl Into<String>, right: impl Into<String>) -> Self
```

- `table` — joined table
- `left` — left column
- `right` — right column

**Returns** [Join](#join)

#### JoinKind

```rust
pub enum JoinKind
```
:::

## Remarks

[Command](./command.md) uses `GetText()` / `getText()` / `get_text()` only. Keep the [WhereClause](./where-clause.md) when the caller needs bound parameters. Empty `fields` compiles `SELECT *`.

## Examples

### Wrap query text and an optional where

::: example
```csharp csharp/Datasources.Tests/CoreTests.cs#L33-L38
```
```rust rust/datasources/src/tests/mod.rs#L989-L992
```
```typescript typescript/datasources/src/builders/builders.test.ts#L24-L26
```
:::

### Compile a SELECT

::: example
```csharp csharp/Datasources.Tests/CoreTests.cs#L127-L130
```
```rust rust/datasources/src/tests/mod.rs#L877-L883
```
```typescript typescript/datasources/src/builders/builders.test.ts#L175-L180
```

```output [After: select]
SELECT id, name FROM users WHERE id = $id AND age > $age
```

```output:typescript [After: select]
SELECT id, name FROM users WHERE id = $id
```
:::
