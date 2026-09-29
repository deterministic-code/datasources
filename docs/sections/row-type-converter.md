# RowTypeConverter

TypeConverter for a row. Holds {{Types|types|types}} and {{Fields|fields|fields}}. {{Convert|convert|convert}} uses {{Fields[name]|fields[name]|fields[name]}}. {{Types|types|types}} is the provider catalog merge and is unused at convert time.

## Classes

::: reference
### typescript
#### RowTypeConverter

```typescript
class RowTypeConverter<T = Cells> extends TypeConverter<T, Cells>
```

**Constructor**

##### RowTypeConverter(types, fields)

Builds a row converter. Convert uses [fields](#rowtypeconverter-fields), not [types](#rowtypeconverter-types).

```typescript
constructor(types?: Record<string, TypeConverter<Cell, Cell>>, fields?: Record<string, TypeConverter<Cell, Cell>>)
```

- `types` — provider catalog; unused at convert time
- `fields` — per-column TypeConverter

**Properties**

##### types

Provider type catalog. Convert does not read this map.

```typescript
types: Record<string, TypeConverter<Cell, Cell>>
```

##### fields

Per-column converters. Convert uses `fields[name]`.

```typescript
fields: Record<string, TypeConverter<Cell, Cell>>
```

**Methods**

##### convert(row, type?)

Converts each cell through [fields](#rowtypeconverter-fields). Optional `type` constructs a destination instance.

```typescript
convert(row: Cells, type?: Constructable<T>): T
```

- `row` — source cells
- `type` — optional `new () => T`

**Returns** `T`

##### reverseConvert(row)

Reverse-converts each cell through [fields](#rowtypeconverter-fields). Skips array values.

```typescript
reverseConvert(row: T): Cells
```

- `row` — converted object

**Returns** `Cells`

### csharp
#### RowTypeConverter

```csharp
public sealed class RowTypeConverter<T> : TypeConverter<T, Row> where T : new()
```

**Constructor**

##### RowTypeConverter(types, fields)

Builds a row converter. Convert uses [Fields](#rowtypeconverter-fields), not [Types](#rowtypeconverter-types).

```csharp
public RowTypeConverter(IReadOnlyDictionary<string, TypeConverter<Value, Value>>? types = null, IReadOnlyDictionary<string, TypeConverter<Value, Value>>? fields = null)
```

- `types` — provider catalog; unused at convert time
- `fields` — per-column TypeConverter

**Properties**

##### Types

Provider type catalog. Convert does not read this map.

```csharp
Dictionary<string, TypeConverter<Value, Value>> Types { get; set; }
```

##### Fields

Per-column converters. Convert uses `Fields[name]`.

```csharp
Dictionary<string, TypeConverter<Value, Value>> Fields { get; set; }
```

**Methods**

##### Convert(from)

Converts a row to `T`. Applies [Fields](#rowtypeconverter-fields), then assigns writable scalar properties (or dictionary keys).

```csharp
public override T Convert(Row from)
```

- `from` — source row

**Returns** `T`

##### ReverseConvert(to)

Reverse-converts `T` to a row through [Fields](#rowtypeconverter-fields).

```csharp
public override Row ReverseConvert(T to)
```

- `to` — converted object

**Returns** `Row`

### rust
#### RowTypeConverter

```rust
pub struct RowTypeConverter
```

**Constructor**

##### new(columns)

Builds a row converter for the given columns.

```rust
pub fn new(columns: Vec<String>) -> Self
```

- `columns` — column names

**Returns** [RowTypeConverter](#rowtypeconverter)

##### of()

Builds a converter from `T: ConvertSchema`.

```rust
pub fn of<T: ConvertSchema>() -> Self
```

**Returns** [RowTypeConverter](#rowtypeconverter)

**Properties**

##### types

Provider type catalog. Convert does not read this map.

```rust
pub types: HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>
```

##### fields

Per-column converters. Convert uses `fields[name]`.

```rust
pub fields: HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>
```

**Methods**

##### with_types(types)

Merges a provider catalog into [types](#rowtypeconverter-types).

```rust
pub fn with_types(self, types: HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>) -> Self
```

- `types` — provider catalog; unused at convert time

**Returns** [RowTypeConverter](#rowtypeconverter)

##### with_fields(fields)

Merges per-column converters into [fields](#rowtypeconverter-fields).

```rust
pub fn with_fields(self, fields: HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>) -> Self
```

- `fields` — per-column TypeConverter

**Returns** [RowTypeConverter](#rowtypeconverter)

##### convert(row)

Converts each cell through [fields](#rowtypeconverter-fields).

```rust
pub fn convert(&self, row: &Row) -> Result<Row>
```

- `row` — source row

**Returns** `Result<Row>`

##### reverse_convert(row)

Reverse-converts each cell through [fields](#rowtypeconverter-fields).

```rust
pub fn reverse_convert(&self, row: &Row) -> Result<Row>
```

- `row` — converted row

**Returns** `Result<Row>`
:::

## Remarks

{{Types|types|types}} is the provider catalog merge. {{Convert|convert|convert}} and {{ReverseConvert|reverseConvert|reverse_convert}} only consult {{Fields|fields|fields}}.

Repository read is convert, then map. See [TransformMapper](./transform-mapper.md) and [CRUD repository](./crud-repository.md).

## Examples

### Convert a row

::: example
```csharp csharp/Datasources.Tests/RowTypeConverterTests.cs#ConvertAndReverseConvertRoundTrip
```
```rust rust/datasources/src/converters/row_type_converter.rs#type_converter_round_trips_rows
```
```typescript typescript/datasources/src/converters/RowTypeConverter.test.ts#convert-and-reverseconvert-round-trip
```
:::

### Field converter

::: example
```csharp csharp/Datasources.Tests/RowTypeConverterTests.cs#AppliesFieldOverrideOnDictionaryRows
```
```rust rust/datasources/src/converters/row_type_converter.rs#field_override_beats_type_default
```
```typescript typescript/datasources/src/converters/RowTypeConverter.test.ts#lets-field-converters-override
```
:::
