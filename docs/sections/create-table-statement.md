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

**Methods**

##### GetText()

Command text.

```csharp
public string GetText()
```

**Returns** `string`

#### Column

```csharp
public sealed record Column(string Name, string Type, ColumnDefault? Default = null, NotNull? NotNull = null)
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

##### new(table, columns)

Compiles a `CREATE TABLE`.

```rust
pub fn new(table: impl AsRef<str>, columns: &[Column]) -> Self
```

- `table` — table name
- `columns` — [Column](#column) names and types

**Returns** [CreateTableStatement](#createtablestatement)

##### with_constraints(table, columns, constraints)

Compiles a `CREATE TABLE` with named [TableConstraint](#tableconstraint) values.

```rust
pub fn with_constraints(table: impl AsRef<str>, columns: &[Column], constraints: &[TableConstraint]) -> Self
```

- `table` — table name
- `columns` — [Column](#column) names, types, optional [ColumnDefault](#columndefault) and [NotNull](#notnull)
- `constraints` — named [TableConstraint](#tableconstraint) list

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

##### with_default(self, default)

Sets an optional [ColumnDefault](#columndefault).

```rust
pub fn with_default(self, default: ColumnDefault) -> Self
```

- `default` — [ColumnDefault](#columndefault)

**Returns** [Column](#column)

##### with_not_null(self, not_null)

Sets an optional [NotNull](#notnull).

```rust
pub fn with_not_null(self, not_null: NotNull) -> Self
```

- `not_null` — [NotNull](#notnull)

**Returns** [Column](#column)

#### ColumnDefault

```rust
pub struct ColumnDefault
```

**Constructor**

##### new(value)

Unnamed default value.

```rust
pub fn new(value: impl Into<String>) -> Self
```

- `value` — raw SQL default fragment

**Returns** [ColumnDefault](#columndefault)

##### named(name, value)

Named default value.

```rust
pub fn named(name: impl Into<String>, value: impl Into<String>) -> Self
```

- `name` — constraint name
- `value` — raw SQL default fragment

**Returns** [ColumnDefault](#columndefault)

#### NotNull

```rust
pub struct NotNull
```

**Constructor**

##### new()

Unnamed `NOT NULL`.

```rust
pub fn new() -> Self
```

**Returns** [NotNull](#notnull)

##### named(name)

Named `NOT NULL`.

```rust
pub fn named(name: impl Into<String>) -> Self
```

- `name` — constraint name

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
pub fn new(name: impl Into<String>, columns: &[&str]) -> Self
```

- `name` — constraint name
- `columns` — key columns

**Returns** [PrimaryKey](#primarykey)

#### ForeignKey

```rust
pub struct ForeignKey
```

**Constructor**

##### new(name, columns, references_table, references_columns)

Named foreign key.

```rust
pub fn new(name: impl Into<String>, columns: &[&str], references_table: impl Into<String>, references_columns: &[&str]) -> Self
```

- `name` — constraint name
- `columns` — local columns
- `references_table` — referenced table
- `references_columns` — referenced columns

**Returns** [ForeignKey](#foreignkey)

##### with_on_delete(self, action)

Sets `ON DELETE`.

```rust
pub fn with_on_delete(self, action: ReferentialAction) -> Self
```

- `action` — [ReferentialAction](#referentialaction)

**Returns** [ForeignKey](#foreignkey)

##### with_on_update(self, action)

Sets `ON UPDATE`.

```rust
pub fn with_on_update(self, action: ReferentialAction) -> Self
```

- `action` — [ReferentialAction](#referentialaction)

**Returns** [ForeignKey](#foreignkey)

#### UniqueConstraint

```rust
pub struct UniqueConstraint
```

**Constructor**

##### new(name, columns)

Named unique constraint.

```rust
pub fn new(name: impl Into<String>, columns: &[&str]) -> Self
```

- `name` — constraint name
- `columns` — unique columns

**Returns** [UniqueConstraint](#uniqueconstraint)

#### CheckConstraint

```rust
pub struct CheckConstraint
```

**Constructor**

##### new(name, expression)

Named check constraint.

```rust
pub fn new(name: impl Into<String>, expression: impl Into<String>) -> Self
```

- `name` — constraint name
- `expression` — raw SQL check fragment

**Returns** [CheckConstraint](#checkconstraint)

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

Compiled form is `CREATE TABLE table (name TYPE [CONSTRAINT n DEFAULT v] [CONSTRAINT n NOT NULL], …, CONSTRAINT name …)`. Table and column names are interpolated as given. Default and check values are raw SQL fragments. Indexes use [CreateIndexStatement](./create-index-statement.md). [Command](./command.md) uses `GetText()` / `getText()` / `get_text()` only.

## Examples

### Compile a CREATE TABLE

::: example
```csharp csharp/Datasources.Tests/CoreTests.cs#L162-L163
```
```rust rust/datasources/src/tests/mod.rs#L947-L954
```
```typescript typescript/datasources/src/builders/builders.test.ts#L239-L243
```

```output [After: create table]
CREATE TABLE users (id INTEGER, name TEXT)
```
:::

### Compile named constraints

::: example
```csharp csharp/Datasources.Tests/CoreTests.cs#L164-L177
```
```rust rust/datasources/src/tests/mod.rs#L955-L973
```
```typescript typescript/datasources/src/builders/builders.test.ts#L244-L258
```

```output [After: named constraints]
CREATE TABLE orders (id INTEGER CONSTRAINT df_id DEFAULT 0 CONSTRAINT nn_id NOT NULL, tenant_id INTEGER, user_id INTEGER, qty INTEGER, CONSTRAINT pk_orders PRIMARY KEY (id, tenant_id), CONSTRAINT fk_orders_user FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE, CONSTRAINT uq_orders_user UNIQUE (user_id), CONSTRAINT chk_orders_qty CHECK (qty > 0))
```
:::
