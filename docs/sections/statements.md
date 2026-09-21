# Statements

Command text. `IQuery` / `IUpdateStatement` / `IDeleteStatement` also keep the `Where` object.

## Classes

::: reference
### typescript
#### IText

```typescript
interface IText
```

**Methods**

##### getText()

Rendered text.

```typescript
getText(): string
```

**Returns** `string`

#### IStatement

```typescript
interface IStatement extends IText
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

#### QueryStatement

```typescript
class QueryStatement implements IQuery
```

**Constructor**

##### QueryStatement(text, where?)

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
interface IWhere extends IExpression, IText {}
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

##### StoredProcedureStatement(name, parameters)

Compiles a `CALL`. Empty [Parameter](./command.md#parameter) list is `CALL name()`. [returnValue](./command.md#parameterdirection) parameters are omitted from the argument list.

```typescript
constructor(name: string, parameters: Parameter[])
```

- `name` — procedure name
- `parameters` — bound values (empty for `CALL name()`)

**Methods**

##### getText()

Command text.

```typescript
getText(): string
```

**Returns** `string`

#### IReturnStatement

```typescript
interface IReturnStatement extends IStatement {}
```

#### ReturnStatement

```typescript
class ReturnStatement implements IReturnStatement
```

**Constructor**

##### ReturnStatement(text)

Wraps function-call text.

```typescript
constructor(text: string)
```

- `text` — command text

##### ReturnStatement(name, parameters)

Compiles a `SELECT`. Empty [Parameter](./command.md#parameter) list is `SELECT name()`. [returnValue](./command.md#parameterdirection) parameters are omitted from the argument list.

```typescript
constructor(name: string, parameters: Parameter[])
```

- `name` — function name
- `parameters` — bound values (empty for `SELECT name()`)

**Methods**

##### getText()

Command text.

```typescript
getText(): string
```

**Returns** `string`

### csharp
#### IText

```csharp
public interface IText
```

**Methods**

##### GetText()

Rendered text.

```csharp
string GetText()
```

**Returns** `string`

#### IStatement

```csharp
public interface IStatement : IText
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

#### QueryStatement

```csharp
public sealed record QueryStatement : IQuery
```

**Constructor**

##### QueryStatement(text, where)

Wraps query text and an optional [IWhere](#iwhere).

```csharp
public QueryStatement(string text, IWhere? where = null)
```

- `text` — command text
- `where` — optional [IWhere](#iwhere)

##### QueryStatement(table, fields, where, joins)

Compiles a `SELECT` from a table, fields, optional [IWhere](#iwhere), and joins.

```csharp
public QueryStatement(
    string table,
    IReadOnlyList<string> fields,
    IWhere? where = null,
    IReadOnlyList<Join>? joins = null)
```

- `table` — table name
- `fields` — selected columns (`*` when empty)
- `where` — optional [IWhere](#iwhere)
- `joins` — optional joins

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
public interface IWhere : IExpression, IText
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
public sealed record StoredProcedureStatement : IStoredProcedureStatement
```

**Constructor**

##### StoredProcedureStatement(text)

Wraps stored-procedure call text.

```csharp
public StoredProcedureStatement(string text)
```

- `text` — command text

##### StoredProcedureStatement(name, parameters)

Compiles a `CALL`. Empty [Parameter](./command.md#parameter) list is `CALL name()`. [ReturnValue](./command.md#parameterdirection) parameters are omitted from the argument list.

```csharp
public StoredProcedureStatement(string name, IReadOnlyList<Parameter> parameters)
```

- `name` — procedure name
- `parameters` — bound values (empty for `CALL name()`)

**Methods**

##### GetText()

Command text.

```csharp
string GetText()
```

**Returns** `string`

#### IReturnStatement

```csharp
public interface IReturnStatement : IStatement
```

#### ReturnStatement

```csharp
public sealed record ReturnStatement : IReturnStatement
```

**Constructor**

##### ReturnStatement(text)

Wraps function-call text.

```csharp
public ReturnStatement(string text)
```

- `text` — command text

##### ReturnStatement(name, parameters)

Compiles a `SELECT`. Empty [Parameter](./command.md#parameter) list is `SELECT name()`. [ReturnValue](./command.md#parameterdirection) parameters are omitted from the argument list.

```csharp
public ReturnStatement(string name, IReadOnlyList<Parameter> parameters)
```

- `name` — function name
- `parameters` — bound values (empty for `SELECT name()`)

**Methods**

##### GetText()

Command text.

```csharp
string GetText()
```

**Returns** `string`

### rust
#### Text

```rust
pub trait Text: Send + Sync
```

**Methods**

##### get_text()

Rendered text.

```rust
fn get_text(&self) -> String
```

**Returns** `String`

#### Statement

```rust
pub trait Statement: Text
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

Wraps query text and a [Where](#where).

```rust
pub fn with_where(text: impl Into<String>, where_clause: impl Where + 'static) -> Self
```

- `text` — command text
- `where_clause` — [Where](#where)

**Returns** [QueryStatement](#querystatement)

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
pub trait Where: Text {}
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
pub struct InsertStatement
```

**Constructor**

##### text(text)

Wraps insert text.

```rust
pub fn text(text: impl Into<String>) -> Self
```

- `text` — command text

**Returns** [InsertStatement](#insertstatement)

##### new(table, fields)

Compiles an `INSERT`.

```rust
pub fn new(table: impl AsRef<str>, fields: &[Parameter]) -> Self
```

- `table` — table name
- `fields` — columns and bound values

**Returns** [InsertStatement](#insertstatement)

**Methods**

##### get_text()

Command text.

```rust
fn get_text(&self) -> String
```

**Returns** `String`

#### UpdateStatement

```rust
pub struct UpdateStatement
```

**Constructor**

##### text(text, where_clause)

Wraps update text and an optional [Where](#where).

```rust
pub fn text(text: impl Into<String>, where_clause: Option<Box<dyn Where>>) -> Self
```

- `text` — command text
- `where_clause` — optional [Where](#where)

**Returns** [UpdateStatement](#updatestatement)

##### new(table, fields, where_clause)

Compiles an `UPDATE`.

```rust
pub fn new(table: impl AsRef<str>, fields: &[Parameter], where_clause: Option<&dyn Where>) -> Self
```

- `table` — table name
- `fields` — columns and bound values
- `where_clause` — optional [Where](#where)

**Returns** [UpdateStatement](#updatestatement)

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
pub struct DeleteStatement
```

**Constructor**

##### raw(text, where_clause)

Wraps delete text and an optional [Where](#where).

```rust
pub fn raw(text: impl Into<String>, where_clause: Option<Box<dyn Where>>) -> Self
```

- `text` — command text
- `where_clause` — optional [Where](#where)

**Returns** [DeleteStatement](#deletestatement)

##### new(table, where_clause)

Compiles a `DELETE`.

```rust
pub fn new(table: impl AsRef<str>, where_clause: Option<&dyn Where>) -> Self
```

- `table` — table name
- `where_clause` — optional [Where](#where)

**Returns** [DeleteStatement](#deletestatement)

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
- `columns` — column names and types

**Returns** [CreateTableStatement](#createtablestatement)

**Methods**

##### get_text()

Command text.

```rust
fn get_text(&self) -> String
```

**Returns** `String`

#### StoredProcedureStatement

```rust
pub struct StoredProcedureStatement
```

**Constructor**

##### text(text)

Wraps stored-procedure call text.

```rust
pub fn text(text: impl Into<String>) -> Self
```

- `text` — command text

**Returns** [StoredProcedureStatement](#storedprocedurestatement)

##### new(name, parameters)

Compiles a `CALL`.

```rust
pub fn new(name: impl AsRef<str>, parameters: &[Parameter]) -> Self
```

- `name` — procedure name
- `parameters` — bound values (empty for `CALL name()`)

**Returns** [StoredProcedureStatement](#storedprocedurestatement)

**Methods**

##### get_text()

Command text.

```rust
fn get_text(&self) -> String
```

**Returns** `String`

#### ReturnStatement

```rust
pub struct ReturnStatement
```

**Constructor**

##### text(text)

Wraps function-call text.

```rust
pub fn text(text: impl Into<String>) -> Self
```

- `text` — command text

**Returns** [ReturnStatement](#returnstatement)

##### new(name, parameters)

Compiles a `SELECT`. Empty [Parameter](./command.md#parameter) list is `SELECT name()`. [ReturnValue](./command.md#parameterdirection) parameters are omitted from the argument list.

```rust
pub fn new(name: impl AsRef<str>, parameters: &[Parameter]) -> Self
```

- `name` — function name
- `parameters` — bound values (empty for `SELECT name()`)

**Returns** [ReturnStatement](#returnstatement)

**Methods**

##### get_text()

Command text.

```rust
fn get_text(&self) -> String
```

**Returns** `String`
:::

## Remarks

[Command](./command.md) `Execute` / `ExecuteReader` overloads take any statement and use `GetText()` / `getText()` / `get_text()` only. Bound values live on [WhereClause](./where-clause.md).

Construct SQL with [QueryStatement](#querystatement) / [InsertStatement](#insertstatement) / [UpdateStatement](#updatestatement) / [DeleteStatement](#deletestatement). [StoredProcedureStatement](#storedprocedurestatement) compiles `CALL name($p)` or `CALL name()`. [ReturnStatement](#returnstatement) compiles `SELECT name($p)` or `SELECT name()`. [ReturnValue](./command.md#parameterdirection) parameters are binds only and stay out of the argument list. Compose predicates with [WhereClause](./where-clause.md). [IStatement](#istatement) is not an [IExpression](./expressions.md#iexpression); [IText](#itext) is the shared `GetText()` / `getText()` / `get_text()` surface. [IWhere](#iwhere) is a clause ([IExpression](./expressions.md#iexpression) + [IText](#itext)), not an [IStatement](#istatement).

## See also

- [WhereClause](./where-clause.md)
- [Expressions](./expressions.md)
- [StatementEvaluator](./statement-evaluator.md)

## Examples

### Wrap query text and an optional where

::: example
```csharp csharp/Datasources.Tests/CoreTests.cs#L33-L38
```
```rust rust/datasources/src/tests/mod.rs#L992-L995
```
```typescript typescript/datasources/src/builders/builders.test.ts#L21-L25
```
:::

### Compile CALL and SELECT

::: example
```csharp csharp/Datasources.Tests/CoreTests.cs#L165-L174
```
```rust rust/datasources/src/tests/mod.rs#L953-L982
```
```typescript typescript/datasources/src/builders/builders.test.ts#L234-L255
```
:::
