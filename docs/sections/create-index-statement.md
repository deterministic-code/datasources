# Create Index Statement

`CREATE INDEX` text, or a compiled `CREATE [UNIQUE] INDEX` from a name, table, and columns.

## Classes

::: reference
### typescript
#### ICreateIndexStatement

```typescript
interface ICreateIndexStatement extends IStatement {}
```

#### CreateIndexStatement

```typescript
class CreateIndexStatement implements ICreateIndexStatement
```

**Constructor**

##### CreateIndexStatement(text)

Wraps create-index text.

```typescript
constructor(text: string)
```

- `text` — command text

##### CreateIndexStatement(name, table, columns, unique?)

Compiles a `CREATE INDEX`.

```typescript
constructor(name: string, table: string, columns: string[], unique?: boolean)
```

- `name` — index name
- `table` — table name
- `columns` — indexed columns
- `unique` — `CREATE UNIQUE INDEX` when true

**Methods**

##### getText()

Command text.

```typescript
getText(): string
```

**Returns** `string`

### csharp
#### ICreateIndexStatement

```csharp
public interface ICreateIndexStatement : IStatement
```

#### CreateIndexStatement

```csharp
public sealed record CreateIndexStatement : ICreateIndexStatement
```

**Constructor**

##### CreateIndexStatement(text)

Wraps create-index text.

```csharp
public CreateIndexStatement(string text)
```

- `text` — command text

##### CreateIndexStatement(name, table, columns, unique?)

Compiles a `CREATE INDEX`.

```csharp
public CreateIndexStatement(string name, string table, IReadOnlyList<string> columns, bool unique = false)
```

- `name` — index name
- `table` — table name
- `columns` — indexed columns
- `unique` — `CREATE UNIQUE INDEX` when true

**Methods**

##### GetText()

Command text.

```csharp
public string GetText()
```

**Returns** `string`

### rust
#### CreateIndexStatement

```rust
pub struct CreateIndexStatement
```

**Constructor**

##### text(text)

Wraps create-index text.

```rust
pub fn text(text: impl Into<String>) -> Self
```

- `text` — command text

**Returns** [CreateIndexStatement](#createindexstatement)

##### new(name, table, columns)

Compiles a `CREATE INDEX`.

```rust
pub fn new(name: impl AsRef<str>, table: impl AsRef<str>, columns: &[&str]) -> Self
```

- `name` — index name
- `table` — table name
- `columns` — indexed columns

**Returns** [CreateIndexStatement](#createindexstatement)

##### unique(name, table, columns)

Compiles a `CREATE UNIQUE INDEX`.

```rust
pub fn unique(name: impl AsRef<str>, table: impl AsRef<str>, columns: &[&str]) -> Self
```

- `name` — index name
- `table` — table name
- `columns` — indexed columns

**Returns** [CreateIndexStatement](#createindexstatement)

**Methods**

##### get_text()

Command text.

```rust
fn get_text(&self) -> String
```

**Returns** `String`
:::

## Remarks

Compiled form is `CREATE INDEX name ON table (cols)` or `CREATE UNIQUE INDEX name ON table (cols)`. Table and column names are interpolated as given. [Command](./command.md) uses `GetText()` / `getText()` / `get_text()` only.

## Examples

### Compile a CREATE INDEX

::: example
```csharp csharp/Datasources.Tests/CoreTests.cs#L209-L210
```
```rust rust/datasources/src/tests/mod.rs#L1014-L1018
```
```typescript typescript/datasources/src/builders/builders.test.ts#L300-L301
```

```output [After: create index]
CREATE INDEX idx_orders_user ON orders (user_id)
```
:::

### Compile a CREATE UNIQUE INDEX

::: example
```csharp csharp/Datasources.Tests/CoreTests.cs#L211-L213
```
```rust rust/datasources/src/tests/mod.rs#L1019-L1022
```
```typescript typescript/datasources/src/builders/builders.test.ts#L302-L304
```

```output [After: create unique index]
CREATE UNIQUE INDEX uq_orders_code ON orders (code)
```
:::
