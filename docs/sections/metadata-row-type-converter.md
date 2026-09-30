# Metadata Row Type Converter

Default object converter. Read order is the renamed row, then {{RowConverter|rowConverter|row_converter}} when one is set, then this converter, then the rows mapper.

{{RowConverter|rowConverter|row_converter}} is the `Row` to `Row` cell step. Omitted, that step is skipped and the renamed row is passed through. This converter still runs. It assigns the row onto `T`. A handwritten object converter assigns each column and does not reflect.

{{Convert|convert|convert}} and {{ReverseConvert|reverseConvert|reverse_convert}} use {{Fields|fields|fields}}.

## Classes

::: reference
### typescript
#### MetaDataRowTypeConverter

```typescript
class MetaDataRowTypeConverter<T = Cells> extends TypeConverter<T, Cells>
```

**Constructor**

##### MetaDataRowTypeConverter(fields)

Builds the default object converter. Convert uses [fields](#metadatarowtypeconverter-fields).

```typescript
constructor(fields?: Record<string, TypeConverter<Cell, Cell>>)
```

- `fields` — per-column TypeConverter

**Properties**

##### fields

Per-column converters. Convert uses `fields[name]`.

```typescript
fields: Record<string, TypeConverter<Cell, Cell>>
```

**Methods**

##### convert(row, type?)

Assigns cells onto `T`. Optional `type` constructs the destination. A field converter replaces that cell.

```typescript
convert(row: Cells, type?: Constructable<T>): T
```

- `row` — source cells
- `type` — optional `new () => T`

**Returns** `T`

##### reverseConvert(row)

Reverse-converts each cell through [fields](#metadatarowtypeconverter-fields). Skips array values.

```typescript
reverseConvert(row: T): Cells
```

- `row` — converted object

**Returns** `Cells`

### csharp
#### MetaDataRowTypeConverter

```csharp
public sealed class MetaDataRowTypeConverter<T> : TypeConverter<T, Row> where T : new()
```

**Constructor**

##### MetaDataRowTypeConverter(fields)

Builds the default object converter. Convert assigns public scalar properties, or every key when `T` is a dictionary.

```csharp
public MetaDataRowTypeConverter(IReadOnlyDictionary<string, TypeConverter<Value, Value>>? fields = null)
```

- `fields` — per-column TypeConverter

**Properties**

##### Fields

Per-column converters. Convert uses `Fields[name]`.

```csharp
public Dictionary<string, TypeConverter<Value, Value>> Fields { get; set; }
```

**Methods**

##### Convert(from)

Assigns each cell onto a new `T`. A property is set only when the cell is already that type.

```csharp
public override T Convert(Row from)
```

- `from` — source row

**Returns** `T`

##### ReverseConvert(to)

Writes public scalar properties back to a row through [Fields](#metadatarowtypeconverter-fields).

```csharp
public override Row ReverseConvert(T to)
```

- `to` — converted object

**Returns** `Row`

### rust
#### MetaDataRowTypeConverter

```rust
pub struct MetaDataRowTypeConverter
```

**Constructor**

##### new(columns)

Builds an object converter. An empty column list copies every cell. A non-empty list copies those columns and drops the rest.

```rust
pub fn new(columns: Vec<String>) -> Self
```

- `columns` — schema column names; empty copies every cell

**Returns** [MetaDataRowTypeConverter](#metadatarowtypeconverter)

##### of()

Builds a converter from `T: ConvertSchema`. Convert copies those columns.

```rust
pub fn of<T: ConvertSchema>() -> Self
```

**Returns** [MetaDataRowTypeConverter](#metadatarowtypeconverter)

**Properties**

##### fields

Per-column converters. Convert uses `fields[name]`.

```rust
pub fields: HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>
```

**Methods**

##### with_fields(fields)

Merges per-column converters into [fields](#metadatarowtypeconverter-fields).

```rust
pub fn with_fields(self, fields: HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>) -> Self
```

- `fields` — per-column TypeConverter

**Returns** [MetaDataRowTypeConverter](#metadatarowtypeconverter)

##### convert(row)

Copies schema columns, or every cell when the column list is empty. A field converter replaces that cell.

```rust
pub fn convert(&self, row: &Row) -> Result<Row>
```

- `row` — source row

**Returns** `Result<Row>`

##### reverse_convert(row)

Reverse-converts the same columns through [fields](#metadatarowtypeconverter-fields).

```rust
pub fn reverse_convert(&self, row: &Row) -> Result<Row>
```

- `row` — converted row

**Returns** `Result<Row>`
:::

## Remarks

The repository uses this converter when {{ObjectConverter|objectConverter|object_converter}} is omitted. {{RowConverter|rowConverter|row_converter}} stays the cell step. Rust rows are `Row`, so this converter is `Row` to `Row`. `of::<T>()` copies the `ConvertSchema` columns. The repository default has an empty column list and copies every cell.

## Examples

### Convert a row

::: example
```csharp csharp/Datasources.Tests/RowTypeConverterTests.cs#ConvertAndReverseConvertRoundTrip
```
```rust rust/datasources/src/converters/metadata_row_type_converter.rs#copies_schema_columns_and_drops_the_rest
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
