# TransformMapper

Mapper built from a pair of functions.

## Classes

::: reference
### typescript
#### TransformMapper

```typescript
class TransformMapper<TFrom, TTo> implements IMapper<TFrom, TTo>
```

**Constructor**

##### TransformMapper(map, reverseMap)

Builds a mapper from a pair of functions.

```typescript
constructor(map: (value: TFrom) => TTo, reverseMap: (value: TTo) => TFrom)
```

- `map` — `TFrom` to `TTo`
- `reverseMap` — `TTo` to `TFrom`

**Methods**

##### map(value)

Runs the map function.

```typescript
map(value: TFrom): TTo
```

- `value` — source value

**Returns** `TTo`

##### reverseMap(value)

Runs the reverse-map function.

```typescript
reverseMap(value: TTo): TFrom
```

- `value` — mapped value

**Returns** `TFrom`

### csharp
#### TransformMapper

```csharp
public class TransformMapper<TFrom, TTo> : IMapper<TFrom, TTo>
```

**Constructor**

##### TransformMapper(map, reverseMap)

Builds a mapper from a pair of functions.

```csharp
public TransformMapper(Func<TFrom, TTo> map, Func<TTo, TFrom> reverseMap)
```

- `map` — `TFrom` to `TTo`
- `reverseMap` — `TTo` to `TFrom`

**Methods**

##### Map(value)

Runs the map function.

```csharp
TTo Map(TFrom value)
```

- `value` — source value

**Returns** `TTo`

##### ReverseMap(value)

Runs the reverse-map function.

```csharp
TFrom ReverseMap(TTo value)
```

- `value` — mapped value

**Returns** `TFrom`

### rust
#### TransformMapper

```rust
pub struct TransformMapper<TFrom, TTo>
```

**Constructor**

##### new(map, reverse_map)

Builds a mapper from a pair of functions.

```rust
pub fn new(map: impl Fn(TFrom) -> TTo + Send + Sync + 'static, reverse_map: impl Fn(TTo) -> TFrom + Send + Sync + 'static) -> Self
```

- `map` — `TFrom` to `TTo`
- `reverse_map` — `TTo` to `TFrom`

**Returns** [TransformMapper](#transformmapper)

**Methods**

##### map(value)

Runs the map function.

```rust
fn map(&self, value: TFrom) -> TTo
```

- `value` — source value

**Returns** `TTo`

##### reverse_map(value)

Runs the reverse-map function.

```rust
fn reverse_map(&self, value: TTo) -> TFrom
```

- `value` — mapped value

**Returns** `TFrom`
:::

## Remarks

Use [TransformMapper](#transformmapper) when the mapping is a pair of closures. Name and statement mappers wrap this for their specific shapes.

## Examples

### Flatten with TransformMapper

::: example
```csharp csharp/Datasources.Tests/TransformMapperTests.cs#L13-L24
```
```rust rust/datasources/src/mappers/mod.rs#L122-L157
```
```typescript typescript/datasources/src/mappers/transformMapper.test.ts#L16-L40
```
:::
