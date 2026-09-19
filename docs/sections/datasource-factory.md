# DatasourceFactory

Resolves a provider from options and returns one configured datasource.

## Classes

::: reference
### typescript
#### IDatasourceFactory

```typescript
interface IDatasourceFactory
```

**Properties**

##### provider

Registry key used by `Datasource.register`.

```typescript
readonly provider: string
```

**Methods**

##### create(options?)

Creates a configured datasource.

```typescript
create(options?: DatasourceOptions): IDatasource
```

- `options` — [DatasourceOptions](./datasource.md#datasourceoptions)

**Returns** [IDatasource](./datasource.md#idatasource)

### csharp
#### DatasourceFactory

```csharp
public sealed class DatasourceFactory : IDatasourceFactory
```

**Methods**

##### Defaults(options)

Registers listed provider packages. Does not apply to a non-null `Create` bag.

```csharp
public static void Defaults(IReadOnlyDictionary<string, string?> options)
```

- `options` — install keys such as `INSTALLED_PROVIDERS`; not [Create options](./datasource.md#datasourceoptions)

##### Register(provider, connection, repository)

Registers a connection factory and a repository factory by provider name.

```csharp
public static void Register(string provider, Func<IReadOnlyDictionary<string, string?>?, IConnection> connection, Func<IConnection, object> repository)
```

- `provider` — registry key (`sqlite`, `mysql`, …)
- `connection` — builds an [IConnection](./connection.md) from the options bag
- `repository` — builds a repository from an [IConnection](./connection.md)

##### Create(options)

Creates one datasource. `null` options read process `.env`; a dictionary is used as-is.

```csharp
public static IDatasource Create(IReadOnlyDictionary<string, string?>? options = null)
```

- `options` — [DatasourceOptions](./datasource.md#datasourceoptions); omitted uses `DATABASE_URL` / `DATABASE_SCHEMA`

**Returns** [IDatasource](./datasource.md#idatasource)

##### Reset()

Clears the registry. Use in tests.

```csharp
public static void Reset()
```

##### Registered()

Lists registered provider names.

```csharp
public static IReadOnlyList<string> Registered()
```

**Returns** `IReadOnlyList<string>`

#### IDatasourceFactory

```csharp
public interface IDatasourceFactory
```

**Methods**

##### Create(options)

Creates one datasource from a string dictionary.

```csharp
IDatasource Create(IReadOnlyDictionary<string, string?>? options = null)
```

- `options` — [DatasourceOptions](./datasource.md#datasourceoptions)

**Returns** [IDatasource](./datasource.md#idatasource)

### rust
#### DatasourceFactory

```rust
pub trait DatasourceFactory
```

**Methods**

##### provider()

Registry key used by `register`.

```rust
fn provider(&self) -> &str
```

**Returns** `&str`

##### create(options)

Creates a configured datasource.

```rust
fn create(&self, options: Option<&HashMap<String, String>>) -> Arc<dyn Datasource>
```

- `options` — [DatasourceOptions](./datasource.md#datasourceoptions)

**Returns** `Arc<dyn `[Datasource](./datasource.md#datasource)`>`
:::

## Remarks

C# registers a connection factory and a repository factory with `DatasourceFactory.Register`, then `Create` returns one [Datasource](./datasource.md). TypeScript and Rust still register a provider type that implements both the factory and the datasource. See [Datasource](./datasource.md) for the instance contract.

A missing factory throws `ProviderNotInstalledError` / `ProviderNotInstalledException` / `DatasourceError::ProviderNotInstalled`.

`Datasources.Hosting` registers each name in `INSTALLED_PROVIDERS` (and the provider inferred from `DATABASE_URL`) when `Create` is called with null options. `@datasources/register` and `datasources-register` still do that when TypeScript / Rust apply defaults.

## Examples

### Register a factory

::: dialects
```csharp
DatasourceFactory.Register("memory", opts => new MemoryConnection(opts), MemoryConnection.Repository);
var ds = DatasourceFactory.Create(new Dictionary<string, string?> { ["provider"] = "memory" });
```
```rust
register(MemoryDatasource::instance());
let ds = create("memory", None)?;
```
```typescript
Datasource.register(MemoryDatasource.instance);
const ds = Datasource.create("memory");
```
:::
