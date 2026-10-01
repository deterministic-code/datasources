# Command

Executes a command against the datasource and returns a result or a streaming [Reader](./reader.md).

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

Parses `text` with [StatementParser.parse](./statements.md) and runs [executeReaderStatement](#icommand-executereaderstatement-statement-parameters). Blank text runs [executeReaderNative](#icommand-executereadernative-text-parameters).

```typescript
executeReader(text: string, parameters?: Parameter[]): Promise<IReader<Cells>>
```

- `text` — SQL parsed as a statement
- `parameters` — optional [Parameter](#parameter) list

**Returns** `Promise<[IReader](./reader.md)<Cells>>`

##### execute(text, parameters?)

Parses `text` with [StatementParser.parse](./statements.md) and runs [executeStatement](#icommand-executestatement-statement-parameters). Blank text runs [executeNative](#icommand-executenative-text-parameters).

```typescript
execute(text: string, parameters?: Parameter[]): Promise<Response>
```

- `text` — SQL parsed as a statement
- `parameters` — optional [Parameter](#parameter) list

**Returns** `Promise<[Response](#response)>`

##### executeReaderNative(text, parameters?)

Runs provider text without parsing it.

```typescript
executeReaderNative(text: Native, parameters?: Parameter[]): Promise<IReader<Cells>>
```

- `text` — [Native](#native) engine text
- `parameters` — optional [Parameter](#parameter) list

**Returns** `Promise<[IReader](./reader.md)<Cells>>`

##### executeNative(text, parameters?)

Runs provider text without parsing it.

```typescript
executeNative(text: Native, parameters?: Parameter[]): Promise<Response>
```

- `text` — [Native](#native) engine text
- `parameters` — optional [Parameter](#parameter) list

**Returns** `Promise<[Response](#response)>`

##### execute(command, statement, parameters?)

Runs [executeStatement](#icommand-executestatement-statement-parameters) when the command implements it, otherwise [executeNative](#icommand-executenative-text-parameters) with `statement.getText()`.

```typescript
function execute(command: ICommand, statement: IStatement, parameters?: Parameter[]): Promise<Response>
```

- `command` — [ICommand](#icommand)
- `statement` — [IStatement](./statements.md#istatement)
- `parameters` — optional [Parameter](#parameter) list

**Returns** `Promise<[Response](#response)>`

##### executeReader(command, statement, parameters?)

Runs [executeReaderStatement](#icommand-executereaderstatement-statement-parameters) when the command implements it, otherwise [executeReaderNative](#icommand-executereadernative-text-parameters) with `statement.getText()`.

```typescript
function executeReader(command: ICommand, statement: IStatement, parameters?: Parameter[]): Promise<IReader<Cells>>
```

- `command` — [ICommand](#icommand)
- `statement` — [IStatement](./statements.md#istatement)
- `parameters` — optional [Parameter](#parameter) list

**Returns** `Promise<[IReader](./reader.md)<Cells>>`

#### Native

```typescript
class Native
```

**Constructor**

##### Native(text)

Engine text. `n(text)` builds the same value.

```typescript
constructor(text: string)
```

- `text` — provider command text

**Returns** [Native](#native)

**Properties**

##### text

Provider command text.

```typescript
readonly text: string
```

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
class Response
```

**Constructor**

##### Response(result, rowsAffected, parameters?)

Builds a [Response](#response). Copies a [returnValue](#parameterdirection) onto [result](#response-result) when `result` is null.

```typescript
constructor(result: unknown, rowsAffected: number, parameters?: Parameter[])
```

- `result` — scalar or row result
- `rowsAffected` — rows inserted, updated, or deleted
- `parameters` — bound [Parameter](#parameter) list

**Returns** [Response](#response)

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

Parses `text` with [StatementParser.Parse](./statements.md) and runs [ExecuteReader(statement)](#icommand-executereader-statement-parameters). Blank text runs [ExecuteReader(Native)](#icommand-executereader-native-parameters).

```csharp
Task<IReader<Row>> ExecuteReader(string text, IReadOnlyList<Parameter>? parameters = null)
```

- `text` — SQL parsed as a statement
- `parameters` — optional [Parameter](#parameter) list

**Returns** `Task<[IReader](./reader.md)<Row>>`

##### Execute(text, parameters)

Parses `text` with [StatementParser.Parse](./statements.md) and runs [Execute(statement)](#icommand-execute-statement-parameters). Blank text runs [Execute(Native)](#icommand-execute-native-parameters).

```csharp
Task<Response> Execute(string text, IReadOnlyList<Parameter>? parameters = null)
```

- `text` — SQL parsed as a statement
- `parameters` — optional [Parameter](#parameter) list

**Returns** `Task<[Response](#response)>`

##### ExecuteReader(native, parameters)

Runs provider text without parsing it.

```csharp
Task<IReader<Row>> ExecuteReader(Native text, IReadOnlyList<Parameter>? parameters = null)
```

- `text` — [Native](#native) engine text
- `parameters` — optional [Parameter](#parameter) list

**Returns** `Task<[IReader](./reader.md)<Row>>`

##### Execute(native, parameters)

Runs provider text without parsing it.

```csharp
Task<Response> Execute(Native text, IReadOnlyList<Parameter>? parameters = null)
```

- `text` — [Native](#native) engine text
- `parameters` — optional [Parameter](#parameter) list

**Returns** `Task<[Response](#response)>`

##### ExecuteReader(statement, parameters)

Runs the statement. The default renders [Native](#native) from `statement.GetText()`.

```csharp
Task<IReader<Row>> ExecuteReader(IStatement statement, IReadOnlyList<Parameter>? parameters = null)
```

- `statement` — [IStatement](./statements.md#istatement)
- `parameters` — optional [Parameter](#parameter) list

**Returns** `Task<[IReader](./reader.md)<Row>>`

##### Execute(statement, parameters)

Runs the statement. The default renders [Native](#native) from `statement.GetText()`.

```csharp
Task<Response> Execute(IStatement statement, IReadOnlyList<Parameter>? parameters = null)
```

- `statement` — [IStatement](./statements.md#istatement)
- `parameters` — optional [Parameter](#parameter) list

**Returns** `Task<[Response](#response)>`

#### Native

```csharp
public readonly record struct Native(string Text)
```

**Constructor**

##### Native(Text)

Engine text. [N](#native-n-text) builds the same value.

```csharp
Native(string Text)
```

- `Text` — provider command text

**Returns** [Native](#native)

**Properties**

##### Text

Provider command text.

```csharp
string Text { get; }
```

**Methods**

##### N(text)

Builds a [Native](#native).

```csharp
static Native N(string text)
```

- `text` — provider command text

**Returns** [Native](#native)

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
public sealed record Response
```

**Constructor**

##### Response(result, rowsAffected, parameters?)

Builds a [Response](#response). Copies a [ReturnValue](#parameterdirection) onto [Result](#response-result) when `result` is `Value.Null`.

```csharp
public Response(Value result, int rowsAffected, IReadOnlyList<Parameter>? parameters = null)
```

- `result` — scalar or row result
- `rowsAffected` — rows inserted, updated, or deleted
- `parameters` — bound [Parameter](#parameter) list

**Returns** [Response](#response)

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

Parses `text` with [StatementParser::parse](./statements.md) and runs [execute_reader_statement](#command-execute_reader_statement-statement-parameters). Blank text runs [execute_reader_native](#command-execute_reader_native-text-parameters).

```rust
async fn execute_reader(&self, text: &str, parameters: Option<&[Parameter]>) -> Result<Box<dyn Reader>>
```

- `text` — SQL parsed as a statement
- `parameters` — optional [Parameter](#parameter) slice

**Returns** `Result<Box<dyn `[Reader](./reader.md)`>>`

##### execute(text, parameters)

Parses `text` with [StatementParser::parse](./statements.md) and runs [execute_statement](#command-execute_statement-statement-parameters). Blank text runs [execute_native](#command-execute_native-text-parameters).

```rust
async fn execute(&self, text: &str, parameters: Option<&[Parameter]>) -> Result<Response>
```

- `text` — SQL parsed as a statement
- `parameters` — optional [Parameter](#parameter) slice

**Returns** `Result<[Response](#response)>`

##### execute_reader_native(text, parameters)

Runs provider text without parsing it.

```rust
async fn execute_reader_native(&self, text: &Native, parameters: Option<&[Parameter]>) -> Result<Box<dyn Reader>>
```

- `text` — [Native](#native) engine text
- `parameters` — optional [Parameter](#parameter) slice

**Returns** `Result<Box<dyn `[Reader](./reader.md)`>>`

##### execute_native(text, parameters)

Runs provider text without parsing it.

```rust
async fn execute_native(&self, text: &Native, parameters: Option<&[Parameter]>) -> Result<Response>
```

- `text` — [Native](#native) engine text
- `parameters` — optional [Parameter](#parameter) slice

**Returns** `Result<[Response](#response)>`

##### execute_reader_statement(statement, parameters)

Runs the statement. The default renders [Native](#native) from `statement.get_text()`.

```rust
async fn execute_reader_statement(&self, statement: &dyn Statement, parameters: Option<&[Parameter]>) -> Result<Box<dyn Reader>>
```

- `statement` — [Statement](./statements.md#statement)
- `parameters` — optional [Parameter](#parameter) slice

**Returns** `Result<Box<dyn `[Reader](./reader.md)`>>`

##### execute_statement(statement, parameters)

Runs the statement. The default renders [Native](#native) from `statement.get_text()`.

```rust
async fn execute_statement(&self, statement: &dyn Statement, parameters: Option<&[Parameter]>) -> Result<Response>
```

- `statement` — [Statement](./statements.md#statement)
- `parameters` — optional [Parameter](#parameter) slice

**Returns** `Result<[Response](#response)>`

#### Native

```rust
pub struct Native(String)
```

**Methods**

##### new(text)

Engine text. [n](#n-text) builds the same value.

```rust
pub fn new(text: impl Into<String>) -> Self
```

- `text` — provider command text

**Returns** [Native](#native)

##### text()

Provider command text.

```rust
pub fn text(&self) -> &str
```

**Returns** `&str`

##### n(text)

Builds a [Native](#native).

```rust
pub fn n(text: impl Into<String>) -> Native
```

- `text` — provider command text

**Returns** [Native](#native)

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

**Constructor**

##### new(result, rows_affected, parameters)

Builds a [Response](#response). Copies a [ReturnValue](#parameterdirection) onto [result](#response-result) when `result` is `Value::Null`.

```rust
pub fn new(result: impl Into<Value>, rows_affected: i64, parameters: Option<Vec<Parameter>>) -> Self
```

- `result` — scalar or row result
- `rows_affected` — rows inserted, updated, or deleted
- `parameters` — bound [Parameter](#parameter) list

**Returns** [Response](#response)

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

:::

## Remarks

The connection must be open before {{Execute|execute|execute}} or {{ExecuteReader|executeReader|execute_reader}}.

Use {{AddMiddleware|addMiddleware|add_middleware}} to add low-level diagnostics. It runs {{BeforeCommand|beforeCommand|before_command}}, then the command, then {{AfterCommand|afterCommand|after_command}}. A before error skips the command. A command error still runs after, with the error and elapsed time set.

`json` and `memory` treat `text` as insert, update, or delete. SQLite, MySQL, and Postgres treat `text` as SQL.

::: prose
### rust
The `Command` trait has no connection getter. Stub `create_command` stores the optional connection on the named type; working dialects still construct their own typed connection.
:::

{{ExecuteReader|executeReader|execute_reader}} returns a [Reader](./reader.md) for the first result set. Convert with [MetaDataRowTypeConverter](./metadata-row-type-converter.md), then map with a [Mapper](./mappers.md).

[Parameter](#parameter) defaults to [Input](#parameterdirection). [Output](#parameterdirection), [InputOutput](#parameterdirection), and [ReturnValue](#parameterdirection) are filled on [Response.Parameters](#response-parameters) after {{Execute|execute|execute}}. A [ReturnValue](#parameterdirection) is copied onto [Response.Result](#response-result) when `Result` would otherwise be null.

## Examples

### Run executeReader on a simple list

::: example
```csharp csharp/Providers/Sqlite.Tests/SqliteCommandIntegrationTests.cs#RunExecuteReaderOnASimpleList
```
```rust rust/providers/sqlite/tests/command.rs#run_execute_reader_on_a_simple_list
```
```typescript typescript/providers/sqlite/tests/sqlite.integration.test.ts#run-executereader-on-a-simple-list
```

```output [Rows]
[
  { id: 1, name: 'ada' },
  { id: 2, name: 'grace' },
  { id: 3, name: 'bea' }
]
```
:::

### Run ExecuteReader and return NameValues

::: example
```csharp csharp/Providers/Sqlite.Tests/SqliteCommandIntegrationTests.cs#NameValue
```
```csharp csharp/Providers/Sqlite.Tests/SqliteCommandIntegrationTests.cs#RunExecuteReaderAndReturnNameValues
```
```rust rust/providers/sqlite/tests/command.rs#ConvertSchemaForNameValue
```
```rust rust/providers/sqlite/tests/command.rs#run_execute_reader_and_return_name_values
```
```typescript typescript/providers/sqlite/tests/sqlite.integration.test.ts#NameValue
```
```typescript typescript/providers/sqlite/tests/sqlite.integration.test.ts#run-executereader-and-return-namevalues
```

```output [NameValues]
[
  { name: 'theme', value: 'dark' },
  { name: 'lang', value: 'en' },
  { name: 'size', value: 'lg' }
]
```
:::

### Execute, Insert, Read, Update, and Delete

::: example
```csharp csharp/Providers/Sqlite.Tests/SqliteCommandIntegrationTests.cs#Item
```
```csharp csharp/Providers/Sqlite.Tests/SqliteCommandIntegrationTests.cs#ExecuteInsertReadUpdateAndDelete
```
```rust rust/providers/sqlite/tests/command.rs#ConvertSchemaForItem
```
```rust rust/providers/sqlite/tests/command.rs#execute_insert_read_update_and_delete
```
```typescript typescript/providers/sqlite/tests/sqlite.integration.test.ts#Item
```
```typescript typescript/providers/sqlite/tests/sqlite.integration.test.ts#execute-insert-read-update-and-delete
```

```output [After: insert]
[
  { id: 1, name: 'a' }
]
```

```output [Update]
{ rowsAffected: 1 }
```

```output [After: update]
[
  { id: 1, name: 'A' }
]
```

```output [After: delete]
[]
```
:::

### Map flat contacts

::: example
```csharp csharp/Providers/Sqlite.Tests/SqliteCommandIntegrationTests.cs#User
```
```csharp csharp/Providers/Sqlite.Tests/SqliteCommandIntegrationTests.cs#UserContactsMapper
```
```csharp csharp/Providers/Sqlite.Tests/SqliteCommandIntegrationTests.cs#ExecuteReaderMapsFlatContacts
```
```rust rust/providers/sqlite/tests/command.rs#user_contacts_mapper
```
```rust rust/providers/sqlite/tests/command.rs#execute_reader_maps_flat_contacts
```
```typescript typescript/providers/sqlite/tests/sqlite.integration.test.ts#Contact
```
```typescript typescript/providers/sqlite/tests/sqlite.integration.test.ts#User
```
```typescript typescript/providers/sqlite/tests/sqlite.integration.test.ts#userContactsMapper
```
```typescript typescript/providers/sqlite/tests/sqlite.integration.test.ts#maps-flat-contacts
```

```output [User]
{
  id: 1,
  username: 'bobby',
  email: 'bob@asdf.com',
  contacts: [
    { name: 'bob' },
    { name: 'roberta' }
  ]
}
```
:::

### Fill an output parameter

::: example
```csharp csharp/Providers/Mysql.Tests/MysqlCommandIntegrationTests.cs#FillOutputParameter
```
```rust rust/providers/mysql/tests/integration.rs#fill_output_parameter
```
```typescript typescript/providers/mysql/tests/mysql.integration.test.ts#fillOutputParameter
```

```output [Parameters]
result: 5
```
:::

### Read a return value

::: example
```csharp csharp/Providers/Postgres.Tests/PostgresCommandIntegrationTests.cs#ReadReturnValue
```
```rust rust/providers/postgres/tests/integration.rs#read_return_value
```
```typescript typescript/providers/postgres/tests/postgres.integration.test.ts#readReturnValue
```

```output [Result]
5
```
:::
