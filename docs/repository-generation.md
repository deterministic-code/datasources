# Repository Generation

`generate all` reads a Types Specification and writes the types, the repositories, and SQL for one language. `setup` and `generate` wire the generated registry so {{Datasource.Default|Datasource.default|default()}} binds each plural name.

See [Generate](./cli.md#generate) for flags and output paths.

## User

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

`generate all` writes the `User` type, the {{Users|users|users()}} repository, and SQL.

::: dialects
```csharp
npx @datasources/cli generate all --dialect csharp
```
```rust
npx @datasources/cli generate all --dialect rust
```
```typescript
npx @datasources/cli generate all --dialect typescript
```
:::

`generate all` writes the SQL migrations and runs migrate-up against `DATASOURCE_URL`. `--no-migrations` skips the files. `--no-migrate` skips applying them. See [Migrations](./cli.md#migrations) to write, apply, or roll back a migration on its own.

{{Create|create|create}} returns that entity. {{ReadSingle|readSingle|read_single}} returns the same entity. `Update` and `Delete` return `Response`.

::: dialects
```csharp
using Backend.Types.Datasource;

dynamic ds = Datasource.Default;

User created = await ds.Users.Create(new User { Username = "ada", IsEnabled = true });
User? loaded = await ds.Users.ReadSingle();
created.Username = "ada-v2";
Response updated = await ds.Users.Update(created);
Response deleted = await ds.Users.Delete(created);
```
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
```typescript
import { Datasource } from "@datasources/core";

const ds = Datasource.default;

const created = await ds.users.create({ id: 0, username: "ada", isEnabled: true });
const loaded = await ds.users.readSingle();
created.username = "ada-v2";
const updated = await ds.users.update(created);
const deleted = await ds.users.delete(created);
```
:::

## Users and roles

The same `user` table, plus `role` and a `user_role` junction tagged `many_to_many`. A `roles` list on `user` that references `user_role.user_id` generates a linked enrichment: {{AddLinked|addLinked|add_linked}}.

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

::: dialects
```csharp
npx @datasources/cli generate all --dialect csharp
```
```rust
npx @datasources/cli generate all --dialect rust
```
```typescript
npx @datasources/cli generate all --dialect typescript
```
:::

`generate all` writes the SQL migrations and runs migrate-up against `DATASOURCE_URL`. `--no-migrations` skips the files. `--no-migrate` skips applying them. See [Migrations](./cli.md#migrations) to write, apply, or roll back a migration on its own.

Create a role, then a user that lists that role. {{Create|create|create}} writes the junction row. {{ReadSingle|readSingle|read_single}} fills {{Roles|roles|roles}}. Delete removes the user and the junction; the role row stays.

::: dialects
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
```typescript
import { Datasource } from "@datasources/core";

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
:::

## Legacy tables

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

::: dialects
```csharp
npx @datasources/cli generate all --dialect csharp
```
```rust
npx @datasources/cli generate all --dialect rust
```
```typescript
npx @datasources/cli generate all --dialect typescript
```
:::

`generate all` writes the SQL migrations and runs migrate-up against `DATASOURCE_URL`. `--no-migrations` skips the files. `--no-migrate` skips applying them. See [Migrations](./cli.md#migrations) to write, apply, or roll back a migration on its own.

The generated repository maps those stored names. The CRUD call still uses {{Username|username|username}}.

::: dialects
```csharp
using Backend.Types.Datasource;

dynamic ds = Datasource.Default;

User created = await ds.Users.Create(new User { Username = "ada", IsEnabled = true });
User? loaded = await ds.Users.ReadSingle();
created.Username = "ada-v2";
Response updated = await ds.Users.Update(created);
Response deleted = await ds.Users.Delete(created);
```
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
```typescript
import { Datasource } from "@datasources/core";

const ds = Datasource.default;

const created = await ds.users.create({ id: 0, username: "ada", isEnabled: true });
const loaded = await ds.users.readSingle();
created.username = "ada-v2";
const updated = await ds.users.update(created);
const deleted = await ds.users.delete(created);
```
:::
