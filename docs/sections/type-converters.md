# TypeConverters

`TypeConverter` is a named wrapper. `RowConverter` applies dialect default type converters, then field-name overrides.

Lookup per field: field override, then default type, then identity. The same order is used on reverse conversion.

## TypeConverter

| C# | Rust | TypeScript |
| --- | --- | --- |
| `TypeConverter<T>` | `TypeConverter` | `TypeConverter<T>` |
| `Convert` | `convert` | `convert` |
| `ReverseConvert` | `reverse_convert` | `reverseConvert` |

## RowConverter

| C# | Rust | TypeScript |
| --- | --- | --- |
| `RowConverter` | `RowConverter` | `RowConverter` |
| `Types` / `Fields` | `types` / `fields` | `types` / `fields` |
| `Convert` | `convert` | `convert` |
| `ReverseConvert` | `reverse_convert` | `reverseConvert` |

## ValueConverter

| C# | Rust | TypeScript |
| --- | --- | --- |
| `IValueConverter` | `ValueConverter` | `ValueConverter` |
| `Convert` | `convert` | `convert` |
| `ReverseConvert` | `reverse_convert` | `reverseConvert` |

Default type maps live on `CoreTypes` / `core_types`. Dialects pass theirs when building a repository (`SqliteTypes`, `converters::types()`).
