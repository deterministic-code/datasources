# Query Statement

`SELECT` text, or a compiled `SELECT` from [QueryStatementOptions](#querystatementoptions): table, fields, optional [IWhere](./where-clause.md#iwhere), joins, grouping, order, and limit.

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

##### QueryStatement(options)

Compiles a `SELECT` from [QueryStatementOptions](#querystatementoptions).

```typescript
constructor(options: QueryStatementOptions)
```

- `options` — table, fields, and optional where, joins, group, order, and limit

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

#### QueryStatementOptions

```typescript
type QueryStatementOptions = {
  table: string
  fields: string[]
  where?: IWhere
  joins?: Join[]
  groupBy?: string[]
  orderBy?: OrderBy[]
  limit?: number
}
```

**Properties**

##### table

Table name.

```typescript
table: string
```

##### fields

Selected columns. Empty compiles `SELECT *`. Aggregate fragments from [Aggregate](#aggregate) belong here.

```typescript
fields: string[]
```

##### where

Optional [IWhere](./where-clause.md#iwhere).

```typescript
where?: IWhere
```

##### joins

Optional [Join](#join) list.

```typescript
joins?: Join[]
```

##### groupBy

Optional `GROUP BY` columns. Empty or omitted leaves the clause off.

```typescript
groupBy?: string[]
```

##### orderBy

Optional [OrderBy](#orderby) list. Empty or omitted leaves the clause off.

```typescript
orderBy?: OrderBy[]
```

##### limit

Optional `LIMIT`. `0` is kept. A negative value throws.

```typescript
limit?: number
```

#### Aggregate

```typescript
const Aggregate: {
  count: (column: string, alias?: string) => string
  sum: (column: string, alias?: string) => string
  min: (column: string, alias?: string) => string
  max: (column: string, alias?: string) => string
}
```

**Methods**

##### count(column, alias?)

`COUNT(column)`, or `COUNT(column) AS alias`. Pass `"*"` for `COUNT(*)`.

```typescript
count(column: string, alias?: string): string
```

- `column` — column or `*`
- `alias` — optional result name

**Returns** `string`

##### sum(column, alias?)

`SUM(column)`, or `SUM(column) AS alias`.

```typescript
sum(column: string, alias?: string): string
```

- `column` — column
- `alias` — optional result name

**Returns** `string`

##### min(column, alias?)

`MIN(column)`, or `MIN(column) AS alias`.

```typescript
min(column: string, alias?: string): string
```

- `column` — column
- `alias` — optional result name

**Returns** `string`

##### max(column, alias?)

`MAX(column)`, or `MAX(column) AS alias`.

```typescript
max(column: string, alias?: string): string
```

- `column` — column
- `alias` — optional result name

**Returns** `string`

#### OrderBy

```typescript
type OrderBy = { column: string; direction: SortDirection }
```

**Methods**

##### asc(column)

Sort ascending.

```typescript
asc(column: string): OrderBy
```

- `column` — sort column

**Returns** [OrderBy](#orderby)

##### desc(column)

Sort descending.

```typescript
desc(column: string): OrderBy
```

- `column` — sort column

**Returns** [OrderBy](#orderby)

#### SortDirection

```typescript
type SortDirection = "asc" | "desc"
```

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

##### QueryStatement(options)

Compiles a `SELECT` from [QueryStatementOptions](#querystatementoptions).

```csharp
public QueryStatement(QueryStatementOptions options)
```

- `options` — table, fields, and optional where, joins, group, order, and limit

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

#### QueryStatementOptions

```csharp
public sealed class QueryStatementOptions
```

**Properties**

##### Table

Table name.

```csharp
public required string Table { get; init; }
```

##### Fields

Selected columns. Empty compiles `SELECT *`. Aggregate fragments from [Aggregate](#aggregate) belong here.

```csharp
public required IReadOnlyList<string> Fields { get; init; }
```

##### Where

Optional [IWhere](./where-clause.md#iwhere).

```csharp
public IWhere? Where { get; init; }
```

##### Joins

Optional [Join](#join) list.

```csharp
public IReadOnlyList<Join>? Joins { get; init; }
```

##### GroupBy

Optional `GROUP BY` columns. Empty or omitted leaves the clause off.

```csharp
public IReadOnlyList<string>? GroupBy { get; init; }
```

##### OrderBy

Optional [OrderBy](#orderby) list. Empty or omitted leaves the clause off.

```csharp
public IReadOnlyList<OrderBy>? OrderBy { get; init; }
```

##### Limit

Optional `LIMIT`. `0` is kept. A negative value throws.

```csharp
public int? Limit { get; init; }
```

#### Aggregate

```csharp
public static class Aggregate
```

**Methods**

##### Count(column, alias)

`COUNT(column)`, or `COUNT(column) AS alias`. Pass `"*"` for `COUNT(*)`.

```csharp
public static string Count(string column, string? alias = null)
```

- `column` — column or `*`
- `alias` — optional result name

**Returns** `string`

##### Sum(column, alias)

`SUM(column)`, or `SUM(column) AS alias`.

```csharp
public static string Sum(string column, string? alias = null)
```

- `column` — column
- `alias` — optional result name

**Returns** `string`

##### Min(column, alias)

`MIN(column)`, or `MIN(column) AS alias`.

```csharp
public static string Min(string column, string? alias = null)
```

- `column` — column
- `alias` — optional result name

**Returns** `string`

##### Max(column, alias)

`MAX(column)`, or `MAX(column) AS alias`.

```csharp
public static string Max(string column, string? alias = null)
```

- `column` — column
- `alias` — optional result name

**Returns** `string`

#### OrderBy

```csharp
public sealed record OrderBy(string Column, SortDirection Direction)
```

**Methods**

##### Asc(column)

Sort ascending.

```csharp
public static OrderBy Asc(string column)
```

- `column` — sort column

**Returns** [OrderBy](#orderby)

##### Desc(column)

Sort descending.

```csharp
public static OrderBy Desc(string column)
```

- `column` — sort column

**Returns** [OrderBy](#orderby)

#### SortDirection

```csharp
public enum SortDirection
```

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

##### new(options)

Compiles a `SELECT` from [QueryStatementOptions](#querystatementoptions).

```rust
pub fn new(options: QueryStatementOptions<'_>) -> Self
```

- `options` — table, fields, and optional where, joins, group, order, and limit

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

#### QueryStatementOptions

```rust
pub struct QueryStatementOptions<'a>
```

**Constructor**

##### new(table, fields)

Table and fields, with where, joins, group, order, and limit unset.

```rust
pub fn new(table: &'a str, fields: &'a [&'a str]) -> Self
```

- `table` — table name
- `fields` — selected columns (`*` when empty)

**Returns** [QueryStatementOptions](#querystatementoptions)

**Properties**

##### table

Table name.

```rust
pub table: &'a str
```

##### fields

Selected columns. Empty compiles `SELECT *`. Aggregate fragments from [Aggregate](#aggregate) belong here.

```rust
pub fields: &'a [&'a str]
```

##### where_clause

Optional [Where](./where-clause.md#where).

```rust
pub where_clause: Option<&'a dyn Where>
```

##### joins

[Join](#join) list. Empty leaves the clause off.

```rust
pub joins: &'a [Join]
```

##### group_by

`GROUP BY` columns. Empty leaves the clause off.

```rust
pub group_by: &'a [&'a str]
```

##### order_by

[OrderBy](#orderby) list. Empty leaves the clause off.

```rust
pub order_by: &'a [OrderBy]
```

##### limit

Optional `LIMIT`. `Some(0)` is kept. A negative value panics.

```rust
pub limit: Option<i64>
```

#### Aggregate

```rust
pub struct Aggregate
```

**Methods**

##### count(column, alias)

`COUNT(column)`, or `COUNT(column) AS alias`. Pass `"*"` for `COUNT(*)`.

```rust
pub fn count(column: &str, alias: Option<&str>) -> String
```

- `column` — column or `*`
- `alias` — optional result name

**Returns** `String`

##### sum(column, alias)

`SUM(column)`, or `SUM(column) AS alias`.

```rust
pub fn sum(column: &str, alias: Option<&str>) -> String
```

- `column` — column
- `alias` — optional result name

**Returns** `String`

##### min(column, alias)

`MIN(column)`, or `MIN(column) AS alias`.

```rust
pub fn min(column: &str, alias: Option<&str>) -> String
```

- `column` — column
- `alias` — optional result name

**Returns** `String`

##### max(column, alias)

`MAX(column)`, or `MAX(column) AS alias`.

```rust
pub fn max(column: &str, alias: Option<&str>) -> String
```

- `column` — column
- `alias` — optional result name

**Returns** `String`

#### OrderBy

```rust
pub struct OrderBy
```

**Methods**

##### asc(column)

Sort ascending.

```rust
pub fn asc(column: impl Into<String>) -> Self
```

- `column` — sort column

**Returns** [OrderBy](#orderby)

##### desc(column)

Sort descending.

```rust
pub fn desc(column: impl Into<String>) -> Self
```

- `column` — sort column

**Returns** [OrderBy](#orderby)

#### SortDirection

```rust
pub enum SortDirection
```

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

[Command](./command.md) uses `GetText()` / `getText()` / `get_text()` only. Keep the [WhereClause](./where-clause.md) when the caller needs bound parameters. Empty `fields` compiles `SELECT *`. Clauses append as `GROUP BY`, `ORDER BY`, then `LIMIT`. An empty group or order list is omitted. `Aggregate` returns a select-list fragment such as `COUNT(*) AS n`.

## Examples

### Wrap query text and an optional where

::: example
```csharp csharp/Datasources.Tests/CoreTests.cs#L33-L38
```
```rust rust/datasources/src/tests/mod.rs#L1177-L1180
```
```typescript typescript/datasources/src/builders/builders.test.ts#L41-L43
```
:::

### Compile a SELECT

::: example
```csharp csharp/Datasources.Tests/CoreTests.cs#L128-L130
```
```rust rust/datasources/src/tests/mod.rs#L926-L933
```
```typescript typescript/datasources/src/builders/builders.test.ts#L193-L196
```

```output [After: select]
SELECT id, name FROM users WHERE id = $id AND age > $age
```

```output:typescript [After: select]
SELECT id, name FROM users WHERE id = $id
```
:::

### Group, aggregate, order, and limit

::: example
```csharp csharp/Datasources.Tests/CoreTests.cs#L157-L166
```
```rust rust/datasources/src/tests/mod.rs#L977-L991
```
```typescript typescript/datasources/src/builders/builders.test.ts#L231-L239
```

```output [After: grouped select]
SELECT status, COUNT(*) AS n, SUM(amount) AS total FROM orders INNER JOIN customers ON orders.customer_id = customers.id WHERE status = $status GROUP BY status ORDER BY total DESC, status ASC LIMIT 10
```
:::

### Group rows

::: example
```csharp csharp/Datasources.Tests/CoreTests.cs#L168-L168
```
```rust rust/datasources/src/tests/mod.rs#L996-L999
```
```typescript typescript/datasources/src/builders/builders.test.ts#L243-L243
```

```output [After: group by]
SELECT id FROM users GROUP BY id
```
:::

### Aggregate without grouping

::: example
```csharp csharp/Datasources.Tests/CoreTests.cs#L170-L170
```
```rust rust/datasources/src/tests/mod.rs#L1001-L1004
```
```typescript typescript/datasources/src/builders/builders.test.ts#L245-L245
```

```output [After: min and max]
SELECT MIN(n), MAX(n) FROM users
```
:::

### Order rows

::: example
```csharp csharp/Datasources.Tests/CoreTests.cs#L172-L172
```
```rust rust/datasources/src/tests/mod.rs#L1006-L1009
```
```typescript typescript/datasources/src/builders/builders.test.ts#L247-L251
```

```output [After: order by]
SELECT id FROM users ORDER BY id ASC, name DESC
```
:::

### Limit rows

::: example
```csharp csharp/Datasources.Tests/CoreTests.cs#L174-L174
```
```rust rust/datasources/src/tests/mod.rs#L1014-L1017
```
```typescript typescript/datasources/src/builders/builders.test.ts#L253-L253
```

```output [After: limit]
SELECT id FROM users LIMIT 10
```
:::
