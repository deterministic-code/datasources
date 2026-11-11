# FlatToNestedMapper

Maps flat objects to nested objects. {{ReverseMap|reverseMap|reverse_map}} maps the nested object back.

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

Collapse spec. Scalars are column names; array fields are [TypeMapGroup](#typemapgroup) values; string-keyed cell records are [TypeMapDictionary](#typemapdictionary) values.

**Properties**

##### key

Parent key column.

```typescript
key: string
```

##### fields

Column, group, or dictionary per destination field.

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

#### TypeMapDictionary

```typescript
type TypeMapDictionary
```

Scalar dictionary in a [TypeMap](#typemap). `key` and `value` are source columns.

**Properties**

##### key

Source column used as the dictionary key.

```typescript
key: string
```

##### value

Source column used as the dictionary value.

```typescript
value: string
```

#### TypeMapFields

```typescript
type TypeMapFields<T>
```

Per-field mapping: a column name, a [TypeMapGroup](#typemapgroup) when the field is an array, or a [TypeMapDictionary](#typemapdictionary) when the field is a string-keyed record of cells.

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

Collapse spec. A string field is a column; a [TypeMapGroup](#typemapgroup) is a child collection; [TypeMapField.Dictionary](#typemapfield-dictionary) is a string-keyed map of cells.

**Properties**

##### Key

Parent key column.

```csharp
required string Key { get; init; }
```

##### Fields

Column, group, or dictionary per destination field.

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

A column name, a [TypeMapGroup](#typemapgroup), or source key and value columns. A `string` or [TypeMapGroup](#typemapgroup) converts to this type.

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

##### Dictionary

Maps a destination field to source key and value columns.

```csharp
TypeMapField.Dictionary(string Key, string Value)
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

##### Map

A string-keyed map of cells.

```csharp
MappedValue.Map(Dictionary<string, Value> Items)
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

Column, group, or dictionary per destination field.

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

A column name, a [TypeMapGroup](#typemapgroup), or a dictionary.

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

##### Dictionary

Maps a destination field to source key and value columns.

```rust
Dictionary { key: String, value: String }
```

**Methods**

##### column(source)

Builds a [Column](#typemapfield-column) field.

```rust
pub fn column(source: impl Into<String>) -> Self
```

- `source` — source column name

**Returns** [TypeMapField](#typemapfield)

##### dictionary(key, value)

Builds a [Dictionary](#typemapfield-dictionary) field.

```rust
pub fn dictionary(key: impl Into<String>, value: impl Into<String>) -> Self
```

- `key` — source column used as the dictionary key
- `value` — source column used as the dictionary value

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

##### Map

A string-keyed map of cells.

```rust
Map(HashMap<String, Value>)
```

#### MappedRow

```rust
pub type MappedRow = HashMap<String, MappedValue>
```

A mapped graph row. Keys are destination field names.
:::

## Remarks

Collapses denormalized join rows into unique parents with unique children. Scalars map column to field; groups collect children by key; dictionaries collect unique key/value entries. The first row wins a duplicate key. A null key is omitted.

A dictionary field names a key column and a value column. The key cell becomes the string key and the value cell becomes the entry. Rows that share a parent key merge. {{ReverseMap|reverseMap|reverse_map}} writes one flat row per entry. An empty dictionary becomes one row with a null key and a null value. [NestedToObjectEnrichmentStrategy](./nested-to-object-enrichment-strategy.md) applies this mapper and replaces the parents from those join rows.

::: prose
### typescript
Map to generic `T`. A dictionary field is a record of the reader cell, and a missing value is `null`. A bool key is `true` or `false`. A non-string value stays the reader cell.

### csharp
Map to a [MappedValue](#mappedvalue) graph. A dictionary field is `MappedValue.Map`. The strategy writes it onto `Dictionary<string, TValue>` through `MappedValueMapper`. `TValue` is a column type. Each cell goes through `ToObject`, then `ChangeType` when the runtime value does not match. A null cell stays null. A bool key is `True` or `False`.

### rust
Map to a [MappedValue](#mappedvalue) graph. A dictionary field is `MappedValue::Map`. The strategy stores that map as `key` and `value` rows in one text cell. Hydration reads `key` or `Key` and `value` or `Value` through `wire_text`. A bool key is `true` or `false`. A map of structs receives only that value cell.
:::

Nested group fields on a collection are omitted.

This is a collapse mapper, not enrichment. Related fills after read use [IEnriched](./i-enriched.md).

## Examples

### Map join rows to a nested contact

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

### Map join rows to a nested dictionary

::: example
```csharp csharp/Datasources.Tests/FlatToNestedMapperTests.cs#MapsJoinRowsToANestedDictionary
```
```rust rust/datasources/src/mappers/flat_to_nested_mapper.rs#maps_join_rows_to_a_nested_dictionary
```
```typescript typescript/datasources/src/mappers/FlatToNestedMapper.test.ts#maps-join-rows-to-a-nested-dictionary
```

```output [After: nested dictionary]
{
  Id: 1,
  Name: "star",
  Settings: { lives: "3", level: "2" }
}
```
:::

