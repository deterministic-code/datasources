<!-- Generated from docs/. Edit the markdown in docs/ and run npm run docs:skill. -->

# Command

Executes a command against the datasource and returns a result or a streaming [Reader](reader.md).

## Classes

#### TypeScript

#### ICommand {#typescript-icommand}

```typescript
interface ICommand
```

**Properties**

| Name | Description |
| --- | --- |
| connection | The [IConnection](docs/sections/connection.md) this command is bound to. |

**Methods**

| Name | Description |
| --- | --- |
| [addMiddleware(middleware)](#typescript-icommand-addmiddleware-middleware) | Adds [ICommandMiddleware](#typescript-icommandmiddleware). Returns this command. |
| [executeReader(text, parameters?)](#typescript-icommand-executereader-text-parameters) | Parses `text` with [StatementParser.parse](statements.md) and runs [executeReaderStatement](#typescript-icommand-executereaderstatement-statement-parameters). Blank text runs [executeReaderNative](#typescript-icommand-executereadernative-text-parameters). |
| [execute(text, parameters?)](#typescript-icommand-execute-text-parameters) | Parses `text` with [StatementParser.parse](statements.md) and runs [executeStatement](#typescript-icommand-executestatement-statement-parameters). Blank text runs [executeNative](#typescript-icommand-executenative-text-parameters). |
| [executeReaderNative(text, parameters?)](#typescript-icommand-executereadernative-text-parameters) | Runs provider text without parsing it. |
| [executeNative(text, parameters?)](#typescript-icommand-executenative-text-parameters) | Runs provider text without parsing it. |
| [execute(command, statement, parameters?)](#typescript-icommand-execute-command-statement-parameters) | Runs [executeStatement](#typescript-icommand-executestatement-statement-parameters) when the command implements it, otherwise [executeNative](#typescript-icommand-executenative-text-parameters) with `DefaultStatementTextRenderer.instance.render(statement)`. |
| [executeReader(command, statement, parameters?)](#typescript-icommand-executereader-command-statement-parameters) | Runs [executeReaderStatement](#typescript-icommand-executereaderstatement-statement-parameters) when the command implements it, otherwise [executeReaderNative](#typescript-icommand-executereadernative-text-parameters) with `DefaultStatementTextRenderer.instance.render(statement)`. |

##### addMiddleware(middleware) {#typescript-icommand-addmiddleware-middleware}

Adds [ICommandMiddleware](#typescript-icommandmiddleware). Returns this command.

**Definition**

```typescript
addMiddleware(middleware: ICommandMiddleware): this
```

**Parameters**

| Name | Description |
| --- | --- |
| `middleware` | [ICommandMiddleware](#typescript-icommandmiddleware) |

**Returns**

`this`

##### executeReader(text, parameters?) {#typescript-icommand-executereader-text-parameters}

Parses `text` with [StatementParser.parse](statements.md) and runs [executeReaderStatement](#typescript-icommand-executereaderstatement-statement-parameters). Blank text runs [executeReaderNative](#typescript-icommand-executereadernative-text-parameters).

**Definition**

```typescript
executeReader(text: string, parameters?: Parameter[]): Promise<IReader<Cells>>
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | SQL parsed as a statement |
| `parameters` | optional [Parameter](#typescript-parameter) list |

**Returns**

`Promise<[IReader](reader.md)<Cells>>`

##### execute(text, parameters?) {#typescript-icommand-execute-text-parameters}

Parses `text` with [StatementParser.parse](statements.md) and runs [executeStatement](#typescript-icommand-executestatement-statement-parameters). Blank text runs [executeNative](#typescript-icommand-executenative-text-parameters).

**Definition**

```typescript
execute(text: string, parameters?: Parameter[]): Promise<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | SQL parsed as a statement |
| `parameters` | optional [Parameter](#typescript-parameter) list |

**Returns**

`Promise<[Response](#typescript-response)>`

##### executeReaderNative(text, parameters?) {#typescript-icommand-executereadernative-text-parameters}

Runs provider text without parsing it.

**Definition**

```typescript
executeReaderNative(text: Native, parameters?: Parameter[]): Promise<IReader<Cells>>
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | [Native](#typescript-native) engine text |
| `parameters` | optional [Parameter](#typescript-parameter) list |

**Returns**

`Promise<[IReader](reader.md)<Cells>>`

##### executeNative(text, parameters?) {#typescript-icommand-executenative-text-parameters}

Runs provider text without parsing it.

**Definition**

```typescript
executeNative(text: Native, parameters?: Parameter[]): Promise<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | [Native](#typescript-native) engine text |
| `parameters` | optional [Parameter](#typescript-parameter) list |

**Returns**

`Promise<[Response](#typescript-response)>`

##### execute(command, statement, parameters?) {#typescript-icommand-execute-command-statement-parameters}

Runs [executeStatement](#typescript-icommand-executestatement-statement-parameters) when the command implements it, otherwise [executeNative](#typescript-icommand-executenative-text-parameters) with `DefaultStatementTextRenderer.instance.render(statement)`.

**Definition**

```typescript
function execute(command: ICommand, statement: IStatement, parameters?: Parameter[]): Promise<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `command` | [ICommand](#typescript-icommand) |
| `statement` | [IStatement](statements.md#typescript-istatement) |
| `parameters` | optional [Parameter](#typescript-parameter) list |

**Returns**

`Promise<[Response](#typescript-response)>`

##### executeReader(command, statement, parameters?) {#typescript-icommand-executereader-command-statement-parameters}

Runs [executeReaderStatement](#typescript-icommand-executereaderstatement-statement-parameters) when the command implements it, otherwise [executeReaderNative](#typescript-icommand-executereadernative-text-parameters) with `DefaultStatementTextRenderer.instance.render(statement)`.

**Definition**

```typescript
function executeReader(command: ICommand, statement: IStatement, parameters?: Parameter[]): Promise<IReader<Cells>>
```

**Parameters**

| Name | Description |
| --- | --- |
| `command` | [ICommand](#typescript-icommand) |
| `statement` | [IStatement](statements.md#typescript-istatement) |
| `parameters` | optional [Parameter](#typescript-parameter) list |

**Returns**

`Promise<[IReader](reader.md)<Cells>>`

#### Native {#typescript-native}

```typescript
class Native
```

**Constructor**

| Name | Description |
| --- | --- |
| [Native(text)](#typescript-native-native-text) | Engine text. `n(text)` builds the same value. |

**Properties**

| Name | Description |
| --- | --- |
| text | Provider command text. |

##### Native(text) {#typescript-native-native-text}

Engine text. `n(text)` builds the same value.

**Definition**

```typescript
constructor(text: string)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | provider command text |

**Returns**

[Native](#typescript-native)

#### ICommandMiddleware {#typescript-icommandmiddleware}

```typescript
interface ICommandMiddleware
```

**Methods**

| Name | Description |
| --- | --- |
| [beforeCommand(text, params?)](#typescript-icommandmiddleware-beforecommand-text-params) | Runs before the command. A thrown error skips the command. A `{ text, params }` return rewrites it. |
| [afterCommand(text, params, results, elapsedMs, error?)](#typescript-icommandmiddleware-aftercommand-text-params-results-elapsedms-error) | Runs after the command, including when the command failed. |

##### beforeCommand(text, params?) {#typescript-icommandmiddleware-beforecommand-text-params}

Runs before the command. A thrown error skips the command. A `{ text, params }` return rewrites it.

**Definition**

```typescript
beforeCommand(text: string, params?: ReadonlyArray<Parameter>): void | { text: string; params?: ReadonlyArray<Parameter> } | Promise<void | { text: string; params?: ReadonlyArray<Parameter> }>
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |
| `params` | optional [Parameter](#typescript-parameter) list |

##### afterCommand(text, params, results, elapsedMs, error?) {#typescript-icommandmiddleware-aftercommand-text-params-results-elapsedms-error}

Runs after the command, including when the command failed.

**Definition**

```typescript
afterCommand(text: string, params: ReadonlyArray<Parameter> | undefined, results: ReadonlyArray<Row>, elapsedMs: number, error?: Error): void | Promise<void>
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |
| `params` | [Parameter](#typescript-parameter) list used |
| `results` | rows produced |
| `elapsedMs` | elapsed milliseconds |
| `error` | command error when the command failed |

#### Parameter {#typescript-parameter}

```typescript
interface Parameter<T = Scalar>
```

**Properties**

| Name | Description |
| --- | --- |
| name | Parameter name. |
| value | Parameter value. Omitted for [output](#typescript-parameterdirection) / [returnValue](#typescript-parameterdirection). |
| direction | Bind direction. Omitted means [input](#typescript-parameterdirection). |

#### ParameterDirection {#typescript-parameterdirection}

```typescript
const ParameterDirection
```

#### Response {#typescript-response}

```typescript
class Response
```

**Constructor**

| Name | Description |
| --- | --- |
| [Response(result, rowsAffected, parameters?)](#typescript-response-response-result-rowsaffected-parameters) | Builds a [Response](#typescript-response). Copies a [returnValue](#typescript-parameterdirection) onto [result](#typescript-response-result) when `result` is null. |

**Properties**

| Name | Description |
| --- | --- |
| result | Scalar or row result from the command. |
| rowsAffected | Rows inserted, updated, or deleted. |
| parameters | Bound [Parameter](#typescript-parameter) list after [execute](#typescript-icommand-execute-text-parameters). Holds filled [output](#typescript-parameterdirection) / [returnValue](#typescript-parameterdirection) values. |

##### Response(result, rowsAffected, parameters?) {#typescript-response-response-result-rowsaffected-parameters}

Builds a [Response](#typescript-response). Copies a [returnValue](#typescript-parameterdirection) onto [result](#typescript-response-result) when `result` is null.

**Definition**

```typescript
constructor(result: unknown, rowsAffected: number, parameters?: Parameter[])
```

**Parameters**

| Name | Description |
| --- | --- |
| `result` | scalar or row result |
| `rowsAffected` | rows inserted, updated, or deleted |
| `parameters` | bound [Parameter](#typescript-parameter) list |

**Returns**

[Response](#typescript-response)

#### C#

#### ICommand {#csharp-icommand}

```csharp
public interface ICommand
```

**Properties**

| Name | Description |
| --- | --- |
| Connection | The [IConnection](docs/sections/connection.md) this command is bound to. |

**Methods**

| Name | Description |
| --- | --- |
| [AddMiddleware(middleware)](#csharp-icommand-addmiddleware-middleware) | Adds [ICommandMiddleware](#csharp-icommandmiddleware). Returns this command. |
| [ExecuteReader(text, parameters)](#csharp-icommand-executereader-text-parameters) | Parses `text` with [StatementParser.Parse](statements.md) and runs [ExecuteReader(statement)](#csharp-icommand-executereader-statement-parameters). Blank text runs [ExecuteReader(Native)](#csharp-icommand-executereader-native-parameters). |
| [Execute(text, parameters)](#csharp-icommand-execute-text-parameters) | Parses `text` with [StatementParser.Parse](statements.md) and runs [Execute(statement)](#csharp-icommand-execute-statement-parameters). Blank text runs [Execute(Native)](#csharp-icommand-execute-native-parameters). |
| [ExecuteReader(native, parameters)](#csharp-icommand-executereader-native-parameters) | Runs provider text without parsing it. |
| [Execute(native, parameters)](#csharp-icommand-execute-native-parameters) | Runs provider text without parsing it. |
| [ExecuteReader(statement, parameters)](#csharp-icommand-executereader-statement-parameters) | Runs the statement. The default renders [Native](#csharp-native) from `DefaultStatementTextRenderer.Instance.Render(statement)`. |
| [Execute(statement, parameters)](#csharp-icommand-execute-statement-parameters) | Runs the statement. The default renders [Native](#csharp-native) from `DefaultStatementTextRenderer.Instance.Render(statement)`. |

##### AddMiddleware(middleware) {#csharp-icommand-addmiddleware-middleware}

Adds [ICommandMiddleware](#csharp-icommandmiddleware). Returns this command.

**Definition**

```csharp
ICommand AddMiddleware(ICommandMiddleware middleware)
```

**Parameters**

| Name | Description |
| --- | --- |
| `middleware` | [ICommandMiddleware](#csharp-icommandmiddleware) |

**Returns**

[ICommand](#csharp-icommand)

##### ExecuteReader(text, parameters) {#csharp-icommand-executereader-text-parameters}

Parses `text` with [StatementParser.Parse](statements.md) and runs [ExecuteReader(statement)](#csharp-icommand-executereader-statement-parameters). Blank text runs [ExecuteReader(Native)](#csharp-icommand-executereader-native-parameters).

**Definition**

```csharp
Task<IReader<Row>> ExecuteReader(string text, IReadOnlyList<Parameter>? parameters = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | SQL parsed as a statement |
| `parameters` | optional [Parameter](#csharp-parameter) list |

**Returns**

`Task<[IReader](reader.md)<Row>>`

##### Execute(text, parameters) {#csharp-icommand-execute-text-parameters}

Parses `text` with [StatementParser.Parse](statements.md) and runs [Execute(statement)](#csharp-icommand-execute-statement-parameters). Blank text runs [Execute(Native)](#csharp-icommand-execute-native-parameters).

**Definition**

```csharp
Task<Response> Execute(string text, IReadOnlyList<Parameter>? parameters = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | SQL parsed as a statement |
| `parameters` | optional [Parameter](#csharp-parameter) list |

**Returns**

`Task<[Response](#csharp-response)>`

##### ExecuteReader(native, parameters) {#csharp-icommand-executereader-native-parameters}

Runs provider text without parsing it.

**Definition**

```csharp
Task<IReader<Row>> ExecuteReader(Native text, IReadOnlyList<Parameter>? parameters = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | [Native](#csharp-native) engine text |
| `parameters` | optional [Parameter](#csharp-parameter) list |

**Returns**

`Task<[IReader](reader.md)<Row>>`

##### Execute(native, parameters) {#csharp-icommand-execute-native-parameters}

Runs provider text without parsing it.

**Definition**

```csharp
Task<Response> Execute(Native text, IReadOnlyList<Parameter>? parameters = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | [Native](#csharp-native) engine text |
| `parameters` | optional [Parameter](#csharp-parameter) list |

**Returns**

`Task<[Response](#csharp-response)>`

##### ExecuteReader(statement, parameters) {#csharp-icommand-executereader-statement-parameters}

Runs the statement. The default renders [Native](#csharp-native) from `DefaultStatementTextRenderer.Instance.Render(statement)`.

**Definition**

```csharp
Task<IReader<Row>> ExecuteReader(IStatement statement, IReadOnlyList<Parameter>? parameters = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `statement` | [IStatement](statements.md#csharp-istatement) |
| `parameters` | optional [Parameter](#csharp-parameter) list |

**Returns**

`Task<[IReader](reader.md)<Row>>`

##### Execute(statement, parameters) {#csharp-icommand-execute-statement-parameters}

Runs the statement. The default renders [Native](#csharp-native) from `DefaultStatementTextRenderer.Instance.Render(statement)`.

**Definition**

```csharp
Task<Response> Execute(IStatement statement, IReadOnlyList<Parameter>? parameters = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `statement` | [IStatement](statements.md#csharp-istatement) |
| `parameters` | optional [Parameter](#csharp-parameter) list |

**Returns**

`Task<[Response](#csharp-response)>`

#### Native {#csharp-native}

```csharp
public readonly record struct Native(string Text)
```

**Constructor**

| Name | Description |
| --- | --- |
| [Native(Text)](#csharp-native-native-text) | Engine text. [N](#csharp-native-n-text) builds the same value. |

**Properties**

| Name | Description |
| --- | --- |
| Text | Provider command text. |

**Methods**

| Name | Description |
| --- | --- |
| [N(text)](#csharp-native-n-text) | Builds a [Native](#csharp-native). |

##### Native(Text) {#csharp-native-native-text}

Engine text. [N](#csharp-native-n-text) builds the same value.

**Definition**

```csharp
Native(string Text)
```

**Parameters**

| Name | Description |
| --- | --- |
| `Text` | provider command text |

**Returns**

[Native](#csharp-native)

##### N(text) {#csharp-native-n-text}

Builds a [Native](#csharp-native).

**Definition**

```csharp
static Native N(string text)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | provider command text |

**Returns**

[Native](#csharp-native)

#### ICommandMiddleware {#csharp-icommandmiddleware}

```csharp
public interface ICommandMiddleware
```

**Methods**

| Name | Description |
| --- | --- |
| [BeforeCommand(text, parameters)](#csharp-icommandmiddleware-beforecommand-text-parameters) | Runs before the command. A thrown error skips the command. A non-null return rewrites it. |
| [AfterCommand(text, parameters, results, elapsedMs, error)](#csharp-icommandmiddleware-aftercommand-text-parameters-results-elapsedms-error) | Runs after the command, including when the command failed. |

##### BeforeCommand(text, parameters) {#csharp-icommandmiddleware-beforecommand-text-parameters}

Runs before the command. A thrown error skips the command. A non-null return rewrites it.

**Definition**

```csharp
Task<(string Text, IReadOnlyList<Parameter>? Parameters)?> BeforeCommand(string text, IReadOnlyList<Parameter>? parameters)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |
| `parameters` | optional [Parameter](#csharp-parameter) list |

**Returns**

`Task<(string Text, IReadOnlyList<Parameter>? Parameters)?>`

##### AfterCommand(text, parameters, results, elapsedMs, error) {#csharp-icommandmiddleware-aftercommand-text-parameters-results-elapsedms-error}

Runs after the command, including when the command failed.

**Definition**

```csharp
Task AfterCommand(string text, IReadOnlyList<Parameter>? parameters, IReadOnlyList<Row> results, double elapsedMs, Exception? error = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |
| `parameters` | [Parameter](#csharp-parameter) list used |
| `results` | rows produced |
| `elapsedMs` | elapsed milliseconds |
| `error` | command error when the command failed |

#### Parameter {#csharp-parameter}

```csharp
public sealed record Parameter(string Name, Value Value, ParameterDirection Direction = ParameterDirection.Input)
```

**Properties**

| Name | Description |
| --- | --- |
| Name | Parameter name. |
| Value | Parameter value. `Value.Null` for output-only / return slots. |
| Direction | Bind direction. Default is [Input](#csharp-parameterdirection). |

**Constructor**

| Name | Description |
| --- | --- |
| [Parameter(name, direction)](#csharp-parameter-parameter-name-direction) | Output-only / return slot. [Value](#csharp-parameter-value) is `Value.Null`. |

##### Parameter(name, direction) {#csharp-parameter-parameter-name-direction}

Output-only / return slot. [Value](#csharp-parameter-value) is `Value.Null`.

**Definition**

```csharp
public Parameter(string name, ParameterDirection direction)
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | parameter name |
| `direction` | [Output](#csharp-parameterdirection) / [InputOutput](#csharp-parameterdirection) / [ReturnValue](#csharp-parameterdirection) |

#### ParameterDirection {#csharp-parameterdirection}

```csharp
public enum ParameterDirection
```

#### Response {#csharp-response}

```csharp
public sealed record Response
```

**Constructor**

| Name | Description |
| --- | --- |
| [Response(result, rowsAffected, parameters?)](#csharp-response-response-result-rowsaffected-parameters) | Builds a [Response](#csharp-response). Copies a [ReturnValue](#csharp-parameterdirection) onto [Result](#csharp-response-result) when `result` is `Value.Null`. |

**Properties**

| Name | Description |
| --- | --- |
| Result | Scalar or row result from the command. |
| RowsAffected | Rows inserted, updated, or deleted. |
| Parameters | Bound [Parameter](#csharp-parameter) list after [Execute](#csharp-icommand-execute-text-parameters). Holds filled [Output](#csharp-parameterdirection) / [ReturnValue](#csharp-parameterdirection) values. |

##### Response(result, rowsAffected, parameters?) {#csharp-response-response-result-rowsaffected-parameters}

Builds a [Response](#csharp-response). Copies a [ReturnValue](#csharp-parameterdirection) onto [Result](#csharp-response-result) when `result` is `Value.Null`.

**Definition**

```csharp
public Response(Value result, int rowsAffected, IReadOnlyList<Parameter>? parameters = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `result` | scalar or row result |
| `rowsAffected` | rows inserted, updated, or deleted |
| `parameters` | bound [Parameter](#csharp-parameter) list |

**Returns**

[Response](#csharp-response)

#### Rust

#### Command {#rust-command}

```rust
pub trait Command: Send + Sync
```

**Methods**

| Name | Description |
| --- | --- |
| [add_middleware(middleware)](#rust-command-add_middleware-middleware) | Adds [CommandMiddleware](#rust-commandmiddleware). |
| [execute_reader(text, parameters)](#rust-command-execute_reader-text-parameters) | Parses `text` with [StatementParser::parse](statements.md) and runs [execute_reader_statement](#rust-command-execute_reader_statement-statement-parameters). Blank text runs [execute_reader_native](#rust-command-execute_reader_native-text-parameters). |
| [execute(text, parameters)](#rust-command-execute-text-parameters) | Parses `text` with [StatementParser::parse](statements.md) and runs [execute_statement](#rust-command-execute_statement-statement-parameters). Blank text runs [execute_native](#rust-command-execute_native-text-parameters). |
| [execute_reader_native(text, parameters)](#rust-command-execute_reader_native-text-parameters) | Runs provider text without parsing it. |
| [execute_native(text, parameters)](#rust-command-execute_native-text-parameters) | Runs provider text without parsing it. |
| [execute_reader_statement(statement, parameters)](#rust-command-execute_reader_statement-statement-parameters) | Runs the statement. The default renders [Native](#rust-native) from `DefaultStatementTextRenderer::instance().render(statement)`. |
| [execute_statement(statement, parameters)](#rust-command-execute_statement-statement-parameters) | Runs the statement. The default renders [Native](#rust-native) from `DefaultStatementTextRenderer::instance().render(statement)`. |

##### add_middleware(middleware) {#rust-command-add_middleware-middleware}

Adds [CommandMiddleware](#rust-commandmiddleware).

**Definition**

```rust
fn add_middleware(&self, middleware: Arc<dyn CommandMiddleware>)
```

**Parameters**

| Name | Description |
| --- | --- |
| `middleware` | [CommandMiddleware](#rust-commandmiddleware) |

##### execute_reader(text, parameters) {#rust-command-execute_reader-text-parameters}

Parses `text` with [StatementParser::parse](statements.md) and runs [execute_reader_statement](#rust-command-execute_reader_statement-statement-parameters). Blank text runs [execute_reader_native](#rust-command-execute_reader_native-text-parameters).

**Definition**

```rust
async fn execute_reader(&self, text: &str, parameters: Option<&[Parameter]>) -> Result<Box<dyn Reader>>
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | SQL parsed as a statement |
| `parameters` | optional [Parameter](#rust-parameter) slice |

**Returns**

`Result<Box<dyn `[Reader](reader.md)`>>`

##### execute(text, parameters) {#rust-command-execute-text-parameters}

Parses `text` with [StatementParser::parse](statements.md) and runs [execute_statement](#rust-command-execute_statement-statement-parameters). Blank text runs [execute_native](#rust-command-execute_native-text-parameters).

**Definition**

```rust
async fn execute(&self, text: &str, parameters: Option<&[Parameter]>) -> Result<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | SQL parsed as a statement |
| `parameters` | optional [Parameter](#rust-parameter) slice |

**Returns**

`Result<[Response](#rust-response)>`

##### execute_reader_native(text, parameters) {#rust-command-execute_reader_native-text-parameters}

Runs provider text without parsing it.

**Definition**

```rust
async fn execute_reader_native(&self, text: &Native, parameters: Option<&[Parameter]>) -> Result<Box<dyn Reader>>
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | [Native](#rust-native) engine text |
| `parameters` | optional [Parameter](#rust-parameter) slice |

**Returns**

`Result<Box<dyn `[Reader](reader.md)`>>`

##### execute_native(text, parameters) {#rust-command-execute_native-text-parameters}

Runs provider text without parsing it.

**Definition**

```rust
async fn execute_native(&self, text: &Native, parameters: Option<&[Parameter]>) -> Result<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | [Native](#rust-native) engine text |
| `parameters` | optional [Parameter](#rust-parameter) slice |

**Returns**

`Result<[Response](#rust-response)>`

##### execute_reader_statement(statement, parameters) {#rust-command-execute_reader_statement-statement-parameters}

Runs the statement. The default renders [Native](#rust-native) from `DefaultStatementTextRenderer::instance().render(statement)`.

**Definition**

```rust
async fn execute_reader_statement(&self, statement: &dyn Statement, parameters: Option<&[Parameter]>) -> Result<Box<dyn Reader>>
```

**Parameters**

| Name | Description |
| --- | --- |
| `statement` | [Statement](statements.md#rust-statement) |
| `parameters` | optional [Parameter](#rust-parameter) slice |

**Returns**

`Result<Box<dyn `[Reader](reader.md)`>>`

##### execute_statement(statement, parameters) {#rust-command-execute_statement-statement-parameters}

Runs the statement. The default renders [Native](#rust-native) from `DefaultStatementTextRenderer::instance().render(statement)`.

**Definition**

```rust
async fn execute_statement(&self, statement: &dyn Statement, parameters: Option<&[Parameter]>) -> Result<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `statement` | [Statement](statements.md#rust-statement) |
| `parameters` | optional [Parameter](#rust-parameter) slice |

**Returns**

`Result<[Response](#rust-response)>`

#### Native {#rust-native}

```rust
pub struct Native(String)
```

**Methods**

| Name | Description |
| --- | --- |
| [new(text)](#rust-native-new-text) | Engine text. [n](#rust-n-text) builds the same value. |
| [text()](#rust-native-text) | Provider command text. |
| [n(text)](#rust-native-n-text) | Builds a [Native](#rust-native). |

##### new(text) {#rust-native-new-text}

Engine text. [n](#rust-n-text) builds the same value.

**Definition**

```rust
pub fn new(text: impl Into<String>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | provider command text |

**Returns**

[Native](#rust-native)

##### text() {#rust-native-text}

Provider command text.

**Definition**

```rust
pub fn text(&self) -> &str
```

**Returns**

`&str`

##### n(text) {#rust-native-n-text}

Builds a [Native](#rust-native).

**Definition**

```rust
pub fn n(text: impl Into<String>) -> Native
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | provider command text |

**Returns**

[Native](#rust-native)

#### CommandMiddleware {#rust-commandmiddleware}

```rust
pub trait CommandMiddleware: Send + Sync
```

**Methods**

| Name | Description |
| --- | --- |
| [before_command(text, params)](#rust-commandmiddleware-before_command-text-params) | Runs before the command. An `Err` skips the command. `Some` rewrites it. |
| [after_command(text, params, results, elapsed_ms, error)](#rust-commandmiddleware-after_command-text-params-results-elapsed_ms-error) | Runs after the command, including when the command failed. |

##### before_command(text, params) {#rust-commandmiddleware-before_command-text-params}

Runs before the command. An `Err` skips the command. `Some` rewrites it.

**Definition**

```rust
async fn before_command(&self, text: &str, params: Option<&[Parameter]>) -> Result<Option<(String, Option<Vec<Parameter>>)>>
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |
| `params` | optional [Parameter](#rust-parameter) slice |

**Returns**

`Result<Option<(String, Option<Vec<Parameter>>)>>`

##### after_command(text, params, results, elapsed_ms, error) {#rust-commandmiddleware-after_command-text-params-results-elapsed_ms-error}

Runs after the command, including when the command failed.

**Definition**

```rust
async fn after_command(&self, text: &str, params: Option<&[Parameter]>, results: &[Row], elapsed_ms: f64, error: Option<&DatasourceError>) -> Result<()>
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |
| `params` | [Parameter](#rust-parameter) slice used |
| `results` | rows produced |
| `elapsed_ms` | elapsed milliseconds |
| `error` | command error when the command failed |

**Returns**

`Result<()>`

#### Parameter {#rust-parameter}

```rust
pub struct Parameter<T = Value>
```

**Properties**

| Name | Description |
| --- | --- |
| name | Parameter name. |
| value | Parameter value. `Value::Null` for output-only / return slots. |
| direction | Bind direction. Default is [Input](#rust-parameterdirection). |

**Methods**

| Name | Description |
| --- | --- |
| [new(name, value)](#rust-parameter-new-name-value) | Input [Parameter](#rust-parameter). |
| [output(name)](#rust-parameter-output-name) | Output-only slot. [value](#rust-parameter-value) is `Value::Null`. |
| [return_value(name)](#rust-parameter-return_value-name) | Return slot. [value](#rust-parameter-value) is `Value::Null`. |
| [with_direction(name, value, direction)](#rust-parameter-with_direction-name-value-direction) | Parameter with an explicit [ParameterDirection](#rust-parameterdirection). |

##### new(name, value) {#rust-parameter-new-name-value}

Input [Parameter](#rust-parameter).

**Definition**

```rust
pub fn new(name: impl Into<String>, value: impl Into<Value>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | parameter name |
| `value` | parameter value |

**Returns**

[Parameter](#rust-parameter)

##### output(name) {#rust-parameter-output-name}

Output-only slot. [value](#rust-parameter-value) is `Value::Null`.

**Definition**

```rust
pub fn output(name: impl Into<String>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | parameter name |

**Returns**

[Parameter](#rust-parameter)

##### return_value(name) {#rust-parameter-return_value-name}

Return slot. [value](#rust-parameter-value) is `Value::Null`.

**Definition**

```rust
pub fn return_value(name: impl Into<String>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | parameter name |

**Returns**

[Parameter](#rust-parameter)

##### with_direction(name, value, direction) {#rust-parameter-with_direction-name-value-direction}

Parameter with an explicit [ParameterDirection](#rust-parameterdirection).

**Definition**

```rust
pub fn with_direction(name: impl Into<String>, value: impl Into<Value>, direction: ParameterDirection) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | parameter name |
| `value` | parameter value |
| `direction` | bind direction |

**Returns**

[Parameter](#rust-parameter)

#### ParameterDirection {#rust-parameterdirection}

```rust
pub enum ParameterDirection
```

#### Response {#rust-response}

```rust
pub struct Response
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(result, rows_affected, parameters)](#rust-response-new-result-rows_affected-parameters) | Builds a [Response](#rust-response). Copies a [ReturnValue](#rust-parameterdirection) onto [result](#rust-response-result) when `result` is `Value::Null`. |

**Properties**

| Name | Description |
| --- | --- |
| result | Scalar or row result from the command. |
| rows_affected | Rows inserted, updated, or deleted. |
| parameters | Bound [Parameter](#rust-parameter) list after [execute](#rust-command-execute-text-parameters). Holds filled [Output](#rust-parameterdirection) / [ReturnValue](#rust-parameterdirection) values. |

##### new(result, rows_affected, parameters) {#rust-response-new-result-rows_affected-parameters}

Builds a [Response](#rust-response). Copies a [ReturnValue](#rust-parameterdirection) onto [result](#rust-response-result) when `result` is `Value::Null`.

**Definition**

```rust
pub fn new(result: impl Into<Value>, rows_affected: i64, parameters: Option<Vec<Parameter>>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `result` | scalar or row result |
| `rows_affected` | rows inserted, updated, or deleted |
| `parameters` | bound [Parameter](#rust-parameter) list |

**Returns**

[Response](#rust-response)

## Remarks

The connection must be open before `Execute` / `execute` / `execute` or `ExecuteReader` / `executeReader` / `execute_reader`.

Use `AddMiddleware` / `addMiddleware` / `add_middleware` to add low-level diagnostics. It runs `BeforeCommand` / `beforeCommand` / `before_command`, then the command, then `AfterCommand` / `afterCommand` / `after_command`. A before error skips the command. A command error still runs after, with the error and elapsed time set.

`json` and `memory` treat `text` as insert, update, or delete. SQLite, MySQL, and Postgres treat `text` as SQL.

> **Rust**
>
> The `Command` trait has no connection getter. Stub `create_command` stores the optional connection on the named type; working dialects still construct their own typed connection.

`ExecuteReader` / `executeReader` / `execute_reader` returns a [Reader](reader.md) for the first result set. Convert with [MetaDataRowTypeConverter](type-converters.md), then map with a [Mapper](mappings.md).

[Parameter](#parameter) defaults to [Input](#parameterdirection). [Output](#parameterdirection), [InputOutput](#parameterdirection), and [ReturnValue](#parameterdirection) are filled on [Response.Parameters](#response-parameters) after `Execute` / `execute` / `execute`. A [ReturnValue](#parameterdirection) is copied onto [Response.Result](#response-result) when `Result` would otherwise be null.

## Examples

### Run executeReader on a simple list

#### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/SqliteCommandIntegrationTests.cs#L72-L90)

```csharp
var ds = Datasource.Default;
var command = await ds.CreateCommand();
await command.Execute(Native.N("INSERT INTO items (id, name) VALUES (1, 'ada')"));
await command.Execute(Native.N("INSERT INTO items (id, name) VALUES (2, 'grace')"));
await command.Execute(Native.N("INSERT INTO items (id, name) VALUES (3, 'bea')"));
await using var reader = await command.ExecuteReader("SELECT id, name FROM items");
var rows = await reader.Read().ToListAsync();
```

**Rows**

```
[
  { id: 1, name: 'ada' },
  { id: 2, name: 'grace' },
  { id: 3, name: 'bea' }
]
```

#### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/command.rs#L77-L107)

```rust
let ds = default().unwrap();
let command = ds.create_command(None).await.unwrap();
command
    .execute_native(&Native::new("INSERT INTO items (id, name) VALUES (1, 'ada')"), None)
    .await
    .unwrap();
command
    .execute_native(&Native::new("INSERT INTO items (id, name) VALUES (2, 'grace')"), None)
    .await
    .unwrap();
command
    .execute_native(&Native::new("INSERT INTO items (id, name) VALUES (3, 'bea')"), None)
    .await
    .unwrap();
let reader = command
    .execute_reader("SELECT id, name FROM items", None)
    .await
    .unwrap();
let rows: Vec<_> = reader.read().try_collect().await.unwrap();
```

**Rows**

```
[
  { id: 1, name: 'ada' },
  { id: 2, name: 'grace' },
  { id: 3, name: 'bea' }
]
```

#### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/sqlite.integration.test.ts#L91-L105)

```typescript
const ds = Datasource.default;
const command = await ds.createCommand();
await command.executeNative(new Native("INSERT INTO items (id, name) VALUES (1, 'ada')"));
await command.executeNative(new Native("INSERT INTO items (id, name) VALUES (2, 'grace')"));
await command.executeNative(new Native("INSERT INTO items (id, name) VALUES (3, 'bea')"));
await using reader = await command.executeReader("SELECT id, name FROM items");
const rows = await Array.fromAsync(reader.read());
```

**Rows**

```
[
  { id: 1, name: 'ada' },
  { id: 2, name: 'grace' },
  { id: 3, name: 'bea' }
]
```

Schema: [items](docs/examples.md#items)

```sql
CREATE TABLE items (id int, name string, CONSTRAINT pk_items PRIMARY KEY (id))
```

### Run ExecuteReader and return NameValues

#### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/SqliteCommandIntegrationTests.cs#L17-L21) [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/SqliteCommandIntegrationTests.cs#L138-L158)

```csharp
sealed class NameValue
{
    public string name { get; set; } = "";
    public string value { get; set; } = "";
}

var ds = Datasource.Default;
await using var connection = await ds.CreateConnection();
await connection.Open();
var command = connection.CreateCommand();
await command.Execute(Native.N("INSERT INTO settings (name, value) VALUES ('theme', 'dark')"));
await command.Execute(Native.N("INSERT INTO settings (name, value) VALUES ('lang', 'en')"));
await command.Execute(Native.N("INSERT INTO settings (name, value) VALUES ('size', 'lg')"));
await using var reader = await command.ExecuteReader("SELECT name, value FROM settings");
var converter = new MetaDataRowTypeConverter<NameValue>();
var items = await reader.Read().Select(row => converter.Convert(row)).ToListAsync();
```

**NameValues**

```
[
  { name: 'theme', value: 'dark' },
  { name: 'lang', value: 'en' },
  { name: 'size', value: 'lg' }
]
```

#### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/command.rs#L12-L16) [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/command.rs#L197-L243)

```rust
impl ConvertSchema for NameValue {
    fn schema() -> Vec<(&'static str, &'static str)> {
        vec![("name", "TEXT"), ("value", "TEXT")]
    }
}

let ds = default().unwrap();
let connection = ds.create_connection().await.unwrap();
connection.open().await.unwrap();
let command = connection.create_command();
command
    .execute_native(
        &Native::new("INSERT INTO settings (name, value) VALUES ('theme', 'dark')"),
        None,
    )
    .await
    .unwrap();
command
    .execute_native(
        &Native::new("INSERT INTO settings (name, value) VALUES ('lang', 'en')"),
        None,
    )
    .await
    .unwrap();
command
    .execute_native(
        &Native::new("INSERT INTO settings (name, value) VALUES ('size', 'lg')"),
        None,
    )
    .await
    .unwrap();
let reader = command
    .execute_reader("SELECT name, value FROM settings", None)
    .await
    .unwrap();
let converter = RowTypeConverter::of::<NameValue>();
let items: Vec<_> = reader
    .read()
    .map_ok(|row| converter.convert(&row).unwrap())
    .try_collect()
    .await
    .unwrap();
```

**NameValues**

```
[
  { name: 'theme', value: 'dark' },
  { name: 'lang', value: 'en' },
  { name: 'size', value: 'lg' }
]
```

#### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/sqlite.integration.test.ts#L30-L33) [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/sqlite.integration.test.ts#L146-L163)

```typescript
class NameValue {
  name = "";
  value = "";
}

const ds = Datasource.default;
await using connection = await ds.createConnection();
await connection.open();
const command = connection.createCommand();
await command.executeNative(new Native("INSERT INTO settings (name, value) VALUES ('theme', 'dark')"));
await command.executeNative(new Native("INSERT INTO settings (name, value) VALUES ('lang', 'en')"));
await command.executeNative(new Native("INSERT INTO settings (name, value) VALUES ('size', 'lg')"));
await using reader = await command.executeReader("SELECT name, value FROM settings");
const converter = new MetaDataRowTypeConverter<NameValue>();
const items = await Array.fromAsync(reader.read(), (row) => converter.convert(row, NameValue));
```

**NameValues**

```
[
  { name: 'theme', value: 'dark' },
  { name: 'lang', value: 'en' },
  { name: 'size', value: 'lg' }
]
```

Schema: [settings](docs/examples.md#settings)

```sql
CREATE TABLE settings (name string, value string)
```

### Execute, Insert, Read, Update, and Delete

#### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/SqliteCommandIntegrationTests.cs#L23-L27) [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/SqliteCommandIntegrationTests.cs#L160-L194)

```csharp
sealed class Item
{
    public long id { get; set; }
    public string name { get; set; } = "";
}

var ds = Datasource.Default;
await using var connection = await ds.CreateConnection();
await connection.Open();
var command = connection.CreateCommand();
await command.Execute(
    "INSERT INTO items (id, name) VALUES ($id, $name)",
    [new Parameter("id", 1), new Parameter("name", "a")]);
var converter = new MetaDataRowTypeConverter<Item>();
await using var createdReader = await command.ExecuteReader(
    "SELECT id, name FROM items WHERE id = $id",
    [new Parameter("id", 1)]);
var created = await createdReader.Read().Select(row => converter.Convert(row)).ToListAsync();
var response = await command.Execute(
    "UPDATE items SET name = $name WHERE id = $id",
    [new Parameter("id", 1), new Parameter("name", "A")]);
await using var updatedReader = await command.ExecuteReader(
    "SELECT id, name FROM items WHERE id = $id",
    [new Parameter("id", 1)]);
var updated = await updatedReader.Read().Select(row => converter.Convert(row)).ToListAsync();
await command.Execute("DELETE FROM items WHERE id = $id", [new Parameter("id", 1)]);
await using var goneReader = await command.ExecuteReader(
    "SELECT id, name FROM items WHERE id = $id",
    [new Parameter("id", 1)]);
var gone = await goneReader.Read().Select(row => converter.Convert(row)).ToListAsync();
```

**After: insert**

```
[
  { id: 1, name: 'a' }
]
```

**Update**

```
{ rowsAffected: 1 }
```

**After: update**

```
[
  { id: 1, name: 'A' }
]
```

**After: delete**

```
[]
```

#### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/command.rs#L20-L24) [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/command.rs#L245-L322)

```rust
impl ConvertSchema for Item {
    fn schema() -> Vec<(&'static str, &'static str)> {
        vec![("id", "INTEGER"), ("name", "TEXT")]
    }
}

let ds = default().unwrap();
let connection = ds.create_connection().await.unwrap();
connection.open().await.unwrap();
let command = connection.create_command();
command
    .execute(
        "INSERT INTO items (id, name) VALUES ($id, $name)",
        Some(&[Parameter::new("id", 1_i64), Parameter::new("name", "a")]),
    )
    .await
    .unwrap();
let converter = RowTypeConverter::of::<Item>();
let created_reader = command
    .execute_reader(
        "SELECT id, name FROM items WHERE id = $id",
        Some(&[Parameter::new("id", 1_i64)]),
    )
    .await
    .unwrap();
let created: Vec<_> = created_reader
    .read()
    .map_ok(|row| converter.convert(&row).unwrap())
    .try_collect()
    .await
    .unwrap();
drop(created_reader);
let response = command
    .execute(
        "UPDATE items SET name = $name WHERE id = $id",
        Some(&[Parameter::new("id", 1_i64), Parameter::new("name", "A")]),
    )
    .await
    .unwrap();
let updated_reader = command
    .execute_reader(
        "SELECT id, name FROM items WHERE id = $id",
        Some(&[Parameter::new("id", 1_i64)]),
    )
    .await
    .unwrap();
let updated: Vec<_> = updated_reader
    .read()
    .map_ok(|row| converter.convert(&row).unwrap())
    .try_collect()
    .await
    .unwrap();
drop(updated_reader);
command
    .execute(
        "DELETE FROM items WHERE id = $id",
        Some(&[Parameter::new("id", 1_i64)]),
    )
    .await
    .unwrap();
let gone_reader = command
    .execute_reader(
        "SELECT id, name FROM items WHERE id = $id",
        Some(&[Parameter::new("id", 1_i64)]),
    )
    .await
    .unwrap();
let gone: Vec<_> = gone_reader
    .read()
    .map_ok(|row| converter.convert(&row).unwrap())
    .try_collect()
    .await
    .unwrap();
```

**After: insert**

```
[
  { id: 1, name: 'a' }
]
```

**Update**

```
{ rowsAffected: 1 }
```

**After: update**

```
[
  { id: 1, name: 'A' }
]
```

**After: delete**

```
[]
```

#### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/sqlite.integration.test.ts#L35-L38) [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/sqlite.integration.test.ts#L165-L197)

```typescript
class Item {
  id = 0;
  name = "";
}

const ds = Datasource.default;
await using connection = await ds.createConnection();
await connection.open();
const command = connection.createCommand();
await command.execute("INSERT INTO items (id, name) VALUES ($id, $name)", [
  { name: "id", value: 1 },
  { name: "name", value: "a" },
]);
const converter = new MetaDataRowTypeConverter<Item>();
await using createdReader = await command.executeReader("SELECT id, name FROM items WHERE id = $id", [
  { name: "id", value: 1 },
]);
const created = await Array.fromAsync(createdReader.read(), (row) => converter.convert(row, Item));
const response = await command.execute("UPDATE items SET name = $name WHERE id = $id", [
  { name: "id", value: 1 },
  { name: "name", value: "A" },
]);
await using updatedReader = await command.executeReader("SELECT id, name FROM items WHERE id = $id", [
  { name: "id", value: 1 },
]);
const updated = await Array.fromAsync(updatedReader.read(), (row) => converter.convert(row, Item));
await command.execute("DELETE FROM items WHERE id = $id", [{ name: "id", value: 1 }]);
await using goneReader = await command.executeReader("SELECT id, name FROM items WHERE id = $id", [
  { name: "id", value: 1 },
]);
const gone = await Array.fromAsync(goneReader.read(), (row) => converter.convert(row, Item));
```

**After: insert**

```
[
  { id: 1, name: 'a' }
]
```

**Update**

```
{ rowsAffected: 1 }
```

**After: update**

```
[
  { id: 1, name: 'A' }
]
```

**After: delete**

```
[]
```

Schema: [items](docs/examples.md#items)

```sql
CREATE TABLE items (id int, name string, CONSTRAINT pk_items PRIMARY KEY (id))
```

### Map flat contacts

#### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/SqliteCommandIntegrationTests.cs#L29-L42) [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/SqliteCommandIntegrationTests.cs#L196-L229)

```csharp
record User
{
    public int id { get; set; }
    public string username { get; set; } = "";
    public string email { get; set; } = "";
    public string contact_name1 { get; set; } = "";
    public string contact_name2 { get; set; } = "";
    public List<Contact> contacts { get; set; } = [];

    public record Contact
    {
        public string name { get; set; } = "";
    }
}

var ds = Datasource.Default;
var command = await ds.CreateCommand();
await command.Execute(Native.N(
    "INSERT INTO users (id, username, email, contact_name1, contact_name2) VALUES (1, 'bobby', 'bob@asdf.com', 'bob', 'roberta')"));
await using var reader = await command.ExecuteReader(
    "SELECT contact_name1, contact_name2, email, id, username FROM users");
var converter = new MetaDataRowTypeConverter<User>();
TransformMapper<User, User> UserContactsMapper() =>
    new(
        row =>
        {
            row.contacts =
            [
                new User.Contact { name = row.contact_name1 },
                new User.Contact { name = row.contact_name2 }
            ];
            return row;
        },
        user =>
        {
            user.contact_name1 = user.contacts.ElementAtOrDefault(0)?.name ?? "";
            user.contact_name2 = user.contacts.ElementAtOrDefault(1)?.name ?? "";
            return user;
        });
var mapper = UserContactsMapper();
var users = await reader.Read().Select(row => mapper.Map(converter.Convert(row))).ToListAsync();
```

**User**

```
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

#### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/command.rs#L26-L28) [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/command.rs#L30-L37) [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/command.rs#L324-L419)

```rust
struct Contact {
    name: String,
}

struct User {
    id: i64,
    username: String,
    email: String,
    contact_name1: String,
    contact_name2: String,
    contacts: Vec<Contact>,
}

let ds = default().unwrap();
let command = ds.create_command(None).await.unwrap();
command
    .execute_native(
        &Native::new("INSERT INTO users (id, username, email, contact_name1, contact_name2)
         VALUES (1, 'bobby', 'bob@asdf.com', 'bob', 'roberta')"),
        None,
    )
    .await
    .unwrap();
let reader = command
    .execute_reader(
        "SELECT contact_name1, contact_name2, email, id, username FROM users",
        None,
    )
    .await
    .unwrap();
let converter = RowTypeConverter::of::<User>();
fn user_contacts_mapper() -> TransformMapper<Row, User> {
    fn text_of(row: &Row, key: &str) -> String {
        match row.get(key) {
            Some(Value::Text(text)) => text.clone(),
            _ => String::new(),
        }
    }
    TransformMapper::new(
        |row| {
            let contact_name1 = text_of(&row, "contact_name1");
            let contact_name2 = text_of(&row, "contact_name2");
            User {
                id: match row.get("id") {
                    Some(Value::Int(id)) => *id,
                    _ => 0,
                },
                username: text_of(&row, "username"),
                email: text_of(&row, "email"),
                contacts: vec![
                    Contact {
                        name: contact_name1.clone(),
                    },
                    Contact {
                        name: contact_name2.clone(),
                    },
                ],
                contact_name1,
                contact_name2,
            }
        },
        |mut user| {
            user.contact_name1 = user
                .contacts
                .first()
                .map(|contact| contact.name.clone())
                .unwrap_or_default();
            user.contact_name2 = user
                .contacts
                .get(1)
                .map(|contact| contact.name.clone())
                .unwrap_or_default();
            let mut row = Row::new();
            row.insert("id".into(), Value::Int(user.id));
            row.insert("username".into(), Value::from(user.username));
            row.insert("email".into(), Value::from(user.email));
            row.insert("contact_name1".into(), Value::from(user.contact_name1));
            row.insert("contact_name2".into(), Value::from(user.contact_name2));
            row
        },
    )
}
let mapper = user_contacts_mapper();
let users: Vec<_> = reader
    .read()
    .map_ok(|row| mapper.map(converter.convert(&row).unwrap()))
    .try_collect()
    .await
    .unwrap();
```

**User**

```
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

#### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/sqlite.integration.test.ts#L40-L42) [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/sqlite.integration.test.ts#L44-L51) [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/sqlite.integration.test.ts#L199-L232)

```typescript
class Contact {
  name = "";
}

class User {
  id = 0;
  username = "";
  email = "";
  contact_name1 = "";
  contact_name2 = "";
  contacts: Contact[] = [];
}

const ds = Datasource.default;
const command = await ds.createCommand();
await command.executeNative(
  new Native(
    "INSERT INTO users (id, username, email, contact_name1, contact_name2) VALUES (1, 'bobby', 'bob@asdf.com', 'bob', 'roberta')",
  ),
);
await using reader = await command.executeReader(
  "SELECT contact_name1, contact_name2, email, id, username FROM users",
);
const converter = new MetaDataRowTypeConverter<User>();
const userContactsMapper = () =>
  new TransformMapper<User, User>(
    (row) => {
      row.contacts = [
        Object.assign(new Contact(), { name: row.contact_name1 }),
        Object.assign(new Contact(), { name: row.contact_name2 }),
      ];
      return row;
    },
    (user) => {
      user.contact_name1 = user.contacts[0]?.name ?? "";
      user.contact_name2 = user.contacts[1]?.name ?? "";
      return user;
    },
  );
const mapper = userContactsMapper();
const users = await Array.fromAsync(reader.read(), (row) => mapper.map(converter.convert(row, User)));
```

**User**

```
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

Schema: [flat-contact-users](docs/examples.md#flat-contact-users)

```sql
CREATE TABLE users (id int, username string, email string, contact_name1 string, contact_name2 string, CONSTRAINT pk_users PRIMARY KEY (id))
```

### Fill an output parameter

#### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Mysql.Tests/MysqlCommandIntegrationTests.cs#L68-L80)

```csharp
await command.Execute(Native.N("""
    CREATE PROCEDURE bump(IN start_n INT, OUT result INT)
    BEGIN
      SET result = start_n + 1;
    END
    """));
var start = new Parameter("start_n", 4);
var result = new Parameter("result", ParameterDirection.Output);
var call = await command.Execute(new ExecuteStoredProcedureStatement("bump", [start, result]), [start, result]);
```

**Parameters**

```
result: 5
```

#### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/mysql/tests/integration.rs#L346-L370)

```rust
async fn fill_output_parameter(command: &dyn Command) {
    use datasources::{ExecuteStoredProcedureStatement, Parameter};
    command
        .execute_native(
            &Native::new("CREATE PROCEDURE bump(IN start_n INT, OUT result INT) BEGIN SET result = start_n + 1; END"),
            None,
        )
        .await
        .unwrap();
    let parameters = [
        Parameter::new("start_n", 4_i64),
        Parameter::output("result"),
    ];
    let call = command
        .execute_statement(
            &ExecuteStoredProcedureStatement::new("bump", &parameters),
            Some(&parameters),
        )
        .await
        .unwrap();
```

**Parameters**

```
result: 5
```

#### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/mysql/tests/mysql.integration.test.ts#L29-L43)

```typescript
async function fillOutputParameter(command: MysqlCommand) {
  await command.executeNative(new Native(`
    CREATE PROCEDURE bump(IN start_n INT, OUT result INT)
    BEGIN
      SET result = start_n + 1;
    END
  `));
  const start = { name: "start_n", value: 4 };
  const result = { name: "result", direction: ParameterDirection.Output };
  const call = await command.executeStatement(
    new ExecuteStoredProcedureStatement("bump", [start, result]),
    [start, result],
  );
```

**Parameters**

```
result: 5
```

### Read a return value

#### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Postgres.Tests/PostgresCommandIntegrationTests.cs#L56-L66)

```csharp
await command.Execute(Native.N("""
    CREATE FUNCTION add_one(n INT) RETURNS INT
    LANGUAGE sql
    AS $$ SELECT n + 1 $$
    """));
var n = new Parameter("n", 4);
var ret = await command.Execute(new ReturnStatement("add_one", [n]), [n]);
```

**Result**

```
5
```

#### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/postgres/tests/integration.rs#L333-L351)

```rust
async fn read_return_value(command: &dyn Command) {
    use datasources::{Parameter, ReturnStatement};
    command
        .execute_native(
            &Native::new("CREATE FUNCTION add_one(n INT) RETURNS INT LANGUAGE sql AS $$ SELECT n + 1 $$"),
            None,
        )
        .await
        .unwrap();
    let parameters = [Parameter::new("n", 4_i64)];
    let ret = command
        .execute_statement(
            &ReturnStatement::new("add_one", &parameters),
            Some(&parameters),
        )
        .await
        .unwrap();
```

**Result**

```
5
```

#### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/postgres/tests/postgres.integration.test.ts#L29-L38)

```typescript
async function readReturnValue(command: PostgresCommand) {
  await command.executeNative(new Native(`
    CREATE FUNCTION add_one(n INT) RETURNS INT
    LANGUAGE sql
    AS $$ SELECT n + 1 $$
  `));
  const n = { name: "n", value: 4 };
  const ret = await command.executeStatement(new ReturnStatement("add_one", [n]), [n]);
```

**Result**

```
5
```
