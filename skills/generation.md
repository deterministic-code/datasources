<!-- Generated from docs/. Edit the markdown in docs/ and run npm run docs:skill. -->

# Repository generation

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
| `--no-migrate` | Skip migrate-up. `generate all` applies them by default |
| `--migrations-root <dir>` | Writes `<root>/<dialect>/migrations/` on `generate all` (default: `sql`) |
| `--migrations-path <dir>` | One migrations directory on `generate all`. The spec must list a single SQL dialect |
| `--stored-procedures` | Emit stored-procedure migrations when the dialect can. `settings.yaml` `datasource.use_stored_procedures: true` has the same effect |

`generate all` writes SQL migrations into the [migraters](https://github.com/deterministic-code/migraters) layout and runs migrate-up. The connection is `PROVIDER` and `DATASOURCE_URL` from `.env`. Process env overrides the file. An in-memory `DATASOURCE_URL` skips migrate-up. `--dry-run` prints the migrate-up command and leaves the migration files unwritten. `--migrations-path` wins over `--migrations-root`. `--stored-procedures` and `settings.yaml` `datasource.use_stored_procedures: true` both turn on stored-procedure migrations.

When `datasource.yaml` is omitted, generate includes every `datasource_type` and `view_type`. Table and column names use the default mapping: a plural snake table and snake columns. An explicit `datasource.yaml` still wins.

`types` writes datasource and view types under `<output>/<dialect>/`. `repositories` writes one repository class per datasource table under `<output>/<dialect>/repositories/`. `datasource` writes SQL under `<output>/sql/`. `all` writes each of those.

`setup` and `generate` (for `all` or `repositories`) write a project hook so `Datasource.Default` / `Datasource.default` / `default()` applies the generated registry: `src/datasources-register.ts` or `src/datasources_register.rs`, plus one attach line on the detected entry (`src/index.ts` / `src/lib.rs` / `src/main.rs`). Rust also adds `ctor = "0.4"` to `Cargo.toml` so `#[ctor::ctor]` can queue `RepositoryFactory::bind` at load. C# compiles `RepositoryRegistry` when it sits in the project; generate adds a csproj `Compile` include only when `--output` is outside the project.

## Migrations

`generate all` writes these files and runs migrate-up. `migrations create` writes the same SQL and leaves the database unchanged. Dialects come from `settings.yaml` (`backend.datasources`, default `sqlite`). Without `version-*/diff.yaml`, the files are `0001_initial_up.sql` and `0001_initial_down.sql`. Version snapshots write `0001_up.sql`, `0002_up.sql`, and so on.

A pair under `deterministic/custom/<dialect>/` named `<order>_<name>_<up|down>.sql` is copied into the migrations directory. `order` is 1–999. `npx --yes --package @deterministic-code/migraters migrate-create --provider sqlite --name add_notes` scaffolds an empty pair in that directory.

```sh
npx @deterministic-code/datasources-cli migrations create
npx @deterministic-code/datasources-cli migrations create --input ./deterministic --migrations-root sql
npx @deterministic-code/datasources-cli migrations create --migrations-path ./db/sqlite/migrations
```

| Flag | Meaning |
| --- | --- |
| `--provider <name>` | Spec provider (default: `deterministic`) |
| `--input <dir>` | Deterministic project (default: `./deterministic`) |
| `--migrations-root <dir>` | Writes `<root>/<dialect>/migrations/` (default: `sql`) |
| `--migrations-path <dir>` | One migrations directory. The spec must list a single SQL dialect. |
| `--stored-procedures` | Emit stored-procedure migrations when the dialect can. `settings.yaml` `datasource.use_stored_procedures: true` has the same effect |

`--migrations-path` wins over `--migrations-root`. Re-running overwrites the same files.

Apply and roll back with migraters. `setup` / `add` installs `@deterministic-code/migraters` and runs `migrate-init --language` for the ecosystem and its SQL dialects. `--migrations-root` defaults to `sql`, matching `migrations create`. Omit `--connection` when the dialect env var is set (`SQLITE_PATH` for sqlite). `migrate-down` rolls back one migration and requires the token it prints, passed as `--confirm`.

```sh
npx --yes --package @deterministic-code/migraters migrate-up --provider sqlite --connection ./app.sqlite
npx --yes --package @deterministic-code/migraters migrate-down --provider sqlite --connection ./app.sqlite --confirm TOKEN
```

## Repository Generation

`generate all` reads a Types Specification and writes the types, the repositories, and SQL for one language. `setup` and `generate` wire the generated registry so `Datasource.Default` / `Datasource.default` / `default()` binds each plural name.

See [Generate](#generate) for flags and output paths.

### User

A Types Specification is the `deterministic` directory. `types.yaml` lists each entity. Tag a type `datasource_type` when it is a table.

```yaml
version: 1.0.0
types:
  - user:
      tags:
        - datasource_type
      fields:
        - id:
            type: integer
            is_id: true
        - username:
            type: string
            size: 128
        - is_enabled:
            type: boolean
```

`generate all` writes the `User` type, the `Users` / `users` / `users()` repository, and SQL.

##### C#

```csharp
npx @deterministic-code/datasources-cli generate all --dialect csharp
```

##### Rust

```rust
npx @deterministic-code/datasources-cli generate all --dialect rust
```

##### TypeScript

```typescript
npx @deterministic-code/datasources-cli generate all --dialect typescript
```

`generate all` writes the SQL migrations and runs migrate-up against `DATASOURCE_URL`. `--no-migrations` skips the files. `--no-migrate` skips applying them. See [Migrations](#migrations) to write, apply, or roll back a migration on its own.

`Create` / `create` / `create` returns that entity. `ReadSingle` / `readSingle` / `read_single` returns the same entity. `Update` and `Delete` return `Response`.

##### C#

```csharp
using Backend.Types.Datasource;

dynamic ds = Datasource.Default;

User created = await ds.Users.Create(new User { Username = "ada", IsEnabled = true });
User? loaded = await ds.Users.ReadSingle();
created.Username = "ada-v2";
Response updated = await ds.Users.Update(created);
Response deleted = await ds.Users.Delete(created);
```

##### Rust

```rust
use datasources::{default, Response, Row, Value};

let ds = default()?;
let users = ds.as_ref().users();
let mut created = Row::new();
created.insert("username".into(), Value::from("ada"));
created.insert("is_enabled".into(), Value::from(true));
let mut created = users.create(&mut created).await?;
let loaded = users.read_single(None, None, None).await?;
created.insert("username".into(), Value::from("ada-v2"));
let updated: Response = users.update(&created, None).await?;
let deleted: Response = users.delete(&created, None).await?;
```

##### TypeScript

```typescript
import { Datasource } from "@deterministic-code/datasources";

const ds = Datasource.default;

const created = await ds.users.create({ id: 0, username: "ada", isEnabled: true });
const loaded = await ds.users.readSingle();
created.username = "ada-v2";
const updated = await ds.users.update(created);
const deleted = await ds.users.delete(created);
```

### Users and roles

The same `user` table, plus `role` and a `user_role` junction tagged `many_to_many`. A `roles` list on `user` that references `user_role.user_id` generates a linked enrichment: `AddLinked` / `addLinked` / `add_linked`.

```yaml
version: 1.0.0
types:
  - user:
      tags:
        - datasource_type
      fields:
        - id:
            type: integer
            is_id: true
        - username:
            type: string
            size: 128
        - is_enabled:
            type: boolean
        - roles:
            type: role[]
            references: user_role.user_id
  - role:
      tags:
        - datasource_type
      fields:
        - id:
            type: integer
            is_id: true
        - name:
            type: string
            size: 64
  - user_role:
      tags:
        - datasource_type
        - many_to_many
      fields:
        - user_id:
            references: user.id
        - role_id:
            references: role.id
```

##### C#

```csharp
npx @deterministic-code/datasources-cli generate all --dialect csharp
```

##### Rust

```rust
npx @deterministic-code/datasources-cli generate all --dialect rust
```

##### TypeScript

```typescript
npx @deterministic-code/datasources-cli generate all --dialect typescript
```

`generate all` writes the SQL migrations and runs migrate-up against `DATASOURCE_URL`. `--no-migrations` skips the files. `--no-migrate` skips applying them. See [Migrations](#migrations) to write, apply, or roll back a migration on its own.

Create a role, then a user that lists that role. `Create` / `create` / `create` writes the junction row. `ReadSingle` / `readSingle` / `read_single` fills `Roles` / `roles` / `roles`. Delete removes the user and the junction; the role row stays.

##### C#

```csharp
using Backend.Types.Datasource;

dynamic ds = Datasource.Default;

Role admin = await ds.Roles.Create(new Role { Name = "admin" });
User created = await ds.Users.Create(new User
{
    Username = "ada",
    IsEnabled = true,
    Roles = [admin],
});
User? loaded = await ds.Users.ReadSingle();
Response deleted = await ds.Users.Delete(created);
```

##### Rust

```rust
use datasources::{default, encode_rows, Response, Row, Value};

let ds = default()?;
let roles = ds.as_ref().roles();
let mut admin = Row::new();
admin.insert("name".into(), Value::from("admin"));
let admin = roles.create(&mut admin).await?;
let users = ds.as_ref().users();
let mut created = Row::new();
created.insert("username".into(), Value::from("ada"));
created.insert("is_enabled".into(), Value::from(true));
created.insert("roles".into(), encode_rows(&[admin]));
let created = users.create(&mut created).await?;
let loaded = users.read_single(None, None, None).await?;
let deleted: Response = users.delete(&created, None).await?;
```

##### TypeScript

```typescript
import { Datasource } from "@deterministic-code/datasources";

const ds = Datasource.default;

const admin = await ds.roles.create({ id: 0, name: "admin" });
const created = await ds.users.create({
  id: 0,
  username: "ada",
  isEnabled: true,
  roles: [admin],
});
const loaded = await ds.users.readSingle();
const deleted = await ds.users.delete(created);
```

### Legacy tables

When `datasource.yaml` is omitted, generate maps a plural snake table and snake columns. An explicit `datasource.yaml` still wins. `mapping` on a type is the stored table. `mapping` on a field is the stored column. `is_unique` writes a unique index.

The type stays `user`. The stored table is `OldUsersTbl`. `Username` is unique and stored as `LoginNm`.

```yaml
version: 1.0.0
types:
  - user:
      tags:
        - datasource_type
      fields:
        - id:
            type: integer
            is_id: true
        - username:
            type: string
            size: 128
        - is_enabled:
            type: boolean
```

```yaml
version: 1.0.0
includes:
  - types:
      filter: tag == "datasource_type"
types:
  - user:
      mapping: OldUsersTbl
      fields:
        - id:
            mapping: UsrID
        - username:
            is_unique: true
            mapping: LoginNm
        - is_enabled:
            mapping: ActvFlg
```

##### C#

```csharp
npx @deterministic-code/datasources-cli generate all --dialect csharp
```

##### Rust

```rust
npx @deterministic-code/datasources-cli generate all --dialect rust
```

##### TypeScript

```typescript
npx @deterministic-code/datasources-cli generate all --dialect typescript
```

`generate all` writes the SQL migrations and runs migrate-up against `DATASOURCE_URL`. `--no-migrations` skips the files. `--no-migrate` skips applying them. See [Migrations](#migrations) to write, apply, or roll back a migration on its own.

The generated repository maps those stored names. The CRUD call still uses `Username` / `username` / `username`.

##### C#

```csharp
using Backend.Types.Datasource;

dynamic ds = Datasource.Default;

User created = await ds.Users.Create(new User { Username = "ada", IsEnabled = true });
User? loaded = await ds.Users.ReadSingle();
created.Username = "ada-v2";
Response updated = await ds.Users.Update(created);
Response deleted = await ds.Users.Delete(created);
```

##### Rust

```rust
use datasources::{default, Response, Row, Value};

let ds = default()?;
let users = ds.as_ref().users();
let mut created = Row::new();
created.insert("username".into(), Value::from("ada"));
created.insert("is_enabled".into(), Value::from(true));
let mut created = users.create(&mut created).await?;
let loaded = users.read_single(None, None, None).await?;
created.insert("username".into(), Value::from("ada-v2"));
let updated: Response = users.update(&created, None).await?;
let deleted: Response = users.delete(&created, None).await?;
```

##### TypeScript

```typescript
import { Datasource } from "@deterministic-code/datasources";

const ds = Datasource.default;

const created = await ds.users.create({ id: 0, username: "ada", isEnabled: true });
const loaded = await ds.users.readSingle();
created.username = "ada-v2";
const updated = await ds.users.update(created);
const deleted = await ds.users.delete(created);
```

## After the spec changes

After you change `types.yaml` or `datasource.yaml`, snapshot the next version, then run `generate all`.

```sh
npx @deterministic-code/datasources-cli version
npx @deterministic-code/datasources-cli generate all --dialect typescript
```

`version` writes `deterministic/version-<stamp>-<seq>/diff.yaml` from the top-level YAML. `--input` selects the deterministic project (default `./deterministic`). An unchanged spec prints `unchanged` and writes nothing; stop there. `generate all` then reads those snapshots and writes the next migration. Pass `--dialect csharp`, `typescript`, or `rust`. Run it once per dialect when the project has more than one.

## Generated directory hook

Do not create, edit, or delete a file under a directory named `generated`. The hook replies with `edit types.yaml or datasource.yaml and run generate all`.

In a consumer project, if `.cursor/hooks/block-generated.mjs` is missing, offer to install the hook before editing and wait for a yes. Skip the offer when the project root contains `scripts/generate-skill.mjs`.

Copy `hooks/block-generated.mjs` from the same directory as this file to `.cursor/hooks/block-generated.mjs`. `datasources-cli generate` and `npm run cli` may still write there. Merge the entries below and keep every hook the project already has. Add a command only when that command is not already listed.

`.cursor/hooks.json`:

```json
{
  "version": 1,
  "hooks": {
    "preToolUse": [
      { "command": "node .cursor/hooks/block-generated.mjs", "failClosed": true }
    ],
    "beforeShellExecution": [
      { "command": "node .cursor/hooks/block-generated.mjs", "failClosed": true }
    ]
  }
}
```

`.claude/settings.json`:

```json
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Write|Edit|MultiEdit|NotebookEdit|Bash|PowerShell",
        "hooks": [
          {
            "type": "command",
            "command": "node \"${CLAUDE_PROJECT_DIR}/.cursor/hooks/block-generated.mjs\""
          }
        ]
      }
    ]
  }
}
```
