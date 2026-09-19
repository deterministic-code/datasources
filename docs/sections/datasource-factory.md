# DatasourceFactory

Resolves a provider from options and returns one configured datasource.

## Classes

::: reference
### typescript
#### DatasourceFactory

```typescript
class DatasourceFactory implements IDatasourceFactory
```

**Properties**

##### default

Loads `.env` `PROVIDER` / `DATASOURCE_URL`, then `DEFAULT_PROVIDER` / `DEFAULT_DATASOURCE_URL`. Throws when neither is set. Uses [create](#create-options).

```typescript
static get default(): IDatasource
```

**Returns** [IDatasource](./datasource.md#idatasource)

**Methods**

##### defaults(options)

Registers listed provider packages. Does not apply to a non-null `create` bag.

```typescript
static defaults(options: DatasourceOptions): void
```

- `options` — install keys such as `INSTALLED_PROVIDERS`; not [Create options](./datasource.md#datasourceoptions)

##### register(provider, connection, repository)

Registers a connection factory and a repository factory by provider name.

```typescript
static register(provider: string, connection: (options?: DatasourceOptions) => IConnection, repository: (connection: IConnection) => object): void
```

- `provider` — registry key (`sqlite`, `mysql`, …)
- `connection` — builds an [IConnection](./connection.md) from the options bag
- `repository` — builds a repository from an [IConnection](./connection.md)

##### create(options?)

Creates one datasource. `undefined` options read process env; a dictionary is used as-is.

```typescript
static create(options?: DatasourceOptions): IDatasource
```

- `options` — [DatasourceOptions](./datasource.md#datasourceoptions); omitted uses `.env` then `DATASOURCE_URL` / `DATABASE_SCHEMA`

**Returns** [IDatasource](./datasource.md#idatasource)

##### reset()

Clears the registry. Use in tests.

```typescript
static reset(): void
```

##### registered()

Lists registered provider names.

```typescript
static registered(): string[]
```

**Returns** `string[]`

#### IDatasourceFactory

```typescript
interface IDatasourceFactory
```

**Methods**

##### create(options?)

Creates one datasource from a string dictionary.

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

**Properties**

##### Default

Loads `.env` `PROVIDER` / `DATASOURCE_URL`, then `DEFAULT_PROVIDER` / `DEFAULT_DATASOURCE_URL`. Throws when neither is set. Uses [Create](#create-options).

```csharp
public static IDatasource Default { get; }
```

**Returns** [IDatasource](./datasource.md#idatasource)

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

- `options` — [DatasourceOptions](./datasource.md#datasourceoptions); omitted uses `.env` then `DATASOURCE_URL` / `DATABASE_SCHEMA`

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
#### datasources

```rust
// datasources
```

**Methods**

##### defaults(options)

Registers listed provider packages. Does not apply to a `Some` `create` bag.

```rust
pub fn defaults(options: HashMap<String, String>) -> Result<()>
```

- `options` — install keys such as `INSTALLED_PROVIDERS`; not [Create options](./datasource.md#datasourceoptions)

**Returns** `Result<()>`

##### register(provider, connection, repository)

Registers a connection factory and a repository factory by provider name.

```rust
pub fn register(provider: impl Into<String>, connection: impl Fn(Option<&HashMap<String, String>>) -> Arc<dyn Connection> + Send + Sync + 'static, repository: impl Fn(Arc<dyn Connection>) -> Arc<dyn CRUDRepository> + Send + Sync + 'static)
```

- `provider` — registry key (`sqlite`, `mysql`, …)
- `connection` — builds a [Connection](./connection.md) from the options bag
- `repository` — builds a repository from a [Connection](./connection.md)

##### create(options)

Creates one datasource. `None` reads process env; `Some` is used as-is.

```rust
pub fn create(options: Option<&HashMap<String, String>>) -> Result<Arc<dyn Datasource>>
```

- `options` — [DatasourceOptions](./datasource.md#datasourceoptions); omitted uses `.env` then `DATASOURCE_URL` / `DATABASE_SCHEMA`

**Returns** `Result<Arc<dyn `[Datasource](./datasource.md#datasource)`>>`

##### default()

Loads `.env` `PROVIDER` / `DATASOURCE_URL`, then `DEFAULT_PROVIDER` / `DEFAULT_DATASOURCE_URL`. Throws when neither is set. Uses [create](#create-options).

```rust
pub fn default() -> Result<Arc<dyn Datasource>>
```

**Returns** `Result<Arc<dyn `[Datasource](./datasource.md#datasource)`>>`

##### create_default()

`default()`.

```rust
pub fn create_default() -> Result<Arc<dyn Datasource>>
```

**Returns** `Result<Arc<dyn `[Datasource](./datasource.md#datasource)`>>`

##### reset()

Clears the registry. Use in tests.

```rust
pub fn reset()
```

##### registered()

Lists registered provider names.

```rust
pub fn registered() -> Vec<String>
```

**Returns** `Vec<String>`
:::

## Remarks

Each language registers a connection factory and a repository factory, then `Default` / `default` / `DatasourceFactory.default` returns one [Datasource](./datasource.md). See [Datasource](./datasource.md) for the instance contract.

A missing factory throws `ProviderNotInstalledError` / `ProviderNotInstalledException` / `DatasourceError::ProviderNotInstalled`.

`Datasources.Hosting`, `@datasources/register`, and `datasources-register` register each name in `INSTALLED_PROVIDERS` (and the provider inferred from `DATASOURCE_URL`) when `Create` is called with null / `undefined` / `None` options.

## Examples

### Register a factory

::: dialects
```csharp
DatasourceFactory.Register("memory", opts => new MemoryConnection(opts), MemoryConnection.Repository);
var ds = DatasourceFactory.Create(new Dictionary<string, string?> { ["provider"] = "memory" });
```
```rust
register("memory", MemoryConnection::from_options, MemoryConnection::repository);
let ds = create(Some(&HashMap::from([("provider".into(), "memory".into())])))?;
```
```typescript
DatasourceFactory.register("memory", opts => new MemoryConnection(opts), MemoryConnection.repository);
const ds = Datasource.create({ provider: "memory" });
```
:::

### Default from environment

::: dialects
```csharp
var ds = DatasourceFactory.Default;
```
```rust
let ds = default()?;
```
```typescript
const ds = DatasourceFactory.default;
```
:::
