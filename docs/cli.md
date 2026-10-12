# Command Line (CLI)

`@datasources/cli` installs provider packages and writes a local `.env`. {{Datasource.Default|Datasource.default|default()}} reads `INSTALLED_PROVIDERS`, `PROVIDER`, and `DATASOURCE_URL`. Process env overrides the same keys in `.env`. Missing `PROVIDER` throws.

```sh
npx @datasources/cli setup
npx @datasources/cli add sqlite
npx @datasources/cli remove sqlite
npx @datasources/cli generate all --dialect typescript
```

Pass `--ecosystem ts|rust|csharp` when the directory has more than one project type. From this repo: `npm run setup` or `npm run cli -- add sqlite`.

## Setup

Interactive `.env` and provider install. A confirm writes `deterministic/types.yaml` (a starter `item` type) and an empty `deterministic/datasource.yaml` when that directory is missing. `--yes` skips prompts and uses defaults / existing `.env`; it does not scaffold the spec. Setup also writes the generated-registry hook for the detected ecosystem (see Generate).

```sh
npx @datasources/cli setup
npx @datasources/cli setup --yes
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
npx @datasources/cli add sqlite postgres
```

| Flag | Meaning |
| --- | --- |
| `--cwd <dir>` | Consumer project directory (default: `.`) |
| `--ecosystem ts\|rust\|csharp` | Required when more than one project type exists |
| `--dry-run` | Print package-manager commands without running them |

## Remove

Remove provider packages and drop them from `.env`.

```sh
npx @datasources/cli remove postgres
```

| Flag | Meaning |
| --- | --- |
| `--cwd <dir>` | Consumer project directory (default: `.`) |
| `--ecosystem ts\|rust\|csharp` | Required when more than one project type exists |
| `--dry-run` | Print package-manager commands without running them |

## Generate

`generate` reads a deterministic project and writes one language dialect. The word after `generate` is `all`, `types`, `repositories`, or `datasource`. `--dialect` is `csharp`, `typescript`, or `rust`. SQL dialects come from `settings.yaml` (`backend.datasources`, default `sqlite`).

```sh
npx @datasources/cli generate all --dialect typescript --input ./deterministic --output ./generated
npx @datasources/cli generate repositories --dialect csharp
```

| Flag | Meaning |
| --- | --- |
| `--dialect csharp\|typescript\|rust` | Language of the generated types and repositories |
| `--provider <name>` | Spec provider (default: `deterministic`) |
| `--input <dir>` | Deterministic project (default: `./deterministic`) |
| `--output <dir>` | Generated files (default: `./generated`) |

When `datasource.yaml` is omitted, generate includes every `datasource_type` and `view_type`. Table and column names use the default mapping: a plural snake table and snake columns. An explicit `datasource.yaml` still wins.

`types` writes datasource and view types under `<output>/<dialect>/`. `repositories` writes one repository class per datasource table under `<output>/<dialect>/repositories/`. `datasource` writes SQL under `<output>/sql/`. `all` writes each of those.

`setup` and `generate` (for `all` or `repositories`) write a project hook so {{Datasource.Default|Datasource.default|default()}} applies the generated registry: `src/datasources-register.ts` or `src/datasources_register.rs`, plus one attach line on the detected entry (`src/index.ts` / `src/lib.rs` / `src/main.rs`). Rust also adds `ctor = "0.4"` to `Cargo.toml` so `#[ctor::ctor]` can queue `RepositoryFactory::bind` at load. C# compiles `RepositoryRegistry` when it sits in the project; generate adds a csproj `Compile` include only when `--output` is outside the project.

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
