<!-- Generated from docs/. Edit the markdown in docs/ and run npm run docs:skill. -->

# Statements

Command text. A string passed to [Command](command.md) `Execute` / `execute` / `execute` or `ExecuteReader` / `executeReader` / `execute_reader` is parsed with `StatementParser.Parse` / `StatementParser.parse` / `StatementParser::parse`. `Native` / `Native` / `Native` and `N` / `n` / `n` send that text to the provider engine. A statement overload renders with `StatementTextRenderer.Render` / `StatementTextRenderer.render` / `StatementTextRenderer::render` — by default `DefaultStatementTextRenderer.Instance` / `DefaultStatementTextRenderer.instance` / `DefaultStatementTextRenderer::instance()` — after the provider transforms it, then runs `Native` / `Native` / `Native`.

## Classes

#### TypeScript

#### IStatement {#typescript-istatement}

```typescript
interface IStatement
```

**Methods**

| Name | Description |
| --- | --- |
| [accept(visitor)](#typescript-istatement-accept-visitor) | Dispatches to a statement visitor that renders command text. |

##### accept(visitor) {#typescript-istatement-accept-visitor}

Dispatches to a statement visitor that renders command text.

**Definition**

```typescript
accept(visitor: IStatementVisitor): string
```

**Parameters**

| Name | Description |
| --- | --- |
| `visitor` | [IStatementVisitor](#typescript-istatementvisitor) |

**Returns**

`string`

#### StatementTextRenderer {#typescript-statementtextrenderer}

```typescript
class StatementTextRenderer
```

**Constructor**

| Name | Description |
| --- | --- |
| [StatementTextRenderer(statements?, conditions?, expressions?)](#typescript-statementtextrenderer-statementtextrenderer-statements-conditions-expressions) | Builds a renderer. Omitting factories uses [StatementRenderer](#typescript-statementrenderer) and [ConditionRenderer](where.md#typescript-conditionrenderer). |

**Methods**

| Name | Description |
| --- | --- |
| [render(statement)](#typescript-statementtextrenderer-render-statement) | Renders a statement to command text. |
| [render(condition)](#typescript-statementtextrenderer-render-condition) | Renders a condition to predicate text. |

##### StatementTextRenderer(statements?, conditions?, expressions?) {#typescript-statementtextrenderer-statementtextrenderer-statements-conditions-expressions}

Builds a renderer. Omitting factories uses [StatementRenderer](#typescript-statementrenderer) and [ConditionRenderer](where.md#typescript-conditionrenderer).

**Definition**

```typescript
constructor(
  statements?: (text: StatementTextRenderer) => StatementRenderer,
  conditions?: (expressions: ExpressionRenderer) => ConditionRenderer,
  expressions?: (text: StatementTextRenderer) => ExpressionRenderer,
)
```

**Parameters**

| Name | Description |
| --- | --- |
| `statements` | optional factory for the statement visitor |
| `conditions` | optional factory for the condition visitor |
| `expressions` | optional factory for the expression renderer |

##### render(statement) {#typescript-statementtextrenderer-render-statement}

Renders a statement to command text.

**Definition**

```typescript
render(statement: IStatement): string
```

**Parameters**

| Name | Description |
| --- | --- |
| `statement` | [IStatement](#typescript-istatement) |

**Returns**

`string`

##### render(condition) {#typescript-statementtextrenderer-render-condition}

Renders a condition to predicate text.

**Definition**

```typescript
render(condition: ICondition): string
```

**Parameters**

| Name | Description |
| --- | --- |
| `condition` | [ICondition](where.md#typescript-icondition) |

**Returns**

`string`

#### DefaultStatementTextRenderer {#typescript-defaultstatementtextrenderer}

```typescript
class DefaultStatementTextRenderer extends StatementTextRenderer
```

**Properties**

| Name | Description |
| --- | --- |
| instance |  |

#### IStatementVisitor {#typescript-istatementvisitor}

```typescript
interface IStatementVisitor
```

**Methods**

| Name | Description |
| --- | --- |
| [visit(statement)](#typescript-istatementvisitor-visit-statement) |  |

##### visit(statement) {#typescript-istatementvisitor-visit-statement}



**Definition**

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

**Parameters**

| Name | Description |
| --- | --- |
| `statement` | concrete [IStatement](#typescript-istatement) |

**Returns**

`string`

#### StatementRenderer {#typescript-statementrenderer}

```typescript
class StatementRenderer implements IStatementVisitor
```

**Constructor**

| Name | Description |
| --- | --- |
| [StatementRenderer(text)](#typescript-statementrenderer-statementrenderer-text) |  |

##### StatementRenderer(text) {#typescript-statementrenderer-statementrenderer-text}



**Definition**

```typescript
constructor(text: StatementTextRenderer)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | [StatementTextRenderer](#typescript-statementtextrenderer) |

#### C#

#### IStatement {#csharp-istatement}

```csharp
public interface IStatement
```

**Methods**

| Name | Description |
| --- | --- |
| [Accept(visitor)](#csharp-istatement-accept-visitor) | Dispatches to a statement visitor that renders command text. |

##### Accept(visitor) {#csharp-istatement-accept-visitor}

Dispatches to a statement visitor that renders command text.

**Definition**

```csharp
string Accept(IStatementVisitor visitor)
```

**Parameters**

| Name | Description |
| --- | --- |
| `visitor` | [IStatementVisitor](#csharp-istatementvisitor) |

**Returns**

`string`

#### StatementTextRenderer {#csharp-statementtextrenderer}

```csharp
public class StatementTextRenderer
```

**Constructor**

| Name | Description |
| --- | --- |
| [StatementTextRenderer(statements, conditions, expressions)](#csharp-statementtextrenderer-statementtextrenderer-statements-conditions-expressions) | Builds a renderer. Omitting factories uses [StatementRenderer](#csharp-statementrenderer) and [ConditionRenderer](where.md#csharp-conditionrenderer). |

**Methods**

| Name | Description |
| --- | --- |
| [Render(statement)](#csharp-statementtextrenderer-render-statement) | Renders a statement to command text. |
| [Render(condition)](#csharp-statementtextrenderer-render-condition) | Renders a condition to predicate text. |

##### StatementTextRenderer(statements, conditions, expressions) {#csharp-statementtextrenderer-statementtextrenderer-statements-conditions-expressions}

Builds a renderer. Omitting factories uses [StatementRenderer](#csharp-statementrenderer) and [ConditionRenderer](where.md#csharp-conditionrenderer).

**Definition**

```csharp
public StatementTextRenderer(
    Func<StatementTextRenderer, StatementRenderer>? statements = null,
    Func<ExpressionRenderer, ConditionRenderer>? conditions = null,
    Func<StatementTextRenderer, ExpressionRenderer>? expressions = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `statements` | optional factory for the statement visitor |
| `conditions` | optional factory for the condition visitor |
| `expressions` | optional factory for the expression renderer |

##### Render(statement) {#csharp-statementtextrenderer-render-statement}

Renders a statement to command text.

**Definition**

```csharp
public string Render(IStatement statement)
```

**Parameters**

| Name | Description |
| --- | --- |
| `statement` | [IStatement](#csharp-istatement) |

**Returns**

`string`

##### Render(condition) {#csharp-statementtextrenderer-render-condition}

Renders a condition to predicate text.

**Definition**

```csharp
public string Render(ICondition condition)
```

**Parameters**

| Name | Description |
| --- | --- |
| `condition` | [ICondition](where.md#csharp-icondition) |

**Returns**

`string`

#### DefaultStatementTextRenderer {#csharp-defaultstatementtextrenderer}

```csharp
public sealed class DefaultStatementTextRenderer : StatementTextRenderer
```

**Constructor**

| Name | Description |
| --- | --- |
| [DefaultStatementTextRenderer()](#csharp-defaultstatementtextrenderer-defaultstatementtextrenderer) |  |

**Properties**

| Name | Description |
| --- | --- |
| Instance |  |

##### DefaultStatementTextRenderer() {#csharp-defaultstatementtextrenderer-defaultstatementtextrenderer}



**Definition**

```csharp
public DefaultStatementTextRenderer()
```

#### IStatementVisitor {#csharp-istatementvisitor}

```csharp
public interface IStatementVisitor
```

**Methods**

| Name | Description |
| --- | --- |
| [Visit(statement)](#csharp-istatementvisitor-visit-statement) |  |

##### Visit(statement) {#csharp-istatementvisitor-visit-statement}



**Definition**

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

**Parameters**

| Name | Description |
| --- | --- |
| `statement` | concrete [IStatement](#csharp-istatement) |

**Returns**

`string`

#### StatementRenderer {#csharp-statementrenderer}

```csharp
public class StatementRenderer : IStatementVisitor
```

**Constructor**

| Name | Description |
| --- | --- |
| [StatementRenderer(text)](#csharp-statementrenderer-statementrenderer-text) |  |

##### StatementRenderer(text) {#csharp-statementrenderer-statementrenderer-text}



**Definition**

```csharp
public StatementRenderer(StatementTextRenderer text)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | [StatementTextRenderer](#csharp-statementtextrenderer) |

#### Rust

#### Statement {#rust-statement}

```rust
pub trait Statement: Send + Sync
```

**Methods**

| Name | Description |
| --- | --- |
| [accept()](#rust-statement-accept) | Dispatches through the renderer's statement visitor. |

##### accept() {#rust-statement-accept}

Dispatches through the renderer's statement visitor.

**Definition**

```rust
fn accept(&self, text: &StatementTextRenderer) -> String
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | [StatementTextRenderer](#rust-statementtextrenderer) |

**Returns**

`String`

#### StatementTextRenderer {#rust-statementtextrenderer}

```rust
pub struct StatementTextRenderer
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(statements, conditions)](#rust-statementtextrenderer-new-statements-conditions) | Builds a renderer. `None` uses [StatementRenderer](#rust-statementrenderer) and [ConditionRenderer](where.md#rust-conditionrenderer). |

**Methods**

| Name | Description |
| --- | --- |
| [render()](#rust-statementtextrenderer-render) | Renders a statement to command text. |
| [render_condition() / render_where()](#rust-statementtextrenderer-render_condition-render_where) | Renders a condition or where clause to predicate text. |

##### new(statements, conditions) {#rust-statementtextrenderer-new-statements-conditions}

Builds a renderer. `None` uses [StatementRenderer](#rust-statementrenderer) and [ConditionRenderer](where.md#rust-conditionrenderer).

**Definition**

```rust
pub fn new(
    statements: Option<Box<dyn StatementVisitor>>,
    conditions: Option<Box<dyn ConditionVisitor>>,
) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `statements` | optional statement visitor |
| `conditions` | optional condition visitor |

**Returns**

[StatementTextRenderer](#rust-statementtextrenderer)

##### render() {#rust-statementtextrenderer-render}

Renders a statement to command text.

**Definition**

```rust
pub fn render(&self, statement: &dyn Statement) -> String
```

**Parameters**

| Name | Description |
| --- | --- |
| `statement` | [Statement](#rust-statement) |

**Returns**

`String`

##### render_condition() / render_where() {#rust-statementtextrenderer-render_condition-render_where}

Renders a condition or where clause to predicate text.

**Definition**

```rust
pub fn render_condition(&self, condition: &dyn ICondition) -> String
pub fn render_where(&self, where_clause: &dyn IWhere) -> String
```

**Parameters**

| Name | Description |
| --- | --- |
| `condition` | [ICondition](where.md#rust-icondition) |
| `where_clause` | [IWhere](where.md#rust-iwhere) |

**Returns**

`String`

#### DefaultStatementTextRenderer {#rust-defaultstatementtextrenderer}

```rust
pub struct DefaultStatementTextRenderer
```

**Methods**

| Name | Description |
| --- | --- |
| [instance()](#rust-defaultstatementtextrenderer-instance) |  |

##### instance() {#rust-defaultstatementtextrenderer-instance}



**Definition**

```rust
pub fn instance() -> &'static StatementTextRenderer
```

**Returns**

[StatementTextRenderer](#rust-statementtextrenderer)

#### StatementVisitor {#rust-statementvisitor}

```rust
pub trait StatementVisitor: Send + Sync
```

#### StatementRenderer {#rust-statementrenderer}

```rust
pub struct StatementRenderer
```

## Remarks

Bound values live on [Where](where.md). [IStatement](#istatement) is not an [IExpression](expressions.md#iexpression). Call `Accept` / `accept` / `accept` to dispatch; `StatementTextRenderer.Render` / `StatementTextRenderer.render` / `StatementTextRenderer::render` is the usual entry point for SQL text. [IWhere](where.md#iwhere) is a clause ([IExpression](expressions.md#iexpression) + [ICondition](where.md#icondition)), not an [IStatement](#istatement).

## See also

- [SelectStatement](select-statement.md)
- [InsertStatement](insert-statement.md)
- [UpdateStatement](update-statement.md)
- [DeleteStatement](delete-statement.md)
- [CreateTableStatement](other-statements.md)
- [CreateIndexStatement](other-statements.md)
- [ExecuteStoredProcedureStatement](other-statements.md)
- [ReturnStatement](other-statements.md)
- [Where](where.md)
- [Expressions](expressions.md)
- [ExpressionEvaluator](expressions.md)
- [StatementEvaluator](expressions.md)
