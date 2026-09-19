# Command Line (CLI)

`@datasources/cli` installs provider packages and writes a local `.env`. `Default` / `default` / `DatasourceFactory.default` reads that file for `PROVIDER` and `DATASOURCE_URL`. `DEFAULT_PROVIDER` and `DEFAULT_DATASOURCE_URL` in the process override `.env`. If neither source sets a provider or URL, create throws.

```sh
npx @datasources/cli setup
npx @datasources/cli add sqlite
npx @datasources/cli remove sqlite
```

Pass `--ecosystem ts|rust|csharp` when the directory has more than one project type. From this repo: `npm run setup` or `npm run cli -- add sqlite`.

## Setup

Interactive `.env` and provider install. `--yes` skips prompts and uses defaults / existing `.env`.

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

## .env

`setup` writes these keys. `add` / `remove` update the provider list. Other keys already in the file are kept. `Default` uses the `DATASOURCE_URL` scheme as the default provider (`sqlite://`, `postgres://`, `mysql://`, …) unless `PROVIDER` or `DEFAULT_PROVIDER` is set.

| Key | Default | Meaning |
| --- | --- | --- |
| `INSTALLED_PROVIDERS` | scheme of `DATASOURCE_URL` | Comma-separated providers `Create` registers. |
| `DATASOURCE_URL` | `sqlite:///:memory:` | Connection URL. The scheme is the default provider. |
| `DATABASE_SCHEMA` | *(empty)* | Schema / search path (postgres). |

```sh
INSTALLED_PROVIDERS=sqlite,postgres
DATASOURCE_URL=sqlite:///:memory:
DATABASE_SCHEMA=public
```

Process overrides (not written by `setup`):

| Key | Meaning |
| --- | --- |
| `DEFAULT_PROVIDER` | Overrides `.env` `PROVIDER` / inferred scheme. |
| `DEFAULT_DATASOURCE_URL` | Overrides `.env` `DATASOURCE_URL`. |

SQLAlchemy-style SQLite URLs: `sqlite:///:memory:`, `sqlite:///app.db`, `sqlite:////tmp/app.db`.
