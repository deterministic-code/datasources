# FlatToNestedMapper

Collapses denormalized join rows into unique parents with unique children. Scalars map column to field; groups collect children by key.

## Classes

::: reference
### typescript
#### IFlatToNestedMapper

```typescript
interface IFlatToNestedMapper<T> extends IMapper<Iterable<T | Cells>, T[]>
```

**Properties**

##### spec

The [TypeMap](#typemap) used to collapse join rows.

```typescript
readonly spec: TypeMap<T>
```

#### FlatToNestedMapper

```typescript
class FlatToNestedMapper<T> implements IFlatToNestedMapper<T>
```

**Constructor**

##### FlatToNestedMapper(spec)

Builds a collapse mapper from a [TypeMap](#typemap).

```typescript
constructor(spec: TypeMap<T>)
```

- `spec` — [TypeMap](#typemap); scalars map column to field, groups collect unique children by key

**Properties**

##### spec

The [TypeMap](#typemap) used to collapse join rows.

```typescript
readonly spec: TypeMap<T>
```

**Methods**

##### map(rows)

Maps join rows to unique parents with unique children. Nested group fields on a collection are omitted.

```typescript
map(rows: Iterable<T | Cells>): T[]
```

- `rows` — denormalized join rows, or already-mapped `T`

**Returns** `T[]`

##### reverseMap(values)

Flattens nested parents back to denormalized join rows.

```typescript
reverseMap(values: T[]): Cells[]
```

- `values` — nested parents

**Returns** `Cells[]`

#### TypeMap

```typescript
type TypeMap<T>
```

Collapse spec. Scalars are column names; array fields are [TypeMapGroup](#typemapgroup) values.

**Properties**

##### key

Parent key column.

```typescript
key: string
```

##### fields

Column or group per destination field.

```typescript
fields: TypeMapFields<T>
```

#### TypeMapGroup

```typescript
type TypeMapGroup<E>
```

Child collection in a [TypeMap](#typemap). Nested group fields on a collection are omitted.

**Properties**

##### key

Child key column used to unique children.

```typescript
key: string
```

##### fields

Column names for the child.

```typescript
fields: TypeMapFields<E>
```

#### TypeMapFields

```typescript
type TypeMapFields<T>
```

Per-field mapping: a column name, or a [TypeMapGroup](#typemapgroup) when the field is an array.

### csharp
#### IFlatToNestedMapper

```csharp
public interface IFlatToNestedMapper : IMapper<IEnumerable<Row>, IReadOnlyList<Dictionary<string, MappedValue>>>
```

**Properties**

##### Spec

The [TypeMap](#typemap) used to collapse join rows.

```csharp
TypeMap Spec { get; }
```

#### FlatToNestedMapper

```csharp
public sealed class FlatToNestedMapper : IFlatToNestedMapper
```

**Constructor**

##### FlatToNestedMapper(spec)

Builds a collapse mapper from a [TypeMap](#typemap).

```csharp
public FlatToNestedMapper(TypeMap spec)
```

- `spec` — [TypeMap](#typemap); scalars map column to field, groups collect unique children by key

**Properties**

##### Spec

The [TypeMap](#typemap) used to collapse join rows.

```csharp
TypeMap Spec { get; }
```

**Methods**

##### Map(rows)

Maps join rows to unique parents with unique children. Nested group fields on a collection are omitted.

```csharp
IReadOnlyList<Dictionary<string, MappedValue>> Map(IEnumerable<Row> rows)
```

- `rows` — denormalized join rows

**Returns** `IReadOnlyList<Dictionary<string, `[MappedValue](#mappedvalue)`>>`

##### ReverseMap(values)

Flattens nested parents back to denormalized join rows.

```csharp
IReadOnlyList<Row> ReverseMap(IReadOnlyList<Dictionary<string, MappedValue>> values)
```

- `values` — nested parents

**Returns** `IReadOnlyList<`[Row](./repository.md)`>`

#### TypeMap

```csharp
public sealed class TypeMap
```

Collapse spec. A string field is a column; a [TypeMapGroup](#typemapgroup) is a child collection.

**Properties**

##### Key

Parent key column.

```csharp
required string Key { get; init; }
```

##### Fields

Column or group per destination field.

```csharp
required IReadOnlyDictionary<string, TypeMapField> Fields { get; init; }
```

#### TypeMapGroup

```csharp
public sealed class TypeMapGroup
```

Child collection in a [TypeMap](#typemap). Nested group fields on a collection are omitted.

**Properties**

##### Key

Child key column used to unique children.

```csharp
required string Key { get; init; }
```

##### Fields

Column or group per child field.

```csharp
required IReadOnlyDictionary<string, TypeMapField> Fields { get; init; }
```

#### TypeMapField

```csharp
public abstract record TypeMapField
```

A column name or a [TypeMapGroup](#typemapgroup). A `string` or [TypeMapGroup](#typemapgroup) converts to this type.

**Properties**

##### Column

Maps a destination field to a source column.

```csharp
TypeMapField.Column(string Source)
```

##### Group

Maps a destination field to a child [TypeMapGroup](#typemapgroup).

```csharp
TypeMapField.Group(TypeMapGroup Map)
```

#### MappedValue

```csharp
public abstract record MappedValue
```

Intermediate graph node after [FlatToNestedMapper.Map](#flattonestedmapper-map-value).

**Properties**

##### Cell

A scalar cell.

```csharp
MappedValue.Cell(Value Value)
```

##### Many

A child collection.

```csharp
MappedValue.Many(List<Dictionary<string, MappedValue>> Items)
```

### rust
#### FlatToNestedMapperTrait

```rust
pub trait FlatToNestedMapper: Mapper<Vec<Row>, Vec<MappedRow>>
```

**Methods**

##### spec()

The [TypeMap](#typemap) used to collapse join rows.

```rust
fn spec(&self) -> &TypeMap
```

**Returns** `&`[TypeMap](#typemap)

#### FlatToNestedMapper

```rust
pub struct FlatToNestedMapper
```

**Constructor**

##### new(spec)

Builds a collapse mapper from a [TypeMap](#typemap).

```rust
pub fn new(spec: TypeMap) -> Self
```

- `spec` — [TypeMap](#typemap); scalars map column to field, groups collect unique children by key

**Returns** [FlatToNestedMapper](#flattonestedmapper)

**Methods**

##### map(rows)

Maps join rows to unique parents with unique children. Nested group fields on a collection are omitted.

```rust
fn map(&self, rows: Vec<Row>) -> Vec<MappedRow>
```

- `rows` — denormalized join rows

**Returns** `Vec<`[MappedRow](#mappedrow)`>`

##### reverse_map(values)

Flattens nested parents back to denormalized join rows.

```rust
fn reverse_map(&self, values: Vec<MappedRow>) -> Vec<Row>
```

- `values` — nested parents

**Returns** `Vec<Row>`

##### spec()

The [TypeMap](#typemap) used to collapse join rows.

```rust
fn spec(&self) -> &TypeMap
```

**Returns** `&`[TypeMap](#typemap)

#### TypeMap

```rust
pub struct TypeMap
```

Collapse spec. Fields are [TypeMapField](#typemapfield) values.

**Constructor**

##### new(key, fields)

Builds a collapse spec.

```rust
pub fn new(key: impl Into<String>, fields: Vec<(String, TypeMapField)>) -> Self
```

- `key` — parent key column
- `fields` — destination name and [TypeMapField](#typemapfield) pairs

**Returns** [TypeMap](#typemap)

**Properties**

##### key

Parent key column.

```rust
pub key: String
```

##### fields

Column or group per destination field.

```rust
pub fields: Vec<(String, TypeMapField)>
```

#### TypeMapGroup

```rust
pub struct TypeMapGroup
```

Child collection in a [TypeMap](#typemap). Nested group fields on a collection are omitted.

**Constructor**

##### new(key, fields)

Builds a child-group spec.

```rust
pub fn new(key: impl Into<String>, fields: Vec<(String, TypeMapField)>) -> Self
```

- `key` — child key column
- `fields` — destination name and [TypeMapField](#typemapfield) pairs

**Returns** [TypeMapGroup](#typemapgroup)

**Properties**

##### key

Child key column used to unique children.

```rust
pub key: String
```

##### fields

Column or group per child field.

```rust
pub fields: Vec<(String, TypeMapField)>
```

#### TypeMapField

```rust
pub enum TypeMapField
```

A column name or a [TypeMapGroup](#typemapgroup).

**Properties**

##### Column

Maps a destination field to a source column.

```rust
Column(String)
```

##### Group

Maps a destination field to a child [TypeMapGroup](#typemapgroup).

```rust
Group(TypeMapGroup)
```

**Methods**

##### column(source)

Builds a [Column](#typemapfield-column) field.

```rust
pub fn column(source: impl Into<String>) -> Self
```

- `source` — source column name

**Returns** [TypeMapField](#typemapfield)

#### MappedValue

```rust
pub enum MappedValue
```

Intermediate graph node after [FlatToNestedMapper::map](#flattonestedmapper-map-row).

**Properties**

##### Cell

A scalar cell.

```rust
Cell(Value)
```

##### Many

A child collection.

```rust
Many(Vec<MappedRow>)
```

#### MappedRow

```rust
pub type MappedRow = HashMap<String, MappedValue>
```

A mapped graph row. Keys are destination field names.
:::

## Remarks

C# and Rust map to a [MappedValue](#mappedvalue) graph. TypeScript maps to generic `T`. Nested group fields on a collection are omitted.

This is a collapse mapper, not enrichment. Related fills after read use [Enriched repository](./enriched-repository.md).

## Examples

### Map join rows to a nested contact

::: example
```csharp csharp/Datasources.Tests/FlatToNestedMapperTests.cs#L101-L164
```
```rust rust/datasources/src/mappers/flat_to_nested_mapper.rs#L383-L453
```
```typescript typescript/datasources/src/mappers/FlatToNestedMapper.test.ts#L90-L155
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

