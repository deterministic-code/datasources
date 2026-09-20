# NameSourceDestinationMapper

[INameMapper](./i-name-mapper.md) that holds source and destination labels for type and field name differences. Names still pass through.

## Classes

::: reference
### typescript
#### NameSourceDestinationMapper

```typescript
class NameSourceDestinationMapper implements INameMapper
```

**Constructor**

##### NameSourceDestinationMapper(source, destination)

Holds source and destination labels. Names still pass through.

```typescript
constructor(source: string, destination: string)
```

- `source` — source label
- `destination` — destination label

**Properties**

##### source

Dialect or provider label for the source side.

```typescript
readonly source: string
```

##### destination

Dialect or provider label for the destination side.

```typescript
readonly destination: string
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
#### NameSourceDestinationMapper

```csharp
public sealed class NameSourceDestinationMapper : INameMapper
```

**Constructor**

##### NameSourceDestinationMapper(source, destination)

Holds source and destination labels. Names still pass through.

```csharp
public NameSourceDestinationMapper(string source, string destination)
```

- `source` — source label
- `destination` — destination label

**Properties**

##### Source

Dialect or provider label for the source side.

```csharp
string Source { get; }
```

##### Destination

Dialect or provider label for the destination side.

```csharp
string Destination { get; }
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
#### NameSourceDestinationMapper

```rust
pub struct NameSourceDestinationMapper
```

**Constructor**

##### new(source, destination)

Holds source and destination labels. Names still pass through.

```rust
pub fn new(source: impl Into<String>, destination: impl Into<String>) -> Self
```

- `source` — source label
- `destination` — destination label

**Returns** [NameSourceDestinationMapper](#namesourcedestinationmapper)

**Properties**

##### source

Dialect or provider label for the source side.

```rust
pub source: String
```

##### destination

Dialect or provider label for the destination side.

```rust
pub destination: String
```

**Methods**

##### map(name)

Returns the name unchanged.

```rust
pub fn map(&self, name: &str) -> String
```

- `name` — column or entity name

**Returns** `String`

##### reverse_map(name)

Returns the name unchanged.

```rust
pub fn reverse_map(&self, name: &str) -> String
```

- `name` — column or entity name

**Returns** `String`
:::

## Remarks

Labels are metadata. [INameMapper](./i-name-mapper.md) still returns names unchanged.

## Examples

### Source and destination labels

::: example
```csharp csharp/Datasources.Tests/NameMapperTests.cs#L16-L23
```
```rust rust/datasources/src/mappers/mod.rs#L85-L90
```
```typescript typescript/datasources/src/mappers/nameMapper.test.ts#L18-L24
```
:::
