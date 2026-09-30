# IdentityMapper

Mapper that returns the value unchanged.

## Classes

::: reference
### typescript
#### IdentityMapper

```typescript
class IdentityMapper<T> implements IMapper<T, T>
```

**Methods**

##### map(value)

Returns `value`.

```typescript
map(value: T): T
```

- `value` — source value

**Returns** `T`

##### reverseMap(value)

Returns `value`.

```typescript
reverseMap(value: T): T
```

- `value` — mapped value

**Returns** `T`

### csharp
#### IdentityMapper

```csharp
public class IdentityMapper<T> : IMapper<T, T>
```

**Methods**

##### Map(value)

Returns `value`.

```csharp
T Map(T value)
```

- `value` — source value

**Returns** `T`

##### ReverseMap(value)

Returns `value`.

```csharp
T ReverseMap(T value)
```

- `value` — mapped value

**Returns** `T`

### rust
#### IdentityMapper

```rust
pub struct IdentityMapper<T>
```

**Constructor**

##### new()

Builds an identity mapper.

```rust
pub fn new() -> Self
```

**Returns** [IdentityMapper](#identitymapper)

**Methods**

##### map(value)

Returns `value`.

```rust
fn map(&self, value: T) -> T
```

- `value` — source value

**Returns** `T`

##### reverse_map(value)

Returns `value`.

```rust
fn reverse_map(&self, value: T) -> T
```

- `value` — mapped value

**Returns** `T`
:::

## Examples

### Map and reverse map

{{Map|map|map}} and {{ReverseMap|reverseMap|reverse_map}} return the value unchanged.

::: example
```csharp csharp/Datasources.Tests/IdentityMapperTests.cs#MapsAndReversesTheSameValue
```
```rust rust/datasources/src/mappers/identity_mapper.rs#maps_and_reverses_the_same_value
```
```typescript typescript/datasources/src/mappers/IdentityMapper.test.ts#maps-and-reverses-the-same-value
```

```output [Map]
contact
```

```output [Reverse map]
contact
```
:::
