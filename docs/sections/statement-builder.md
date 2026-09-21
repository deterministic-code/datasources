# StatementBuilder

Stateless factory for query / insert / update / delete / table / stored-procedure SQL. Identifiers are unquoted. Placeholders are `$name`.

## Classes

::: reference
### typescript
#### StatementBuilder

```typescript
const StatementBuilder
```

**Methods**

##### createQuery(table, fields, where?, joins?)

Builds a select. An empty field list is `SELECT *`.

```typescript
createQuery(table: string, fields: string[], where?: IWhere, joins?: Join[]): IQuery
```

- `table` — table name
- `fields` — selected columns; empty selects `*`
- `where` — optional [IWhere](./statements.md#iwhere)
- `joins` — optional [Join](#join) list

**Returns** [IQuery](./statements.md#iquery)

##### createInsert(table, fields)

Builds an insert. Values are `$name` placeholders.

```typescript
createInsert(table: string, fields: Field[]): IInsertStatement
```

- `table` — table name
- `fields` — [Field](#field) name and value pairs

**Returns** [IInsertStatement](./statements.md#iinsertstatement)

##### createUpdate(table, fields, where?)

Builds an update. Assignments are `name = $name`.

```typescript
createUpdate(table: string, fields: Field[], where?: IWhere): IUpdateStatement
```

- `table` — table name
- `fields` — [Field](#field) name and value pairs
- `where` — optional [IWhere](./statements.md#iwhere)

**Returns** [IUpdateStatement](./statements.md#iupdatestatement)

##### createDelete(table, where?)

Builds a delete.

```typescript
createDelete(table: string, where?: IWhere): IDeleteStatement
```

- `table` — table name
- `where` — optional [IWhere](./statements.md#iwhere)

**Returns** [IDeleteStatement](./statements.md#ideletestatement)

##### createTable(table, columns)

Builds a create-table statement.

```typescript
createTable(table: string, columns: Column[]): ICreateTableStatement
```

- `table` — table name
- `columns` — [Column](#column) name and type pairs

**Returns** [ICreateTableStatement](./statements.md#icreatetablestatement)

##### createStoredProcedure(name, parameters?)

Builds a `CALL` with `$name` placeholders.

```typescript
createStoredProcedure(name: string, parameters?: Field[]): IStoredProcedureStatement
```

- `name` — procedure name
- `parameters` — optional [Field](#field) list

**Returns** [IStoredProcedureStatement](./statements.md#istoredprocedurestatement)

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

SQL type text.

```typescript
type: string
```

#### Field

```typescript
interface Field
```

Name and value used by insert, update, and stored-procedure builders.

**Properties**

##### name

Field name.

```typescript
name: string
```

##### value

Field [Scalar](./where-statement.md#scalar).

```typescript
value: Scalar
```

#### JoinKind

```typescript
type JoinKind = "inner" | "left" | "right"
```

Join kind for [Join](#join).

#### Join

```typescript
type Join
```

**Properties**

##### kind

[JoinKind](#joinkind).

```typescript
kind: JoinKind
```

##### table

Joined table.

```typescript
table: string
```

##### left

Left-hand join column.

```typescript
left: string
```

##### right

Right-hand join column.

```typescript
right: string
```

**Methods**

##### inner(table, left, right)

Builds an inner [Join](#join).

```typescript
inner(table: string, left: string, right: string): Join
```

- `table` — joined table
- `left` — left-hand column
- `right` — right-hand column

**Returns** [Join](#join)

##### left(table, left, right)

Builds a left [Join](#join).

```typescript
left(table: string, left: string, right: string): Join
```

- `table` — joined table
- `left` — left-hand column
- `right` — right-hand column

**Returns** [Join](#join)

##### right(table, left, right)

Builds a right [Join](#join).

```typescript
right(table: string, left: string, right: string): Join
```

- `table` — joined table
- `left` — left-hand column
- `right` — right-hand column

**Returns** [Join](#join)

### csharp
#### StatementBuilder

```csharp
public static class StatementBuilder
```

**Methods**

##### CreateQuery(table, fields, where, joins)

Builds a select. An empty field list is `SELECT *`.

```csharp
public static IQuery CreateQuery(string table, IReadOnlyList<string> fields, IWhere? where = null, IReadOnlyList<Join>? joins = null)
```

- `table` — table name
- `fields` — selected columns; empty selects `*`
- `where` — optional [IWhere](./statements.md#iwhere)
- `joins` — optional [Join](#join) list

**Returns** [IQuery](./statements.md#iquery)

##### CreateInsert(table, fields)

Builds an insert. Values are `$name` placeholders.

```csharp
public static IInsertStatement CreateInsert(string table, IReadOnlyList<Parameter> fields)
```

- `table` — table name
- `fields` — [Parameter](./command.md#parameter) name and value pairs

**Returns** [IInsertStatement](./statements.md#iinsertstatement)

##### CreateUpdate(table, fields, where)

Builds an update. Assignments are `name = $name`.

```csharp
public static IUpdateStatement CreateUpdate(string table, IReadOnlyList<Parameter> fields, IWhere? where = null)
```

- `table` — table name
- `fields` — [Parameter](./command.md#parameter) name and value pairs
- `where` — optional [IWhere](./statements.md#iwhere)

**Returns** [IUpdateStatement](./statements.md#iupdatestatement)

##### CreateDelete(table, where)

Builds a delete.

```csharp
public static IDeleteStatement CreateDelete(string table, IWhere? where = null)
```

- `table` — table name
- `where` — optional [IWhere](./statements.md#iwhere)

**Returns** [IDeleteStatement](./statements.md#ideletestatement)

##### CreateTable(table, columns)

Builds a create-table statement.

```csharp
public static ICreateTableStatement CreateTable(string table, IReadOnlyList<Column> columns)
```

- `table` — table name
- `columns` — [Column](#column) name and type pairs

**Returns** [ICreateTableStatement](./statements.md#icreatetablestatement)

##### CreateStoredProcedure(name, parameters)

Builds a `CALL` with `$name` placeholders.

```csharp
public static IStoredProcedureStatement CreateStoredProcedure(string name, IReadOnlyList<Parameter>? parameters = null)
```

- `name` — procedure name
- `parameters` — optional [Parameter](./command.md#parameter) list

**Returns** [IStoredProcedureStatement](./statements.md#istoredprocedurestatement)

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

SQL type text.

```csharp
string Type { get; }
```

#### JoinKind

```csharp
public enum JoinKind
```

**Properties**

##### Inner

Inner join.

```csharp
Inner
```

##### Left

Left join.

```csharp
Left
```

##### Right

Right join.

```csharp
Right
```

#### Join

```csharp
public sealed record Join(JoinKind Kind, string Table, string LeftColumn, string RightColumn)
```

**Properties**

##### Kind

[JoinKind](#joinkind).

```csharp
JoinKind Kind { get; }
```

##### Table

Joined table.

```csharp
string Table { get; }
```

##### LeftColumn

Left-hand join column.

```csharp
string LeftColumn { get; }
```

##### RightColumn

Right-hand join column.

```csharp
string RightColumn { get; }
```

**Methods**

##### Inner(table, left, right)

Builds an inner [Join](#join).

```csharp
public static Join Inner(string table, string left, string right)
```

- `table` — joined table
- `left` — left-hand column
- `right` — right-hand column

**Returns** [Join](#join)

##### Left(table, left, right)

Builds a left [Join](#join).

```csharp
public static Join Left(string table, string left, string right)
```

- `table` — joined table
- `left` — left-hand column
- `right` — right-hand column

**Returns** [Join](#join)

##### Right(table, left, right)

Builds a right [Join](#join).

```csharp
public static Join Right(string table, string left, string right)
```

- `table` — joined table
- `left` — left-hand column
- `right` — right-hand column

**Returns** [Join](#join)

### rust
#### StatementBuilder

```rust
pub struct StatementBuilder
```

**Methods**

##### create_query(table, fields, where_clause, joins)

Builds a select. An empty field list is `SELECT *`.

```rust
pub fn create_query(table: impl AsRef<str>, fields: &[&str], where_clause: Option<&dyn Where>, joins: &[Join]) -> TextQuery
```

- `table` — table name
- `fields` — selected columns; empty selects `*`
- `where_clause` — optional [Where](./statements.md#where)
- `joins` — [Join](#join) slice

**Returns** [TextQuery](./statements.md#textquery)

##### create_insert(table, fields)

Builds an insert. Values are `$name` placeholders.

```rust
pub fn create_insert(table: impl AsRef<str>, fields: &[Parameter]) -> TextInsertStatement
```

- `table` — table name
- `fields` — [Parameter](./command.md#parameter) name and value pairs

**Returns** [TextInsertStatement](./statements.md#textinsertstatement)

##### create_update(table, fields, where_clause)

Builds an update. Assignments are `name = $name`.

```rust
pub fn create_update(table: impl AsRef<str>, fields: &[Parameter], where_clause: Option<&dyn Where>) -> TextUpdateStatement
```

- `table` — table name
- `fields` — [Parameter](./command.md#parameter) name and value pairs
- `where_clause` — optional [Where](./statements.md#where)

**Returns** [TextUpdateStatement](./statements.md#textupdatestatement)

##### create_delete(table, where_clause)

Builds a delete.

```rust
pub fn create_delete(table: impl AsRef<str>, where_clause: Option<&dyn Where>) -> TextDeleteStatement
```

- `table` — table name
- `where_clause` — optional [Where](./statements.md#where)

**Returns** [TextDeleteStatement](./statements.md#textdeletestatement)

##### create_table(table, columns)

Builds a create-table statement.

```rust
pub fn create_table(table: impl AsRef<str>, columns: &[Column]) -> TextCreateTableStatement
```

- `table` — table name
- `columns` — [Column](#column) name and type pairs

**Returns** [TextCreateTableStatement](./statements.md#textcreatetablestatement)

##### create_stored_procedure(name, parameters)

Builds a `CALL` with `$name` placeholders.

```rust
pub fn create_stored_procedure(name: impl AsRef<str>, parameters: &[Parameter]) -> TextStoredProcedureStatement
```

- `name` — procedure name
- `parameters` — [Parameter](./command.md#parameter) slice

**Returns** [TextStoredProcedureStatement](./statements.md#textstoredprocedurestatement)

#### Column

```rust
pub struct Column
```

**Constructor**

##### new(name, type_name)

Builds a column name and SQL type.

```rust
pub fn new(name: impl Into<String>, type_name: impl Into<String>) -> Self
```

- `name` — column name
- `type_name` — SQL type text

**Returns** [Column](#column)

**Properties**

##### name

Column name.

```rust
pub name: String
```

##### type_name

SQL type text.

```rust
pub type_name: String
```

#### JoinKind

```rust
pub enum JoinKind
```

**Properties**

##### Inner

Inner join.

```rust
Inner
```

##### Left

Left join.

```rust
Left
```

##### Right

Right join.

```rust
Right
```

#### Join

```rust
pub struct Join
```

**Properties**

##### kind

[JoinKind](#joinkind).

```rust
pub kind: JoinKind
```

##### table

Joined table.

```rust
pub table: String
```

##### left

Left-hand join column.

```rust
pub left: String
```

##### right

Right-hand join column.

```rust
pub right: String
```

**Methods**

##### inner(table, left, right)

Builds an inner [Join](#join).

```rust
pub fn inner(table: impl Into<String>, left: impl Into<String>, right: impl Into<String>) -> Self
```

- `table` — joined table
- `left` — left-hand column
- `right` — right-hand column

**Returns** [Join](#join)

##### left(table, left, right)

Builds a left [Join](#join).

```rust
pub fn left(table: impl Into<String>, left: impl Into<String>, right: impl Into<String>) -> Self
```

- `table` — joined table
- `left` — left-hand column
- `right` — right-hand column

**Returns** [Join](#join)

##### right(table, left, right)

Builds a right [Join](#join).

```rust
pub fn right(table: impl Into<String>, left: impl Into<String>, right: impl Into<String>) -> Self
```

- `table` — joined table
- `left` — left-hand column
- `right` — right-hand column

**Returns** [Join](#join)
:::

## Remarks

Dialect packages wrap [StatementBuilder](#statementbuilder) for quoting and native column types, including `CreateStoredProcedure` the same way as `CreateInsert`. C# and TypeScript keep the original [IWhere](./statements.md#iwhere) on query / update / delete. Rust stores `where_clause()`. Joins exist on the core builder. The SQLite dialect forwards them; Postgres and MySQL dialect wrappers do not.

[RepositoryStatements](./crud-repository.md) is table-scoped sugar over this builder.

## Examples

### Build a select

::: example
```csharp csharp/Datasources.Tests/CoreTests.cs#L78-L79
```
```rust rust/datasources/src/tests/mod.rs#L696-L702
```
```typescript typescript/datasources/src/builders/builders.test.ts#L70-L71
```

```output [After: select]
SELECT id, name FROM users WHERE id = $id AND age > $age
```

```output:typescript [After: select]
SELECT id, name FROM users WHERE id = $id
```

```output [After: insert]
INSERT INTO users (id, name) VALUES ($id, $name)
```

```output [After: call]
CALL save($id, $name)
```
:::

### Query with joins

::: example
```csharp csharp/Datasources.Tests/Contact/ContactsJoinQuery.cs#L8-L34
```
```rust rust/sql-specs/src/contacts_join_query.rs#L3-L39
```
```typescript typescript/datasources/src/tests/contact/contactsJoinQuery.ts#L3-L30
```
:::
