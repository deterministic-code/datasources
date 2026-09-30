# PluralizerMapper

Mapper that pluralizes a name and lowercases it. Reverse singularizes and keeps the first letter's case.

## Classes

::: reference
### typescript
#### PluralizerMapper

```typescript
class PluralizerMapper implements IMapper<string, string>
```

**Methods**

##### map(value)

Pluralizes `value` and lowercases it. `User` becomes `users`.

```typescript
map(value: string): string
```

- `value` — source name

**Returns** `string`

##### reverseMap(value)

Singularizes `value`. `users` becomes `user`.

```typescript
reverseMap(value: string): string
```

- `value` — plural name

**Returns** `string`

### csharp
#### PluralizerMapper

```csharp
public sealed class PluralizerMapper : IMapper<string, string>
```

**Methods**

##### Map(value)

Pluralizes `value` and lowercases it. `User` becomes `users`.

```csharp
string Map(string value)
```

- `value` — source name

**Returns** `string`

##### ReverseMap(value)

Singularizes `value`. `users` becomes `user`.

```csharp
string ReverseMap(string value)
```

- `value` — plural name

**Returns** `string`

### rust
#### PluralizerMapper

```rust
pub struct PluralizerMapper
```

**Constructor**

##### new()

Builds a pluralizer mapper.

```rust
pub fn new() -> Self
```

**Returns** [PluralizerMapper](#pluralizermapper)

**Methods**

##### map(value)

Pluralizes `value` and lowercases it. `User` becomes `users`.

```rust
fn map(&self, value: String) -> String
```

- `value` — source name

**Returns** `String`

##### reverse_map(value)

Singularizes `value`. `users` becomes `user`.

```rust
fn reverse_map(&self, value: String) -> String
```

- `value` — plural name

**Returns** `String`
:::

## Remarks

Use this as [EntityMapper](./readonly-repository.md#entitymapper) when the type name is singular and the table name is a lowercase plural. Setting {{Entity|entity|entity}} skips the mapper.

## Examples

### Map contacts

::: example
```csharp csharp/Datasources.Tests/PluralizerMapperTests.cs#MapsContacts
```
```rust rust/datasources/src/mappers/pluralizer_mapper.rs#maps_contacts
```
```typescript typescript/datasources/src/mappers/PluralizerMapper.test.ts#maps-contacts
```

```output [Map]
contacts
addresses
```
:::

### Reverse map contacts

::: example
```csharp csharp/Datasources.Tests/PluralizerMapperTests.cs#ReverseMapsContacts
```
```rust rust/datasources/src/mappers/pluralizer_mapper.rs#reverse_maps_contacts
```
```typescript typescript/datasources/src/mappers/PluralizerMapper.test.ts#reverse-maps-contacts
```

```output [Reverse map]
contact
address
```
:::
