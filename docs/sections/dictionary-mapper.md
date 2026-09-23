# DictionaryMapper

Mapper that rewrites known keys and passes the rest through. Reverse lookup uses the mapped value. When two keys share a value, the first one wins on reverse.

## Classes

::: reference
### typescript
#### DictionaryMapper

```typescript
class DictionaryMapper<T extends string | number> implements IMapper<T, T>
```

**Constructor**

##### DictionaryMapper(map)

Builds the forward map and its reverse.

```typescript
constructor(map: readonly (readonly [T, T])[])
```

- `map` — pairs of source key and stored name

**Returns** [DictionaryMapper](#dictionarymapper)

**Methods**

##### map(value)

Returns the stored name for a known key, or `value`.

```typescript
map(value: T): T
```

- `value` — source key

**Returns** `T`

##### reverseMap(value)

Returns the source key for a stored name, or `value`.

```typescript
reverseMap(value: T): T
```

- `value` — stored name

**Returns** `T`

### csharp
#### DictionaryMapper

```csharp
public sealed class DictionaryMapper<T> : IMapper<T, T> where T : notnull
```

**Constructor**

##### DictionaryMapper(map)

Builds the forward map and its reverse.

```csharp
public DictionaryMapper(IReadOnlyDictionary<T, T> map)
```

- `map` — pairs of source key and stored name

**Methods**

##### Map(value)

Returns the stored name for a known key, or `value`.

```csharp
T Map(T value)
```

- `value` — source key

**Returns** `T`

##### ReverseMap(value)

Returns the source key for a stored name, or `value`.

```csharp
T ReverseMap(T value)
```

- `value` — stored name

**Returns** `T`

### rust
#### DictionaryMapper

```rust
pub struct DictionaryMapper<T>
```

**Constructor**

##### new(map)

Builds the forward map and its reverse. `T: Clone + Eq + Hash`.

```rust
pub fn new(map: impl IntoIterator<Item = (T, T)>) -> Self
```

- `map` — pairs of source key and stored name, in first-wins order

**Returns** [DictionaryMapper](#dictionarymapper)

**Methods**

##### map(value)

Returns the stored name for a known key, or `value`.

```rust
fn map(&self, value: T) -> T
```

- `value` — source key

**Returns** `T`

##### reverse_map(value)

Returns the source key for a stored name, or `value`.

```rust
fn reverse_map(&self, value: T) -> T
```

- `value` — stored name

**Returns** `T`
:::

## Remarks

Use this for [ColumnMappings](./readonly-repository.md#columnmappings) when property names and stored column names differ. `Map` rewrites the SELECT list. `ReverseMap` restores property names on each result key before the row converter.

## Examples

### Map and reverse map

::: example
```csharp csharp/Datasources.Tests/DictionaryMapperTests.cs#L14-L25
```
```rust rust/datasources/src/mappers/dictionary_mapper.rs#L51-L61
```
```typescript typescript/datasources/src/mappers/DictionaryMapper.test.ts#L14-L23
```
:::
