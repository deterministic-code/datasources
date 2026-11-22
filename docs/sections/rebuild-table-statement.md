# Rebuild Table Statement

A compiled copy-and-swap of a table: create `{table}__new`, copy rows, then put the new table on the live name. SQLite drops the old table and renames. MySQL copies, then `RENAME TABLE` swaps both names in one command.

## Classes

::: reference
### typescript
#### IRebuildTableStatement

```typescript
interface IRebuildTableStatement extends IStatement {}
```

#### RebuildTableStatement

```typescript
class RebuildTableStatement implements IRebuildTableStatement
```

**Constructor**

##### RebuildTableStatement(table, create, columns, recreate?)

Compiles a table rebuild.

```typescript
constructor(table: string, create: CreateTableStatement, columns: RebuildColumn[], recreate?: IStatement[])
```

- `table` — existing table name
- `create` — [CreateTableStatement](./create-table-statement.md) of the final schema
- `columns` — [RebuildColumn](#rebuildcolumn) copy list
- `recreate` — [CreateIndexStatement](./create-index-statement.md) and CreateTriggerStatement values to run after SQLite's rename, or on `{table}__new` before MySQL's `RENAME TABLE`

**Methods**

##### accept(visitor)

Dispatches to a statement visitor that renders command text.

```typescript
accept(visitor: IStatementVisitor): string
```

- `visitor` — [IStatementVisitor](./statements.md#istatementvisitor)

**Returns** `string`

#### RebuildColumn

```typescript
class RebuildColumn
```

**Constructor**

##### RebuildColumn(name, source?, cast?, literal?)

One destination column in the copy.

```typescript
constructor(name: string, source?: string, cast?: string, literal?: string)
```

- `name` — destination column
- `source` — existing column to copy; omitted selects `literal` or `NULL`
- `cast` — optional `CAST` target type
- `literal` — raw SQL for a new column

### csharp
#### IRebuildTableStatement

```csharp
public interface IRebuildTableStatement : IStatement
```

#### RebuildTableStatement

```csharp
public sealed record RebuildTableStatement : IRebuildTableStatement
```

**Constructor**

##### RebuildTableStatement(table, create, columns, recreate?)

Compiles a table rebuild.

```csharp
public RebuildTableStatement(string table, CreateTableStatement create, IReadOnlyList<RebuildColumn> columns, IReadOnlyList<IStatement>? recreate = null)
```

- `table` — existing table name
- `create` — [CreateTableStatement](./create-table-statement.md) of the final schema
- `columns` — [RebuildColumn](#rebuildcolumn) copy list
- `recreate` — [CreateIndexStatement](./create-index-statement.md) and CreateTriggerStatement values to run after SQLite's rename, or on `{table}__new` before MySQL's `RENAME TABLE`

**Methods**

##### Accept(visitor)

Dispatches to a statement visitor that renders command text.

```csharp
public string Accept(IStatementVisitor visitor)
```

- `visitor` — [IStatementVisitor](./statements.md#istatementvisitor)

**Returns** `string`

#### RebuildColumn

```csharp
public sealed record RebuildColumn
```

**Constructor**

##### RebuildColumn(name, source?, cast?, literal?)

One destination column in the copy.

```csharp
public RebuildColumn(string name, string? source = null, string? cast = null, string? literal = null)
```

- `name` — destination column
- `source` — existing column to copy; omitted selects `literal` or `NULL`
- `cast` — optional `CAST` target type
- `literal` — raw SQL for a new column

### rust
#### RebuildTableStatement

```rust
pub struct RebuildTableStatement
```

**Constructor**

##### new(table, create, columns, indexes, triggers)

Compiles a table rebuild.

```rust
pub fn new(table: impl Into<String>, create: CreateTableStatement, columns: Vec<RebuildColumn>, indexes: Vec<CreateIndexStatement>, triggers: Vec<CreateTriggerStatement>) -> Self
```

- `table` — existing table name
- `create` — [CreateTableStatement](./create-table-statement.md) of the final schema
- `columns` — [RebuildColumn](#rebuildcolumn) copy list
- `indexes` — [CreateIndexStatement](./create-index-statement.md) values to recreate
- `triggers` — CreateTriggerStatement values to recreate

**Returns** [RebuildTableStatement](#rebuildtablestatement)

**Methods**

##### accept()

Dispatches through the renderer's statement visitor.

```rust
fn accept(&self, text: &StatementTextRenderer) -> String
```

- `text` — [StatementTextRenderer](./statements.md#statementtextrenderer)

**Returns** `String`

#### RebuildColumn

```rust
pub struct RebuildColumn
```
:::

## Remarks

One statement is one script of SQL commands joined with `;\n`. SQLite creates `{table}__new` first, copies with `CAST` where the type changed, drops the old table, then renames. Views stay; their SQL still names the original table. {{Execute|execute|execute_statement}} of this statement on SQLite turns `PRAGMA foreign_keys` off, runs the script in a transaction, then `PRAGMA foreign_key_check` and `PRAGMA integrity_check` before commit.

MySQL keeps {{AlterTableStatement|AlterTableStatement|AlterTableStatement}} `MODIFY COLUMN` unless you construct this statement or set `datasource.rebuild_table` to include `mysql`. MySQL copies into `{table}__new`, builds indexes and triggers there, then `RENAME TABLE {table} TO {table}__old, {table}__new TO {table}` and drops `{table}__old`. Each of those commands commits. `SET FOREIGN_KEY_CHECKS` is off around the swap.

Schema diff on SQLite emits this statement for a column type, null, or default change, and for a foreign-key add, drop, or retarget. Postgres, SQL Server, and Oracle keep `ALTER TABLE`.

## Examples

### Compile a SQLite table rebuild

::: example
```csharp csharp/Providers/Sqlite.Tests/SqliteStatementRendererTests.cs#RendersASqliteTableRebuild
```
```rust rust/providers/sqlite/src/statement_renderer.rs#renders_a_sqlite_table_rebuild
```
```typescript typescript/providers/sqlite/src/SqliteStatementRenderer.test.ts#renders-a-sqlite-table-rebuild
```
:::
