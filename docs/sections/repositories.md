# Repositories

Typed read and write over a connection. `IReadonlyRepository<T>` is get-only; `ICRUDRepository<T>` (C#) / `IRepository<T>` (TypeScript) / `Repository` (Rust) adds `create`/`add`, `update`, and `delete`. SQL providers typically build a table repository from `RepositoryBuilder` and a `TypeConverter` mapping.

## IReadonlyRepository / ICRUDRepository

| C# | Rust | TypeScript |
| --- | --- | --- |
| `IReadonlyRepository<T>` | `ReadonlyRepository` | `IReadonlyRepository<T>` |
| `ICRUDRepository<T>` | `Repository` | `IRepository<T>` |
| `GetSingle` | `get_single` | `getSingle` |
| `Get` | `get` | `get` |
| `Create` | `add` | `add` |
| `Update` | `update` | `update` |
| `Delete` | `delete` | `delete` |

## Factory

Instance registry of factory callbacks. Register a builder, then create by entity type name or by type. The callback receives an optional connection.

| C# | Rust | TypeScript |
| --- | --- | --- |
| `IRepositoryFactory` | `ReadonlyRepositoryFactory` | `IReadonlyRepositoryFactory` |
| `ICRUDRepositoryFactory` | `CrudRepositoryFactory` | `IRepositoryFactory` |
| `RepositoryFactory` | `RepositoryFactory` | `RepositoryFactory` |
| `Register` / `Create` | `register` / `create` | `register` / `create` |
| `Create<T>` / `Create(typeof(T))` | `register_type` / `create_type` | `register(type)` / `create(type)` |
| `RepositoryNotRegisteredException` | `DatasourceError` | `RepositoryNotRegisteredError` |

Registering CRUD also registers readonly. Registering readonly-only makes CRUD `create` throw.

```csharp
var factory = new RepositoryFactory();
factory.Register<Contact>(connection => new Repository<Contact>(connection!));
var repo = factory.Create<Contact>();
var byName = factory.Create("Contact");
```

## Enrichment

A read-only service wraps an inner repository, then fills properties already on `T` from related repositories. This is a post-Get step (after convert-then-map). It does not join or collapse rows.

| C# | Rust | TypeScript |
| --- | --- | --- |
| `IEnriched<T>` | `Enriched` | `IEnriched<T>` |
| `IRepositoryTree` | `RepositoryTree` | `IRepositoryTree` |
| `IRepositoryTreeNode` | `RepositoryTreeNode` | `IRepositoryTreeNode` |
| `EnrichedReadonlyRepository<T>` | `EnrichedReadonlyRepository` | `EnrichedReadonlyRepository<T>` |
| `DeepCRUDRepository<T>` | `DeepCRUDRepository` | `DeepCRUDRepository<T>` |
| `AddEnriched` | `add_enriched` / `add_enriched_repo` | `addEnriched` |
| `Enrich` | `enrich` | `enrich` |

Lookups use `GetSingle` / `get_single_where` (cached per distinct key). Lists use `Get` / `get_where`. Missing lookups leave the destination default.

Contacts as the model — fill existing fields only (`contact_type_name`, `contact_source_name`, `addresses`, `phones`):

```csharp
AddEnriched("contact_type_name", "contact_type_id", types, "id", "name");
AddEnriched("contact_source_name", "contact_source_id", sources, "id", "name");
AddEnriched("addresses", "id", addresses, "contact_id", many: true);
AddEnriched("phones", "id", phones, "contact_id", many: true);
```

Nested rules run on the related objects after that node fills. `AddEnriched` stays fluent for siblings; `nest` receives the child node:

```csharp
AddEnriched("children", "id", children, "parent_id", many: true,
    nest: node => node.Include(childTypes, "type_name", "type_id", "id", "name"));
```

Nest only on walkable objects (`many: true` or a whole related row). A `select` scalar is not walked. Depth is unbounded; each node applies, then children run on the related items.

`DeepCRUDRepository` is the write side of the same tree. `Get` still enriches. `select` lookups and 1:1 embeds are not written. Each `many` node must be a CRUD repository (Rust: `add_enriched_repo`).

- **Create** — write the parent, apply generated identity to `id`, stamp each child's `by` from the parent `from`, then create children (and nested many).
- **Update** — write the parent, then replace-sync each many list (delete missing, update matches by `id`, create the rest).
- **Delete** — delete nested many rows first, then children, then the parent. Loads existing children by FK; does not trust payload lists.

## Examples

TODO: fill as the generic `Repository<T>` / `TableRepository` surface settles.
