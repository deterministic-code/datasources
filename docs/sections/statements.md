# Statements

Command text. A string passed to [Command](./command.md) {{Execute|execute|execute}} or {{ExecuteReader|executeReader|execute_reader}} is parsed with {{StatementParser.Parse|StatementParser.parse|StatementParser::parse}}. {{Native|Native|Native}} and {{N|n|n}} send that text to the provider engine. A statement overload renders with {{StatementTextRenderer.Render|StatementTextRenderer.render|StatementTextRenderer::render}} — by default {{DefaultStatementTextRenderer.Instance|DefaultStatementTextRenderer.instance|DefaultStatementTextRenderer::instance()}} — after the provider transforms it, then runs {{Native|Native|Native}}.

## Classes

::: reference
### typescript
#### IStatement

```typescript
interface IStatement
```

**Methods**

##### accept(visitor)

Dispatches to a statement visitor that renders command text.

```typescript
accept(visitor: IStatementVisitor): string
```

- `visitor` — [IStatementVisitor](#istatementvisitor)

**Returns** `string`

#### StatementTextRenderer

```typescript
class StatementTextRenderer
```

Renders an [IStatement](#istatement) or [ICondition](./where-clause.md#icondition) to SQL.

**Constructor**

##### StatementTextRenderer(statements?, conditions?, expressions?)

Builds a renderer. Omitting factories uses [StatementRenderer](#statementrenderer) and [ConditionRenderer](./where-clause.md#conditionrenderer).

```typescript
constructor(
  statements?: (text: StatementTextRenderer) => StatementRenderer,
  conditions?: (expressions: ExpressionRenderer) => ConditionRenderer,
  expressions?: (text: StatementTextRenderer) => ExpressionRenderer,
)
```

- `statements` — optional factory for the statement visitor
- `conditions` — optional factory for the condition visitor
- `expressions` — optional factory for the expression renderer

**Methods**

##### render(statement)

Renders a statement to command text.

```typescript
render(statement: IStatement): string
```

- `statement` — [IStatement](#istatement)

**Returns** `string`

##### render(condition)

Renders a condition to predicate text.

```typescript
render(condition: ICondition): string
```

- `condition` — [ICondition](./where-clause.md#icondition)

**Returns** `string`

#### DefaultStatementTextRenderer

```typescript
class DefaultStatementTextRenderer extends StatementTextRenderer
```

Shared default renderer.

**Properties**

##### instance

```typescript
static readonly instance: DefaultStatementTextRenderer
```

#### IStatementVisitor

```typescript
interface IStatementVisitor
```

Visitor that turns each statement type into SQL. [StatementRenderer](#statementrenderer) is the default.

**Methods**

##### visit(statement)

```typescript
visit(statement: SelectStatement): string
visit(statement: InsertStatement): string
visit(statement: UpdateStatement): string
visit(statement: DeleteStatement): string
visit(statement: CreateTableStatement): string
visit(statement: CreateIndexStatement): string
visit(statement: ExecuteStoredProcedureStatement): string
visit(statement: ReturnStatement): string
visit(statement: SelectInsertedIdStatement): string
```

- `statement` — concrete [IStatement](#istatement)

**Returns** `string`

#### StatementRenderer

```typescript
class StatementRenderer implements IStatementVisitor
```

Default [IStatementVisitor](#istatementvisitor). Delegates each visit to a `*StatementTextRenderer`. Providers subclass this to override dialect text.

**Constructor**

##### StatementRenderer(text)

```typescript
constructor(text: StatementTextRenderer)
```

- `text` — [StatementTextRenderer](#statementtextrenderer)

### csharp
#### IStatement

```csharp
public interface IStatement
```

**Methods**

##### Accept(visitor)

Dispatches to a statement visitor that renders command text.

```csharp
string Accept(IStatementVisitor visitor)
```

- `visitor` — [IStatementVisitor](#istatementvisitor)

**Returns** `string`

#### StatementTextRenderer

```csharp
public class StatementTextRenderer
```

Renders an [IStatement](#istatement) or [ICondition](./where-clause.md#icondition) to SQL.

**Constructor**

##### StatementTextRenderer(statements, conditions, expressions)

Builds a renderer. Omitting factories uses [StatementRenderer](#statementrenderer) and [ConditionRenderer](./where-clause.md#conditionrenderer).

```csharp
public StatementTextRenderer(
    Func<StatementTextRenderer, StatementRenderer>? statements = null,
    Func<ExpressionRenderer, ConditionRenderer>? conditions = null,
    Func<StatementTextRenderer, ExpressionRenderer>? expressions = null)
```

- `statements` — optional factory for the statement visitor
- `conditions` — optional factory for the condition visitor
- `expressions` — optional factory for the expression renderer

**Methods**

##### Render(statement)

Renders a statement to command text.

```csharp
public string Render(IStatement statement)
```

- `statement` — [IStatement](#istatement)

**Returns** `string`

##### Render(condition)

Renders a condition to predicate text.

```csharp
public string Render(ICondition condition)
```

- `condition` — [ICondition](./where-clause.md#icondition)

**Returns** `string`

#### DefaultStatementTextRenderer

```csharp
public sealed class DefaultStatementTextRenderer : StatementTextRenderer
```

Shared default renderer.

**Constructor**

##### DefaultStatementTextRenderer()

```csharp
public DefaultStatementTextRenderer()
```

**Properties**

##### Instance

```csharp
public static DefaultStatementTextRenderer Instance { get; }
```

#### IStatementVisitor

```csharp
public interface IStatementVisitor
```

Visitor that turns each statement type into SQL. [StatementRenderer](#statementrenderer) is the default.

**Methods**

##### Visit(statement)

```csharp
string Visit(SelectStatement statement)
string Visit(InsertStatement statement)
string Visit(UpdateStatement statement)
string Visit(DeleteStatement statement)
string Visit(CreateTableStatement statement)
string Visit(CreateIndexStatement statement)
string Visit(ExecuteStoredProcedureStatement statement)
string Visit(ReturnStatement statement)
string Visit(SelectInsertedIdStatement statement)
```

- `statement` — concrete [IStatement](#istatement)

**Returns** `string`

#### StatementRenderer

```csharp
public class StatementRenderer : IStatementVisitor
```

Default [IStatementVisitor](#istatementvisitor). Delegates each visit to a `*StatementTextRenderer`. Providers subclass this to override dialect text.

**Constructor**

##### StatementRenderer(text)

```csharp
public StatementRenderer(StatementTextRenderer text)
```

- `text` — [StatementTextRenderer](#statementtextrenderer)

### rust
#### Statement

```rust
pub trait Statement: Send + Sync
```

**Methods**

##### accept()

Dispatches through the renderer's statement visitor.

```rust
fn accept(&self, text: &StatementTextRenderer) -> String
```

- `text` — [StatementTextRenderer](#statementtextrenderer)

**Returns** `String`

#### StatementTextRenderer

```rust
pub struct StatementTextRenderer
```

Renders a [Statement](#statement) or condition to SQL.

**Constructor**

##### new(statements, conditions)

Builds a renderer. `None` uses [StatementRenderer](#statementrenderer) and [ConditionRenderer](./where-clause.md#conditionrenderer).

```rust
pub fn new(
    statements: Option<Box<dyn StatementVisitor>>,
    conditions: Option<Box<dyn ConditionVisitor>>,
) -> Self
```

- `statements` — optional statement visitor
- `conditions` — optional condition visitor

**Returns** [StatementTextRenderer](#statementtextrenderer)

**Methods**

##### render()

Renders a statement to command text.

```rust
pub fn render(&self, statement: &dyn Statement) -> String
```

- `statement` — [Statement](#statement)

**Returns** `String`

##### render_condition() / render_where()

Renders a condition or where clause to predicate text.

```rust
pub fn render_condition(&self, condition: &dyn ICondition) -> String
pub fn render_where(&self, where_clause: &dyn IWhere) -> String
```

- `condition` — [ICondition](./where-clause.md#icondition)
- `where_clause` — [IWhere](./where-clause.md#iwhere)

**Returns** `String`

#### DefaultStatementTextRenderer

```rust
pub struct DefaultStatementTextRenderer
```

Shared default renderer.

**Methods**

##### instance()

```rust
pub fn instance() -> &'static StatementTextRenderer
```

**Returns** [StatementTextRenderer](#statementtextrenderer)

#### StatementVisitor

```rust
pub trait StatementVisitor: Send + Sync
```

Visitor that turns each statement type into SQL. [StatementRenderer](#statementrenderer) is the default. Providers implement this to override dialect text.

#### StatementRenderer

```rust
pub struct StatementRenderer
```

Default [StatementVisitor](#statementvisitor). Delegates each visit to a `*StatementTextRenderer`.
:::

## Remarks

Bound values live on [Where](./where-clause.md). [IStatement](#istatement) is not an [IExpression](./expressions.md#iexpression). Call {{Accept|accept|accept}} to dispatch; {{StatementTextRenderer.Render|StatementTextRenderer.render|StatementTextRenderer::render}} is the usual entry point for SQL text. [IWhere](./where-clause.md#iwhere) is a clause ([IExpression](./expressions.md#iexpression) + [ICondition](./where-clause.md#icondition)), not an [IStatement](#istatement).

## See also

- [SelectStatement](./select-statement.md)
- [InsertStatement](./insert-statement.md)
- [UpdateStatement](./update-statement.md)
- [DeleteStatement](./delete-statement.md)
- [CreateTableStatement](./create-table-statement.md)
- [CreateIndexStatement](./create-index-statement.md)
- [ExecuteStoredProcedureStatement](./execute-stored-procedure-statement.md)
- [ReturnStatement](./return-statement.md)
- [Where](./where-clause.md)
- [Expressions](./expressions.md)
- [ExpressionEvaluator](./expression-evaluator.md)
- [StatementEvaluator](./statement-evaluator.md)
