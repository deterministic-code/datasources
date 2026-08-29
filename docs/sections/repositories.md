# Repositories

Typed read and write over a connection. `IReadonlyRepository<T>` is get-only; `IRepository<T>` adds `add`, `update`, and `delete`. SQL providers typically build a table repository from `RepositoryBuilder` and a `TypeConverter` mapping.

## IReadonlyRepository / IRepository

| C# | Rust | TypeScript |
| --- | --- | --- |
| `IReadonlyRepository<T>` | `ReadonlyRepository` | `IReadonlyRepository<T>` |
| `IRepository<T>` | `Repository` | `IRepository<T>` |
| `GetSingle` | `get_single` | `getSingle` |
| `Get` | `get` | `get` |
| `Add` | `add` | `add` |
| `Update` | `update` | `update` |
| `Delete` | `delete` | `delete` |

## Examples

TODO: fill as the generic `Repository<T>` / `TableRepository` surface settles.
