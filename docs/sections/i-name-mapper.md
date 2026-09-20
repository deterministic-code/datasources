# INameMapper

[IMapper](./mappers.md) for entity and column names.

## Classes

::: reference
### typescript
#### INameMapper

```typescript
interface INameMapper extends IMapper<string, string>
```

### csharp
#### INameMapper

```csharp
public interface INameMapper : IMapper<string, string>
```

### rust
#### NameMapper

```rust
pub trait NameMapper: Mapper<String, String> {}
```
:::

## Remarks

Use [NameSourceDestinationMapper](./name-source-destination-mapper.md) when the mapper also holds source and destination labels.

## Examples

### Pass-through names

::: example
```csharp csharp/Datasources.Tests/NameMapperTests.cs#L8-L13
```
```rust rust/datasources/src/mappers/mod.rs#L70-L81
```
```typescript typescript/datasources/src/mappers/nameMapper.test.ts#L6-L13
```
:::
