<!-- Generated from docs/. Edit the markdown in docs/ and run npm run docs:skill. -->

# Command Line (CLI)

`@deterministic-code/datasources-cli` installs provider packages and writes a local `.env`. `Datasource.Default` / `Datasource.default` / `default()` reads `INSTALLED_PROVIDERS`, `PROVIDER`, and `DATASOURCE_URL`. Process env overrides the same keys in `.env`. Missing `PROVIDER` throws.

```sh
npx @deterministic-code/datasources-cli setup
npx @deterministic-code/datasources-cli add sqlite
npx @deterministic-code/datasources-cli remove sqlite
npx @deterministic-code/datasources-cli generate all --dialect typescript
npx @deterministic-code/datasources-cli migrations create
```

Pass `--ecosystem ts|rust|csharp` when the directory has more than one project type. From this repo: `npm run setup` or `npm run cli -- add sqlite`.

## Setup

Interactive `.env` and provider install. A confirm writes `deterministic/types.yaml` (a starter `item` type) and an empty `deterministic/datasource.yaml` when that directory is missing. `--yes` skips prompts and uses defaults / existing `.env`; it does not scaffold the spec. Setup also writes the generated-registry hook for the detected ecosystem (see Generate). Checked SQL dialects (`sqlite`, `postgres`, `mysql`, `sqlserver`, `oracle`) install `@deterministic-code/migraters` and run `migrate-init` for that set, for every ecosystem. `add` and `remove` re-run init with the remaining SQL dialects.

```sh
npx @deterministic-code/datasources-cli setup
npx @deterministic-code/datasources-cli setup --yes
```

| Flag | Meaning |
| --- | --- |
| `--cwd <dir>` | Consumer project directory (default: `.`) |
| `--ecosystem ts\|rust\|csharp` | Required when more than one project type exists |
| `--dry-run` | Print package-manager commands without running them |
| `--yes` | Setup without prompts |

## Add

Add provider packages. Updates `.env` `INSTALLED_PROVIDERS` when a `.env` already exists.

```sh
npx @deterministic-code/datasources-cli add sqlite postgres
```

| Flag | Meaning |
| --- | --- |
| `--cwd <dir>` | Consumer project directory (default: `.`) |
| `--ecosystem ts\|rust\|csharp` | Required when more than one project type exists |
| `--dry-run` | Print package-manager commands without running them |

## Remove

Remove provider packages and drop them from `.env`.

```sh
npx @deterministic-code/datasources-cli remove postgres
```

| Flag | Meaning |
| --- | --- |
| `--cwd <dir>` | Consumer project directory (default: `.`) |
| `--ecosystem ts\|rust\|csharp` | Required when more than one project type exists |
| `--dry-run` | Print package-manager commands without running them |

## .env

`setup` writes these keys. `add` / `remove` update the provider list. Other keys already in the file are kept. Process env overrides the same three keys. `Create` / `create` / `create` with omitted options registers each name in `INSTALLED_PROVIDERS` and the provider inferred from `DATASOURCE_URL`. The datasource itself is `PROVIDER`.

| Key | Default | Meaning |
| --- | --- | --- |
| `INSTALLED_PROVIDERS` | scheme of `DATASOURCE_URL` | Comma-separated providers `Create` registers. |
| `PROVIDER` | | Datasource `Create` returns. |
| `DATASOURCE_URL` | `sqlite:///:memory:` | Connection URL. Schema is the `schema` query. |

```sh
INSTALLED_PROVIDERS=sqlite,postgres
PROVIDER=postgres
DATASOURCE_URL=postgres://localhost/db?schema=public
```

SQLAlchemy-style SQLite URLs: `sqlite:///:memory:`, `sqlite:///app.db`, `sqlite:////tmp/app.db`.
