# Statements

Command text. A string passed to [Command](./command.md) {{Execute|execute|execute}} or {{ExecuteReader|executeReader|execute_reader}} is parsed with {{StatementParser.Parse|StatementParser.parse|StatementParser::parse}}. {{Native|Native|Native}} and {{N|n|n}} send that text to the provider engine. A statement overload renders with {{GetText()|getText()|get_text()}} after the provider transforms it, then runs {{Native|Native|Native}}.

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
:::

## Remarks

Bound values live on [Where](./where-clause.md). [IStatement](#istatement) is not an [IExpression](./expressions.md#iexpression); [IText](#itext) is the shared {{GetText()|getText()|get_text()}} surface. [IWhere](./where-clause.md#iwhere) is a clause ([IExpression](./expressions.md#iexpression) + [IText](#itext)), not an [IStatement](#istatement).

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
