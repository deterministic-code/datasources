<!-- Generated from docs/. Edit the markdown in docs/ and run npm run docs:skill. -->

# Type Converters

`TypeConverter` is only `Convert` / `convert` / `convert` and `ReverseConvert` / `reverseConvert` / `reverse_convert`. A dialect adds a converter when the native provider cannot map the type. Core does not ship bool / datetime / uuid / decimal / binary converters.

## Classes

#### TypeScript

#### TypeConverter {#typescript-typeconverter}

```typescript
abstract class TypeConverter<To = Cells, From = Cells>
```

**Methods**

| Name | Description |
| --- | --- |
| [convert(from)](#typescript-typeconverter-convert-from) | Converts `From` to `To`. |
| [reverseConvert(to)](#typescript-typeconverter-reverseconvert-to) | Converts `To` back to `From`. |

##### convert(from) {#typescript-typeconverter-convert-from}

Converts `From` to `To`.

**Definition**

```typescript
abstract convert(from: From): To
```

**Parameters**

| Name | Description |
| --- | --- |
| `from` | source value |

**Returns**

`To`

##### reverseConvert(to) {#typescript-typeconverter-reverseconvert-to}

Converts `To` back to `From`.

**Definition**

```typescript
abstract reverseConvert(to: To): From
```

**Parameters**

| Name | Description |
| --- | --- |
| `to` | converted value |

**Returns**

`From`

#### C#

#### ITypeConverter {#csharp-itypeconverter}

```csharp
public interface ITypeConverter<To, From>
```

**Methods**

| Name | Description |
| --- | --- |
| [Convert(from)](#csharp-itypeconverter-convert-from) | Converts `From` to `To`. |
| [ReverseConvert(to)](#csharp-itypeconverter-reverseconvert-to) | Converts `To` back to `From`. |

##### Convert(from) {#csharp-itypeconverter-convert-from}

Converts `From` to `To`.

**Definition**

```csharp
To Convert(From from)
```

**Parameters**

| Name | Description |
| --- | --- |
| `from` | source value |

**Returns**

`To`

##### ReverseConvert(to) {#csharp-itypeconverter-reverseconvert-to}

Converts `To` back to `From`.

**Definition**

```csharp
From ReverseConvert(To to)
```

**Parameters**

| Name | Description |
| --- | --- |
| `to` | converted value |

**Returns**

`From`

#### TypeConverter {#csharp-typeconverter}

```csharp
public abstract class TypeConverter<To, From> : ITypeConverter<To, From>
```

**Methods**

| Name | Description |
| --- | --- |
| [Convert(from)](#csharp-typeconverter-convert-from) | Converts `From` to `To`. |
| [ReverseConvert(to)](#csharp-typeconverter-reverseconvert-to) | Converts `To` back to `From`. |

##### Convert(from) {#csharp-typeconverter-convert-from}

Converts `From` to `To`.

**Definition**

```csharp
public abstract To Convert(From from)
```

**Parameters**

| Name | Description |
| --- | --- |
| `from` | source value |

**Returns**

`To`

##### ReverseConvert(to) {#csharp-typeconverter-reverseconvert-to}

Converts `To` back to `From`.

**Definition**

```csharp
public abstract From ReverseConvert(To to)
```

**Parameters**

| Name | Description |
| --- | --- |
| `to` | converted value |

**Returns**

`From`

#### Rust

#### TypeConverter {#rust-typeconverter}

```rust
pub trait TypeConverter<To, From>
```

**Methods**

| Name | Description |
| --- | --- |
| [convert(from)](#rust-typeconverter-convert-from) | Converts `From` to `To`. |
| [reverse_convert(to)](#rust-typeconverter-reverse_convert-to) | Converts `To` back to `From`. |

##### convert(from) {#rust-typeconverter-convert-from}

Converts `From` to `To`.

**Definition**

```rust
fn convert(&self, from: &From) -> Result<To>
```

**Parameters**

| Name | Description |
| --- | --- |
| `from` | source value |

**Returns**

`Result<To>`

##### reverse_convert(to) {#rust-typeconverter-reverse_convert-to}

Converts `To` back to `From`.

**Definition**

```rust
fn reverse_convert(&self, to: &To) -> Result<From>
```

**Parameters**

| Name | Description |
| --- | --- |
| `to` | converted value |

**Returns**

`Result<From>`

## Remarks

Repository read is convert, then map. See [TransformMapper](mappings.md) and [Repositories](repositories.md).

## See also

- [MetaDataRowTypeConverter](#)

## Metadata Row Type Converter

Default object converter. Read order is the renamed row, then `RowConverter` / `rowConverter` / `row_converter` when one is set, then this converter.

`RowConverter` / `rowConverter` / `row_converter` is the `Row` to `Row` cell step. Omitted, that step is skipped and the renamed row is passed through. This converter still runs. It assigns the row onto `T`. A handwritten object converter assigns each column and does not reflect.

`Convert` / `convert` / `convert` and `ReverseConvert` / `reverseConvert` / `reverse_convert` use `Fields` / `fields` / `fields`.

### Classes

##### TypeScript

##### MetaDataRowTypeConverter {#typescript-metadatarowtypeconverter}

```typescript
class MetaDataRowTypeConverter<T = Cells> extends TypeConverter<T, Cells>
```

**Constructor**

| Name | Description |
| --- | --- |
| [MetaDataRowTypeConverter(fields)](#typescript-metadatarowtypeconverter-metadatarowtypeconverter-fields) | Builds the default object converter. Convert uses [fields](#typescript-metadatarowtypeconverter-fields). |

**Properties**

| Name | Description |
| --- | --- |
| fields | Per-column converters. Convert uses `fields[name]`. |

**Methods**

| Name | Description |
| --- | --- |
| [convert(row, type?)](#typescript-metadatarowtypeconverter-convert-row-type) | Assigns cells onto `T`. Optional `type` constructs the destination. A field converter replaces that cell. |
| [reverseConvert(row)](#typescript-metadatarowtypeconverter-reverseconvert-row) | Reverse-converts each cell through [fields](#typescript-metadatarowtypeconverter-fields). Skips array values. |

###### MetaDataRowTypeConverter(fields) {#typescript-metadatarowtypeconverter-metadatarowtypeconverter-fields}

Builds the default object converter. Convert uses [fields](#typescript-metadatarowtypeconverter-fields).

**Definition**

```typescript
constructor(fields?: Record<string, TypeConverter<Cell, Cell>>)
```

**Parameters**

| Name | Description |
| --- | --- |
| `fields` | per-column TypeConverter |

###### convert(row, type?) {#typescript-metadatarowtypeconverter-convert-row-type}

Assigns cells onto `T`. Optional `type` constructs the destination. A field converter replaces that cell.

**Definition**

```typescript
convert(row: Cells, type?: Constructable<T>): T
```

**Parameters**

| Name | Description |
| --- | --- |
| `row` | source cells |
| `type` | optional `new () => T` |

**Returns**

`T`

###### reverseConvert(row) {#typescript-metadatarowtypeconverter-reverseconvert-row}

Reverse-converts each cell through [fields](#typescript-metadatarowtypeconverter-fields). Skips array values.

**Definition**

```typescript
reverseConvert(row: T): Cells
```

**Parameters**

| Name | Description |
| --- | --- |
| `row` | converted object |

**Returns**

`Cells`

##### C#

##### MetaDataRowTypeConverter {#csharp-metadatarowtypeconverter}

```csharp
public sealed class MetaDataRowTypeConverter<T> : TypeConverter<T, Row> where T : new()
```

**Constructor**

| Name | Description |
| --- | --- |
| [MetaDataRowTypeConverter(fields)](#csharp-metadatarowtypeconverter-metadatarowtypeconverter-fields) | Builds the default object converter. Convert assigns public scalar properties, or every key when `T` is a dictionary. |

**Properties**

| Name | Description |
| --- | --- |
| Fields | Per-column converters. Convert uses `Fields[name]`. |

**Methods**

| Name | Description |
| --- | --- |
| [Convert(from)](#csharp-metadatarowtypeconverter-convert-from) | Assigns each cell onto a new `T`. A property is set only when the cell is already that type. |
| [ReverseConvert(to)](#csharp-metadatarowtypeconverter-reverseconvert-to) | Writes public scalar properties back to a row through [Fields](#csharp-metadatarowtypeconverter-fields). |

###### MetaDataRowTypeConverter(fields) {#csharp-metadatarowtypeconverter-metadatarowtypeconverter-fields}

Builds the default object converter. Convert assigns public scalar properties, or every key when `T` is a dictionary.

**Definition**

```csharp
public MetaDataRowTypeConverter(IReadOnlyDictionary<string, TypeConverter<Value, Value>>? fields = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `fields` | per-column TypeConverter |

###### Convert(from) {#csharp-metadatarowtypeconverter-convert-from}

Assigns each cell onto a new `T`. A property is set only when the cell is already that type.

**Definition**

```csharp
public override T Convert(Row from)
```

**Parameters**

| Name | Description |
| --- | --- |
| `from` | source row |

**Returns**

`T`

###### ReverseConvert(to) {#csharp-metadatarowtypeconverter-reverseconvert-to}

Writes public scalar properties back to a row through [Fields](#csharp-metadatarowtypeconverter-fields).

**Definition**

```csharp
public override Row ReverseConvert(T to)
```

**Parameters**

| Name | Description |
| --- | --- |
| `to` | converted object |

**Returns**

`Row`

##### Rust

##### MetaDataRowTypeConverter {#rust-metadatarowtypeconverter}

```rust
pub struct MetaDataRowTypeConverter
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(columns)](#rust-metadatarowtypeconverter-new-columns) | Builds an object converter. An empty column list copies every cell. A non-empty list copies those columns and drops the rest. |
| [of()](#rust-metadatarowtypeconverter-of) | Builds a converter from `T: ConvertSchema`. Convert copies those columns. |

**Properties**

| Name | Description |
| --- | --- |
| fields | Per-column converters. Convert uses `fields[name]`. |

**Methods**

| Name | Description |
| --- | --- |
| [with_fields(fields)](#rust-metadatarowtypeconverter-with_fields-fields) | Merges per-column converters into [fields](#rust-metadatarowtypeconverter-fields). |
| [convert(row)](#rust-metadatarowtypeconverter-convert-row) | Copies schema columns, or every cell when the column list is empty. A field converter replaces that cell. |
| [reverse_convert(row)](#rust-metadatarowtypeconverter-reverse_convert-row) | Reverse-converts the same columns through [fields](#rust-metadatarowtypeconverter-fields). |

###### new(columns) {#rust-metadatarowtypeconverter-new-columns}

Builds an object converter. An empty column list copies every cell. A non-empty list copies those columns and drops the rest.

**Definition**

```rust
pub fn new(columns: Vec<String>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `columns` | schema column names; empty copies every cell |

**Returns**

[MetaDataRowTypeConverter](#rust-metadatarowtypeconverter)

###### of() {#rust-metadatarowtypeconverter-of}

Builds a converter from `T: ConvertSchema`. Convert copies those columns.

**Definition**

```rust
pub fn of<T: ConvertSchema>() -> Self
```

**Returns**

[MetaDataRowTypeConverter](#rust-metadatarowtypeconverter)

###### with_fields(fields) {#rust-metadatarowtypeconverter-with_fields-fields}

Merges per-column converters into [fields](#rust-metadatarowtypeconverter-fields).

**Definition**

```rust
pub fn with_fields(self, fields: HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `fields` | per-column TypeConverter |

**Returns**

[MetaDataRowTypeConverter](#rust-metadatarowtypeconverter)

###### convert(row) {#rust-metadatarowtypeconverter-convert-row}

Copies schema columns, or every cell when the column list is empty. A field converter replaces that cell.

**Definition**

```rust
pub fn convert(&self, row: &Row) -> Result<Row>
```

**Parameters**

| Name | Description |
| --- | --- |
| `row` | source row |

**Returns**

`Result<Row>`

###### reverse_convert(row) {#rust-metadatarowtypeconverter-reverse_convert-row}

Reverse-converts the same columns through [fields](#rust-metadatarowtypeconverter-fields).

**Definition**

```rust
pub fn reverse_convert(&self, row: &Row) -> Result<Row>
```

**Parameters**

| Name | Description |
| --- | --- |
| `row` | converted row |

**Returns**

`Result<Row>`

### Remarks

The repository uses this converter when `ObjectConverter` / `objectConverter` / `object_converter` is omitted. `RowConverter` / `rowConverter` / `row_converter` stays the cell step. Rust rows are `Row`, so this converter is `Row` to `Row`. `of::<T>()` copies the `ConvertSchema` columns. The repository default has an empty column list and copies every cell.

### Examples

#### Convert a row

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/RowTypeConverterTests.cs#L28-L47)

```csharp
var created = new DateTime(2020, 1, 2, 3, 4, 5, DateTimeKind.Utc);
var converter = new MetaDataRowTypeConverter<User>();
var user = converter.Convert(new Row
{
    ["Id"] = 7L,
    ["Username"] = "ada",
    ["IsEnabled"] = true,
    ["Created"] = created
});
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/converters/metadata_row_type_converter.rs#L117-L130)

```rust
let converter = MetaDataRowTypeConverter::of::<User>();
let row = record(&[
    ("id", Value::Int(1)),
    ("name", Value::from("ada")),
    ("extra", Value::from("no")),
]);
let converted = converter.convert(&row).unwrap();
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/converters/RowTypeConverter.test.ts#L14-L19)

```typescript
const converter = new MetaDataRowTypeConverter();
const item = converter.convert({ id: 7, username: "ada" });
const row = converter.reverseConvert(item);
```

#### Field converter

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/RowTypeConverterTests.cs#L111-L122)

```csharp
var converter = new MetaDataRowTypeConverter<Dictionary<string, Value>>(new Dictionary<string, TypeConverter<Value, Value>>
{
    ["flag"] = new OnlyOne()
});
var dest = converter.Convert(new Row { ["flag"] = 1L, ["name"] = "a" });
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/converters/row_type_converter.rs#L151-L169)

```rust
let converter =
    RowTypeConverter::new(vec!["is_enabled".into()]).with_fields(HashMap::from([(
        "is_enabled".into(),
        Box::new(OnlyOne) as Box<dyn TypeConverter<Value, Value> + Send + Sync>,
    )]));
let row = record(&[("is_enabled", Value::Int(2))]);
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/converters/RowTypeConverter.test.ts#L22-L27)

```typescript
const converter = new MetaDataRowTypeConverter({ flag: new OnlyOne() });
const dest = converter.convert({ flag: 1, name: "a" });
```
