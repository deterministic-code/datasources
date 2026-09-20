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
- `statement` — [IStatement](./builders.md)
- `parameters` — optional [Parameter](#parameter) list

**Returns** `Promise<[Response](#response)>`

##### executeReader(command, statement, parameters?)

Runs [executeReader](#icommand-executereader-text-parameters) with `statement.getText()`.

```typescript
function executeReader(command: ICommand, statement: IStatement, parameters?: Parameter[]): Promise<IReader<Cells>>
```

- `command` — [ICommand](#icommand)
- `statement` — [IStatement](./builders.md)
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
interface Parameter<T = unknown>
```

**Properties**

##### name

Parameter name.

```typescript
name: string
```

##### value

Parameter value.

```typescript
value: T
```

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

- `statement` — [IStatement](./builders.md)
- `parameters` — optional [Parameter](#parameter) list

**Returns** `Task<[IReader](#reader)<Row>>`

##### Execute(statement, parameters)

Runs [Execute](#icommand-execute-text-parameters) with `statement.GetText()`.

```csharp
Task<Response> Execute(IStatement statement, IReadOnlyList<Parameter>? parameters = null)
```

- `statement` — [IStatement](./builders.md)
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
public sealed record Parameter(string Name, Value Value)
```

**Properties**

##### Name

Parameter name.

```csharp
string Name { get; }
```

##### Value

Parameter value.

```csharp
Value Value { get; }
```

#### Response

```csharp
public sealed record Response(Value Result, int RowsAffected)
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

- `statement` — [Statement](./builders.md)
- `parameters` — optional [Parameter](#parameter) slice

**Returns** `Result<Box<dyn `[Reader](#reader)`>>`

##### execute_statement(statement, parameters)

Runs [execute](#command-execute-text-parameters) with `statement.get_text()`.

```rust
async fn execute_statement(&self, statement: &dyn Statement, parameters: Option<&[Parameter]>) -> Result<Response>
```

- `statement` — [Statement](./builders.md)
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

Parameter value.

```rust
pub value: T
```

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
:::

## Remarks

The connection must be open before `Execute` / `execute` / `execute` or `ExecuteReader` / `executeReader` / `execute_reader`.

`AddMiddleware` / `addMiddleware` / `add_middleware` runs `BeforeCommand` / `beforeCommand` / `before_command` then the command, then `AfterCommand` / `afterCommand` / `after_command`. A `beforeCommand` error skips the command. A command error still runs `afterCommand` with `error` and elapsed time set. `ConsoleCommandMiddleware` logs Start/Finish lines to the console. Verbose logs use AfterCommand for the result body.

Working dialects (`json`, `memory`) treat `text` as `insert` / `update` / `delete`. `sqlite` treats `text` as SQL. Stub dialects throw or return not-implemented on execute and execute-reader.

The Rust `Command` trait has no connection getter. Stub `create_command` stores the optional connection on the named type; working dialects still construct their own typed connection today.

`ExecuteReader` returns rows. Convert with [RowTypeConverter](./row-type-converter.md), then map with a [Mapper](./mappers.md).

## Examples

### ExecuteReader to NameValue

::: example
```csharp csharp/Datasources.Tests/CommandTests.cs#L46-L60
```
```rust rust/datasources/src/tests/command.rs#L60-L82
```
```typescript typescript/datasources/src/Command.test.ts#L45-L56
```
:::

### Execute insert, update, and delete

::: example
```csharp csharp/Datasources.Tests/CommandTests.cs#L62-L71
```
```rust rust/datasources/src/tests/command.rs#L84-L116
```
```typescript typescript/datasources/src/Command.test.ts#L58-L78
```
:::

### Map flat contacts

::: example
```csharp csharp/Datasources.Tests/CommandTests.cs#L73-L97
```
```rust rust/datasources/src/tests/command.rs#L118-L150
```
```typescript typescript/datasources/src/Command.test.ts#L80-L103
```
:::
