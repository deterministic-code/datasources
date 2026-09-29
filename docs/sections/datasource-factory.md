# DatasourceFactory

Creates a [Datasource](./datasource.md) for a provider and `DATASOURCE_URL`. After [setup](../cli.md#setup), [Datasource.Default](./datasource.md) builds that datasource from `.env`.

## Classes

::: reference
### typescript
#### DatasourceFactory

```typescript
class DatasourceFactory implements IDatasourceFactory
```

**Methods**

##### defaults(options)

Registers listed provider packages. Does not apply to a non-null `create` bag.

```typescript
static defaults(options: DatasourceOptions): void
```

- `options` — install keys such as `INSTALLED_PROVIDERS`; not [Create options](./datasource.md#datasourceoptions)

##### register(provider, driver)

Registers a provider [IDriver](../writing-providers.md#idriver) by name.

```typescript
static register(provider: string, driver: IDriver): void
```

- `provider` — registry key (`sqlite`, `mysql`, …)
- `driver` — [IDriver](../writing-providers.md#idriver) that builds connections, commands, transactions, and row converters

##### create(options?)

Creates one datasource. `undefined` options read process env; a dictionary is used as-is.

```typescript
static create(options?: DatasourceOptions): IDatasource
```

- `options` — [DatasourceOptions](./datasource.md#datasourceoptions); omitted loads `.env`, then process `INSTALLED_PROVIDERS`, `PROVIDER`, and `DATASOURCE_URL`

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

**Methods**

##### Defaults(options)

Registers listed provider packages. Does not apply to a non-null `Create` bag.

```csharp
public static void Defaults(IReadOnlyDictionary<string, string?> options)
```

- `options` — install keys such as `INSTALLED_PROVIDERS`; not [Create options](./datasource.md#datasourceoptions)

##### Register(provider, driver)

Registers a provider [IDriver](../writing-providers.md#idriver) by name.

```csharp
public static void Register(string provider, IDriver driver)
```

- `provider` — registry key (`sqlite`, `mysql`, …)
- `driver` — [IDriver](../writing-providers.md#idriver) that builds connections, commands, transactions, and row converters

##### Create(options)

Creates one datasource. `null` options read process `.env`; a dictionary is used as-is.

```csharp
public static IDatasource Create(IReadOnlyDictionary<string, string?>? options = null)
```

- `options` — [DatasourceOptions](./datasource.md#datasourceoptions); omitted loads `.env`, then process `INSTALLED_PROVIDERS`, `PROVIDER`, and `DATASOURCE_URL`

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

##### register(provider, driver)

Registers a provider [Driver](../writing-providers.md#driver) by name.

```rust
pub fn register(provider: impl Into<String>, driver: Arc<dyn Driver>)
```

- `provider` — registry key (`sqlite`, `mysql`, …)
- `driver` — [Driver](../writing-providers.md#driver) that builds connections, commands, transactions, and row converters

##### create(options)

Creates one datasource. `None` reads process env; `Some` is used as-is.

```rust
pub fn create(options: Option<&HashMap<String, String>>) -> Result<Arc<dyn Datasource>>
```

- `options` — [DatasourceOptions](./datasource.md#datasourceoptions); omitted loads `.env`, then process `INSTALLED_PROVIDERS`, `PROVIDER`, and `DATASOURCE_URL`

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

{{Create(null)|create()|create(None)}} reads `.env`, then process `INSTALLED_PROVIDERS`, `PROVIDER`, and `DATASOURCE_URL`. Missing `PROVIDER` throws {{DatasourceException|DatasourceError|DatasourceError}}. [Datasource.Default](./datasource.md) uses that path.

A missing factory throws {{ProviderNotInstalledException|ProviderNotInstalledError|DatasourceError::ProviderNotInstalled}}.

When {{Create|create|create}} is called with {{null|undefined|None}} options, each name in `INSTALLED_PROVIDERS` and the provider inferred from `DATASOURCE_URL` is registered. Every dialect loads those with `DriverLoader` and `ProviderCatalog`. Each dialect implements [IDriver](../writing-providers.md#idriver) (`MemoryDriver`, `SqliteDriver`, `MysqlDriver`, `PostgresDriver`, …) and that is what {{Register|register|register}} stores.

## Examples

### Register and Create

::: example
```csharp csharp/Providers/Memory.Tests/RegisterAndCreateTests.cs#RegisterAndCreate
```
```rust rust/providers/memory/src/lib.rs#register_and_create
```
```typescript typescript/providers/memory/src/index.test.ts#register-and-create
```
:::

### Missing provider

::: example
```csharp csharp/Datasources.Tests/DatasourceFactoryTests.cs#MissingProviderThrows
```
```rust rust/datasources/src/tests/datasource_factory.rs#missing_provider_throws
```
```typescript typescript/datasources/src/DatasourceFactory.test.ts#missing-provider-throws
```
:::
