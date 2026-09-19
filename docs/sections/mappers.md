# Mappers

Bidirectional `Map` / `ReverseMap`. Column names pass through unless a mapper rewrites them. Repository get is convert, then map.

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

#### ISourceDestinationMapper

```typescript
interface ISourceDestinationMapper
```

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

#### NameMapper

```typescript
abstract class NameMapper implements IMapper<string, string>
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

#### NameSourceDestinationMapper

```typescript
class NameSourceDestinationMapper extends NameMapper implements ISourceDestinationMapper
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

#### TransformMapper

```typescript
class TransformMapper<TFrom, TTo> implements IMapper<TFrom, TTo>
```

**Constructor**

##### TransformMapper(map, reverseMap)

Builds a mapper from a pair of functions.

```typescript
constructor(map: (value: TFrom) => TTo, reverseMap: (value: TTo) => TFrom)
```

- `map` — `TFrom` to `TTo`
- `reverseMap` — `TTo` to `TFrom`

**Methods**

##### map(value)

Runs the map function.

```typescript
map(value: TFrom): TTo
```

- `value` — source value

**Returns** `TTo`

##### reverseMap(value)

Runs the reverse-map function.

```typescript
reverseMap(value: TTo): TFrom
```

- `value` — mapped value

**Returns** `TFrom`

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

#### StatementMapper

```typescript
class StatementMapper<T> implements IMapper<T, T>
```

**Constructor**

##### StatementMapper(options?)

Builds a statement mapper. Omitted fields default to identity name mappers and an identity [rows](#statementmapper-rows) mapper.

```typescript
constructor(options?: StatementMapperOptions<T>)
```

- `options` — [StatementMapperOptions](#statementmapperoptions)

**Properties**

##### entity

Maps the table or entity name when the repository is built.

```typescript
readonly entity: IMapper<string, string>
```

##### columns

Maps column names when the repository is built.

```typescript
readonly columns: IMapper<string, string>
```

##### query

Rewrites a finished query.

```typescript
readonly query?: IMapper<IQuery, IQuery>
```

##### insert

Rewrites a finished insert.

```typescript
readonly insert?: IMapper<IInsertStatement, IInsertStatement>
```

##### update

Rewrites a finished update.

```typescript
readonly update?: IMapper<IUpdateStatement, IUpdateStatement>
```

##### delete

Rewrites a finished delete.

```typescript
readonly delete?: IMapper<IDeleteStatement, IDeleteStatement>
```

##### storedProcedure

Rewrites a finished stored-procedure statement.

```typescript
readonly storedProcedure?: IMapper<IStoredProcedureStatement, IStoredProcedureStatement>
```

##### rows

Get-row [IMapper](#imapper). [map](#statementmapper-map-value) / [reverseMap](#statementmapper-reversemap-value) delegate here.

```typescript
readonly rows: IMapper<T, T>
```

**Methods**

##### map(value)

Maps a row through [rows](#statementmapper-rows).

```typescript
map(value: T): T
```

- `value` — converted row

**Returns** `T`

##### reverseMap(value)

Reverse-maps a row through [rows](#statementmapper-rows).

```typescript
reverseMap(value: T): T
```

- `value` — shaped row

**Returns** `T`

#### StatementMapperOptions

```typescript
type StatementMapperOptions<T>
```

Constructor bag for [StatementMapper](#statementmapper).

**Properties**

##### entity

Maps the table or entity name.

```typescript
entity?: IMapper<string, string>
```

##### columns

Maps column names.

```typescript
columns?: IMapper<string, string>
```

##### rows

Get-row [IMapper](#imapper).

```typescript
rows?: IMapper<T, T>
```

##### query

Rewrites a finished query.

```typescript
query?: IMapper<IQuery, IQuery>
```

##### insert

Rewrites a finished insert.

```typescript
insert?: IMapper<IInsertStatement, IInsertStatement>
```

##### update

Rewrites a finished update.

```typescript
update?: IMapper<IUpdateStatement, IUpdateStatement>
```

##### delete

Rewrites a finished delete.

```typescript
delete?: IMapper<IDeleteStatement, IDeleteStatement>
```

##### storedProcedure

Rewrites a finished stored-procedure statement.

```typescript
storedProcedure?: IMapper<IStoredProcedureStatement, IStoredProcedureStatement>
```

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

#### ISourceDestinationMapper

```csharp
public interface ISourceDestinationMapper
```

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

#### NameMapper

```csharp
public abstract class NameMapper : IMapper<string, string>
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

#### NameSourceDestinationMapper

```csharp
public sealed class NameSourceDestinationMapper : NameMapper, ISourceDestinationMapper
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

#### TransformMapper

```csharp
public class TransformMapper<TFrom, TTo> : IMapper<TFrom, TTo>
```

**Constructor**

##### TransformMapper(map, reverseMap)

Builds a mapper from a pair of functions.

```csharp
public TransformMapper(Func<TFrom, TTo> map, Func<TTo, TFrom> reverseMap)
```

- `map` — `TFrom` to `TTo`
- `reverseMap` — `TTo` to `TFrom`

**Methods**

##### Map(value)

Runs the map function.

```csharp
TTo Map(TFrom value)
```

- `value` — source value

**Returns** `TTo`

##### ReverseMap(value)

Runs the reverse-map function.

```csharp
TFrom ReverseMap(TTo value)
```

- `value` — mapped value

**Returns** `TFrom`

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

#### StatementMapper

```csharp
public sealed class StatementMapper<T> : IMapper<T, T> where T : new()
```

**Properties**

##### Entity

Maps the table or entity name when the repository is built.

```csharp
IMapper<string, string> Entity { get; init; }
```

##### Columns

Maps column names when the repository is built.

```csharp
IMapper<string, string> Columns { get; init; }
```

##### Query

Rewrites a finished query.

```csharp
IMapper<IQuery, IQuery>? Query { get; init; }
```

##### Insert

Rewrites a finished insert.

```csharp
IMapper<IInsertStatement, IInsertStatement>? Insert { get; init; }
```

##### Update

Rewrites a finished update.

```csharp
IMapper<IUpdateStatement, IUpdateStatement>? Update { get; init; }
```

##### Delete

Rewrites a finished delete.

```csharp
IMapper<IDeleteStatement, IDeleteStatement>? Delete { get; init; }
```

##### StoredProcedure

Rewrites a finished stored-procedure statement.

```csharp
IMapper<IStoredProcedureStatement, IStoredProcedureStatement>? StoredProcedure { get; init; }
```

##### Rows

Get-row [IMapper](#imapper). [Map](#statementmapper-map-value) / [ReverseMap](#statementmapper-reversemap-value) delegate here.

```csharp
IMapper<T, T> Rows { get; init; }
```

**Methods**

##### Map(value)

Maps a row through [Rows](#statementmapper-rows).

```csharp
T Map(T value)
```

- `value` — converted row

**Returns** `T`

##### ReverseMap(value)

Reverse-maps a row through [Rows](#statementmapper-rows).

```csharp
T ReverseMap(T value)
```

- `value` — shaped row

**Returns** `T`

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

#### SourceDestinationMapper

```rust
pub trait SourceDestinationMapper
```

**Methods**

##### source()

Dialect or provider label for the source side.

```rust
fn source(&self) -> &str
```

**Returns** `&str`

##### destination()

Dialect or provider label for the destination side.

```rust
fn destination(&self) -> &str
```

**Returns** `&str`

#### NameMapper

```rust
pub trait NameMapper
```

**Methods**

##### map(name)

Returns the name unchanged.

```rust
fn map(&self, name: &str) -> String
```

- `name` — column or entity name

**Returns** `String`

##### reverse_map(name)

Returns the name unchanged.

```rust
fn reverse_map(&self, name: &str) -> String
```

- `name` — column or entity name

**Returns** `String`

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

#### TransformMapper

```rust
pub struct TransformMapper<TFrom, TTo>
```

**Constructor**

##### new(map, reverse_map)

Builds a mapper from a pair of functions.

```rust
pub fn new(map: impl Fn(TFrom) -> TTo + Send + Sync + 'static, reverse_map: impl Fn(TTo) -> TFrom + Send + Sync + 'static) -> Self
```

- `map` — `TFrom` to `TTo`
- `reverse_map` — `TTo` to `TFrom`

**Returns** [TransformMapper](#transformmapper)

**Methods**

##### map(value)

Runs the map function.

```rust
fn map(&self, value: TFrom) -> TTo
```

- `value` — source value

**Returns** `TTo`

##### reverse_map(value)

Runs the reverse-map function.

```rust
fn reverse_map(&self, value: TTo) -> TFrom
```

- `value` — mapped value

**Returns** `TFrom`

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

- `rows` — rows already passed through [map](#flattotypemapper-map-value)

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

- `rows` — rows already passed through [map](#flattotypemapper-map-value)

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

#### StatementMapper

```rust
pub struct StatementMapper
```

**Properties**

##### entity

Maps the table or entity name when the repository is built.

```rust
pub entity: TransformMapper<String, String>
```

##### columns

Maps column names when the repository is built.

```rust
pub columns: TransformMapper<String, String>
```

##### query

Rewrites finished query SQL.

```rust
pub query: Option<Arc<TransformMapper<String, String>>>
```

##### insert

Rewrites finished insert SQL.

```rust
pub insert: Option<Arc<TransformMapper<String, String>>>
```

##### update

Rewrites finished update SQL.

```rust
pub update: Option<Arc<TransformMapper<String, String>>>
```

##### delete

Rewrites finished delete SQL.

```rust
pub delete: Option<Arc<TransformMapper<String, String>>>
```

##### stored_procedure

Rewrites finished stored-procedure SQL.

```rust
pub stored_procedure: Option<Arc<TransformMapper<String, String>>>
```

##### rows

Get-row mapper. [map](#statementmapper-map-value) / [reverse_map](#statementmapper-reverse_map-value) delegate here.

```rust
pub rows: TransformMapper<Row, Row>
```

**Methods**

##### identity(source, destination)

Identity statement mapper. Source and destination labels are unused.

```rust
pub fn identity(_source: impl Into<String>, _destination: impl Into<String>) -> Self
```

- `source` — unused source label
- `destination` — unused destination label

**Returns** [StatementMapper](#statementmapper)

##### map_sql(mapper, text)

Applies an optional SQL-text mapper.

```rust
pub fn map_sql(mapper: &Option<Arc<TransformMapper<String, String>>>, text: String) -> String
```

- `mapper` — optional SQL-text [TransformMapper](#transformmapper)
- `text` — finished SQL

**Returns** `String`

##### map(value)

Maps a row through [rows](#statementmapper-rows).

```rust
fn map(&self, value: Row) -> Row
```

- `value` — converted row

**Returns** `Row`

##### reverse_map(value)

Reverse-maps a row through [rows](#statementmapper-rows).

```rust
fn reverse_map(&self, value: Row) -> Row
```

- `value` — shaped row

**Returns** `Row`
:::

## Remarks

Repository get is convert, then map. `StatementMapper.Rows` / `rows` is the get-row mapper. `Repository.Mapper` / `mapper` aliases it.

Apply statement mapping in either or both places:

1. **Builder** — `RepositoryBuilder` maps `Entity` / `entity` and `Columns` / `columns` into `RepositoryStatements`.
2. **Per-statement** — `Query` / `Insert` / `Update` / `Delete` rewrite the finished statement.

C# and TypeScript statement mappers rewrite statement objects. Rust `query` / `insert` / `update` / `delete` / `stored_procedure` are `TransformMapper<String, String>` over SQL text via `StatementMapper::map_sql`.

`NameMapper` passes names through as-is.

## Examples

### Flatten with TransformMapper

::: example
```csharp csharp/Datasources.Tests/NameMapperTests.cs#L32-L45
```
```rust rust/datasources/src/mappers/mod.rs#L144-L180
```
```typescript typescript/datasources/src/mappers/nameMapper.test.ts#L41-L66
```
:::

### Pass-through names

::: example
```csharp csharp/Datasources.Tests/NameMapperTests.cs#L15-L25
```
```rust rust/datasources/src/mappers/mod.rs#L107-L116
```
```typescript typescript/datasources/src/mappers/nameMapper.test.ts#L19-L27
```
:::

### Rewrite a finished statement

::: example
```csharp csharp/Datasources.Tests/StatementMapperTests.cs#L55-L81
```
```rust rust/datasources/src/tests/mod.rs#L794-L804
```
```typescript typescript/datasources/src/mappers/StatementMapper.test.ts#L27-L59
```
:::
