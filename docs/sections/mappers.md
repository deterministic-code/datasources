# Mappers

Holds source/destination labels. Column names pass through as-is.

## IMapper

| C# | Rust | TypeScript |
| --- | --- | --- |
| `IMapper<TFrom, TTo>` | `Mapper<TFrom, TTo>` | `IMapper<TFrom, TTo>` |
| `Map` | `map` | `map` |
| `ReverseMap` | `reverse_map` | `reverseMap` |

## ISourceDestinationMapper

| C# | Rust | TypeScript |
| --- | --- | --- |
| `ISourceDestinationMapper` | `SourceDestinationMapper` | `ISourceDestinationMapper` |
| `Source` | `source` | `source` |
| `Destination` | `destination` | `destination` |

## NameMapper

| C# | Rust | TypeScript |
| --- | --- | --- |
| `NameMapper` | `NameMapper` | `NameMapper` |
| `Map` | `map` | `map` |
| `ReverseMap` | `reverse_map` | `reverseMap` |

## NameSourceDestinationMapper

| C# | Rust | TypeScript |
| --- | --- | --- |
| `NameSourceDestinationMapper` | `NameSourceDestinationMapper` | `NameSourceDestinationMapper` |
| constructor | `new` | constructor |
| `Source` | `source` | `source` |
| `Destination` | `destination` | `destination` |

## TransformMapper

| C# | Rust | TypeScript |
| --- | --- | --- |
| `TransformMapper<TFrom, TTo>` | `TransformMapper<TFrom, TTo>` | `TransformMapper<TFrom, TTo>` |
| constructor(`map`, `reverseMap`) | `new(map, reverse_map)` | constructor(`map`, `reverseMap`) |
| `Map` | `map` | `map` |
| `ReverseMap` | `reverse_map` | `reverseMap` |

## FlatToTypeMapper

Collapses denormalized join rows into a typed graph. Scalars map column → field; groups collect unique children by key. Nested group fields on a collection are omitted.

| C# | Rust | TypeScript |
| --- | --- | --- |
| `IFlatToTypeMapper` | `FlatToTypeMapperTrait` | `IFlatToTypeMapper<T>` |
| `FlatToTypeMapper` | `FlatToTypeMapper` | `FlatToTypeMapper<T>` |
| `TypeMap` | `TypeMap` | `TypeMap<T>` |
| `TypeMapGroup` | `TypeMapGroup` | `TypeMapGroup<E>` |
| `TypeMapField` | `TypeMapField` | string or group |
| `MappedValue` | `MappedValue` | `T` fields |
| `Map` / `ReverseMap` | `map` / `reverse_map` | `map` / `reverseMap` |
| `Collapse` | `collapse` / `collapse_rows` | `collapse` |

## StatementMapper

Maps entity and column names when the repository is built, and can map each finished statement. `Rows` is the get-row `IMapper<T, T>`. `Repository` only convert-then-maps rows.

Apply in either or both places:

1. **Builder** — `RepositoryBuilder` maps `Entity` and `Columns` into `RepositoryStatements`.
2. **Per-statement** — `Query` / `Insert` / `Update` / `Delete` mappers rewrite the finished statement.

| C# | Rust | TypeScript |
| --- | --- | --- |
| `StatementMapper<T>` | `StatementMapper` | `StatementMapper<T>` |
| `Entity` | `entity` | `entity` |
| `Columns` | `columns` | `columns` |
| `Query` | `query` | `query` |
| `Insert` | `insert` | `insert` |
| `Update` | `update` | `update` |
| `Delete` | `delete` | `delete` |
| `Rows` | `rows` | `rows` |
| `Map` / `ReverseMap` | `map` / `reverse_map` | `map` / `reverseMap` |
