# Statements

Command text. `IQuery` / `IUpdateStatement` / `IDeleteStatement` also keep the `Where` object.

## Classes

::: reference
### typescript
#### IStatement

```typescript
interface IStatement extends IExpression
```

**Methods**

##### getText()

Command text.

```typescript
getText(): string
```

**Returns** `string`

#### IQuery

```typescript
interface IQuery extends IStatement
```

**Properties**

##### where

Optional [IWhere](#iwhere) used to build this query.

```typescript
readonly where?: IWhere
```

#### Query

```typescript
class Query implements IQuery
```

**Constructor**

##### Query(text, where?)

Wraps query text and an optional [IWhere](#iwhere).

```typescript
constructor(text: string, where?: IWhere)
```

- `text` — command text
- `where` — optional [IWhere](#iwhere)

**Properties**

##### where

Optional [IWhere](#iwhere) used to build this query.

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

#### IWhere

```typescript
interface IWhere extends IStatement {}
```

#### Where

```typescript
class Where implements IWhere
```

**Constructor**

##### Where(text)

Wraps predicate text.

```typescript
constructor(text: string)
```

- `text` — predicate text

**Methods**

##### getText()

Predicate text.

```typescript
getText(): string
```

**Returns** `string`

#### IInsertStatement

```typescript
interface IInsertStatement extends IStatement {}
```

#### InsertStatement

```typescript
class InsertStatement implements IInsertStatement
```

**Constructor**

##### InsertStatement(text)

Wraps insert text.

```typescript
constructor(text: string)
```

- `text` — command text

**Methods**

##### getText()

Command text.

```typescript
getText(): string
```

**Returns** `string`

#### IUpdateStatement

```typescript
interface IUpdateStatement extends IStatement
```

**Properties**

##### where

Optional [IWhere](#iwhere) used to build this update.

```typescript
readonly where?: IWhere
```

#### UpdateStatement

```typescript
class UpdateStatement implements IUpdateStatement
```

**Constructor**

##### UpdateStatement(text, where?)

Wraps update text and an optional [IWhere](#iwhere).

```typescript
constructor(text: string, where?: IWhere)
```

- `text` — command text
- `where` — optional [IWhere](#iwhere)

**Properties**

##### where

Optional [IWhere](#iwhere) used to build this update.

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

#### IDeleteStatement

```typescript
interface IDeleteStatement extends IStatement
```

**Properties**

##### where

Optional [IWhere](#iwhere) used to build this delete.

```typescript
readonly where?: IWhere
```

#### DeleteStatement

```typescript
class DeleteStatement implements IDeleteStatement
```

**Constructor**

##### DeleteStatement(text, where?)

Wraps delete text and an optional [IWhere](#iwhere).

```typescript
constructor(text: string, where?: IWhere)
```

- `text` — command text
- `where` — optional [IWhere](#iwhere)

**Properties**

##### where

Optional [IWhere](#iwhere) used to build this delete.

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

**Methods**

##### getText()

Command text.

```typescript
getText(): string
```

**Returns** `string`

#### IStoredProcedureStatement

```typescript
interface IStoredProcedureStatement extends IStatement {}
```

#### StoredProcedureStatement

```typescript
class StoredProcedureStatement implements IStoredProcedureStatement
```

**Constructor**

##### StoredProcedureStatement(text)

Wraps stored-procedure call text.

```typescript
constructor(text: string)
```

- `text` — command text

**Methods**

##### getText()

Command text.

```typescript
getText(): string
```

**Returns** `string`

### csharp
#### IStatement

```csharp
public interface IStatement : IExpression
```

**Methods**

##### GetText()

Command text.

```csharp
string GetText()
```

**Returns** `string`

#### IQuery

```csharp
public interface IQuery : IStatement
```

**Properties**

##### Where

Optional [IWhere](#iwhere) used to build this query.

```csharp
IWhere? Where { get; }
```

#### Query

```csharp
public sealed class Query : IQuery
```

**Constructor**

##### Query(text, where)

Wraps query text and an optional [IWhere](#iwhere).

```csharp
public Query(string text, IWhere? where = null)
```

- `text` — command text
- `where` — optional [IWhere](#iwhere)

**Properties**

##### Where

Optional [IWhere](#iwhere) used to build this query.

```csharp
IWhere? Where { get; }
```

**Methods**

##### GetText()

Command text.

```csharp
string GetText()
```

**Returns** `string`

#### IWhere

```csharp
public interface IWhere : IStatement
```

#### Where

```csharp
public sealed class Where : IWhere
```

**Constructor**

##### Where(text)

Wraps predicate text.

```csharp
public Where(string text)
```

- `text` — predicate text

**Methods**

##### GetText()

Predicate text.

```csharp
string GetText()
```

**Returns** `string`

#### IInsertStatement

```csharp
public interface IInsertStatement : IStatement
```

#### InsertStatement

```csharp
public sealed class InsertStatement : IInsertStatement
```

**Constructor**

##### InsertStatement(text)

Wraps insert text.

```csharp
public InsertStatement(string text)
```

- `text` — command text

**Methods**

##### GetText()

Command text.

```csharp
string GetText()
```

**Returns** `string`

#### IUpdateStatement

```csharp
public interface IUpdateStatement : IStatement
```

**Properties**

##### Where

Optional [IWhere](#iwhere) used to build this update.

```csharp
IWhere? Where { get; }
```

#### UpdateStatement

```csharp
public sealed class UpdateStatement : IUpdateStatement
```

**Constructor**

##### UpdateStatement(text, where)

Wraps update text and an optional [IWhere](#iwhere).

```csharp
public UpdateStatement(string text, IWhere? where = null)
```

- `text` — command text
- `where` — optional [IWhere](#iwhere)

**Properties**

##### Where

Optional [IWhere](#iwhere) used to build this update.

```csharp
IWhere? Where { get; }
```

**Methods**

##### GetText()

Command text.

```csharp
string GetText()
```

**Returns** `string`

#### IDeleteStatement

```csharp
public interface IDeleteStatement : IStatement
```

**Properties**

##### Where

Optional [IWhere](#iwhere) used to build this delete.

```csharp
IWhere? Where { get; }
```

#### DeleteStatement

```csharp
public sealed class DeleteStatement : IDeleteStatement
```

**Constructor**

##### DeleteStatement(text, where)

Wraps delete text and an optional [IWhere](#iwhere).

```csharp
public DeleteStatement(string text, IWhere? where = null)
```

- `text` — command text
- `where` — optional [IWhere](#iwhere)

**Properties**

##### Where

Optional [IWhere](#iwhere) used to build this delete.

```csharp
IWhere? Where { get; }
```

**Methods**

##### GetText()

Command text.

```csharp
string GetText()
```

**Returns** `string`

#### ICreateTableStatement

```csharp
public interface ICreateTableStatement : IStatement
```

#### CreateTableStatement

```csharp
public sealed class CreateTableStatement : ICreateTableStatement
```

**Constructor**

##### CreateTableStatement(text)

Wraps create-table text.

```csharp
public CreateTableStatement(string text)
```

- `text` — command text

**Methods**

##### GetText()

Command text.

```csharp
string GetText()
```

**Returns** `string`

#### IStoredProcedureStatement

```csharp
public interface IStoredProcedureStatement : IStatement
```

#### StoredProcedureStatement

```csharp
public sealed class StoredProcedureStatement : IStoredProcedureStatement
```

**Constructor**

##### StoredProcedureStatement(text)

Wraps stored-procedure call text.

```csharp
public StoredProcedureStatement(string text)
```

- `text` — command text

**Methods**

##### GetText()

Command text.

```csharp
string GetText()
```

**Returns** `string`

### rust
#### Statement

```rust
pub trait Statement: Send + Sync
```

**Methods**

##### get_text()

Command text.

```rust
fn get_text(&self) -> String
```

**Returns** `String`

#### Query

```rust
pub trait Query: Statement
```

**Methods**

##### where_clause()

Optional [Where](#where) used to build this query.

```rust
fn where_clause(&self) -> Option<&dyn Where>
```

**Returns** `Option<&dyn `[Where](#where)`>`

#### TextQuery

```rust
pub struct TextQuery
```

**Constructor**

##### new(text)

Wraps query text.

```rust
pub fn new(text: impl Into<String>) -> Self
```

- `text` — command text

**Returns** [TextQuery](#textquery)

##### with_where(text, where_clause)

Wraps query text and a [Where](#where).

```rust
pub fn with_where(text: impl Into<String>, where_clause: impl Where + 'static) -> Self
```

- `text` — command text
- `where_clause` — [Where](#where)

**Returns** [TextQuery](#textquery)

**Methods**

##### get_text()

Command text.

```rust
fn get_text(&self) -> String
```

**Returns** `String`

##### where_clause()

Optional [Where](#where) used to build this query.

```rust
fn where_clause(&self) -> Option<&dyn Where>
```

**Returns** `Option<&dyn `[Where](#where)`>`

#### Where

```rust
pub trait Where: Statement {}
```

#### TextWhere

```rust
pub struct TextWhere
```

**Constructor**

##### new(text)

Wraps predicate text.

```rust
pub fn new(text: impl Into<String>) -> Self
```

- `text` — predicate text

**Returns** [TextWhere](#textwhere)

**Methods**

##### get_text()

Predicate text.

```rust
fn get_text(&self) -> String
```

**Returns** `String`

#### InsertStatement

```rust
pub trait InsertStatement: Statement {}
```

#### TextInsertStatement

```rust
pub struct TextInsertStatement
```

**Constructor**

##### new(text)

Wraps insert text.

```rust
pub fn new(text: impl Into<String>) -> Self
```

- `text` — command text

**Returns** [TextInsertStatement](#textinsertstatement)

**Methods**

##### get_text()

Command text.

```rust
fn get_text(&self) -> String
```

**Returns** `String`

#### UpdateStatement

```rust
pub trait UpdateStatement: Statement
```

**Methods**

##### where_clause()

Optional [Where](#where) used to build this update.

```rust
fn where_clause(&self) -> Option<&dyn Where>
```

**Returns** `Option<&dyn `[Where](#where)`>`

#### TextUpdateStatement

```rust
pub struct TextUpdateStatement
```

**Constructor**

##### new(text, where_clause)

Wraps update text and an optional [Where](#where).

```rust
pub fn new(text: impl Into<String>, where_clause: Option<Box<dyn Where>>) -> Self
```

- `text` — command text
- `where_clause` — optional [Where](#where)

**Returns** [TextUpdateStatement](#textupdatestatement)

**Methods**

##### get_text()

Command text.

```rust
fn get_text(&self) -> String
```

**Returns** `String`

##### where_clause()

Optional [Where](#where) used to build this update.

```rust
fn where_clause(&self) -> Option<&dyn Where>
```

**Returns** `Option<&dyn `[Where](#where)`>`

#### DeleteStatement

```rust
pub trait DeleteStatement: Statement
```

**Methods**

##### where_clause()

Optional [Where](#where) used to build this delete.

```rust
fn where_clause(&self) -> Option<&dyn Where>
```

**Returns** `Option<&dyn `[Where](#where)`>`

#### TextDeleteStatement

```rust
pub struct TextDeleteStatement
```

**Constructor**

##### new(text, where_clause)

Wraps delete text and an optional [Where](#where).

```rust
pub fn new(text: impl Into<String>, where_clause: Option<Box<dyn Where>>) -> Self
```

- `text` — command text
- `where_clause` — optional [Where](#where)

**Returns** [TextDeleteStatement](#textdeletestatement)

**Methods**

##### get_text()

Command text.

```rust
fn get_text(&self) -> String
```

**Returns** `String`

##### where_clause()

Optional [Where](#where) used to build this delete.

```rust
fn where_clause(&self) -> Option<&dyn Where>
```

**Returns** `Option<&dyn `[Where](#where)`>`

#### CreateTableStatement

```rust
pub trait CreateTableStatement: Statement {}
```

#### TextCreateTableStatement

```rust
pub struct TextCreateTableStatement
```

**Constructor**

##### new(text)

Wraps create-table text.

```rust
pub fn new(text: impl Into<String>) -> Self
```

- `text` — command text

**Returns** [TextCreateTableStatement](#textcreatetablestatement)

**Methods**

##### get_text()

Command text.

```rust
fn get_text(&self) -> String
```

**Returns** `String`

#### StoredProcedureStatement

```rust
pub trait StoredProcedureStatement: Statement {}
```

#### TextStoredProcedureStatement

```rust
pub struct TextStoredProcedureStatement
```

**Constructor**

##### new(text)

Wraps stored-procedure call text.

```rust
pub fn new(text: impl Into<String>) -> Self
```

- `text` — command text

**Returns** [TextStoredProcedureStatement](#textstoredprocedurestatement)

**Methods**

##### get_text()

Command text.

```rust
fn get_text(&self) -> String
```

**Returns** `String`
:::

## Remarks

[Command](./command.md) `Execute` / `ExecuteReader` overloads take any statement and use `GetText()` / `getText()` / `get_text()` only. Bound values live on [WhereStatement](./where-statement.md).

Build SQL with [StatementBuilder](./statement-builder.md). Compose predicates with [WhereStatement](./where-statement.md).

## See also

- [StatementBuilder](./statement-builder.md)
- [WhereStatement](./where-statement.md)
- [Expressions](./expressions.md)
- [StatementEvaluator](./statement-evaluator.md)

## Examples

### Wrap query text and an optional where

::: example
```csharp csharp/Datasources.Tests/CoreTests.cs#L34-L36
```
```rust rust/datasources/src/tests/mod.rs#L782-L785
```
```typescript typescript/datasources/src/builders/builders.test.ts#L17-L19
```
:::
