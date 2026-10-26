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

Pass `--languages ts|rust|csharp` when the directory has more than one project type, or to install more than one language in one call. From this repo: `npm run setup` or `npm run cli -- add sqlite`.

## Setup

Interactive `.env` and datasource install. `--datasources` names the engines to install for `--languages`. A confirm writes `deterministic/types.yaml` (a starter `item` type) and an empty `deterministic/datasource.yaml` when that directory is missing. `--yes` skips prompts and uses those names, or defaults / existing `.env` when none are given; it does not scaffold the spec. When a requested language has no project (`package.json`, `Cargo.toml`, or `.csproj`/`.sln`), setup, add, and remove stop with a message and do not install packages. Setup also writes the generated-registry hook for each language (see Generate). Checked SQL dialects (`sqlite`, `postgres`, `mysql`, `sqlserver`, `oracle`) install `@deterministic-code/migraters` and run `migrate-init --language <typescript|csharp|rust>` with those dialects. `ts` maps to `typescript`. `add` and `remove` re-run init with the same languages and the remaining SQL dialects.

```sh
npx @deterministic-code/datasources-cli setup
npx @deterministic-code/datasources-cli setup --yes
npx @deterministic-code/datasources-cli setup --yes --languages csharp rust --datasources sqlite postgres
```

| Flag | Meaning |
| --- | --- |
| `--cwd <dir>` | Consumer project directory (default: `.`) |
| `--languages ts\|rust\|csharp` | Project languages (default: every type found) |
| `--datasources <name...>` | Datasources to install (default: existing `.env`) |
| `--dry-run` | Print package-manager commands without running them |
| `--yes` | Setup without prompts |

## Add

Add datasource packages. Updates `.env` `INSTALLED_PROVIDERS` when a `.env` already exists.

```sh
npx @deterministic-code/datasources-cli add sqlite postgres
```

| Flag | Meaning |
| --- | --- |
| `--cwd <dir>` | Consumer project directory (default: `.`) |
| `--languages ts\|rust\|csharp` | Project languages (default: every type found) |
| `--datasources <name...>` | Datasources to add |
| `--dry-run` | Print package-manager commands without running them |

## Remove

Remove datasource packages and drop them from `.env`.

```sh
npx @deterministic-code/datasources-cli remove postgres
```

| Flag | Meaning |
| --- | --- |
| `--cwd <dir>` | Consumer project directory (default: `.`) |
| `--languages ts\|rust\|csharp` | Project languages (default: every type found) |
| `--datasources <name...>` | Datasources to remove |
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
