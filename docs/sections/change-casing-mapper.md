# ChangeCasingMapper

Mapper that rewrites a name from one casing to another through `ChangeCasingConverter`. `Map` writes the stored casing. `ReverseMap` restores the application casing.

## Classes

::: reference
### typescript
#### ChangeCasingMapper

```typescript
class ChangeCasingMapper implements IMapper<string, string>
```

**Constructor**

##### ChangeCasingMapper(from, to)

Maps names from `from` to `to`. Reverse maps back to `from`.

```typescript
constructor(from: Casing, to: Casing)
```

- `from` — application casing, such as `pascalCase`
- `to` — stored casing, such as `snakeCase`

**Returns** [ChangeCasingMapper](#changecasingmapper)

**Methods**

##### map(value)

Rewrites `value` into the stored casing. `UserType` with `snakeCase` becomes `user_type`.

```typescript
map(value: string): string
```

- `value` — application name

**Returns** `string`

##### reverseMap(value)

Rewrites `value` into the application casing. `user_type` with `pascalCase` becomes `UserType`.

```typescript
reverseMap(value: string): string
```

- `value` — stored name

**Returns** `string`

### csharp
#### ChangeCasingMapper

```csharp
public sealed class ChangeCasingMapper : IMapper<string, string>
```

**Constructor**

##### ChangeCasingMapper(from, to)

Maps names from `from` to `to`. Reverse maps back to `from`.

```csharp
public ChangeCasingMapper(Casing from, Casing to)
```

- `from` — application casing, such as `Pascal`
- `to` — stored casing, such as `Snake`

**Returns** [ChangeCasingMapper](#changecasingmapper)

**Methods**

##### Map(value)

Rewrites `value` into the stored casing. `UserType` with `Snake` becomes `user_type`.

```csharp
string Map(string value)
```

- `value` — application name

**Returns** `string`

##### ReverseMap(value)

Rewrites `value` into the application casing. `user_type` with `Pascal` becomes `UserType`.

```csharp
string ReverseMap(string value)
```

- `value` — stored name

**Returns** `string`

### rust
#### ChangeCasingMapper

```rust
pub struct ChangeCasingMapper
```

**Constructor**

##### new(from, to)

Maps names from `from` to `to`. Reverse maps back to `from`.

```rust
pub fn new(from: Casing, to: Casing) -> Self
```

- `from` — application casing, such as `Pascal`
- `to` — stored casing, such as `Snake`

**Returns** [ChangeCasingMapper](#changecasingmapper)

**Methods**

##### map(value)

Rewrites `value` into the stored casing. `UserType` with `Snake` becomes `user_type`.

```rust
fn map(&self, value: String) -> String
```

- `value` — application name

**Returns** `String`

##### reverse_map(value)

Rewrites `value` into the application casing. `user_type` with `Pascal` becomes `UserType`.

```rust
fn reverse_map(&self, value: String) -> String
```

- `value` — stored name

**Returns** `String`
:::

## Remarks

Use this as [EntityMapper](./readonly-repository.md#entitymapper) and {{[PropertyMappings](./readonly-repository.md#propertymappings)|[propertyMappings](./readonly-repository.md#propertymappings)|[property_mappings](./readonly-repository.md#property_mappings)}} when application names are Pascal and stored names are snake case. `UserType` becomes `user_type`, and `Id` becomes `id`.

## Examples

### Map contacts

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

### Reverse map contacts

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
