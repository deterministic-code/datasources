# PluralizerMapper

Mapper that pluralizes a name and keeps its case. Reverse singularizes and keeps the first letter's case.

## Classes

::: reference
### typescript
#### PluralizerMapper

```typescript
class PluralizerMapper implements IMapper<string, string>
```

**Methods**

##### map(value)

Pluralizes `value` and keeps its case. `User` becomes `Users`.

```typescript
map(value: string): string
```

- `value` — source name

**Returns** `string`

##### reverseMap(value)

Singularizes `value` and keeps the first letter's case. `Users` becomes `User`.

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

Pluralizes `value` and keeps its case. `User` becomes `Users`.

```csharp
string Map(string value)
```

- `value` — source name

**Returns** `string`

##### ReverseMap(value)

Singularizes `value` and keeps the first letter's case. `Users` becomes `User`.

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

Pluralizes `value` and keeps its case. `User` becomes `Users`.

```rust
fn map(&self, value: String) -> String
```

- `value` — source name

**Returns** `String`

##### reverse_map(value)

Singularizes `value` and keeps the first letter's case. `Users` becomes `User`.

```rust
fn reverse_map(&self, value: String) -> String
```

- `value` — plural name

**Returns** `String`
:::

## Remarks

Use this as [EntityMapper](./readonly-repository.md#entitymapper) when the type name is singular and the table name is its plural. A lowercase plural table is [ChangeCasingMapper](./change-casing-mapper.md) then `PluralizerMapper`. The builder maps {{Entity|entity|entity}}, and defaults it to the type name.

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
Contacts
Addresses
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
Contact
Address
```
:::

### Lowercase plural

::: example
```csharp csharp/Datasources.Tests/PluralizerMapperTests.cs#LowercasesThenPluralizes
```
```rust rust/datasources/src/mappers/pluralizer_mapper.rs#lowercases_then_pluralizes
```
```typescript typescript/datasources/src/mappers/PluralizerMapper.test.ts#lowercases-then-pluralizes
```

```output [Map]
contacts
addresses
```

```output [Reverse map]
Contact
Address
```
:::
