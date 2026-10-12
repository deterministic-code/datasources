<!-- Generated from docs/. Edit the markdown in docs/ and run npm run docs:skill. -->

# Other statements

## Create Table Statement

`CREATE TABLE` text, or a compiled `CREATE TABLE` from a table, columns, and optional named table constraints.

### Classes

##### TypeScript

##### ICreateTableStatement {#typescript-icreatetablestatement}

```typescript
interface ICreateTableStatement extends IStatement {}
```

##### CreateTableStatement {#typescript-createtablestatement}

```typescript
class CreateTableStatement implements ICreateTableStatement
```

**Constructor**

| Name | Description |
| --- | --- |
| [CreateTableStatement(text)](#typescript-createtablestatement-createtablestatement-text) | Wraps create-table text. |
| [CreateTableStatement(table, columns, constraints?)](#typescript-createtablestatement-createtablestatement-table-columns-constraints) | Compiles a `CREATE TABLE`. |

**Properties**

| Name | Description |
| --- | --- |
| table | Table name when compiled from columns. Absent for wrapped text. |
| columns | Columns on a compiled table. Empty for wrapped text. |
| constraints | Named table constraints on a compiled table. Empty for wrapped text. |

**Methods**

| Name | Description |
| --- | --- |
| [accept(visitor)](#typescript-createtablestatement-accept-visitor) | Dispatches to a statement visitor that renders command text. |

###### CreateTableStatement(text) {#typescript-createtablestatement-createtablestatement-text}

Wraps create-table text.

**Definition**

```typescript
constructor(text: string)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |

###### CreateTableStatement(table, columns, constraints?) {#typescript-createtablestatement-createtablestatement-table-columns-constraints}

Compiles a `CREATE TABLE`.

**Definition**

```typescript
constructor(table: string, columns: Column[], constraints?: ITableConstraint[])
```

**Parameters**

| Name | Description |
| --- | --- |
| `table` | table name |
| `columns` | [Column](#typescript-column) names, types, optional [ColumnDefault](#typescript-columndefault) and [NotNull](#typescript-notnull) |
| `constraints` | optional named [ITableConstraint](#typescript-itableconstraint) list |

###### accept(visitor) {#typescript-createtablestatement-accept-visitor}

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

##### Column {#typescript-column}

```typescript
interface Column
```

**Properties**

| Name | Description |
| --- | --- |
| name | Column name. |
| type | SQL type name. |
| default | Optional [ColumnDefault](#typescript-columndefault). |
| notNull | Optional [NotNull](#typescript-notnull). |
| identity | When true, MySQL and Postgres commands render an identity clause. `DefaultStatementTextRenderer` leaves it off. |

##### ColumnDefault {#typescript-columndefault}

```typescript
class ColumnDefault
```

**Constructor**

| Name | Description |
| --- | --- |
| [ColumnDefault(value, name?)](#typescript-columndefault-columndefault-value-name) | Default value, optionally named. |

**Properties**

| Name | Description |
| --- | --- |
| value | Raw SQL default fragment. |
| name | Optional constraint name. |

###### ColumnDefault(value, name?) {#typescript-columndefault-columndefault-value-name}

Default value, optionally named.

**Definition**

```typescript
constructor(value: string, name?: string)
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | raw SQL default fragment |
| `name` | optional constraint name |

##### NotNull {#typescript-notnull}

```typescript
class NotNull
```

**Constructor**

| Name | Description |
| --- | --- |
| [NotNull(name?)](#typescript-notnull-notnull-name) | `NOT NULL`, optionally named. |

**Properties**

| Name | Description |
| --- | --- |
| name | Optional constraint name. |

###### NotNull(name?) {#typescript-notnull-notnull-name}

`NOT NULL`, optionally named.

**Definition**

```typescript
constructor(name?: string)
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | optional constraint name |

##### ITableConstraint {#typescript-itableconstraint}

```typescript
interface ITableConstraint
```

**Properties**

| Name | Description |
| --- | --- |
| kind | Constraint kind. |
| name | Constraint name. |

##### PrimaryKey {#typescript-primarykey}

```typescript
class PrimaryKey implements ITableConstraint
```

**Constructor**

| Name | Description |
| --- | --- |
| [PrimaryKey(name, columns)](#typescript-primarykey-primarykey-name-columns) | Named primary key. Multiple columns are a composite key. |

###### PrimaryKey(name, columns) {#typescript-primarykey-primarykey-name-columns}

Named primary key. Multiple columns are a composite key.

**Definition**

```typescript
constructor(name: string, columns: string[])
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | constraint name |
| `columns` | key columns |

##### ForeignKey {#typescript-foreignkey}

```typescript
class ForeignKey implements ITableConstraint
```

**Constructor**

| Name | Description |
| --- | --- |
| [ForeignKey(name, columns, referencesTable, referencesColumns, onDelete?, onUpdate?)](#typescript-foreignkey-foreignkey-name-columns-referencestable-referencescolumns-ondelete-onupdate) | Named foreign key. |

###### ForeignKey(name, columns, referencesTable, referencesColumns, onDelete?, onUpdate?) {#typescript-foreignkey-foreignkey-name-columns-referencestable-referencescolumns-ondelete-onupdate}

Named foreign key.

**Definition**

```typescript
constructor(name: string, columns: string[], referencesTable: string, referencesColumns: string[], onDelete?: ReferentialAction, onUpdate?: ReferentialAction)
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | constraint name |
| `columns` | local columns |
| `referencesTable` | referenced table |
| `referencesColumns` | referenced columns |
| `onDelete` | optional [ReferentialAction](#typescript-referentialaction) |
| `onUpdate` | optional [ReferentialAction](#typescript-referentialaction) |

##### UniqueConstraint {#typescript-uniqueconstraint}

```typescript
class UniqueConstraint implements ITableConstraint
```

**Constructor**

| Name | Description |
| --- | --- |
| [UniqueConstraint(name, columns)](#typescript-uniqueconstraint-uniqueconstraint-name-columns) | Named unique constraint. |

###### UniqueConstraint(name, columns) {#typescript-uniqueconstraint-uniqueconstraint-name-columns}

Named unique constraint.

**Definition**

```typescript
constructor(name: string, columns: string[])
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | constraint name |
| `columns` | unique columns |

##### CheckConstraint {#typescript-checkconstraint}

```typescript
class CheckConstraint implements ITableConstraint
```

**Constructor**

| Name | Description |
| --- | --- |
| [CheckConstraint(name, expression)](#typescript-checkconstraint-checkconstraint-name-expression) | Named check constraint. |

###### CheckConstraint(name, expression) {#typescript-checkconstraint-checkconstraint-name-expression}

Named check constraint.

**Definition**

```typescript
constructor(name: string, expression: string)
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | constraint name |
| `expression` | raw SQL check fragment |

##### ReferentialAction {#typescript-referentialaction}

```typescript
const ReferentialAction
```

**Properties**

| Name | Description |
| --- | --- |
| Cascade | `ON DELETE` / `ON UPDATE CASCADE`. |
| SetNull | `ON DELETE` / `ON UPDATE SET NULL`. |
| SetDefault | `ON DELETE` / `ON UPDATE SET DEFAULT`. |
| Restrict | `ON DELETE` / `ON UPDATE RESTRICT`. |
| NoAction | `ON DELETE` / `ON UPDATE NO ACTION`. |

##### C#

##### ICreateTableStatement {#csharp-icreatetablestatement}

```csharp
public interface ICreateTableStatement : IStatement
```

##### CreateTableStatement {#csharp-createtablestatement}

```csharp
public sealed record CreateTableStatement : ICreateTableStatement
```

**Constructor**

| Name | Description |
| --- | --- |
| [CreateTableStatement(text)](#csharp-createtablestatement-createtablestatement-text) | Wraps create-table text. |
| [CreateTableStatement(table, columns, constraints?)](#csharp-createtablestatement-createtablestatement-table-columns-constraints) | Compiles a `CREATE TABLE`. |

**Properties**

| Name | Description |
| --- | --- |
| Table | Table name when compiled from columns. Absent for wrapped text. |
| Columns | Columns on a compiled table. Empty for wrapped text. |
| Constraints | Named table constraints on a compiled table. Empty for wrapped text. |

**Methods**

| Name | Description |
| --- | --- |
| [Accept(visitor)](#csharp-createtablestatement-accept-visitor) | Dispatches to a statement visitor that renders command text. |

###### CreateTableStatement(text) {#csharp-createtablestatement-createtablestatement-text}

Wraps create-table text.

**Definition**

```csharp
public CreateTableStatement(string text)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |

###### CreateTableStatement(table, columns, constraints?) {#csharp-createtablestatement-createtablestatement-table-columns-constraints}

Compiles a `CREATE TABLE`.

**Definition**

```csharp
public CreateTableStatement(string table, IReadOnlyList<Column> columns, IReadOnlyList<ITableConstraint>? constraints = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `table` | table name |
| `columns` | [Column](#csharp-column) names, types, optional [ColumnDefault](#csharp-columndefault) and [NotNull](#csharp-notnull) |
| `constraints` | optional named [ITableConstraint](#csharp-itableconstraint) list |

###### Accept(visitor) {#csharp-createtablestatement-accept-visitor}

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

##### Column {#csharp-column}

```csharp
public sealed record Column(string Name, string Type, ColumnDefault? Default = null, NotNull? NotNull = null, bool Identity = false)
```

**Properties**

| Name | Description |
| --- | --- |
| Name | Column name. |
| Type | SQL type name. |
| Default | Optional [ColumnDefault](#csharp-columndefault). |
| NotNull | Optional [NotNull](#csharp-notnull). |
| Identity | When true, MySQL and Postgres commands render an identity clause. `DefaultStatementTextRenderer` leaves it off. |

##### ColumnDefault {#csharp-columndefault}

```csharp
public sealed record ColumnDefault(string Value, string? Name = null)
```

**Properties**

| Name | Description |
| --- | --- |
| Value | Raw SQL default fragment. |
| Name | Optional constraint name. |

##### NotNull {#csharp-notnull}

```csharp
public sealed record NotNull(string? Name = null)
```

**Properties**

| Name | Description |
| --- | --- |
| Name | Optional constraint name. |

##### ITableConstraint {#csharp-itableconstraint}

```csharp
public interface ITableConstraint
```

##### PrimaryKey {#csharp-primarykey}

```csharp
public sealed record PrimaryKey(string Name, IReadOnlyList<string> Columns) : ITableConstraint
```

**Properties**

| Name | Description |
| --- | --- |
| Name | Constraint name. |
| Columns | Key columns. Multiple columns are a composite key. |

##### ForeignKey {#csharp-foreignkey}

```csharp
public sealed record ForeignKey(string Name, IReadOnlyList<string> Columns, string ReferencesTable, IReadOnlyList<string> ReferencesColumns, ReferentialAction? OnDelete = null, ReferentialAction? OnUpdate = null) : ITableConstraint
```

**Properties**

| Name | Description |
| --- | --- |
| Name | Constraint name. |
| Columns | Local columns. |
| ReferencesTable | Referenced table. |
| ReferencesColumns | Referenced columns. |
| OnDelete | Optional [ReferentialAction](#csharp-referentialaction). |
| OnUpdate | Optional [ReferentialAction](#csharp-referentialaction). |

##### UniqueConstraint {#csharp-uniqueconstraint}

```csharp
public sealed record UniqueConstraint(string Name, IReadOnlyList<string> Columns) : ITableConstraint
```

**Properties**

| Name | Description |
| --- | --- |
| Name | Constraint name. |
| Columns | Unique columns. |

##### CheckConstraint {#csharp-checkconstraint}

```csharp
public sealed record CheckConstraint(string Name, string Expression) : ITableConstraint
```

**Properties**

| Name | Description |
| --- | --- |
| Name | Constraint name. |
| Expression | Raw SQL check fragment. |

##### ReferentialAction {#csharp-referentialaction}

```csharp
public enum ReferentialAction
```

**Properties**

| Name | Description |
| --- | --- |
| Cascade | `ON DELETE` / `ON UPDATE CASCADE`. |
| SetNull | `ON DELETE` / `ON UPDATE SET NULL`. |
| SetDefault | `ON DELETE` / `ON UPDATE SET DEFAULT`. |
| Restrict | `ON DELETE` / `ON UPDATE RESTRICT`. |
| NoAction | `ON DELETE` / `ON UPDATE NO ACTION`. |

##### Rust

##### CreateTableStatement {#rust-createtablestatement}

```rust
pub struct CreateTableStatement
```

**Constructor**

| Name | Description |
| --- | --- |
| [text(text)](#rust-createtablestatement-text-text) | Wraps create-table text. |
| [new(table, columns, constraints)](#rust-createtablestatement-new-table-columns-constraints) | Compiles a `CREATE TABLE`. |

**Properties**

| Name | Description |
| --- | --- |
| table | Table name when compiled from columns. Absent for wrapped text. |
| columns | Columns on a compiled table. Empty for wrapped text. |
| constraints | Named table constraints on a compiled table. Empty for wrapped text. |

**Methods**

| Name | Description |
| --- | --- |
| [accept()](#rust-createtablestatement-accept) | Dispatches through the renderer's statement visitor. |

###### text(text) {#rust-createtablestatement-text-text}

Wraps create-table text.

**Definition**

```rust
pub fn text(text: impl Into<String>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |

**Returns**

[CreateTableStatement](#rust-createtablestatement)

###### new(table, columns, constraints) {#rust-createtablestatement-new-table-columns-constraints}

Compiles a `CREATE TABLE`.

**Definition**

```rust
pub fn new(table: impl AsRef<str>, columns: &[Column], constraints: &[TableConstraint]) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `table` | table name |
| `columns` | [Column](#rust-column) names, types, optional [ColumnDefault](#rust-columndefault) and [NotNull](#rust-notnull) |
| `constraints` | named [TableConstraint](#rust-tableconstraint) list (empty when omitted in C# / TypeScript) |

**Returns**

[CreateTableStatement](#rust-createtablestatement)

###### accept() {#rust-createtablestatement-accept}

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

##### Column {#rust-column}

```rust
pub struct Column
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(name, type_name)](#rust-column-new-name-type_name) | Column name and SQL type. |
| [default(self, default)](#rust-column-default-self-default) | Sets an optional [ColumnDefault](#rust-columndefault). |
| [not_null(self, not_null)](#rust-column-not_null-self-not_null) | Sets an optional [NotNull](#rust-notnull). |
| [identity(self)](#rust-column-identity-self) | Marks the column as an identity. `DefaultStatementTextRenderer` leaves the clause off. MySQL and Postgres commands render it. |

###### new(name, type_name) {#rust-column-new-name-type_name}

Column name and SQL type.

**Definition**

```rust
pub fn new(name: impl Into<String>, type_name: impl Into<String>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | column name |
| `type_name` | SQL type name |

**Returns**

[Column](#rust-column)

###### default(self, default) {#rust-column-default-self-default}

Sets an optional [ColumnDefault](#rust-columndefault).

**Definition**

```rust
pub fn default(self, default: ColumnDefault) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `default` | [ColumnDefault](#rust-columndefault) |

**Returns**

[Column](#rust-column)

###### not_null(self, not_null) {#rust-column-not_null-self-not_null}

Sets an optional [NotNull](#rust-notnull).

**Definition**

```rust
pub fn not_null(self, not_null: NotNull) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `not_null` | [NotNull](#rust-notnull) |

**Returns**

[Column](#rust-column)

###### identity(self) {#rust-column-identity-self}

Marks the column as an identity. `DefaultStatementTextRenderer` leaves the clause off. MySQL and Postgres commands render it.

**Definition**

```rust
pub fn identity(self) -> Self
```

**Returns**

[Column](#rust-column)

##### ColumnDefault {#rust-columndefault}

```rust
pub struct ColumnDefault
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(value, name)](#rust-columndefault-new-value-name) | Default value, optionally named. |

###### new(value, name) {#rust-columndefault-new-value-name}

Default value, optionally named.

**Definition**

```rust
pub fn new(value: impl Into<String>, name: Option<&str>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | raw SQL default fragment |
| `name` | optional constraint name |

**Returns**

[ColumnDefault](#rust-columndefault)

##### NotNull {#rust-notnull}

```rust
pub struct NotNull
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(name)](#rust-notnull-new-name) | `NOT NULL`, optionally named. |

###### new(name) {#rust-notnull-new-name}

`NOT NULL`, optionally named.

**Definition**

```rust
pub fn new(name: Option<&str>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | optional constraint name |

**Returns**

[NotNull](#rust-notnull)

##### TableConstraint {#rust-tableconstraint}

```rust
pub enum TableConstraint
```

##### PrimaryKey {#rust-primarykey}

```rust
pub struct PrimaryKey
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(name, columns)](#rust-primarykey-new-name-columns) | Named primary key. Multiple columns are a composite key. |

###### new(name, columns) {#rust-primarykey-new-name-columns}

Named primary key. Multiple columns are a composite key.

**Definition**

```rust
pub fn new(name: impl Into<String>, columns: &[&str]) -> TableConstraint
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | constraint name |
| `columns` | key columns |

**Returns**

[TableConstraint](#rust-tableconstraint)

##### ForeignKey {#rust-foreignkey}

```rust
pub struct ForeignKey
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(name, columns, references_table, references_columns, on_delete, on_update)](#rust-foreignkey-new-name-columns-references_table-references_columns-on_delete-on_update) | Named foreign key. |

###### new(name, columns, references_table, references_columns, on_delete, on_update) {#rust-foreignkey-new-name-columns-references_table-references_columns-on_delete-on_update}

Named foreign key.

**Definition**

```rust
pub fn new(name: impl Into<String>, columns: &[&str], references_table: impl Into<String>, references_columns: &[&str], on_delete: Option<ReferentialAction>, on_update: Option<ReferentialAction>) -> TableConstraint
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | constraint name |
| `columns` | local columns |
| `references_table` | referenced table |
| `references_columns` | referenced columns |
| `on_delete` | optional [ReferentialAction](#rust-referentialaction) |
| `on_update` | optional [ReferentialAction](#rust-referentialaction) |

**Returns**

[TableConstraint](#rust-tableconstraint)

##### UniqueConstraint {#rust-uniqueconstraint}

```rust
pub struct UniqueConstraint
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(name, columns)](#rust-uniqueconstraint-new-name-columns) | Named unique constraint. |

###### new(name, columns) {#rust-uniqueconstraint-new-name-columns}

Named unique constraint.

**Definition**

```rust
pub fn new(name: impl Into<String>, columns: &[&str]) -> TableConstraint
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | constraint name |
| `columns` | unique columns |

**Returns**

[TableConstraint](#rust-tableconstraint)

##### CheckConstraint {#rust-checkconstraint}

```rust
pub struct CheckConstraint
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(name, expression)](#rust-checkconstraint-new-name-expression) | Named check constraint. |

###### new(name, expression) {#rust-checkconstraint-new-name-expression}

Named check constraint.

**Definition**

```rust
pub fn new(name: impl Into<String>, expression: impl Into<String>) -> TableConstraint
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | constraint name |
| `expression` | raw SQL check fragment |

**Returns**

[TableConstraint](#rust-tableconstraint)

##### ReferentialAction {#rust-referentialaction}

```rust
pub enum ReferentialAction
```

**Properties**

| Name | Description |
| --- | --- |
| Cascade | `ON DELETE` / `ON UPDATE CASCADE`. |
| SetNull | `ON DELETE` / `ON UPDATE SET NULL`. |
| SetDefault | `ON DELETE` / `ON UPDATE SET DEFAULT`. |
| Restrict | `ON DELETE` / `ON UPDATE RESTRICT`. |
| NoAction | `ON DELETE` / `ON UPDATE NO ACTION`. |

### Remarks

A compiled statement stores the table, columns, and constraints. `StatementTextRenderer.Render` / `StatementTextRenderer.render` / `StatementTextRenderer::render` renders `CREATE TABLE table (name TYPE [CONSTRAINT n DEFAULT v] [CONSTRAINT n NOT NULL], …, CONSTRAINT name …)`. Table and column names are interpolated as given. Default and check values are raw SQL fragments. Wrapped text has no table. Indexes use [CreateIndexStatement](#). `StatementTextRenderer.Render` / `StatementTextRenderer.render` / `StatementTextRenderer::render` uses `CreateTableStatementTextRenderer`. A provider subclasses that renderer and can replace `Render` / `render` / `render` or override type, default, and identity. SQLite, MySQL, and Postgres override type so canonical names become engine types. MySQL and Postgres also override default and identity. A Postgres `bool` default goes through the bool type converter, so `1` becomes `TRUE`. Identity renders `AUTO_INCREMENT` or `GENERATED BY DEFAULT AS IDENTITY`. SQLite leaves identity empty. `StatementTextRenderer.Render` / `StatementTextRenderer.render` / `StatementTextRenderer::render` leaves those changes off. Other commands use the rendered text.

### Examples

#### Compile a CREATE TABLE

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/StatementCompileTests.cs#L127-L132)

```csharp
var createTable = new CreateTableStatement("users", [new Column("id", "INTEGER"), new Column("name", "TEXT")]);
```

**After: create table**

```
CREATE TABLE users (id INTEGER, name TEXT)
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/statement_compile.rs#L161-L172)

```rust
let create_table = CreateTableStatement::new(
    "users",
    &[Column::new("id", "INTEGER"), Column::new("name", "TEXT")],
    &[],
);
```

**After: create table**

```
CREATE TABLE users (id INTEGER, name TEXT)
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/statements/StatementCompile.test.ts#L143-L151)

```typescript
const createTable = new CreateTableStatement("users", [
  { name: "id", type: "INTEGER" },
  { name: "name", type: "TEXT" },
]);
```

**After: create table**

```
CREATE TABLE users (id INTEGER, name TEXT)
```

#### Compile named constraints

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/StatementCompileTests.cs#L134-L154)

```csharp
var namedTable = new CreateTableStatement(
    "orders",
    [
        new Column("id", "INTEGER", new ColumnDefault("0", "df_id"), new NotNull("nn_id")),
        new Column("tenant_id", "INTEGER"),
        new Column("user_id", "INTEGER"),
        new Column("qty", "INTEGER")
    ],
    [
        new PrimaryKey("pk_orders", ["id", "tenant_id"]),
        new ForeignKey("fk_orders_user", ["user_id"], "users", ["id"], ReferentialAction.Cascade),
        new UniqueConstraint("uq_orders_user", ["user_id"]),
        new CheckConstraint("chk_orders_qty", "qty > 0")
    ]);
```

**After: named constraints**

```
CREATE TABLE orders (id INTEGER CONSTRAINT df_id DEFAULT 0 CONSTRAINT nn_id NOT NULL, tenant_id INTEGER, user_id INTEGER, qty INTEGER, CONSTRAINT pk_orders PRIMARY KEY (id, tenant_id), CONSTRAINT fk_orders_user FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE, CONSTRAINT uq_orders_user UNIQUE (user_id), CONSTRAINT chk_orders_qty CHECK (qty > 0))
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/statement_compile.rs#L174-L204)

```rust
let named_table = CreateTableStatement::new(
    "orders",
    &[
        Column::new("id", "INTEGER")
            .default(ColumnDefault::new("0", Some("df_id")))
            .not_null(NotNull::new(Some("nn_id"))),
        Column::new("tenant_id", "INTEGER"),
        Column::new("user_id", "INTEGER"),
        Column::new("qty", "INTEGER"),
    ],
    &[
        PrimaryKey::new("pk_orders", &["id", "tenant_id"]),
        ForeignKey::new(
            "fk_orders_user",
            &["user_id"],
            "users",
            &["id"],
            Some(ReferentialAction::Cascade),
            None,
        ),
        UniqueConstraint::new("uq_orders_user", &["user_id"]),
        CheckConstraint::new("chk_orders_qty", "qty > 0"),
    ],
);
```

**After: named constraints**

```
CREATE TABLE orders (id INTEGER CONSTRAINT df_id DEFAULT 0 CONSTRAINT nn_id NOT NULL, tenant_id INTEGER, user_id INTEGER, qty INTEGER, CONSTRAINT pk_orders PRIMARY KEY (id, tenant_id), CONSTRAINT fk_orders_user FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE, CONSTRAINT uq_orders_user UNIQUE (user_id), CONSTRAINT chk_orders_qty CHECK (qty > 0))
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/statements/StatementCompile.test.ts#L153-L172)

```typescript
const namedTable = new CreateTableStatement(
  "orders",
  [
    { name: "id", type: "INTEGER", default: new ColumnDefault("0", "df_id"), notNull: new NotNull("nn_id") },
    { name: "tenant_id", type: "INTEGER" },
    { name: "user_id", type: "INTEGER" },
    { name: "qty", type: "INTEGER" },
  ],
  [
    new PrimaryKey("pk_orders", ["id", "tenant_id"]),
    new ForeignKey("fk_orders_user", ["user_id"], "users", ["id"], ReferentialAction.Cascade),
    new UniqueConstraint("uq_orders_user", ["user_id"]),
    new CheckConstraint("chk_orders_qty", "qty > 0"),
  ],
);
```

**After: named constraints**

```
CREATE TABLE orders (id INTEGER CONSTRAINT df_id DEFAULT 0 CONSTRAINT nn_id NOT NULL, tenant_id INTEGER, user_id INTEGER, qty INTEGER, CONSTRAINT pk_orders PRIMARY KEY (id, tenant_id), CONSTRAINT fk_orders_user FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE, CONSTRAINT uq_orders_user UNIQUE (user_id), CONSTRAINT chk_orders_qty CHECK (qty > 0))
```

## Create Index Statement

`CREATE INDEX` text, or a compiled `CREATE [UNIQUE] INDEX` from a name, table, and columns.

### Classes

##### TypeScript

##### ICreateIndexStatement {#typescript-icreateindexstatement}

```typescript
interface ICreateIndexStatement extends IStatement {}
```

##### CreateIndexStatement {#typescript-createindexstatement}

```typescript
class CreateIndexStatement implements ICreateIndexStatement
```

**Constructor**

| Name | Description |
| --- | --- |
| [CreateIndexStatement(text)](#typescript-createindexstatement-createindexstatement-text) | Wraps create-index text. |
| [CreateIndexStatement(name, table, columns, unique?)](#typescript-createindexstatement-createindexstatement-name-table-columns-unique) | Compiles a `CREATE INDEX`. |

**Methods**

| Name | Description |
| --- | --- |
| [accept(visitor)](#typescript-createindexstatement-accept-visitor) | Dispatches to a statement visitor that renders command text. |

###### CreateIndexStatement(text) {#typescript-createindexstatement-createindexstatement-text}

Wraps create-index text.

**Definition**

```typescript
constructor(text: string)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |

###### CreateIndexStatement(name, table, columns, unique?) {#typescript-createindexstatement-createindexstatement-name-table-columns-unique}

Compiles a `CREATE INDEX`.

**Definition**

```typescript
constructor(name: string, table: string, columns: string[], unique?: boolean)
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | index name |
| `table` | table name |
| `columns` | indexed columns |
| `unique` | `CREATE UNIQUE INDEX` when true |

###### accept(visitor) {#typescript-createindexstatement-accept-visitor}

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

##### C#

##### ICreateIndexStatement {#csharp-icreateindexstatement}

```csharp
public interface ICreateIndexStatement : IStatement
```

##### CreateIndexStatement {#csharp-createindexstatement}

```csharp
public sealed record CreateIndexStatement : ICreateIndexStatement
```

**Constructor**

| Name | Description |
| --- | --- |
| [CreateIndexStatement(text)](#csharp-createindexstatement-createindexstatement-text) | Wraps create-index text. |
| [CreateIndexStatement(name, table, columns, unique?)](#csharp-createindexstatement-createindexstatement-name-table-columns-unique) | Compiles a `CREATE INDEX`. |

**Methods**

| Name | Description |
| --- | --- |
| [Accept(visitor)](#csharp-createindexstatement-accept-visitor) | Dispatches to a statement visitor that renders command text. |

###### CreateIndexStatement(text) {#csharp-createindexstatement-createindexstatement-text}

Wraps create-index text.

**Definition**

```csharp
public CreateIndexStatement(string text)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |

###### CreateIndexStatement(name, table, columns, unique?) {#csharp-createindexstatement-createindexstatement-name-table-columns-unique}

Compiles a `CREATE INDEX`.

**Definition**

```csharp
public CreateIndexStatement(string name, string table, IReadOnlyList<string> columns, bool unique = false)
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | index name |
| `table` | table name |
| `columns` | indexed columns |
| `unique` | `CREATE UNIQUE INDEX` when true |

###### Accept(visitor) {#csharp-createindexstatement-accept-visitor}

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

##### Rust

##### CreateIndexStatement {#rust-createindexstatement}

```rust
pub struct CreateIndexStatement
```

**Constructor**

| Name | Description |
| --- | --- |
| [text(text)](#rust-createindexstatement-text-text) | Wraps create-index text. |
| [new(name, table, columns, unique)](#rust-createindexstatement-new-name-table-columns-unique) | Compiles a `CREATE INDEX`. |

**Methods**

| Name | Description |
| --- | --- |
| [accept()](#rust-createindexstatement-accept) | Dispatches through the renderer's statement visitor. |

###### text(text) {#rust-createindexstatement-text-text}

Wraps create-index text.

**Definition**

```rust
pub fn text(text: impl Into<String>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |

**Returns**

[CreateIndexStatement](#rust-createindexstatement)

###### new(name, table, columns, unique) {#rust-createindexstatement-new-name-table-columns-unique}

Compiles a `CREATE INDEX`.

**Definition**

```rust
pub fn new(name: impl AsRef<str>, table: impl AsRef<str>, columns: &[&str], unique: bool) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | index name |
| `table` | table name |
| `columns` | indexed columns |
| `unique` | `CREATE UNIQUE INDEX` when true |

**Returns**

[CreateIndexStatement](#rust-createindexstatement)

###### accept() {#rust-createindexstatement-accept}

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

### Remarks

Compiled form is `CREATE INDEX name ON table (cols)` or `CREATE UNIQUE INDEX name ON table (cols)`. Table and column names are interpolated as given. [Command](command.md) uses `StatementTextRenderer.Render` / `StatementTextRenderer.render` / `StatementTextRenderer::render` only.

### Examples

#### Compile a CREATE INDEX

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/StatementCompileTests.cs#L156-L161)

```csharp
var createIndex = new CreateIndexStatement("idx_orders_user", "orders", ["user_id"]);
```

**After: create index**

```
CREATE INDEX idx_orders_user ON orders (user_id)
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/statement_compile.rs#L206-L213)

```rust
let create_index = CreateIndexStatement::new("idx_orders_user", "orders", &["user_id"], false);
```

**After: create index**

```
CREATE INDEX idx_orders_user ON orders (user_id)
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/statements/StatementCompile.test.ts#L174-L179)

```typescript
const createIndex = new CreateIndexStatement("idx_orders_user", "orders", ["user_id"]);
```

**After: create index**

```
CREATE INDEX idx_orders_user ON orders (user_id)
```

#### Compile a CREATE UNIQUE INDEX

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/StatementCompileTests.cs#L163-L168)

```csharp
var createIndex = new CreateIndexStatement("uq_orders_code", "orders", ["code"], true);
```

**After: create unique index**

```
CREATE UNIQUE INDEX uq_orders_code ON orders (code)
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/statement_compile.rs#L215-L222)

```rust
let create_index = CreateIndexStatement::new("uq_orders_code", "orders", &["code"], true);
```

**After: create unique index**

```
CREATE UNIQUE INDEX uq_orders_code ON orders (code)
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/statements/StatementCompile.test.ts#L181-L186)

```typescript
const createIndex = new CreateIndexStatement("uq_orders_code", "orders", ["code"], true);
```

**After: create unique index**

```
CREATE UNIQUE INDEX uq_orders_code ON orders (code)
```

## Execute Stored Procedure Statement

`CALL` text, or a compiled `CALL` from a name and [Parameter](command.md#parameter) list.

### Classes

##### TypeScript

##### IExecuteStoredProcedureStatement {#typescript-iexecutestoredprocedurestatement}

```typescript
interface IExecuteStoredProcedureStatement extends IStatement {}
```

##### ExecuteStoredProcedureStatement {#typescript-executestoredprocedurestatement}

```typescript
class ExecuteStoredProcedureStatement implements IExecuteStoredProcedureStatement
```

**Constructor**

| Name | Description |
| --- | --- |
| [ExecuteStoredProcedureStatement(text)](#typescript-executestoredprocedurestatement-executestoredprocedurestatement-text) | Wraps stored-procedure call text. |
| [ExecuteStoredProcedureStatement(name, parameters)](#typescript-executestoredprocedurestatement-executestoredprocedurestatement-name-parameters) | Compiles a `CALL`. Empty [Parameter](command.md#typescript-parameter) list is `CALL name()`. [returnValue](command.md#typescript-parameterdirection) parameters are omitted from the argument list. |

**Methods**

| Name | Description |
| --- | --- |
| [accept(visitor)](#typescript-executestoredprocedurestatement-accept-visitor) | Dispatches to a statement visitor that renders command text. |

###### ExecuteStoredProcedureStatement(text) {#typescript-executestoredprocedurestatement-executestoredprocedurestatement-text}

Wraps stored-procedure call text.

**Definition**

```typescript
constructor(text: string)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |

###### ExecuteStoredProcedureStatement(name, parameters) {#typescript-executestoredprocedurestatement-executestoredprocedurestatement-name-parameters}

Compiles a `CALL`. Empty [Parameter](command.md#typescript-parameter) list is `CALL name()`. [returnValue](command.md#typescript-parameterdirection) parameters are omitted from the argument list.

**Definition**

```typescript
constructor(name: string, parameters: Parameter[])
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | procedure name |
| `parameters` | bound values (empty for `CALL name()`) |

###### accept(visitor) {#typescript-executestoredprocedurestatement-accept-visitor}

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

##### C#

##### IExecuteStoredProcedureStatement {#csharp-iexecutestoredprocedurestatement}

```csharp
public interface IExecuteStoredProcedureStatement : IStatement
```

##### ExecuteStoredProcedureStatement {#csharp-executestoredprocedurestatement}

```csharp
public sealed record ExecuteStoredProcedureStatement : IExecuteStoredProcedureStatement
```

**Constructor**

| Name | Description |
| --- | --- |
| [ExecuteStoredProcedureStatement(text)](#csharp-executestoredprocedurestatement-executestoredprocedurestatement-text) | Wraps stored-procedure call text. |
| [ExecuteStoredProcedureStatement(name, parameters)](#csharp-executestoredprocedurestatement-executestoredprocedurestatement-name-parameters) | Compiles a `CALL`. Empty [Parameter](command.md#csharp-parameter) list is `CALL name()`. [ReturnValue](command.md#csharp-parameterdirection) parameters are omitted from the argument list. |

**Methods**

| Name | Description |
| --- | --- |
| [Accept(visitor)](#csharp-executestoredprocedurestatement-accept-visitor) | Dispatches to a statement visitor that renders command text. |

###### ExecuteStoredProcedureStatement(text) {#csharp-executestoredprocedurestatement-executestoredprocedurestatement-text}

Wraps stored-procedure call text.

**Definition**

```csharp
public ExecuteStoredProcedureStatement(string text)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |

###### ExecuteStoredProcedureStatement(name, parameters) {#csharp-executestoredprocedurestatement-executestoredprocedurestatement-name-parameters}

Compiles a `CALL`. Empty [Parameter](command.md#csharp-parameter) list is `CALL name()`. [ReturnValue](command.md#csharp-parameterdirection) parameters are omitted from the argument list.

**Definition**

```csharp
public ExecuteStoredProcedureStatement(string name, IReadOnlyList<Parameter> parameters)
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | procedure name |
| `parameters` | bound values (empty for `CALL name()`) |

###### Accept(visitor) {#csharp-executestoredprocedurestatement-accept-visitor}

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

##### Rust

##### ExecuteStoredProcedureStatement {#rust-executestoredprocedurestatement}

```rust
pub struct ExecuteStoredProcedureStatement
```

**Constructor**

| Name | Description |
| --- | --- |
| [text(text)](#rust-executestoredprocedurestatement-text-text) | Wraps stored-procedure call text. |
| [new(name, parameters)](#rust-executestoredprocedurestatement-new-name-parameters) | Compiles a `CALL`. Empty [Parameter](command.md#rust-parameter) list is `CALL name()`. [ReturnValue](command.md#rust-parameterdirection) parameters are omitted from the argument list. |

**Methods**

| Name | Description |
| --- | --- |
| [accept()](#rust-executestoredprocedurestatement-accept) | Dispatches through the renderer's statement visitor. |

###### text(text) {#rust-executestoredprocedurestatement-text-text}

Wraps stored-procedure call text.

**Definition**

```rust
pub fn text(text: impl Into<String>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |

**Returns**

[ExecuteStoredProcedureStatement](#rust-executestoredprocedurestatement)

###### new(name, parameters) {#rust-executestoredprocedurestatement-new-name-parameters}

Compiles a `CALL`. Empty [Parameter](command.md#rust-parameter) list is `CALL name()`. [ReturnValue](command.md#rust-parameterdirection) parameters are omitted from the argument list.

**Definition**

```rust
pub fn new(name: impl AsRef<str>, parameters: &[Parameter]) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | procedure name |
| `parameters` | bound values (empty for `CALL name()`) |

**Returns**

[ExecuteStoredProcedureStatement](#rust-executestoredprocedurestatement)

###### accept() {#rust-executestoredprocedurestatement-accept}

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

### Remarks

Compiled form is `CALL name($p)` or `CALL name()`. [ReturnValue](command.md#parameterdirection) parameters are binds only and stay out of the argument list. The name constructor keeps the parameter list on the statement so [ReadonlyRepository](readonly-repository.md) can bind it.

### Examples

#### Compile a CALL

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/StatementCompileTests.cs#L170-L176)

```csharp
var fields = new[] { new Parameter("id", 1), new Parameter("name", "a") };
var call = new ExecuteStoredProcedureStatement("save", fields);
```

**After: call**

```
CALL save($id, $name)
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/statement_compile.rs#L224-L229)

```rust
let fields = [Parameter::new("id", 1_i64), Parameter::new("name", "a")];
let call = ExecuteStoredProcedureStatement::new("save", &fields);
```

**After: call**

```
CALL save($id, $name)
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/statements/StatementCompile.test.ts#L188-L195)

```typescript
const fields = [
  { name: "id", value: 1 },
  { name: "name", value: "a" },
];
const call = new ExecuteStoredProcedureStatement("save", fields);
```

**After: call**

```
CALL save($id, $name)
```

## Return Statement

Function-call text, or a compiled `SELECT name($p)` from a name and [Parameter](command.md#parameter) list.

### Classes

##### TypeScript

##### IReturnStatement {#typescript-ireturnstatement}

```typescript
interface IReturnStatement extends IStatement {}
```

##### ReturnStatement {#typescript-returnstatement}

```typescript
class ReturnStatement implements IReturnStatement
```

**Constructor**

| Name | Description |
| --- | --- |
| [ReturnStatement(text)](#typescript-returnstatement-returnstatement-text) | Wraps function-call text. |
| [ReturnStatement(name, parameters)](#typescript-returnstatement-returnstatement-name-parameters) | Compiles a `SELECT`. Empty [Parameter](command.md#typescript-parameter) list is `SELECT name()`. [returnValue](command.md#typescript-parameterdirection) parameters are omitted from the argument list. |

**Methods**

| Name | Description |
| --- | --- |
| [accept(visitor)](#typescript-returnstatement-accept-visitor) | Dispatches to a statement visitor that renders command text. |

###### ReturnStatement(text) {#typescript-returnstatement-returnstatement-text}

Wraps function-call text.

**Definition**

```typescript
constructor(text: string)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |

###### ReturnStatement(name, parameters) {#typescript-returnstatement-returnstatement-name-parameters}

Compiles a `SELECT`. Empty [Parameter](command.md#typescript-parameter) list is `SELECT name()`. [returnValue](command.md#typescript-parameterdirection) parameters are omitted from the argument list.

**Definition**

```typescript
constructor(name: string, parameters: Parameter[])
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | function name |
| `parameters` | bound values (empty for `SELECT name()`) |

###### accept(visitor) {#typescript-returnstatement-accept-visitor}

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

##### C#

##### IReturnStatement {#csharp-ireturnstatement}

```csharp
public interface IReturnStatement : IStatement
```

##### ReturnStatement {#csharp-returnstatement}

```csharp
public sealed record ReturnStatement : IReturnStatement
```

**Constructor**

| Name | Description |
| --- | --- |
| [ReturnStatement(text)](#csharp-returnstatement-returnstatement-text) | Wraps function-call text. |
| [ReturnStatement(name, parameters)](#csharp-returnstatement-returnstatement-name-parameters) | Compiles a `SELECT`. Empty [Parameter](command.md#csharp-parameter) list is `SELECT name()`. [ReturnValue](command.md#csharp-parameterdirection) parameters are omitted from the argument list. |

**Methods**

| Name | Description |
| --- | --- |
| [Accept(visitor)](#csharp-returnstatement-accept-visitor) | Dispatches to a statement visitor that renders command text. |

###### ReturnStatement(text) {#csharp-returnstatement-returnstatement-text}

Wraps function-call text.

**Definition**

```csharp
public ReturnStatement(string text)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |

###### ReturnStatement(name, parameters) {#csharp-returnstatement-returnstatement-name-parameters}

Compiles a `SELECT`. Empty [Parameter](command.md#csharp-parameter) list is `SELECT name()`. [ReturnValue](command.md#csharp-parameterdirection) parameters are omitted from the argument list.

**Definition**

```csharp
public ReturnStatement(string name, IReadOnlyList<Parameter> parameters)
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | function name |
| `parameters` | bound values (empty for `SELECT name()`) |

###### Accept(visitor) {#csharp-returnstatement-accept-visitor}

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

##### Rust

##### ReturnStatement {#rust-returnstatement}

```rust
pub struct ReturnStatement
```

**Constructor**

| Name | Description |
| --- | --- |
| [text(text)](#rust-returnstatement-text-text) | Wraps function-call text. |
| [new(name, parameters)](#rust-returnstatement-new-name-parameters) | Compiles a `SELECT`. Empty [Parameter](command.md#rust-parameter) list is `SELECT name()`. [ReturnValue](command.md#rust-parameterdirection) parameters are omitted from the argument list. |

**Methods**

| Name | Description |
| --- | --- |
| [accept()](#rust-returnstatement-accept) | Dispatches through the renderer's statement visitor. |

###### text(text) {#rust-returnstatement-text-text}

Wraps function-call text.

**Definition**

```rust
pub fn text(text: impl Into<String>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |

**Returns**

[ReturnStatement](#rust-returnstatement)

###### new(name, parameters) {#rust-returnstatement-new-name-parameters}

Compiles a `SELECT`. Empty [Parameter](command.md#rust-parameter) list is `SELECT name()`. [ReturnValue](command.md#rust-parameterdirection) parameters are omitted from the argument list.

**Definition**

```rust
pub fn new(name: impl AsRef<str>, parameters: &[Parameter]) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | function name |
| `parameters` | bound values (empty for `SELECT name()`) |

**Returns**

[ReturnStatement](#rust-returnstatement)

###### accept() {#rust-returnstatement-accept}

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

### Remarks

Compiled form is `SELECT name($p)` or `SELECT name()`. [ReturnValue](command.md#parameterdirection) parameters are binds only and stay out of the argument list.

### Examples

#### Compile a SELECT function call

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/StatementCompileTests.cs#L178-L184)

```csharp
var fields = new[] { new Parameter("id", 1), new Parameter("name", "a") };
var ret = new ReturnStatement("add", fields);
```

**After: return**

```
SELECT add($id, $name)
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/statement_compile.rs#L231-L236)

```rust
let fields = [Parameter::new("id", 1_i64), Parameter::new("name", "a")];
let ret = ReturnStatement::new("add", &fields);
```

**After: return**

```
SELECT add($id, $name)
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/statements/StatementCompile.test.ts#L197-L204)

```typescript
const fields = [
  { name: "id", value: 1 },
  { name: "name", value: "a" },
];
const ret = new ReturnStatement("add", fields);
```

**After: return**

```
SELECT add($id, $name)
```
