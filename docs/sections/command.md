# Command

Named command bound to an optional open [connection](./connection.md). `CreateCommand` / `createCommand` / `create_command` is sync. The caller executes it.

## Classes

::: reference
### typescript
#### ICommand

```typescript
interface ICommand
```

**Properties**

##### connection

The [IConnection](./connection.md) this command is bound to.

```typescript
readonly connection: IConnection
```

**Methods**

##### addMiddleware(middleware)

Adds [ICommandMiddleware](#icommandmiddleware). Returns this command.

```typescript
addMiddleware(middleware: ICommandMiddleware): this
```

- `middleware` — [ICommandMiddleware](#icommandmiddleware)

**Returns** `this`

##### executeReader(text, parameters?)

Runs a query and returns a reader over the rows.

```typescript
executeReader(text: string, parameters?: Parameter[]): Promise<IReader<Cells>>
```

- `text` — SQL or dialect command text
- `parameters` — optional [Parameter](#parameter) list

**Returns** `Promise<[IReader](#reader)<Cells>>`

##### execute(text, parameters?)

Runs a non-query command.

```typescript
execute(text: string, parameters?: Parameter[]): Promise<Response>
```

- `text` — SQL or dialect command text
- `parameters` — optional [Parameter](#parameter) list

**Returns** `Promise<[Response](#response)>`

##### execute(command, statement, parameters?)

Runs [execute](#icommand-execute-text-parameters) with `statement.getText()`.

```typescript
function execute(command: ICommand, statement: IStatement, parameters?: Parameter[]): Promise<Response>
```

- `command` — [ICommand](#icommand)
- `statement` — [IStatement](./statements.md#istatement)
- `parameters` — optional [Parameter](#parameter) list

**Returns** `Promise<[Response](#response)>`

##### executeReader(command, statement, parameters?)

Runs [executeReader](#icommand-executereader-text-parameters) with `statement.getText()`.

```typescript
function executeReader(command: ICommand, statement: IStatement, parameters?: Parameter[]): Promise<IReader<Cells>>
```

- `command` — [ICommand](#icommand)
- `statement` — [IStatement](./statements.md#istatement)
- `parameters` — optional [Parameter](#parameter) list

**Returns** `Promise<[IReader](#reader)<Cells>>`

#### ICommandMiddleware

```typescript
interface ICommandMiddleware
```

**Methods**

##### beforeCommand(text, params?)

Runs before the command. A thrown error skips the command. A `{ text, params }` return rewrites it.

```typescript
beforeCommand(text: string, params?: ReadonlyArray<Parameter>): void | { text: string; params?: ReadonlyArray<Parameter> } | Promise<void | { text: string; params?: ReadonlyArray<Parameter> }>
```

- `text` — command text
- `params` — optional [Parameter](#parameter) list

##### afterCommand(text, params, results, elapsedMs, error?)

Runs after the command, including when the command failed.

```typescript
afterCommand(text: string, params: ReadonlyArray<Parameter> | undefined, results: ReadonlyArray<Row>, elapsedMs: number, error?: Error): void | Promise<void>
```

- `text` — command text
- `params` — [Parameter](#parameter) list used
- `results` — rows produced
- `elapsedMs` — elapsed milliseconds
- `error` — command error when the command failed

#### Parameter

```typescript
interface Parameter<T = Scalar>
```

**Properties**

##### name

Parameter name.

```typescript
name: string
```

##### value

Parameter value. Omitted for [output](#parameterdirection) / [returnValue](#parameterdirection).

```typescript
value?: T
```

##### direction

Bind direction. Omitted means [input](#parameterdirection).

```typescript
direction?: ParameterDirection
```

#### ParameterDirection

```typescript
const ParameterDirection
```

**Properties**

| Name | Value |
| --- | --- |
| Input | `"input"` |
| Output | `"output"` |
| InputOutput | `"inputOutput"` |
| ReturnValue | `"returnValue"` |

#### Response

```typescript
interface Response
```

**Properties**

##### result

Scalar or row result from the command.

```typescript
result: unknown
```

##### rowsAffected

Rows inserted, updated, or deleted.

```typescript
rowsAffected: number
```

##### parameters

Bound [Parameter](#parameter) list after [execute](#icommand-execute-text-parameters). Holds filled [output](#parameterdirection) / [returnValue](#parameterdirection) values.

```typescript
parameters?: Parameter[]
```

#### responseOf(result, rowsAffected, parameters?)

Builds a [Response](#response). Copies a [returnValue](#parameterdirection) onto [result](#response-result) when `result` is null.

```typescript
function responseOf(result: unknown, rowsAffected: number, parameters?: Parameter[]): Response
```

- `result` — scalar or row result
- `rowsAffected` — rows inserted, updated, or deleted
- `parameters` — bound [Parameter](#parameter) list

**Returns** [Response](#response)

#### IReader {#reader}

```typescript
interface IReader<T>
```

**Properties**

##### current

The row from the last successful [next](#ireader-next).

```typescript
readonly current: T | undefined
```

**Methods**

##### next()

Advances to the next row.

```typescript
next(): Promise<boolean>
```

**Returns** `Promise<boolean>`

##### nextResult()

Moves to the next result set and clears [current](#ireader-current). Returns false when there is no further set. [next](#ireader-next) then reads that set. Calling only [next](#ireader-next) leaves later sets unread.

```typescript
nextResult(): Promise<boolean>
```

**Returns** `Promise<boolean>`

##### dispose()

Releases the reader.

```typescript
dispose(): Promise<void>
```

**Returns** `Promise<void>`

### csharp
#### ICommand

```csharp
public interface ICommand
```

**Properties**

##### Connection

The [IConnection](./connection.md) this command is bound to.

```csharp
IConnection Connection { get; }
```

**Methods**

##### AddMiddleware(middleware)

Adds [ICommandMiddleware](#icommandmiddleware). Returns this command.

```csharp
ICommand AddMiddleware(ICommandMiddleware middleware)
```

- `middleware` — [ICommandMiddleware](#icommandmiddleware)

**Returns** [ICommand](#icommand)

##### ExecuteReader(text, parameters)

Runs a query and returns a reader over the rows.

```csharp
Task<IReader<Row>> ExecuteReader(string text, IReadOnlyList<Parameter>? parameters = null)
```

- `text` — SQL or dialect command text
- `parameters` — optional [Parameter](#parameter) list

**Returns** `Task<[IReader](#reader)<Row>>`

##### Execute(text, parameters)

Runs a non-query command.

```csharp
Task<Response> Execute(string text, IReadOnlyList<Parameter>? parameters = null)
```

- `text` — SQL or dialect command text
- `parameters` — optional [Parameter](#parameter) list

**Returns** `Task<[Response](#response)>`

##### ExecuteReader(statement, parameters)

Runs [ExecuteReader](#icommand-executereader-text-parameters) with `statement.GetText()`.

```csharp
Task<IReader<Row>> ExecuteReader(IStatement statement, IReadOnlyList<Parameter>? parameters = null)
```

- `statement` — [IStatement](./statements.md#istatement)
- `parameters` — optional [Parameter](#parameter) list

**Returns** `Task<[IReader](#reader)<Row>>`

##### Execute(statement, parameters)

Runs [Execute](#icommand-execute-text-parameters) with `statement.GetText()`.

```csharp
Task<Response> Execute(IStatement statement, IReadOnlyList<Parameter>? parameters = null)
```

- `statement` — [IStatement](./statements.md#istatement)
- `parameters` — optional [Parameter](#parameter) list

**Returns** `Task<[Response](#response)>`

#### ICommandMiddleware

```csharp
public interface ICommandMiddleware
```

**Methods**

##### BeforeCommand(text, parameters)

Runs before the command. A thrown error skips the command. A non-null return rewrites it.

```csharp
Task<(string Text, IReadOnlyList<Parameter>? Parameters)?> BeforeCommand(string text, IReadOnlyList<Parameter>? parameters)
```

- `text` — command text
- `parameters` — optional [Parameter](#parameter) list

**Returns** `Task<(string Text, IReadOnlyList<Parameter>? Parameters)?>`

##### AfterCommand(text, parameters, results, elapsedMs, error)

Runs after the command, including when the command failed.

```csharp
Task AfterCommand(string text, IReadOnlyList<Parameter>? parameters, IReadOnlyList<Row> results, double elapsedMs, Exception? error = null)
```

- `text` — command text
- `parameters` — [Parameter](#parameter) list used
- `results` — rows produced
- `elapsedMs` — elapsed milliseconds
- `error` — command error when the command failed

#### Parameter

```csharp
public sealed record Parameter(string Name, Value Value, ParameterDirection Direction = ParameterDirection.Input)
```

**Properties**

##### Name

Parameter name.

```csharp
string Name { get; }
```

##### Value

Parameter value. `Value.Null` for output-only / return slots.

```csharp
Value Value { get; }
```

##### Direction

Bind direction. Default is [Input](#parameterdirection).

```csharp
ParameterDirection Direction { get; }
```

**Constructor**

##### Parameter(name, direction)

Output-only / return slot. [Value](#parameter-value) is `Value.Null`.

```csharp
public Parameter(string name, ParameterDirection direction)
```

- `name` — parameter name
- `direction` — [Output](#parameterdirection) / [InputOutput](#parameterdirection) / [ReturnValue](#parameterdirection)

#### ParameterDirection

```csharp
public enum ParameterDirection
```

**Properties**

| Name | Value |
| --- | --- |
| Input | `Input` |
| Output | `Output` |
| InputOutput | `InputOutput` |
| ReturnValue | `ReturnValue` |

#### Response

```csharp
public sealed record Response(Value Result, int RowsAffected, IReadOnlyList<Parameter>? Parameters = null)
```

**Properties**

##### Result

Scalar or row result from the command.

```csharp
Value Result { get; }
```

##### RowsAffected

Rows inserted, updated, or deleted.

```csharp
int RowsAffected { get; }
```

##### Parameters

Bound [Parameter](#parameter) list after [Execute](#icommand-execute-text-parameters). Holds filled [Output](#parameterdirection) / [ReturnValue](#parameterdirection) values.

```csharp
IReadOnlyList<Parameter>? Parameters { get; }
```

**Methods**

##### Of(result, rowsAffected, parameters)

Builds a [Response](#response). Copies a [ReturnValue](#parameterdirection) onto [Result](#response-result) when `result` is `Value.Null`.

```csharp
public static Response Of(Value result, int rowsAffected, IReadOnlyList<Parameter>? parameters = null)
```

- `result` — scalar or row result
- `rowsAffected` — rows inserted, updated, or deleted
- `parameters` — bound [Parameter](#parameter) list

**Returns** [Response](#response)

#### IReader {#reader}

```csharp
public interface IReader<out T>
```

**Properties**

##### Current

The row from the last successful [Next](#ireader-next).

```csharp
T? Current { get; }
```

**Methods**

##### Next()

Advances to the next row.

```csharp
Task<bool> Next()
```

**Returns** `Task<bool>`

##### NextResult()

Moves to the next result set and clears [Current](#ireader-current). Returns false when there is no further set. [Next](#ireader-next) then reads that set. A reader with one set returns false. Calling only [Next](#ireader-next) leaves later sets unread.

```csharp
Task<bool> NextResult()
```

**Returns** `Task<bool>`

### rust
#### Command

```rust
pub trait Command: Send + Sync
```

**Methods**

##### add_middleware(middleware)

Adds [CommandMiddleware](#commandmiddleware).

```rust
fn add_middleware(&self, middleware: Arc<dyn CommandMiddleware>)
```

- `middleware` — [CommandMiddleware](#commandmiddleware)

##### execute_reader(text, parameters)

Runs a query and returns a reader over the rows.

```rust
async fn execute_reader(&self, text: &str, parameters: Option<&[Parameter]>) -> Result<Box<dyn Reader>>
```

- `text` — SQL or dialect command text
- `parameters` — optional [Parameter](#parameter) slice

**Returns** `Result<Box<dyn `[Reader](#reader)`>>`

##### execute(text, parameters)

Runs a non-query command.

```rust
async fn execute(&self, text: &str, parameters: Option<&[Parameter]>) -> Result<Response>
```

- `text` — SQL or dialect command text
- `parameters` — optional [Parameter](#parameter) slice

**Returns** `Result<[Response](#response)>`

##### execute_reader_statement(statement, parameters)

Runs [execute_reader](#command-execute_reader-text-parameters) with `statement.get_text()`.

```rust
async fn execute_reader_statement(&self, statement: &dyn Statement, parameters: Option<&[Parameter]>) -> Result<Box<dyn Reader>>
```

- `statement` — [Statement](./statements.md#statement)
- `parameters` — optional [Parameter](#parameter) slice

**Returns** `Result<Box<dyn `[Reader](#reader)`>>`

##### execute_statement(statement, parameters)

Runs [execute](#command-execute-text-parameters) with `statement.get_text()`.

```rust
async fn execute_statement(&self, statement: &dyn Statement, parameters: Option<&[Parameter]>) -> Result<Response>
```

- `statement` — [Statement](./statements.md#statement)
- `parameters` — optional [Parameter](#parameter) slice

**Returns** `Result<[Response](#response)>`

#### CommandMiddleware

```rust
pub trait CommandMiddleware: Send + Sync
```

**Methods**

##### before_command(text, params)

Runs before the command. An `Err` skips the command. `Some` rewrites it.

```rust
async fn before_command(&self, text: &str, params: Option<&[Parameter]>) -> Result<Option<(String, Option<Vec<Parameter>>)>>
```

- `text` — command text
- `params` — optional [Parameter](#parameter) slice

**Returns** `Result<Option<(String, Option<Vec<Parameter>>)>>`

##### after_command(text, params, results, elapsed_ms, error)

Runs after the command, including when the command failed.

```rust
async fn after_command(&self, text: &str, params: Option<&[Parameter]>, results: &[Row], elapsed_ms: f64, error: Option<&DatasourceError>) -> Result<()>
```

- `text` — command text
- `params` — [Parameter](#parameter) slice used
- `results` — rows produced
- `elapsed_ms` — elapsed milliseconds
- `error` — command error when the command failed

**Returns** `Result<()>`

#### Parameter

```rust
pub struct Parameter<T = Value>
```

**Properties**

##### name

Parameter name.

```rust
pub name: String
```

##### value

Parameter value. `Value::Null` for output-only / return slots.

```rust
pub value: T
```

##### direction

Bind direction. Default is [Input](#parameterdirection).

```rust
pub direction: ParameterDirection
```

**Methods**

##### new(name, value)

Input [Parameter](#parameter).

```rust
pub fn new(name: impl Into<String>, value: impl Into<Value>) -> Self
```

- `name` — parameter name
- `value` — parameter value

**Returns** [Parameter](#parameter)

##### output(name)

Output-only slot. [value](#parameter-value) is `Value::Null`.

```rust
pub fn output(name: impl Into<String>) -> Self
```

- `name` — parameter name

**Returns** [Parameter](#parameter)

##### return_value(name)

Return slot. [value](#parameter-value) is `Value::Null`.

```rust
pub fn return_value(name: impl Into<String>) -> Self
```

- `name` — parameter name

**Returns** [Parameter](#parameter)

##### with_direction(name, value, direction)

Parameter with an explicit [ParameterDirection](#parameterdirection).

```rust
pub fn with_direction(name: impl Into<String>, value: impl Into<Value>, direction: ParameterDirection) -> Self
```

- `name` — parameter name
- `value` — parameter value
- `direction` — bind direction

**Returns** [Parameter](#parameter)

#### ParameterDirection

```rust
pub enum ParameterDirection
```

**Properties**

| Name | Value |
| --- | --- |
| Input | `Input` |
| Output | `Output` |
| InputOutput | `InputOutput` |
| ReturnValue | `ReturnValue` |

#### Response

```rust
pub struct Response
```

**Properties**

##### result

Scalar or row result from the command.

```rust
pub result: Value
```

##### rows_affected

Rows inserted, updated, or deleted.

```rust
pub rows_affected: i64
```

##### parameters

Bound [Parameter](#parameter) list after [execute](#command-execute-text-parameters). Holds filled [Output](#parameterdirection) / [ReturnValue](#parameterdirection) values.

```rust
pub parameters: Option<Vec<Parameter>>
```

**Methods**

##### new(result, rows_affected)

Builds a [Response](#response) with no [parameters](#response-parameters).

```rust
pub fn new(result: impl Into<Value>, rows_affected: i64) -> Self
```

- `result` — scalar or row result
- `rows_affected` — rows inserted, updated, or deleted

**Returns** [Response](#response)

##### of(result, rows_affected, parameters)

Builds a [Response](#response). Copies a [ReturnValue](#parameterdirection) onto [result](#response-result) when `result` is `Value::Null`.

```rust
pub fn of(result: impl Into<Value>, rows_affected: i64, parameters: Option<Vec<Parameter>>) -> Self
```

- `result` — scalar or row result
- `rows_affected` — rows inserted, updated, or deleted
- `parameters` — bound [Parameter](#parameter) list

**Returns** [Response](#response)

#### Reader {#reader}

```rust
pub trait Reader<T = Row>: Send + Sync
```

**Methods**

##### current()

The row from the last successful [next](#reader-next).

```rust
fn current(&self) -> Option<T>
```

**Returns** `Option<T>`

##### next()

Advances to the next row.

```rust
async fn next(&self) -> Result<bool>
```

**Returns** `Result<bool>`

##### next_result()

Moves to the next result set and clears [current](#reader-current). Returns false when there is no further set. [next](#reader-next) then reads that set. The default returns false. Calling only [next](#reader-next) leaves later sets unread.

```rust
async fn next_result(&self) -> Result<bool>
```

**Returns** `Result<bool>`
:::

## Remarks

The connection must be open before `Execute` / `execute` / `execute` or `ExecuteReader` / `executeReader` / `execute_reader`.

`AddMiddleware` / `addMiddleware` / `add_middleware` runs `BeforeCommand` / `beforeCommand` / `before_command` then the command, then `AfterCommand` / `afterCommand` / `after_command`. A `beforeCommand` error skips the command. A command error still runs `afterCommand` with `error` and elapsed time set. `ConsoleCommandMiddleware` logs Start/Finish lines to the console. Verbose logs use AfterCommand for the result body.

Working dialects (`json`, `memory`) treat `text` as `insert` / `update` / `delete`. `sqlite` treats `text` as SQL. Stub dialects throw or return not-implemented on execute and execute-reader.

MySQL and Postgres run a provider `CreateTableStatementRenderer` on statement `Execute` / `ExecuteReader`. It overrides column type, default, and identity, then the command executes that text. Wrapped create-table text and every other statement pass through. Other dialects send `GetText()` / `getText()` / `get_text()`.

The Rust `Command` trait has no connection getter. Stub `create_command` stores the optional connection on the named type; working dialects still construct their own typed connection today.

`ExecuteReader` returns a reader for the first result set. `Next` / `next` / `next` reads that set. `NextResult` / `nextResult` / `next_result` advances to the next set. Rows stay unread until `Next`. SQL `AfterCommand` for `ExecuteReader` receives an empty row list. MySQL advances each result of a `CALL`. SQLite advances to the next statement in the command. Postgres advances to the next result of a multi-statement batch.

`ExecuteReader` returns rows. Convert with [RowTypeConverter](./row-type-converter.md), then map with a [Mapper](./mappers.md).

[Parameter](#parameter) defaults to [Input](#parameterdirection). [Output](#parameterdirection) / [InputOutput](#parameterdirection) / [ReturnValue](#parameterdirection) values are filled on [Response.Parameters](#response-parameters) after `Execute`. A [ReturnValue](#parameterdirection) copies onto [Response.Result](#response-result) when `Result` would otherwise be null. `Execute` on `SELECT` text reads the first scalar into `Result`.

## Examples

### ExecuteReader to NameValue

::: example
```csharp csharp/Datasources.Tests/CommandTests.cs#ExecuteReaderReturnsNameValueObjects
```
```rust rust/datasources/src/tests/command.rs#execute_reader_returns_name_value_objects
```
```typescript typescript/datasources/src/Command.test.ts#executereader-returns-namevalue-objects
```
:::

### Read the next result set

::: example
```csharp csharp/Providers/Sqlite.Tests/SqliteCommandIntegrationTests.cs#NextResultReadsTheNextStatement
```
```rust rust/providers/sqlite/src/lib.rs#sqlite_command_and_repository
```
```typescript typescript/providers/sqlite/src/sqlite.test.ts#executes-sql-on-the-open-database
```
:::

### Execute insert, update, and delete

::: example
```csharp csharp/Datasources.Tests/CommandTests.cs#ExecuteInsertUpdateDelete
```
```rust rust/datasources/src/tests/command.rs#execute_insert_update_delete
```
```typescript typescript/datasources/src/Command.test.ts#execute-insert-update-and-delete
```
:::

### Map flat contacts

::: example
```csharp csharp/Datasources.Tests/CommandTests.cs#ExecuteReaderMapsFlatContacts
```
```rust rust/datasources/src/tests/command.rs#execute_reader_maps_flat_contacts
```
```typescript typescript/datasources/src/Command.test.ts#executereader-maps-flat-contacts
```
:::

### Parameter direction and Response.Parameters

::: example
```csharp csharp/Datasources.Tests/CoreTests.cs#StatementsAndBuilders
```
```rust rust/datasources/src/tests/mod.rs#statements_and_builders
```
```typescript typescript/datasources/src/builders/builders.test.ts#creates-write-and-ddl-statements
```
:::
