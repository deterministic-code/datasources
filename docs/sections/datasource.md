# Datasource

Factory for connections, commands, and repositories. Register providers, then create by name. The library does not auto-register providers.

## IDatasource

| C# | Rust | TypeScript |
| --- | --- | --- |
| `IDatasource` | `Datasource` trait | `IDatasource` |
| `Provider` | `provider()` | `provider` |
| `CreateConnection` | `create_connection` | `createConnection` |
| `CreateCommand` | `create_command` | `createCommand` |
| `CreateRepository<T>` | `create_repository` | `createRepository` |
| `CreateReadonlyRepository<T>` | `create_readonly_repository` | `createReadonlyRepository` |

## Registry

| C# | Rust | TypeScript |
| --- | --- | --- |
| `Datasource` | `datasources` | `Datasource` |
| `Register` | `register` | `register` |
| `DatasourceInstall.From` (Hosting) | `register_from` (`datasources-register`) | `install` (`@datasources/register`) |
| `Create` | `create` / `create_default` | `create` |
| `Defaults` | `defaults` | `defaults` |
| `Registered` | `registered` | `registered` |
| `Reset` | `reset` | `reset` |

::: dialects
```csharp
Datasource.Register(SqliteDatasource.Instance);
var ds = Datasource.Create("sqlite");
```
```rust
register(SqliteDatasource::instance());
let ds = create("sqlite", None)?;
```
```typescript
Datasource.register(SqliteDatasource.instance);
const ds = Datasource.create("sqlite");
```
:::

See [QuickStart](../quickstart.md) for full registration and default-provider setup.
