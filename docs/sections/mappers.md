# Mapper

Bidirectional `Map` / `ReverseMap`. Column names pass through unless a mapper rewrites them. Repository read is convert, then map.

## Classes

::: reference
### typescript
#### IMapper

```typescript
interface IMapper<TFrom, TTo>
```

**Methods**

##### map(value)

Maps `TFrom` to `TTo`.

```typescript
map(value: TFrom): TTo
```

- `value` — source value

**Returns** `TTo`

##### reverseMap(value)

Maps `TTo` back to `TFrom`.

```typescript
reverseMap(value: TTo): TFrom
```

- `value` — mapped value

**Returns** `TFrom`

### csharp
#### IMapper

```csharp
public interface IMapper<TFrom, TTo>
```

**Methods**

##### Map(value)

Maps `TFrom` to `TTo`.

```csharp
TTo Map(TFrom value)
```

- `value` — source value

**Returns** `TTo`

##### ReverseMap(value)

Maps `TTo` back to `TFrom`.

```csharp
TFrom ReverseMap(TTo value)
```

- `value` — mapped value

**Returns** `TFrom`

### rust
#### Mapper

```rust
pub trait Mapper<TFrom, TTo>
```

**Methods**

##### map(value)

Maps `TFrom` to `TTo`.

```rust
fn map(&self, value: TFrom) -> TTo
```

- `value` — source value

**Returns** `TTo`

##### reverse_map(value)

Maps `TTo` back to `TFrom`.

```rust
fn reverse_map(&self, value: TTo) -> TFrom
```

- `value` — mapped value

**Returns** `TFrom`
:::

## Remarks

Repository read is convert, then map. [ReadonlyRepositoryOptions.RowsMapper](./readonly-repository.md#rowsmapper) is the read-row mapper. `Repository.Mapper` / `mapper` is that mapper.

## See also

- [IdentityMapper](./identity-mapper.md)
- [DictionaryMapper](./dictionary-mapper.md)
- [PluralizerMapper](./pluralizer-mapper.md)
- [ChangeCasingMapper](./change-casing-mapper.md)
- [TransformMapper](./transform-mapper.md)
- [FlatToNestedMapper](./flat-to-nested-mapper.md)
- [ReadonlyRepositoryOptions](./readonly-repository.md#readonlyrepositoryoptions)
