# Statements

Command text. [Command](./command.md) `Execute` / `ExecuteReader` overloads take any statement and use `GetText()` / `getText()` / `get_text()` only.

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

Bound values live on [Where](./where-clause.md). [IStatement](#istatement) is not an [IExpression](./expressions.md#iexpression); [IText](#itext) is the shared `GetText()` / `getText()` / `get_text()` surface. [IWhere](./where-clause.md#iwhere) is a clause ([IExpression](./expressions.md#iexpression) + [IText](#itext)), not an [IStatement](#istatement).

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
- [StatementEvaluator](./statement-evaluator.md)
