# Datasource

Factory for connections, commands, and repositories. `Create` reads `INSTALLED_PROVIDERS` and `DATABASE_URL` from the process environment when defaults are unset, registers each listed factory, then creates.

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
var ds = Datasource.Create();
```
```rust
let ds = create_default()?;
```
```typescript
const ds = Datasource.create();
```
:::

See [CLI](../cli.md#command-line-cli) for `setup` and adding providers.
