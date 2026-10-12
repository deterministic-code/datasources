<!-- Generated from docs/. Edit the markdown in docs/ and run npm run docs:skill. -->

# Select Statement

`SELECT` text, or a compiled `SELECT` from [SelectStatementOptions](#selectstatementoptions): table, fields, optional [IWhere](where.md#iwhere), joins, grouping, order, and limit.

## Classes

#### TypeScript

#### ISelectStatement {#typescript-iselectstatement}

```typescript
interface ISelectStatement extends IStatement
```

**Properties**

| Name | Description |
| --- | --- |
| where | Optional [IWhere](where.md#typescript-iwhere) used to build this query. |

#### SelectStatement {#typescript-selectstatement}

```typescript
class SelectStatement implements ISelectStatement
```

**Constructor**

| Name | Description |
| --- | --- |
| [SelectStatement(text, where?)](#typescript-selectstatement-selectstatement-text-where) | Wraps query text and an optional [IWhere](where.md#typescript-iwhere). |
| [SelectStatement(options)](#typescript-selectstatement-selectstatement-options) | Compiles a `SELECT` from [SelectStatementOptions](#typescript-selectstatementoptions). |

**Properties**

| Name | Description |
| --- | --- |
| where | Optional [IWhere](where.md#typescript-iwhere) used to build this query. |

**Methods**

| Name | Description |
| --- | --- |
| [accept(visitor)](#typescript-selectstatement-accept-visitor) | Dispatches to a statement visitor that renders command text. |

##### SelectStatement(text, where?) {#typescript-selectstatement-selectstatement-text-where}

Wraps query text and an optional [IWhere](where.md#typescript-iwhere).

**Definition**

```typescript
constructor(text: string, where?: IWhere)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |
| `where` | optional [IWhere](where.md#typescript-iwhere) |

##### SelectStatement(options) {#typescript-selectstatement-selectstatement-options}

Compiles a `SELECT` from [SelectStatementOptions](#typescript-selectstatementoptions).

**Definition**

```typescript
constructor(options: SelectStatementOptions)
```

**Parameters**

| Name | Description |
| --- | --- |
| `options` | table, fields, and optional where, joins, group, order, and limit |

##### accept(visitor) {#typescript-selectstatement-accept-visitor}

Dispatches to a statement visitor that renders command text.

**Definition**

```typescript
accept(visitor: IStatementVisitor): string
```

**Parameters**

| Name | Description |
| --- | --- |
| `visitor` | [IStatementVisitor](statements.md#typescript-istatementvisitor) |

**Returns**

`string`

#### SelectStatementOptions {#typescript-selectstatementoptions}

```typescript
type SelectStatementOptions = {
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

| Name | Description |
| --- | --- |
| table | Table name. |
| fields | Selected columns. Empty compiles `SELECT *`. Aggregate fragments from [Aggregate](#typescript-aggregate) belong here. |
| where | Optional [IWhere](where.md#typescript-iwhere). |
| joins | Optional [Join](#typescript-join) list. |
| groupBy | Optional `GROUP BY` columns. Empty or omitted leaves the clause off. |
| orderBy | Optional [OrderBy](#typescript-orderby) list. Empty or omitted leaves the clause off. |
| limit | Optional `LIMIT`. `0` is kept. A negative value throws. |

#### Aggregate {#typescript-aggregate}

```typescript
const Aggregate: {
  count: (column: string, alias?: string) => string
  sum: (column: string, alias?: string) => string
  min: (column: string, alias?: string) => string
  max: (column: string, alias?: string) => string
}
```

**Methods**

| Name | Description |
| --- | --- |
| [count(column, alias?)](#typescript-aggregate-count-column-alias) | `COUNT(column)`, or `COUNT(column) AS alias`. Pass `"*"` for `COUNT(*)`. |
| [sum(column, alias?)](#typescript-aggregate-sum-column-alias) | `SUM(column)`, or `SUM(column) AS alias`. |
| [min(column, alias?)](#typescript-aggregate-min-column-alias) | `MIN(column)`, or `MIN(column) AS alias`. |
| [max(column, alias?)](#typescript-aggregate-max-column-alias) | `MAX(column)`, or `MAX(column) AS alias`. |

##### count(column, alias?) {#typescript-aggregate-count-column-alias}

`COUNT(column)`, or `COUNT(column) AS alias`. Pass `"*"` for `COUNT(*)`.

**Definition**

```typescript
count(column: string, alias?: string): string
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | column or `*` |
| `alias` | optional result name |

**Returns**

`string`

##### sum(column, alias?) {#typescript-aggregate-sum-column-alias}

`SUM(column)`, or `SUM(column) AS alias`.

**Definition**

```typescript
sum(column: string, alias?: string): string
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | column |
| `alias` | optional result name |

**Returns**

`string`

##### min(column, alias?) {#typescript-aggregate-min-column-alias}

`MIN(column)`, or `MIN(column) AS alias`.

**Definition**

```typescript
min(column: string, alias?: string): string
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | column |
| `alias` | optional result name |

**Returns**

`string`

##### max(column, alias?) {#typescript-aggregate-max-column-alias}

`MAX(column)`, or `MAX(column) AS alias`.

**Definition**

```typescript
max(column: string, alias?: string): string
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | column |
| `alias` | optional result name |

**Returns**

`string`

#### OrderBy {#typescript-orderby}

```typescript
type OrderBy = { column: string; direction: SortDirection }
```

**Methods**

| Name | Description |
| --- | --- |
| [asc(column)](#typescript-orderby-asc-column) | Sort ascending. |
| [desc(column)](#typescript-orderby-desc-column) | Sort descending. |

##### asc(column) {#typescript-orderby-asc-column}

Sort ascending.

**Definition**

```typescript
asc(column: string): OrderBy
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | sort column |

**Returns**

[OrderBy](#typescript-orderby)

##### desc(column) {#typescript-orderby-desc-column}

Sort descending.

**Definition**

```typescript
desc(column: string): OrderBy
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | sort column |

**Returns**

[OrderBy](#typescript-orderby)

#### SortDirection {#typescript-sortdirection}

```typescript
type SortDirection = "asc" | "desc"
```

#### Join {#typescript-join}

```typescript
type Join = { kind: JoinKind; table: string; left: string; right: string }
```

**Methods**

| Name | Description |
| --- | --- |
| [inner(table, left, right)](#typescript-join-inner-table-left-right) | Inner join. |
| [left(table, left, right)](#typescript-join-left-table-left-right) | Left join. |
| [right(table, left, right)](#typescript-join-right-table-left-right) | Right join. |

##### inner(table, left, right) {#typescript-join-inner-table-left-right}

Inner join.

**Definition**

```typescript
inner(table: string, left: string, right: string): Join
```

**Parameters**

| Name | Description |
| --- | --- |
| `table` | joined table |
| `left` | left column |
| `right` | right column |

**Returns**

[Join](#typescript-join)

##### left(table, left, right) {#typescript-join-left-table-left-right}

Left join.

**Definition**

```typescript
left(table: string, left: string, right: string): Join
```

**Parameters**

| Name | Description |
| --- | --- |
| `table` | joined table |
| `left` | left column |
| `right` | right column |

**Returns**

[Join](#typescript-join)

##### right(table, left, right) {#typescript-join-right-table-left-right}

Right join.

**Definition**

```typescript
right(table: string, left: string, right: string): Join
```

**Parameters**

| Name | Description |
| --- | --- |
| `table` | joined table |
| `left` | left column |
| `right` | right column |

**Returns**

[Join](#typescript-join)

#### JoinKind {#typescript-joinkind}

```typescript
type JoinKind = "inner" | "left" | "right"
```

#### C#

#### ISelectStatement {#csharp-iselectstatement}

```csharp
public interface ISelectStatement : IStatement
```

**Properties**

| Name | Description |
| --- | --- |
| Where | Optional [IWhere](where.md#csharp-iwhere) used to build this query. |

#### SelectStatement {#csharp-selectstatement}

```csharp
public sealed record SelectStatement : ISelectStatement
```

**Constructor**

| Name | Description |
| --- | --- |
| [SelectStatement(text, where)](#csharp-selectstatement-selectstatement-text-where) | Wraps query text and an optional [IWhere](where.md#csharp-iwhere). |
| [SelectStatement(options)](#csharp-selectstatement-selectstatement-options) | Compiles a `SELECT` from [SelectStatementOptions](#csharp-selectstatementoptions). |

**Properties**

| Name | Description |
| --- | --- |
| Where | Optional [IWhere](where.md#csharp-iwhere) used to build this query. |

**Methods**

| Name | Description |
| --- | --- |
| [Accept(visitor)](#csharp-selectstatement-accept-visitor) | Dispatches to a statement visitor that renders command text. |

##### SelectStatement(text, where) {#csharp-selectstatement-selectstatement-text-where}

Wraps query text and an optional [IWhere](where.md#csharp-iwhere).

**Definition**

```csharp
public SelectStatement(string text, IWhere? where = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |
| `where` | optional [IWhere](where.md#csharp-iwhere) |

##### SelectStatement(options) {#csharp-selectstatement-selectstatement-options}

Compiles a `SELECT` from [SelectStatementOptions](#csharp-selectstatementoptions).

**Definition**

```csharp
public SelectStatement(SelectStatementOptions options)
```

**Parameters**

| Name | Description |
| --- | --- |
| `options` | table, fields, and optional where, joins, group, order, and limit |

##### Accept(visitor) {#csharp-selectstatement-accept-visitor}

Dispatches to a statement visitor that renders command text.

**Definition**

```csharp
public string Accept(IStatementVisitor visitor)
```

**Parameters**

| Name | Description |
| --- | --- |
| `visitor` | [IStatementVisitor](statements.md#csharp-istatementvisitor) |

**Returns**

`string`

#### SelectStatementOptions {#csharp-selectstatementoptions}

```csharp
public sealed class SelectStatementOptions
```

**Properties**

| Name | Description |
| --- | --- |
| Table | Table name. |
| Fields | Selected columns. Empty compiles `SELECT *`. Aggregate fragments from [Aggregate](#csharp-aggregate) belong here. |
| Where | Optional [IWhere](where.md#csharp-iwhere). |
| Joins | Optional [Join](#csharp-join) list. |
| GroupBy | Optional `GROUP BY` columns. Empty or omitted leaves the clause off. |
| OrderBy | Optional [OrderBy](#csharp-orderby) list. Empty or omitted leaves the clause off. |
| Limit | Optional `LIMIT`. `0` is kept. A negative value throws. |

#### Aggregate {#csharp-aggregate}

```csharp
public static class Aggregate
```

**Methods**

| Name | Description |
| --- | --- |
| [Count(column, alias)](#csharp-aggregate-count-column-alias) | `COUNT(column)`, or `COUNT(column) AS alias`. Pass `"*"` for `COUNT(*)`. |
| [Sum(column, alias)](#csharp-aggregate-sum-column-alias) | `SUM(column)`, or `SUM(column) AS alias`. |
| [Min(column, alias)](#csharp-aggregate-min-column-alias) | `MIN(column)`, or `MIN(column) AS alias`. |
| [Max(column, alias)](#csharp-aggregate-max-column-alias) | `MAX(column)`, or `MAX(column) AS alias`. |

##### Count(column, alias) {#csharp-aggregate-count-column-alias}

`COUNT(column)`, or `COUNT(column) AS alias`. Pass `"*"` for `COUNT(*)`.

**Definition**

```csharp
public static string Count(string column, string? alias = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | column or `*` |
| `alias` | optional result name |

**Returns**

`string`

##### Sum(column, alias) {#csharp-aggregate-sum-column-alias}

`SUM(column)`, or `SUM(column) AS alias`.

**Definition**

```csharp
public static string Sum(string column, string? alias = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | column |
| `alias` | optional result name |

**Returns**

`string`

##### Min(column, alias) {#csharp-aggregate-min-column-alias}

`MIN(column)`, or `MIN(column) AS alias`.

**Definition**

```csharp
public static string Min(string column, string? alias = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | column |
| `alias` | optional result name |

**Returns**

`string`

##### Max(column, alias) {#csharp-aggregate-max-column-alias}

`MAX(column)`, or `MAX(column) AS alias`.

**Definition**

```csharp
public static string Max(string column, string? alias = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | column |
| `alias` | optional result name |

**Returns**

`string`

#### OrderBy {#csharp-orderby}

```csharp
public sealed record OrderBy(string Column, SortDirection Direction)
```

**Methods**

| Name | Description |
| --- | --- |
| [Asc(column)](#csharp-orderby-asc-column) | Sort ascending. |
| [Desc(column)](#csharp-orderby-desc-column) | Sort descending. |

##### Asc(column) {#csharp-orderby-asc-column}

Sort ascending.

**Definition**

```csharp
public static OrderBy Asc(string column)
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | sort column |

**Returns**

[OrderBy](#csharp-orderby)

##### Desc(column) {#csharp-orderby-desc-column}

Sort descending.

**Definition**

```csharp
public static OrderBy Desc(string column)
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | sort column |

**Returns**

[OrderBy](#csharp-orderby)

#### SortDirection {#csharp-sortdirection}

```csharp
public enum SortDirection
```

#### Join {#csharp-join}

```csharp
public sealed record Join(JoinKind Kind, string Table, string LeftColumn, string RightColumn)
```

**Methods**

| Name | Description |
| --- | --- |
| [Inner(table, left, right)](#csharp-join-inner-table-left-right) | Inner join. |
| [Left(table, left, right)](#csharp-join-left-table-left-right) | Left join. |
| [Right(table, left, right)](#csharp-join-right-table-left-right) | Right join. |

##### Inner(table, left, right) {#csharp-join-inner-table-left-right}

Inner join.

**Definition**

```csharp
public static Join Inner(string table, string left, string right)
```

**Parameters**

| Name | Description |
| --- | --- |
| `table` | joined table |
| `left` | left column |
| `right` | right column |

**Returns**

[Join](#csharp-join)

##### Left(table, left, right) {#csharp-join-left-table-left-right}

Left join.

**Definition**

```csharp
public static Join Left(string table, string left, string right)
```

**Parameters**

| Name | Description |
| --- | --- |
| `table` | joined table |
| `left` | left column |
| `right` | right column |

**Returns**

[Join](#csharp-join)

##### Right(table, left, right) {#csharp-join-right-table-left-right}

Right join.

**Definition**

```csharp
public static Join Right(string table, string left, string right)
```

**Parameters**

| Name | Description |
| --- | --- |
| `table` | joined table |
| `left` | left column |
| `right` | right column |

**Returns**

[Join](#csharp-join)

#### JoinKind {#csharp-joinkind}

```csharp
public enum JoinKind
```

#### Rust

#### ISelectStatement {#rust-iselectstatement}

```rust
pub trait ISelectStatement: Statement
```

**Methods**

| Name | Description |
| --- | --- |
| [where_clause()](#rust-iselectstatement-where_clause) | Optional [IWhere](where.md#rust-iwhere) used to build this query. |

##### where_clause() {#rust-iselectstatement-where_clause}

Optional [IWhere](where.md#rust-iwhere) used to build this query.

**Definition**

```rust
fn where_clause(&self) -> Option<&dyn IWhere>
```

**Returns**

`Option<&dyn `[IWhere](where.md#rust-iwhere)`>`

#### SelectStatement {#rust-selectstatement}

```rust
pub struct SelectStatement
```

**Constructor**

| Name | Description |
| --- | --- |
| [text(text)](#rust-selectstatement-text-text) | Wraps query text. |
| [with_where(text, where_clause)](#rust-selectstatement-with_where-text-where_clause) | Wraps query text and an [IWhere](where.md#rust-iwhere). |
| [new(options)](#rust-selectstatement-new-options) | Compiles a `SELECT` from [SelectStatementOptions](#rust-selectstatementoptions). |

**Methods**

| Name | Description |
| --- | --- |
| [accept()](#rust-selectstatement-accept) | Dispatches through the renderer's statement visitor. |
| [where_clause()](#rust-selectstatement-where_clause) | Optional [IWhere](where.md#rust-iwhere) used to build this query. |

##### text(text) {#rust-selectstatement-text-text}

Wraps query text.

**Definition**

```rust
pub fn text(text: impl Into<String>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |

**Returns**

[SelectStatement](#rust-selectstatement)

##### with_where(text, where_clause) {#rust-selectstatement-with_where-text-where_clause}

Wraps query text and an [IWhere](where.md#rust-iwhere).

**Definition**

```rust
pub fn with_where(text: impl Into<String>, where_clause: impl IWhere + 'static) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |
| `where_clause` | [IWhere](where.md#rust-iwhere) |

**Returns**

[SelectStatement](#rust-selectstatement)

##### new(options) {#rust-selectstatement-new-options}

Compiles a `SELECT` from [SelectStatementOptions](#rust-selectstatementoptions).

**Definition**

```rust
pub fn new(options: SelectStatementOptions<'_>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `options` | table, fields, and optional where, joins, group, order, and limit |

**Returns**

[SelectStatement](#rust-selectstatement)

##### accept() {#rust-selectstatement-accept}

Dispatches through the renderer's statement visitor.

**Definition**

```rust
fn accept(&self, text: &StatementTextRenderer) -> String
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | [StatementTextRenderer](statements.md#rust-statementtextrenderer) |

**Returns**

`String`

##### where_clause() {#rust-selectstatement-where_clause}

Optional [IWhere](where.md#rust-iwhere) used to build this query.

**Definition**

```rust
fn where_clause(&self) -> Option<&dyn IWhere>
```

**Returns**

`Option<&dyn `[IWhere](where.md#rust-iwhere)`>`

#### SelectStatementOptions {#rust-selectstatementoptions}

```rust
pub struct SelectStatementOptions<'a>
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(table, fields)](#rust-selectstatementoptions-new-table-fields) | Table and fields, with where, joins, group, order, and limit unset. |

**Properties**

| Name | Description |
| --- | --- |
| table | Table name. |
| fields | Selected columns. Empty compiles `SELECT *`. Aggregate fragments from [Aggregate](#rust-aggregate) belong here. |
| where_clause | Optional [IWhere](where.md#rust-iwhere). |
| joins | [Join](#rust-join) list. Empty leaves the clause off. |
| group_by | `GROUP BY` columns. Empty leaves the clause off. |
| order_by | [OrderBy](#rust-orderby) list. Empty leaves the clause off. |
| limit | Optional `LIMIT`. `Some(0)` is kept. A negative value panics. |

##### new(table, fields) {#rust-selectstatementoptions-new-table-fields}

Table and fields, with where, joins, group, order, and limit unset.

**Definition**

```rust
pub fn new(table: &'a str, fields: &'a [&'a str]) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `table` | table name |
| `fields` | selected columns (`*` when empty) |

**Returns**

[SelectStatementOptions](#rust-selectstatementoptions)

#### Aggregate {#rust-aggregate}

```rust
pub struct Aggregate
```

**Methods**

| Name | Description |
| --- | --- |
| [count(column, alias)](#rust-aggregate-count-column-alias) | `COUNT(column)`, or `COUNT(column) AS alias`. Pass `"*"` for `COUNT(*)`. |
| [sum(column, alias)](#rust-aggregate-sum-column-alias) | `SUM(column)`, or `SUM(column) AS alias`. |
| [min(column, alias)](#rust-aggregate-min-column-alias) | `MIN(column)`, or `MIN(column) AS alias`. |
| [max(column, alias)](#rust-aggregate-max-column-alias) | `MAX(column)`, or `MAX(column) AS alias`. |

##### count(column, alias) {#rust-aggregate-count-column-alias}

`COUNT(column)`, or `COUNT(column) AS alias`. Pass `"*"` for `COUNT(*)`.

**Definition**

```rust
pub fn count(column: &str, alias: Option<&str>) -> String
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | column or `*` |
| `alias` | optional result name |

**Returns**

`String`

##### sum(column, alias) {#rust-aggregate-sum-column-alias}

`SUM(column)`, or `SUM(column) AS alias`.

**Definition**

```rust
pub fn sum(column: &str, alias: Option<&str>) -> String
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | column |
| `alias` | optional result name |

**Returns**

`String`

##### min(column, alias) {#rust-aggregate-min-column-alias}

`MIN(column)`, or `MIN(column) AS alias`.

**Definition**

```rust
pub fn min(column: &str, alias: Option<&str>) -> String
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | column |
| `alias` | optional result name |

**Returns**

`String`

##### max(column, alias) {#rust-aggregate-max-column-alias}

`MAX(column)`, or `MAX(column) AS alias`.

**Definition**

```rust
pub fn max(column: &str, alias: Option<&str>) -> String
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | column |
| `alias` | optional result name |

**Returns**

`String`

#### OrderBy {#rust-orderby}

```rust
pub struct OrderBy
```

**Methods**

| Name | Description |
| --- | --- |
| [asc(column)](#rust-orderby-asc-column) | Sort ascending. |
| [desc(column)](#rust-orderby-desc-column) | Sort descending. |

##### asc(column) {#rust-orderby-asc-column}

Sort ascending.

**Definition**

```rust
pub fn asc(column: impl Into<String>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | sort column |

**Returns**

[OrderBy](#rust-orderby)

##### desc(column) {#rust-orderby-desc-column}

Sort descending.

**Definition**

```rust
pub fn desc(column: impl Into<String>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | sort column |

**Returns**

[OrderBy](#rust-orderby)

#### SortDirection {#rust-sortdirection}

```rust
pub enum SortDirection
```

#### Join {#rust-join}

```rust
pub struct Join
```

**Methods**

| Name | Description |
| --- | --- |
| [inner(table, left, right)](#rust-join-inner-table-left-right) | Inner join. |
| [left(table, left, right)](#rust-join-left-table-left-right) | Left join. |
| [right(table, left, right)](#rust-join-right-table-left-right) | Right join. |

##### inner(table, left, right) {#rust-join-inner-table-left-right}

Inner join.

**Definition**

```rust
pub fn inner(table: impl Into<String>, left: impl Into<String>, right: impl Into<String>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `table` | joined table |
| `left` | left column |
| `right` | right column |

**Returns**

[Join](#rust-join)

##### left(table, left, right) {#rust-join-left-table-left-right}

Left join.

**Definition**

```rust
pub fn left(table: impl Into<String>, left: impl Into<String>, right: impl Into<String>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `table` | joined table |
| `left` | left column |
| `right` | right column |

**Returns**

[Join](#rust-join)

##### right(table, left, right) {#rust-join-right-table-left-right}

Right join.

**Definition**

```rust
pub fn right(table: impl Into<String>, left: impl Into<String>, right: impl Into<String>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `table` | joined table |
| `left` | left column |
| `right` | right column |

**Returns**

[Join](#rust-join)

#### JoinKind {#rust-joinkind}

```rust
pub enum JoinKind
```

## Remarks

[Command](command.md) uses `StatementTextRenderer.Render` / `StatementTextRenderer.render` / `StatementTextRenderer::render` only. Keep the [Where](where.md) when the caller needs bound parameters. Empty `fields` compiles `SELECT *`. Clauses append as `GROUP BY`, `ORDER BY`, then `LIMIT`. An empty group or order list is omitted. `Aggregate` returns a select-list fragment such as `COUNT(*) AS n`.

## Examples

### Wrap query text and an optional where

#### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/StatementCompileTests.cs#L7-L17)

```csharp
var select = new SelectStatement(new() { Table = "dual", Fields = ["1"] });
var byId = new WhereText("id = 1");
var withWhere = new SelectStatement(new() { Table = "dual", Fields = ["1"], Where = byId });
```

#### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/statement_compile.rs#L10-L22)

```rust
let select = SelectStatement::new(SelectStatementOptions::new("dual", &["1"]));
let by_id = WhereText::new("id = 1");
let with_where = SelectStatement::new(SelectStatementOptions {
    where_clause: Some(&by_id),
    ..SelectStatementOptions::new("dual", &["1"])
});
```

#### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/statements/StatementCompile.test.ts#L28-L36)

```typescript
const select = new SelectStatement({ table: "dual", fields: ["1"] });
const byId = new WhereText("id = 1");
const withWhere = new SelectStatement({ table: "dual", fields: ["1"], where: byId });
```

### Compile a SELECT

#### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/StatementCompileTests.cs#L19-L28)

```csharp
var where = new Where(new Operator.And(
    new Operator.Equals("id", 1),
    new Operator.GreaterThan("age", 18)));
var query = new SelectStatement(new() { Table = "users", Fields = ["id", "name"], Where = where });
```

**After: select**

```
SELECT id, name FROM users WHERE id = $id AND age > $age
```

#### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/statement_compile.rs#L24-L39)

```rust
let where_clause = Where::new(Operator::and(
    Operator::equals("id", 1_i64),
    Operator::greater_than("age", 18_i64),
));
let query = SelectStatement::new(SelectStatementOptions {
    where_clause: Some(&where_clause),
    ..SelectStatementOptions::new("users", &["id", "name"])
});
```

**After: select**

```
SELECT id, name FROM users WHERE id = $id AND age > $age
```

#### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/statements/StatementCompile.test.ts#L38-L47)

```typescript
const where = new Where(
  new Operator.And(new Operator.Equals("id", 1), new Operator.GreaterThan("age", 18)),
);
const query = new SelectStatement({ table: "users", fields: ["id", "name"], where });
```

**After: select**

```
SELECT id, name FROM users WHERE id = $id AND age > $age
```

### Join tables

#### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/StatementCompileTests.cs#L30-L48)

```csharp
var query = new SelectStatement(new()
{
    Table = "users",
    Fields = ["users.id"],
    Where = new Where(new Operator.Equals("users.id", 1)),
    Joins =
    [
        Join.Inner("orders", "users.id", "orders.user_id"),
        Join.Left("payments", "orders.id", "payments.order_id"),
        Join.Right("profiles", "users.id", "profiles.user_id"),
    ],
});
```

**After: joins**

```
SELECT users.id FROM users INNER JOIN orders ON users.id = orders.user_id LEFT JOIN payments ON orders.id = payments.order_id RIGHT JOIN profiles ON users.id = profiles.user_id WHERE users.id = $users.id
```

#### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/statement_compile.rs#L41-L57)

```rust
let where_clause = Where::new(Operator::equals("users.id", 1_i64));
let query = SelectStatement::new(SelectStatementOptions {
    where_clause: Some(&where_clause),
    joins: &[
        Join::inner("orders", "users.id", "orders.user_id"),
        Join::left("payments", "orders.id", "payments.order_id"),
        Join::right("profiles", "users.id", "profiles.user_id"),
    ],
    ..SelectStatementOptions::new("users", &["users.id"])
});
```

**After: joins**

```
SELECT users.id FROM users INNER JOIN orders ON users.id = orders.user_id LEFT JOIN payments ON orders.id = payments.order_id RIGHT JOIN profiles ON users.id = profiles.user_id WHERE users.id = $users.id
```

#### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/statements/StatementCompile.test.ts#L49-L63)

```typescript
const query = new SelectStatement({
  table: "users",
  fields: ["users.id"],
  where: new Where(new Operator.Equals("users.id", 1)),
  joins: [
    Join.inner("orders", "users.id", "orders.user_id"),
    Join.left("payments", "orders.id", "payments.order_id"),
    Join.right("profiles", "users.id", "profiles.user_id"),
  ],
});
```

**After: joins**

```
SELECT users.id FROM users INNER JOIN orders ON users.id = orders.user_id LEFT JOIN payments ON orders.id = payments.order_id RIGHT JOIN profiles ON users.id = profiles.user_id WHERE users.id = $users.id
```

### Group, aggregate, order, and limit

#### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/StatementCompileTests.cs#L50-L66)

```csharp
var grouped = new SelectStatement(new()
{
    Table = "orders",
    Fields = ["status", Aggregate.Count("*", "n"), Aggregate.Sum("amount", "total")],
    Where = new Where(new Operator.Equals("status", "open")),
    Joins = [Join.Inner("customers", "orders.customer_id", "customers.id")],
    GroupBy = ["status"],
    OrderBy = [OrderBy.Desc("total"), OrderBy.Asc("status")],
    Limit = 10,
});
```

**After: grouped select**

```
SELECT status, COUNT(*) AS n, SUM(amount) AS total FROM orders INNER JOIN customers ON orders.customer_id = customers.id WHERE status = $status GROUP BY status ORDER BY total DESC, status ASC LIMIT 10
```

#### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/statement_compile.rs#L59-L81)

```rust
let count = Aggregate::count("*", Some("n"));
let total = Aggregate::sum("amount", Some("total"));
let grouped_fields = ["status", count.as_str(), total.as_str()];
let where_clause = Where::new(Operator::equals("status", "open"));
let grouped = SelectStatement::new(SelectStatementOptions {
    where_clause: Some(&where_clause),
    joins: &[Join::inner(
        "customers",
        "orders.customer_id",
        "customers.id",
    )],
    group_by: &["status"],
    order_by: &[OrderBy::desc("total"), OrderBy::asc("status")],
    limit: Some(10),
    ..SelectStatementOptions::new("orders", &grouped_fields)
});
```

**After: grouped select**

```
SELECT status, COUNT(*) AS n, SUM(amount) AS total FROM orders INNER JOIN customers ON orders.customer_id = customers.id WHERE status = $status GROUP BY status ORDER BY total DESC, status ASC LIMIT 10
```

#### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/statements/StatementCompile.test.ts#L65-L78)

```typescript
const grouped = new SelectStatement({
  table: "orders",
  fields: ["status", Aggregate.count("*", "n"), Aggregate.sum("amount", "total")],
  where: new Where(new Operator.Equals("status", "open")),
  joins: [Join.inner("customers", "orders.customer_id", "customers.id")],
  groupBy: ["status"],
  orderBy: [OrderBy.desc("total"), OrderBy.asc("status")],
  limit: 10,
});
```

**After: grouped select**

```
SELECT status, COUNT(*) AS n, SUM(amount) AS total FROM orders INNER JOIN customers ON orders.customer_id = customers.id WHERE status = $status GROUP BY status ORDER BY total DESC, status ASC LIMIT 10
```

### Group rows

#### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/StatementCompileTests.cs#L68-L73)

```csharp
var byStatus = new SelectStatement(new() { Table = "users", Fields = ["id"], GroupBy = ["id"] });
```

**After: group by**

```
SELECT id FROM users GROUP BY id
```

#### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/statement_compile.rs#L83-L90)

```rust
let by_status = SelectStatement::new(SelectStatementOptions {
    group_by: &["id"],
    ..SelectStatementOptions::new("users", &["id"])
});
```

**After: group by**

```
SELECT id FROM users GROUP BY id
```

#### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/statements/StatementCompile.test.ts#L80-L83)

```typescript
const byStatus = new SelectStatement({ table: "users", fields: ["id"], groupBy: ["id"] });
```

**After: group by**

```
SELECT id FROM users GROUP BY id
```

### Aggregate without grouping

#### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/StatementCompileTests.cs#L75-L80)

```csharp
var bounds = new SelectStatement(new() { Table = "users", Fields = [Aggregate.Min("n"), Aggregate.Max("n")] });
```

**After: min and max**

```
SELECT MIN(n), MAX(n) FROM users
```

#### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/statement_compile.rs#L92-L99)

```rust
let min_n = Aggregate::min("n", None);
let max_n = Aggregate::max("n", None);
let bound_fields = [min_n.as_str(), max_n.as_str()];
let bounds = SelectStatement::new(SelectStatementOptions::new("users", &bound_fields));
```

**After: min and max**

```
SELECT MIN(n), MAX(n) FROM users
```

#### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/statements/StatementCompile.test.ts#L85-L88)

```typescript
const bounds = new SelectStatement({ table: "users", fields: [Aggregate.min("n"), Aggregate.max("n")] });
```

**After: min and max**

```
SELECT MIN(n), MAX(n) FROM users
```

### Order rows

#### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/StatementCompileTests.cs#L82-L87)

```csharp
var ordered = new SelectStatement(new() { Table = "users", Fields = ["id"], OrderBy = [OrderBy.Asc("id"), OrderBy.Desc("name")] });
```

**After: order by**

```
SELECT id FROM users ORDER BY id ASC, name DESC
```

#### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/statement_compile.rs#L101-L111)

```rust
let ordered = SelectStatement::new(SelectStatementOptions {
    order_by: &[OrderBy::asc("id"), OrderBy::desc("name")],
    ..SelectStatementOptions::new("users", &["id"])
});
```

**After: order by**

```
SELECT id FROM users ORDER BY id ASC, name DESC
```

#### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/statements/StatementCompile.test.ts#L90-L99)

```typescript
const ordered = new SelectStatement({
  table: "users",
  fields: ["id"],
  orderBy: [OrderBy.asc("id"), OrderBy.desc("name")],
});
```

**After: order by**

```
SELECT id FROM users ORDER BY id ASC, name DESC
```

### Limit rows

#### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/StatementCompileTests.cs#L89-L94)

```csharp
var limited = new SelectStatement(new() { Table = "users", Fields = ["id"], Limit = 10 });
```

**After: limit**

```
SELECT id FROM users LIMIT 10
```

#### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/statement_compile.rs#L113-L120)

```rust
let limited = SelectStatement::new(SelectStatementOptions {
    limit: Some(10),
    ..SelectStatementOptions::new("users", &["id"])
});
```

**After: limit**

```
SELECT id FROM users LIMIT 10
```

#### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/statements/StatementCompile.test.ts#L101-L104)

```typescript
const limited = new SelectStatement({ table: "users", fields: ["id"], limit: 10 });
```

**After: limit**

```
SELECT id FROM users LIMIT 10
```
