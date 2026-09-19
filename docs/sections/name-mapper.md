# NameMapper

Identity [IMapper](./mappers.md) for column or entity names. Names pass through as-is.

## Classes

::: reference
### typescript
#### NameMapper

```typescript
abstract class NameMapper implements IMapper<string, string>
```

**Methods**

##### map(name)

Returns the name unchanged.

```typescript
map(name: string): string
```

- `name` — column or entity name

**Returns** `string`

##### reverseMap(name)

Returns the name unchanged.

```typescript
reverseMap(name: string): string
```

- `name` — column or entity name

**Returns** `string`

### csharp
#### NameMapper

```csharp
public abstract class NameMapper : IMapper<string, string>
```

**Methods**

##### Map(name)

Returns the name unchanged.

```csharp
string Map(string name)
```

- `name` — column or entity name

**Returns** `string`

##### ReverseMap(name)

Returns the name unchanged.

```csharp
string ReverseMap(string name)
```

- `name` — column or entity name

**Returns** `string`

### rust
#### NameMapper

```rust
pub trait NameMapper
```

**Methods**

##### map(name)

Returns the name unchanged.

```rust
fn map(&self, name: &str) -> String
```

- `name` — column or entity name

**Returns** `String`

##### reverse_map(name)

Returns the name unchanged.

```rust
fn reverse_map(&self, name: &str) -> String
```

- `name` — column or entity name

**Returns** `String`
:::

## Remarks

Use [NameSourceDestinationMapper](./name-source-destination-mapper.md) when the mapper also holds source and destination labels.

## Examples

### Pass-through names

::: example
```csharp csharp/Datasources.Tests/NameMapperTests.cs#L15-L25
```
```rust rust/datasources/src/mappers/mod.rs#L107-L116
```
```typescript typescript/datasources/src/mappers/nameMapper.test.ts#L19-L27
```
:::
