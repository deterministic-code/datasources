# Mappers

Holds source/destination labels. Column names pass through as-is.

## IMapper

| C# | Rust | TypeScript |
| --- | --- | --- |
| `IMapper<TFrom, TTo>` | `IMapper<TFrom, TTo>` | `IMapper<TFrom, TTo>` |
| `Map` | `map` | `map` |
| `ReverseMap` | `reverse_map` | `reverseMap` |

## ISourceDestinationMapper

| C# | Rust | TypeScript |
| --- | --- | --- |
| `ISourceDestinationMapper` | `ISourceDestinationMapper` | `ISourceDestinationMapper` |
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
