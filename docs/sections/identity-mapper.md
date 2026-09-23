# IdentityMapper

Mapper that returns the value unchanged, in both directions. This is the default entity, column, and row mapper on [ReadonlyRepositoryOptions](./readonly-repository.md#readonlyrepositoryoptions).

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

::: example
```csharp csharp/Datasources.Tests/IdentityMapperTests.cs#L10-L12
```
```rust rust/datasources/src/mappers/identity_mapper.rs#L39-L41
```
```typescript typescript/datasources/src/mappers/IdentityMapper.test.ts#L6-L8
```
:::
