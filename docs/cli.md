# Command Line (CLI)

`@deterministic-code/datasources-cli` installs provider packages and writes a local `.env`. {{Datasource.Default|Datasource.default|default()}} reads `INSTALLED_PROVIDERS`, `PROVIDER`, and `DATASOURCE_URL`. Process env overrides the same keys in `.env`. Missing `PROVIDER` throws.

```sh
npx @deterministic-code/datasources-cli setup
npx @deterministic-code/datasources-cli add sqlite
npx @deterministic-code/datasources-cli remove sqlite
npx @deterministic-code/datasources-cli generate all --dialect typescript
npx @deterministic-code/datasources-cli migrations create
```

Pass `--ecosystem ts|rust|csharp` when the directory has more than one project type. From this repo: `npm run setup` or `npm run cli -- add sqlite`.

## Setup

Interactive `.env` and provider install. Names after `setup` are the dialects to install for `--ecosystem`. A confirm writes `deterministic/types.yaml` (a starter `item` type) and an empty `deterministic/datasource.yaml` when that directory is missing. `--yes` skips prompts and uses those names, or defaults / existing `.env` when none are given; it does not scaffold the spec. Setup also writes the generated-registry hook for the detected ecosystem (see Generate). Checked SQL dialects (`sqlite`, `postgres`, `mysql`, `sqlserver`, `oracle`) install `@deterministic-code/migraters` and run `migrate-init --language <typescript|csharp|rust>` with those dialects. `ts` maps to `typescript`. `add` and `remove` re-run init with the same language and the remaining SQL dialects.

```sh
npx @deterministic-code/datasources-cli setup
npx @deterministic-code/datasources-cli setup --yes
npx @deterministic-code/datasources-cli setup --yes --ecosystem csharp sqlite postgres
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

## Generate

`generate` reads a deterministic project and writes one language dialect. The word after `generate` is `all`, `types`, `repositories`, or `datasource`. `--dialect` is `csharp`, `typescript`, or `rust`. SQL dialects come from `settings.yaml` (`backend.datasources`, default `sqlite`).

```sh
npx @deterministic-code/datasources-cli generate all --dialect typescript --input ./deterministic --output ./generated
npx @deterministic-code/datasources-cli generate repositories --dialect csharp
```

| Flag | Meaning |
| --- | --- |
| `--dialect csharp\|typescript\|rust` | Language of the generated types and repositories |
| `--provider <name>` | Spec provider (default: `deterministic`) |
| `--input <dir>` | Deterministic project (default: `./deterministic`) |
| `--output <dir>` | Generated files (default: `./generated`) |
| `--no-migrations` | Skip SQL migrations. `generate all` writes them by default |
| `--no-auto-migrate-up` | Skip migrate-up. `generate all` applies them by default |
| `--migrations-root <dir>` | Writes `<root>/<dialect>/migrations/` on `generate all` (default: `sql`) |
| `--stored-procedures` | Emit stored-procedure migrations when the dialect can. `settings.yaml` `datasource.use_stored_procedures: true` has the same effect |

`generate all` writes SQL migrations into the [migraters](https://github.com/deterministic-code/migraters) layout and runs migrate-up. The connection is `PROVIDER` and `DATASOURCE_URL` from `.env`. Process env overrides the file. An in-memory `DATASOURCE_URL` skips migrate-up. `--dry-run` prints the migrate-up command and leaves the migration files unwritten. `--stored-procedures` and `settings.yaml` `datasource.use_stored_procedures: true` both turn on stored-procedure migrations.

When `datasource.yaml` is omitted, generate includes every `datasource_type` and `view_type`. Table and column names use the default mapping: a plural snake table and snake columns. An explicit `datasource.yaml` still wins.

`types` writes datasource and view types under `<output>/<dialect>/`. `repositories` writes one repository class per datasource table under `<output>/<dialect>/repositories/`. `datasource` writes SQL under `<output>/sql/`. `all` writes each of those.

`setup` and `generate` (for `all` or `repositories`) write a project hook so {{Datasource.Default|Datasource.default|default()}} applies the generated registry: `src/datasources-register.ts` or `src/datasources_register.rs`, plus one attach line on the detected entry (`src/index.ts` / `src/lib.rs` / `src/main.rs`). Rust also adds `ctor = "0.4"` to `Cargo.toml` so `#[ctor::ctor]` can queue `RepositoryFactory::bind` at load. C# compiles `RepositoryRegistry` when it sits in the project; generate adds a csproj `Compile` include only when `--output` is outside the project.

## Migrations

`generate all` writes these files and runs migrate-up. `migrations create` writes the same SQL and leaves the database unchanged. Dialects come from `settings.yaml` (`backend.datasources`, default `sqlite`). Without `version-*/diff.yaml`, the files are `0001_initial_up.sql` and `0001_initial_down.sql`. Version snapshots write `0001_up.sql`, `0002_up.sql`, and so on.

A pair under `deterministic/custom/<dialect>/` named `<order>_<name>_<up|down>.sql` is copied into the migrations directory. `order` is 1–999. `npx --yes --package @deterministic-code/migraters migrate-create --provider sqlite --name add_notes` scaffolds an empty pair in that directory.

```sh
npx @deterministic-code/datasources-cli migrations create
npx @deterministic-code/datasources-cli migrations create --input ./deterministic --migrations-root sql
```

| Flag | Meaning |
| --- | --- |
| `--provider <name>` | Spec provider (default: `deterministic`) |
| `--input <dir>` | Deterministic project (default: `./deterministic`) |
| `--migrations-root <dir>` | Writes `<root>/<dialect>/migrations/` (default: `sql`) |
| `--stored-procedures` | Emit stored-procedure migrations when the dialect can. `settings.yaml` `datasource.use_stored_procedures: true` has the same effect |

Re-running overwrites the same files.

Apply and roll back with migraters. `setup` / `add` installs `@deterministic-code/migraters` and runs `migrate-init --language` for the ecosystem and its SQL dialects. `--migrations-root` defaults to `sql`, matching `migrations create`. Omit `--connection` when the dialect env var is set (`SQLITE_PATH` for sqlite). `migrate-down` rolls back one migration and requires the token it prints, passed as `--confirm`.

```sh
npx --yes --package @deterministic-code/migraters migrate-up --provider sqlite --connection ./app.sqlite
npx --yes --package @deterministic-code/migraters migrate-down --provider sqlite --connection ./app.sqlite --confirm TOKEN
```

## .env

`setup` writes these keys. `add` / `remove` update the provider list. Other keys already in the file are kept. Process env overrides the same three keys. {{Create|create|create}} with omitted options registers each name in `INSTALLED_PROVIDERS` and the provider inferred from `DATASOURCE_URL`. The datasource itself is `PROVIDER`.

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
