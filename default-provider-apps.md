# Default provider and connection string in apps

Datasources does not read config files or `.env`. The app owns defaults, then calls `create` / `createConnection`. Connections are constructed **closed**; open on first use.

| Value | Where |
| --- | --- |
| Provider (`postgres`, `sqlite`, `memory`) | Committed config or `DATASOURCE_PROVIDER` |
| Non-secrets (sqlite `filename`, json path, `schema`) | Committed config |
| Connection URL | Environment; locally a gitignored `.env` |
| Drivers / file stores | Memory (constructed in code) |

Do not commit URLs with credentials. Do not parse `.env` or `appsettings` inside `@datasources/core` / `Datasources` / `datasources`.

`.env` is for local and CI secrets only. Commit [`.env.example`](.env.example). Production uses the real environment or a secrets manager. This repo’s e2e tests already use `MYSQL_URL` and `POSTGRES_URL`.

## App factory (options dictionary)

Call `Datasource.defaults` / `Datasource.Defaults` / `defaults` with one dictionary. The library does not set this itself. `create()`, `createConnection`, `createCommand`, and `createRepository` use those values and reuse one closed connection.

See:

- [TypeScript](create-factory-typescript.md#default-provider-in-apps)
- [C#](create-factory-csharp.md#default-provider-in-apps)
- [Rust](create-factory-rust.md#default-provider-in-apps)

## Startup

Register factories, create the default datasource, then `createConnection`. Do not `open` during bootstrap. Hold both in DI or a module binding. On first command or repository call: `if (!connection.opened) await connection.open()`.

See [TypeScript](create-connection-typescript.md#startup), [C#](create-connection-csharp.md#startup), [Rust](create-connection-rust.md#startup).
