# FlatToTypeMapper

Collapses denormalized join rows into unique parents with unique children. Scalars map column to field; groups collect children by key.

## Classes

::: reference
### typescript
#### IFlatToTypeMapper

```typescript
interface IFlatToTypeMapper<T> extends IMapper<Cells, T>
```

**Properties**

##### spec

The [TypeMap](#typemap) used to collapse join rows.

```typescript
readonly spec: TypeMap<T>
```

**Methods**

##### collapse(rows)

Collapses denormalized join rows into unique parents with unique children.

```typescript
collapse(rows: Iterable<T | Cells>): T[]
```

- `rows` — raw `Cells` or already-mapped `T`

**Returns** `T[]`

#### FlatToTypeMapper

```typescript
class FlatToTypeMapper<T> implements IFlatToTypeMapper<T>
```

**Constructor**

##### FlatToTypeMapper(spec)

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

##### map(row)

Maps one denormalized row to `T`. Nested group fields on a collection are omitted.

```typescript
map(row: Cells): T
```

- `row` — denormalized join row

**Returns** `T`

##### reverseMap(value)

Flattens `T` back to one row. Groups keep the first child.

```typescript
reverseMap(value: T): Cells
```

- `value` — mapped object

**Returns** `Cells`

##### collapse(rows)

Collapses denormalized join rows into unique parents with unique children.

```typescript
collapse(rows: Iterable<T | Cells>): T[]
```

- `rows` — raw `Cells` or already-mapped `T`

**Returns** `T[]`

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
#### IFlatToTypeMapper

```csharp
public interface IFlatToTypeMapper : IMapper<Row, Dictionary<string, MappedValue>>
```

**Properties**

##### Spec

The [TypeMap](#typemap) used to collapse join rows.

```csharp
TypeMap Spec { get; }
```

**Methods**

##### Collapse(rows)

Maps and collapses denormalized join rows.

```csharp
IReadOnlyList<Dictionary<string, MappedValue>> Collapse(IEnumerable<Row> rows)
```

- `rows` — denormalized join rows

**Returns** `IReadOnlyList<Dictionary<string, `[MappedValue](#mappedvalue)`>>`

##### Collapse(mapped)

Collapses already-mapped rows.

```csharp
IReadOnlyList<Dictionary<string, MappedValue>> Collapse(IEnumerable<Dictionary<string, MappedValue>> rows)
```

- `mapped` — rows already passed through [Map](#flattotypemapper-map-value)

**Returns** `IReadOnlyList<Dictionary<string, `[MappedValue](#mappedvalue)`>>`

#### FlatToTypeMapper

```csharp
public sealed class FlatToTypeMapper : IFlatToTypeMapper
```

**Constructor**

##### FlatToTypeMapper(spec)

Builds a collapse mapper from a [TypeMap](#typemap).

```csharp
public FlatToTypeMapper(TypeMap spec)
```

- `spec` — [TypeMap](#typemap); scalars map column to field, groups collect unique children by key

**Properties**

##### Spec

The [TypeMap](#typemap) used to collapse join rows.

```csharp
TypeMap Spec { get; }
```

**Methods**

##### Map(value)

Maps one denormalized row. Nested group fields on a collection are omitted.

```csharp
Dictionary<string, MappedValue> Map(Row value)
```

- `value` — denormalized join row

**Returns** `Dictionary<string, `[MappedValue](#mappedvalue)`>`

##### ReverseMap(value)

Flattens a mapped row back. Groups keep the first child.

```csharp
Row ReverseMap(Dictionary<string, MappedValue> value)
```

- `value` — mapped graph

**Returns** `Row`

##### Collapse(rows)

Maps and collapses denormalized join rows.

```csharp
IReadOnlyList<Dictionary<string, MappedValue>> Collapse(IEnumerable<Row> rows)
```

- `rows` — denormalized join rows

**Returns** `IReadOnlyList<Dictionary<string, `[MappedValue](#mappedvalue)`>>`

##### Collapse(mapped)

Collapses already-mapped rows.

```csharp
IReadOnlyList<Dictionary<string, MappedValue>> Collapse(IEnumerable<Dictionary<string, MappedValue>> rows)
```

- `mapped` — rows already passed through [Map](#flattotypemapper-map-value)

**Returns** `IReadOnlyList<Dictionary<string, `[MappedValue](#mappedvalue)`>>`

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

Intermediate graph node after [FlatToTypeMapper.Map](#flattotypemapper-map-value).

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
#### FlatToTypeMapperTrait

```rust
pub trait FlatToTypeMapper: Mapper<Row, MappedRow>
```

**Methods**

##### spec()

The [TypeMap](#typemap) used to collapse join rows.

```rust
fn spec(&self) -> &TypeMap
```

**Returns** `&`[TypeMap](#typemap)

##### collapse(rows)

Collapses already-mapped rows.

```rust
fn collapse(&self, rows: Vec<MappedRow>) -> Vec<MappedRow>
```

- `rows` — rows already passed through [map](#flattotypemapper-map-row)

**Returns** `Vec<`[MappedRow](#mappedrow)`>`

##### collapse_rows(rows)

Maps and collapses denormalized join rows.

```rust
fn collapse_rows(&self, rows: Vec<Row>) -> Vec<MappedRow>
```

- `rows` — denormalized join rows

**Returns** `Vec<`[MappedRow](#mappedrow)`>`

#### FlatToTypeMapper

```rust
pub struct FlatToTypeMapper
```

**Constructor**

##### new(spec)

Builds a collapse mapper from a [TypeMap](#typemap).

```rust
pub fn new(spec: TypeMap) -> Self
```

- `spec` — [TypeMap](#typemap); scalars map column to field, groups collect unique children by key

**Returns** [FlatToTypeMapper](#flattotypemapper)

**Methods**

##### map(row)

Maps one denormalized row. Nested group fields on a collection are omitted.

```rust
fn map(&self, row: Row) -> MappedRow
```

- `row` — denormalized join row

**Returns** [MappedRow](#mappedrow)

##### reverse_map(value)

Flattens a mapped row back. Groups keep the first child.

```rust
fn reverse_map(&self, value: MappedRow) -> Row
```

- `value` — mapped graph

**Returns** `Row`

##### spec()

The [TypeMap](#typemap) used to collapse join rows.

```rust
fn spec(&self) -> &TypeMap
```

**Returns** `&`[TypeMap](#typemap)

##### collapse(rows)

Collapses already-mapped rows.

```rust
fn collapse(&self, rows: Vec<MappedRow>) -> Vec<MappedRow>
```

- `rows` — rows already passed through [map](#flattotypemapper-map-row)

**Returns** `Vec<`[MappedRow](#mappedrow)`>`

##### collapse_rows(rows)

Maps and collapses denormalized join rows.

```rust
fn collapse_rows(&self, rows: Vec<Row>) -> Vec<MappedRow>
```

- `rows` — denormalized join rows

**Returns** `Vec<`[MappedRow](#mappedrow)`>`

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

Intermediate graph node after [FlatToTypeMapper::map](#flattotypemapper-map-row).

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

This is a collapse mapper, not enrichment. Related fills after get use [Enriched repository](./enriched-repository.md).
