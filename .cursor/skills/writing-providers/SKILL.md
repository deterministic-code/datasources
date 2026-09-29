---
name: writing-providers
description: >-
  Implement a datasources provider as IDriver / Driver and export only that
  type. Use when adding a provider, writing a Driver, or implementing
  connection, command, transaction, and row type converters.
---

# Writing Providers

C# is the reference. Rust and TypeScript match it.

Read `docs/writing-providers.md` for the interface. Implement these members:

- `connection` — build the connection from the options bag
- `command` — build a command for that connection; delegate to the connection's `createCommand`
- `transaction` — build a transaction; isolation defaults to serializable
- `rowTypeConverters` — canonical cell converters (`bool`, `datetime`, `uuid`, `decimal`, `binary`). Empty when cells are already canonical
- `columnType` — SQL type name for a canonical column type. Default returns the name. C# and Rust inherit that default. TypeScript assigns the exported `columnType`. MySQL, Postgres, and SQLite override it and throw on an unknown name. The statement factory maps statements to general ones, including `SelectInsertedIdStatement` to a `SelectStatement`, and the repository reads that row. SQLite, MySQL, and Postgres keep create-table rendering on their own `CreateTableStatementRenderer`. The factory does not subclass that renderer.

## Public surface

The package exports only the driver type.

- `mysql`, `postgres`, and `sqlite` name it `*Driver`. Every other provider names it `*Provider`.
- C#: that class is `public`. Connection, command, transaction, options, and converters are `internal`. `InternalsVisibleTo` opens them to the test project. `GetExportedTypes()` is exactly the driver. An engine assembly such as `Datasources.Sqlite.Microsoft` exports nothing.
- TypeScript: `src/index.ts` is one line, `export { Name } from "./Name.ts"`. `package.json` `exports` points only at that file.
- Rust: `lib.rs` has private `mod` items and one `pub use` of the driver. `pub` items inside those modules stay crate-private.

## Check

- C#: `PublicSurfaceTests.ExportsOnlyTheProvider` in the provider test project
- TypeScript: `exports only the provider` in `typescript/register`
- Rust: `providers_export_only_their_provider` in `rust/register`
