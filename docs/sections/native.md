# Native

Provider engine text. {{Execute|executeNative|execute_native}} and {{ExecuteReader|executeReaderNative|execute_reader_native}} send it without parsing. {{N|n|n}} builds the same value.

## Classes

::: reference
### typescript
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

### csharp
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

### rust
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
:::

## Remarks

A string passed to [Command](./command.md) {{Execute|execute|execute}} is parsed as a statement. Wrap the same text in {{Native.N|n|n}} when the provider should run it as written.
