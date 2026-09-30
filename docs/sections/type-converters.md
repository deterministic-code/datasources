# Type Converters

`TypeConverter` is only {{Convert|convert|convert}} and {{ReverseConvert|reverseConvert|reverse_convert}}. A dialect adds a converter when the native provider cannot map the type. Core does not ship bool / datetime / uuid / decimal / binary converters.

## Classes

::: reference
### typescript
#### TypeConverter

```typescript
abstract class TypeConverter<To = Cells, From = Cells>
```

**Methods**

##### convert(from)

Converts `From` to `To`.

```typescript
abstract convert(from: From): To
```

- `from` — source value

**Returns** `To`

##### reverseConvert(to)

Converts `To` back to `From`.

```typescript
abstract reverseConvert(to: To): From
```

- `to` — converted value

**Returns** `From`

### csharp
#### ITypeConverter

```csharp
public interface ITypeConverter<To, From>
```

**Methods**

##### Convert(from)

Converts `From` to `To`.

```csharp
To Convert(From from)
```

- `from` — source value

**Returns** `To`

##### ReverseConvert(to)

Converts `To` back to `From`.

```csharp
From ReverseConvert(To to)
```

- `to` — converted value

**Returns** `From`

#### TypeConverter

```csharp
public abstract class TypeConverter<To, From> : ITypeConverter<To, From>
```

**Methods**

##### Convert(from)

Converts `From` to `To`.

```csharp
public abstract To Convert(From from)
```

- `from` — source value

**Returns** `To`

##### ReverseConvert(to)

Converts `To` back to `From`.

```csharp
public abstract From ReverseConvert(To to)
```

- `to` — converted value

**Returns** `From`

### rust
#### TypeConverter

```rust
pub trait TypeConverter<To, From>
```

**Methods**

##### convert(from)

Converts `From` to `To`.

```rust
fn convert(&self, from: &From) -> Result<To>
```

- `from` — source value

**Returns** `Result<To>`

##### reverse_convert(to)

Converts `To` back to `From`.

```rust
fn reverse_convert(&self, to: &To) -> Result<From>
```

- `to` — converted value

**Returns** `Result<From>`
:::

## Remarks

Repository read is convert, then map. See [TransformMapper](./transform-mapper.md) and [Repositories](./repository.md).

## See also

- [MetaDataRowTypeConverter](./metadata-row-type-converter.md)
