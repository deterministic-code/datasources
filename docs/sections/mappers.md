# Mappers

Bidirectional {{Map|map|map}} and {{ReverseMap|reverseMap|reverse_map}}. Column names pass through unless a mapper rewrites them. Repository read is convert, then map.

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

## Examples

### Transform a contact

::: example
```csharp csharp/Datasources.Tests/TransformMapperTests.cs#MapsAContactNameAndEmail
```
```rust rust/datasources/src/mappers/mod.rs#maps_a_contact_name_and_email
```
```typescript typescript/datasources/src/mappers/transformMapper.test.ts#maps-a-contact-name-and-email
```

```output [Before: contact]
{
  name: 'Ada',
  email: 'ada@x.dev'
}
```

```output [After: mapped contact]
{
  contactName: 'Ada',
  contactEmail: 'ada@x.dev'
}
```
:::

### Change casing

::: example
```csharp csharp/Datasources.Tests/ChangeCasingMapperTests.cs#MapsContactNames
```
```rust rust/datasources/src/mappers/change_casing_mapper.rs#maps_contact_names
```
```typescript typescript/datasources/src/mappers/ChangeCasingMapper.test.ts#maps-contact-names
```

```output [Map]
contact
first_name
email_address
```
:::

::: example
```csharp csharp/Datasources.Tests/ChangeCasingMapperTests.cs#ReverseMapsContactNames
```
```rust rust/datasources/src/mappers/change_casing_mapper.rs#reverse_maps_contact_names
```
```typescript typescript/datasources/src/mappers/ChangeCasingMapper.test.ts#reverse-maps-contact-names
```

```output [Reverse map]
Contact
FirstName
EmailAddress
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

### Flat to nested

::: example
```csharp csharp/Datasources.Tests/FlatToNestedMapperTests.cs#MapsJoinRowsToANestedContact
```
```rust rust/datasources/src/mappers/flat_to_nested_mapper.rs#maps_join_rows_to_a_nested_contact
```
```typescript typescript/datasources/src/mappers/FlatToNestedMapper.test.ts#maps-join-rows-to-a-nested-contact
```

```output [After: nested contact]
{
  id: 7,
  first_name: "Ada",
  last_name: "Lovelace",
  email: "ada@ex.com",
  contact_source_name: "Web",
  contact_type_name: "Person",
  addresses: [
    { id: 101, line1: "10 Downing St", city: "London", address_type_name: "Home" },
    { id: 102, line1: "1 Parliament Sq", city: "London", address_type_name: "Home" }
  ],
  phones: [{ id: 201, number: "+44", phone_type_name: "Mobile" }]
}
```

```output [After: reverseMap join rows]
[
  {
    id: 7,
    first_name: "Ada",
    last_name: "Lovelace",
    email: "ada@ex.com",
    contact_source_name: "Web",
    contact_type_name: "Person",
    address_id: 101,
    address_line1: "10 Downing St",
    address_city: "London",
    address_type_name: "Home",
    phone_id: 201,
    phone_number: "+44",
    phone_type_name: "Mobile"
  },
  {
    id: 7,
    first_name: "Ada",
    last_name: "Lovelace",
    email: "ada@ex.com",
    contact_source_name: "Web",
    contact_type_name: "Person",
    address_id: 102,
    address_line1: "1 Parliament Sq",
    address_city: "London",
    address_type_name: "Home",
    phone_id: 201,
    phone_number: "+44",
    phone_type_name: "Mobile"
  }
]
```
:::

## See also

- [IdentityMapper](./identity-mapper.md)
- [DictionaryMapper](./dictionary-mapper.md)
- [PluralizerMapper](./pluralizer-mapper.md)
- [ChangeCasingMapper](./change-casing-mapper.md)
- [TransformMapper](./transform-mapper.md)
- [FlatToNestedMapper](./flat-to-nested-mapper.md)
- [ReadonlyRepositoryOptions](./readonly-repository.md#readonlyrepositoryoptions)
