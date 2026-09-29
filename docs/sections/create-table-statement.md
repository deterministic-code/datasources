# Create Table Statement

`CREATE TABLE` text, or a compiled `CREATE TABLE` from a table, columns, and optional named table constraints.

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

##### CreateTableStatement(table, columns, constraints?)

Compiles a `CREATE TABLE`.

```typescript
constructor(table: string, columns: Column[], constraints?: ITableConstraint[])
```

- `table` — table name
- `columns` — [Column](#column) names, types, optional [ColumnDefault](#columndefault) and [NotNull](#notnull)
- `constraints` — optional named [ITableConstraint](#itableconstraint) list

**Properties**

##### table

Table name when compiled from columns. Absent for wrapped text.

```typescript
readonly table?: string
```

##### columns

Columns on a compiled table. Empty for wrapped text.

```typescript
readonly columns: Column[]
```

##### constraints

Named table constraints on a compiled table. Empty for wrapped text.

```typescript
readonly constraints: ITableConstraint[]
```

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

##### default

Optional [ColumnDefault](#columndefault).

```typescript
default?: ColumnDefault
```

##### notNull

Optional [NotNull](#notnull).

```typescript
notNull?: NotNull
```

##### identity

When true, MySQL and Postgres commands render an identity clause. `getText()` leaves it off.

```typescript
identity?: boolean
```

#### ColumnDefault

```typescript
class ColumnDefault
```

**Constructor**

##### ColumnDefault(value, name?)

Default value, optionally named.

```typescript
constructor(value: string, name?: string)
```

- `value` — raw SQL default fragment
- `name` — optional constraint name

**Properties**

##### value

Raw SQL default fragment.

```typescript
readonly value: string
```

##### name

Optional constraint name.

```typescript
readonly name?: string
```

#### NotNull

```typescript
class NotNull
```

**Constructor**

##### NotNull(name?)

`NOT NULL`, optionally named.

```typescript
constructor(name?: string)
```

- `name` — optional constraint name

**Properties**

##### name

Optional constraint name.

```typescript
readonly name?: string
```

#### ITableConstraint

```typescript
interface ITableConstraint
```

**Properties**

##### kind

Constraint kind.

```typescript
readonly kind: TableConstraintKind
```

##### name

Constraint name.

```typescript
readonly name: string
```

#### PrimaryKey

```typescript
class PrimaryKey implements ITableConstraint
```

**Constructor**

##### PrimaryKey(name, columns)

Named primary key. Multiple columns are a composite key.

```typescript
constructor(name: string, columns: string[])
```

- `name` — constraint name
- `columns` — key columns

#### ForeignKey

```typescript
class ForeignKey implements ITableConstraint
```

**Constructor**

##### ForeignKey(name, columns, referencesTable, referencesColumns, onDelete?, onUpdate?)

Named foreign key.

```typescript
constructor(name: string, columns: string[], referencesTable: string, referencesColumns: string[], onDelete?: ReferentialAction, onUpdate?: ReferentialAction)
```

- `name` — constraint name
- `columns` — local columns
- `referencesTable` — referenced table
- `referencesColumns` — referenced columns
- `onDelete` — optional [ReferentialAction](#referentialaction)
- `onUpdate` — optional [ReferentialAction](#referentialaction)

#### UniqueConstraint

```typescript
class UniqueConstraint implements ITableConstraint
```

**Constructor**

##### UniqueConstraint(name, columns)

Named unique constraint.

```typescript
constructor(name: string, columns: string[])
```

- `name` — constraint name
- `columns` — unique columns

#### CheckConstraint

```typescript
class CheckConstraint implements ITableConstraint
```

**Constructor**

##### CheckConstraint(name, expression)

Named check constraint.

```typescript
constructor(name: string, expression: string)
```

- `name` — constraint name
- `expression` — raw SQL check fragment

#### ReferentialAction

```typescript
const ReferentialAction
```

**Properties**

##### Cascade

`ON DELETE` / `ON UPDATE CASCADE`.

```typescript
Cascade: "cascade"
```

##### SetNull

`ON DELETE` / `ON UPDATE SET NULL`.

```typescript
SetNull: "setNull"
```

##### SetDefault

`ON DELETE` / `ON UPDATE SET DEFAULT`.

```typescript
SetDefault: "setDefault"
```

##### Restrict

`ON DELETE` / `ON UPDATE RESTRICT`.

```typescript
Restrict: "restrict"
```

##### NoAction

`ON DELETE` / `ON UPDATE NO ACTION`.

```typescript
NoAction: "noAction"
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

##### CreateTableStatement(table, columns, constraints?)

Compiles a `CREATE TABLE`.

```csharp
public CreateTableStatement(string table, IReadOnlyList<Column> columns, IReadOnlyList<ITableConstraint>? constraints = null)
```

- `table` — table name
- `columns` — [Column](#column) names, types, optional [ColumnDefault](#columndefault) and [NotNull](#notnull)
- `constraints` — optional named [ITableConstraint](#itableconstraint) list

**Properties**

##### Table

Table name when compiled from columns. Absent for wrapped text.

```csharp
public string? Table { get; }
```

##### Columns

Columns on a compiled table. Empty for wrapped text.

```csharp
public IReadOnlyList<Column> Columns { get; }
```

##### Constraints

Named table constraints on a compiled table. Empty for wrapped text.

```csharp
public IReadOnlyList<ITableConstraint> Constraints { get; }
```

**Methods**

##### GetText()

Command text.

```csharp
public string GetText()
```

**Returns** `string`

#### Column

```csharp
public sealed record Column(string Name, string Type, ColumnDefault? Default = null, NotNull? NotNull = null, bool Identity = false)
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

##### Default

Optional [ColumnDefault](#columndefault).

```csharp
ColumnDefault? Default { get; }
```

##### NotNull

Optional [NotNull](#notnull).

```csharp
NotNull? NotNull { get; }
```

##### Identity

When true, MySQL and Postgres commands render an identity clause. `GetText()` leaves it off.

```csharp
bool Identity { get; }
```

#### ColumnDefault

```csharp
public sealed record ColumnDefault(string Value, string? Name = null)
```

**Properties**

##### Value

Raw SQL default fragment.

```csharp
string Value { get; }
```

##### Name

Optional constraint name.

```csharp
string? Name { get; }
```

#### NotNull

```csharp
public sealed record NotNull(string? Name = null)
```

**Properties**

##### Name

Optional constraint name.

```csharp
string? Name { get; }
```

#### ITableConstraint

```csharp
public interface ITableConstraint
```

#### PrimaryKey

```csharp
public sealed record PrimaryKey(string Name, IReadOnlyList<string> Columns) : ITableConstraint
```

**Properties**

##### Name

Constraint name.

```csharp
string Name { get; }
```

##### Columns

Key columns. Multiple columns are a composite key.

```csharp
IReadOnlyList<string> Columns { get; }
```

#### ForeignKey

```csharp
public sealed record ForeignKey(string Name, IReadOnlyList<string> Columns, string ReferencesTable, IReadOnlyList<string> ReferencesColumns, ReferentialAction? OnDelete = null, ReferentialAction? OnUpdate = null) : ITableConstraint
```

**Properties**

##### Name

Constraint name.

```csharp
string Name { get; }
```

##### Columns

Local columns.

```csharp
IReadOnlyList<string> Columns { get; }
```

##### ReferencesTable

Referenced table.

```csharp
string ReferencesTable { get; }
```

##### ReferencesColumns

Referenced columns.

```csharp
IReadOnlyList<string> ReferencesColumns { get; }
```

##### OnDelete

Optional [ReferentialAction](#referentialaction).

```csharp
ReferentialAction? OnDelete { get; }
```

##### OnUpdate

Optional [ReferentialAction](#referentialaction).

```csharp
ReferentialAction? OnUpdate { get; }
```

#### UniqueConstraint

```csharp
public sealed record UniqueConstraint(string Name, IReadOnlyList<string> Columns) : ITableConstraint
```

**Properties**

##### Name

Constraint name.

```csharp
string Name { get; }
```

##### Columns

Unique columns.

```csharp
IReadOnlyList<string> Columns { get; }
```

#### CheckConstraint

```csharp
public sealed record CheckConstraint(string Name, string Expression) : ITableConstraint
```

**Properties**

##### Name

Constraint name.

```csharp
string Name { get; }
```

##### Expression

Raw SQL check fragment.

```csharp
string Expression { get; }
```

#### ReferentialAction

```csharp
public enum ReferentialAction
```

**Properties**

##### Cascade

`ON DELETE` / `ON UPDATE CASCADE`.

```csharp
Cascade
```

##### SetNull

`ON DELETE` / `ON UPDATE SET NULL`.

```csharp
SetNull
```

##### SetDefault

`ON DELETE` / `ON UPDATE SET DEFAULT`.

```csharp
SetDefault
```

##### Restrict

`ON DELETE` / `ON UPDATE RESTRICT`.

```csharp
Restrict
```

##### NoAction

`ON DELETE` / `ON UPDATE NO ACTION`.

```csharp
NoAction
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

##### new(table, columns, constraints)

Compiles a `CREATE TABLE`.

```rust
pub fn new(table: impl AsRef<str>, columns: &[Column], constraints: &[TableConstraint]) -> Self
```

- `table` — table name
- `columns` — [Column](#column) names, types, optional [ColumnDefault](#columndefault) and [NotNull](#notnull)
- `constraints` — named [TableConstraint](#tableconstraint) list (empty when omitted in C# / TypeScript)

**Returns** [CreateTableStatement](#createtablestatement)

**Properties**

##### table

Table name when compiled from columns. Absent for wrapped text.

```rust
pub table: Option<String>
```

##### columns

Columns on a compiled table. Empty for wrapped text.

```rust
pub columns: Vec<Column>
```

##### constraints

Named table constraints on a compiled table. Empty for wrapped text.

```rust
pub constraints: Vec<TableConstraint>
```

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

##### default(self, default)

Sets an optional [ColumnDefault](#columndefault).

```rust
pub fn default(self, default: ColumnDefault) -> Self
```

- `default` — [ColumnDefault](#columndefault)

**Returns** [Column](#column)

##### not_null(self, not_null)

Sets an optional [NotNull](#notnull).

```rust
pub fn not_null(self, not_null: NotNull) -> Self
```

- `not_null` — [NotNull](#notnull)

**Returns** [Column](#column)

##### identity(self)

Marks the column as an identity. `get_text()` leaves the clause off. MySQL and Postgres commands render it.

```rust
pub fn identity(self) -> Self
```

**Returns** [Column](#column)

#### ColumnDefault

```rust
pub struct ColumnDefault
```

**Constructor**

##### new(value, name)

Default value, optionally named.

```rust
pub fn new(value: impl Into<String>, name: Option<&str>) -> Self
```

- `value` — raw SQL default fragment
- `name` — optional constraint name

**Returns** [ColumnDefault](#columndefault)

#### NotNull

```rust
pub struct NotNull
```

**Constructor**

##### new(name)

`NOT NULL`, optionally named.

```rust
pub fn new(name: Option<&str>) -> Self
```

- `name` — optional constraint name

**Returns** [NotNull](#notnull)

#### TableConstraint

```rust
pub enum TableConstraint
```

#### PrimaryKey

```rust
pub struct PrimaryKey
```

**Constructor**

##### new(name, columns)

Named primary key. Multiple columns are a composite key.

```rust
pub fn new(name: impl Into<String>, columns: &[&str]) -> TableConstraint
```

- `name` — constraint name
- `columns` — key columns

**Returns** [TableConstraint](#tableconstraint)

#### ForeignKey

```rust
pub struct ForeignKey
```

**Constructor**

##### new(name, columns, references_table, references_columns, on_delete, on_update)

Named foreign key.

```rust
pub fn new(name: impl Into<String>, columns: &[&str], references_table: impl Into<String>, references_columns: &[&str], on_delete: Option<ReferentialAction>, on_update: Option<ReferentialAction>) -> TableConstraint
```

- `name` — constraint name
- `columns` — local columns
- `references_table` — referenced table
- `references_columns` — referenced columns
- `on_delete` — optional [ReferentialAction](#referentialaction)
- `on_update` — optional [ReferentialAction](#referentialaction)

**Returns** [TableConstraint](#tableconstraint)

#### UniqueConstraint

```rust
pub struct UniqueConstraint
```

**Constructor**

##### new(name, columns)

Named unique constraint.

```rust
pub fn new(name: impl Into<String>, columns: &[&str]) -> TableConstraint
```

- `name` — constraint name
- `columns` — unique columns

**Returns** [TableConstraint](#tableconstraint)

#### CheckConstraint

```rust
pub struct CheckConstraint
```

**Constructor**

##### new(name, expression)

Named check constraint.

```rust
pub fn new(name: impl Into<String>, expression: impl Into<String>) -> TableConstraint
```

- `name` — constraint name
- `expression` — raw SQL check fragment

**Returns** [TableConstraint](#tableconstraint)

#### ReferentialAction

```rust
pub enum ReferentialAction
```

**Properties**

##### Cascade

`ON DELETE` / `ON UPDATE CASCADE`.

```rust
Cascade
```

##### SetNull

`ON DELETE` / `ON UPDATE SET NULL`.

```rust
SetNull
```

##### SetDefault

`ON DELETE` / `ON UPDATE SET DEFAULT`.

```rust
SetDefault
```

##### Restrict

`ON DELETE` / `ON UPDATE RESTRICT`.

```rust
Restrict
```

##### NoAction

`ON DELETE` / `ON UPDATE NO ACTION`.

```rust
NoAction
```
:::

## Remarks

A compiled statement stores the table, columns, and constraints. {{GetText()|getText()|get_text()}} renders `CREATE TABLE table (name TYPE [CONSTRAINT n DEFAULT v] [CONSTRAINT n NOT NULL], …, CONSTRAINT name …)`. Table and column names are interpolated as given. Default and check values are raw SQL fragments. Wrapped text has no table. Indexes use [CreateIndexStatement](./create-index-statement.md). {{GetText()|getText()|get_text()}} uses `CreateTableStatementRenderer`. A provider subclasses that renderer and can replace {{Render|render|render}} or override type, default, and identity. SQLite, MySQL, and Postgres override type so canonical names become engine types. MySQL and Postgres also override default and identity. A Postgres `bool` default goes through the bool type converter, so `1` becomes `TRUE`. Identity renders `AUTO_INCREMENT` or `GENERATED BY DEFAULT AS IDENTITY`. SQLite leaves identity empty. {{GetText()|getText()|get_text()}} leaves those changes off. Other commands use the rendered text.

## Examples

### Compile a CREATE TABLE

::: example
```csharp csharp/Datasources.Tests/CoreTests.cs#StatementsAndBuilders
```
```rust rust/datasources/src/tests/mod.rs#statements_and_builders
```
```typescript typescript/datasources/src/builders/builders.test.ts#creates-write-and-ddl-statements
```

```output [After: create table]
CREATE TABLE users (id INTEGER, name TEXT)
```
:::

### Compile named constraints

::: example
```csharp csharp/Datasources.Tests/CoreTests.cs#StatementsAndBuilders
```
```rust rust/datasources/src/tests/mod.rs#statements_and_builders
```
```typescript typescript/datasources/src/builders/builders.test.ts#creates-write-and-ddl-statements
```

```output [After: named constraints]
CREATE TABLE orders (id INTEGER CONSTRAINT df_id DEFAULT 0 CONSTRAINT nn_id NOT NULL, tenant_id INTEGER, user_id INTEGER, qty INTEGER, CONSTRAINT pk_orders PRIMARY KEY (id, tenant_id), CONSTRAINT fk_orders_user FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE, CONSTRAINT uq_orders_user UNIQUE (user_id), CONSTRAINT chk_orders_qty CHECK (qty > 0))
```
:::
