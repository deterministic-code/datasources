# TypeConverters

`TypeConverter` is only `Convert` / `ReverseConvert`. `RowTypeConverter` has `Types` and `Fields`. Convert uses `Fields[name]`.

A dialect adds a `TypeConverter` only when the native provider cannot map the type. Core does not ship bool / datetime / uuid / decimal / binary converters.

## TypeConverter

| C# | Rust | TypeScript |
| --- | --- | --- |
| `TypeConverter<To, From>` | `TypeConverter<To, From>` | `TypeConverter` |
| `Convert` | `convert` | `convert` |
| `ReverseConvert` | `reverse_convert` | `reverseConvert` |

## RowTypeConverter

| C# | Rust | TypeScript |
| --- | --- | --- |
| `RowTypeConverter<T>` | `RowTypeConverter` | `RowTypeConverter` |
| `Convert` / `ReverseConvert` | `convert` / `reverse_convert` | `convert` / `reverseConvert` |
| `Types` / `Fields` | `types` / `fields` | `types` / `fields` |
