# DatasourceFactory

Provider plug-in. A factory has a provider name and creates a configured datasource.

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
#### IDatasourceFactory

```csharp
public interface IDatasourceFactory
```

**Properties**

##### Provider

Registry key used by `Datasource.Register`.

```csharp
string Provider { get; }
```

**Methods**

##### Create(options)

Creates a configured datasource.

```csharp
IDatasource Create(object? options = null)
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

Providers implement the factory and the datasource. Register a factory with `Datasource.register` / `Datasource.Register` / `register`, then `Create` / `create` looks it up by name. See [Datasource](./datasource.md) for the registry methods.

A missing factory throws `ProviderNotInstalledError` / `ProviderNotInstalledException` / `DatasourceError::ProviderNotInstalled`.

`Datasources.Hosting`, `@datasources/register`, and `datasources-register` register each name in `INSTALLED_PROVIDERS` (and the provider inferred from `DATABASE_URL`) when `Create` / `create` / `create_default` applies defaults.

## Examples

### Register a factory

::: dialects
```csharp
Datasource.Register(MemoryDatasource.Instance);
var ds = Datasource.Create("memory");
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
