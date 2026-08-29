# TypeConverters

Registry of provider type tables and per-field converters. Register a provider’s types, then `Mapping` / `mapping` to project rows to a host type. The host destination is `"csharp"`, `"rust"`, or `"typescript"`.

## TypeConverter

| C# | Rust | TypeScript |
| --- | --- | --- |
| `TypeConverter` | `use_provider` / `mapping` | `TypeConverter` |
| `Use` | `use_provider` | `use` |
| `Mapping<T>` | `mapping` / `mapping_with` | `mapping` |
| `Reset` | `reset` | `reset` |
| `SqlProviderTypes` | `sql_provider_types` | `sqlProviderTypes` |
| `IdentityProviderTypes` | `identity_provider_types` | `identityProviderTypes` |

## FieldConverter

| C# | Rust | TypeScript |
| --- | --- | --- |
| `IFieldConverter` / `FieldConverter` | `FieldConverter` | `FieldConverter` |
| `FieldConverters` (Identity, Boolean, DateTime, Decimal, Binary, Uuid) | `identity_field_converter`, `boolean_field_converter`, … | `identityFieldConverter`, … |

## Examples

TODO: register a provider table and map a row type as converters are documented.
