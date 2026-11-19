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

Tag a type `readonly_lookup` when the generated repository is read-only. Create, update, and delete stay off that class. Rows for a `datasource_type` with that tag come from `datasource_seeds.yaml`, not from repository writes.

A `view_type` that does not inherit a base type (`inherits` is missing, or is `dictionary`) is not a table. Generate emits no SQL for it. A dictionary view always gets a type in `types/view/`. Generate emits a repository for an unused dictionary only when `key.references` names a table; without those references, generate emits the view type only. A host field `settings: game_settings_map{}` stays enrichment on the host repository. A view that inherits a datasource type overlays that parent table.

::: dialects
```csharp
npx @deterministic-code/datasources-cli generate all --dialect csharp
```
```rust
npx @deterministic-code/datasources-cli generate all --dialect rust
```
```typescript
npx @deterministic-code/datasources-cli generate all --dialect typescript
```
:::

`generate all` writes the SQL migrations and runs migrate-up against `DATASOURCE_URL`. `--no-migrations` skips the files. `--no-auto-migrate-up` skips applying them. See [Migrations](./cli.md#migrations) to write, apply, or roll back a migration on its own.

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
import { Datasource } from "@deterministic-code/datasources";

const ds = Datasource.default;

const created = await ds.users.create({ id: 0, username: "ada", isEnabled: true });
const loaded = await ds.users.readSingle();
created.username = "ada-v2";
const updated = await ds.users.update(created);
const deleted = await ds.users.delete(created);
```
:::

## Tree and graph views

A `tree` or `graph` tag on a `view_type` fills nested children on read. The inherited `datasource_type` stays a flat table. Tag the view `readonly_lookup` so create, update, and delete stay on that table's repository.

A single-parent tree. Roots are rows whose parent key is null.

```yaml
- category:
    tags: [datasource_type]
    fields:
      - id: { type: integer, is_id: true }
      - parent_category_id:
          references: category.id
          is_nullable: true
      - name: { type: string, size: 128 }
- category_tree:
    tags: [view_type, tree, readonly_lookup]
    inherits: category
    fields:
      - children:
          type: category_tree[]
          references: category.parent_category_id
```

A composite parent matches every key pair.

```yaml
- region_category_tree:
    tags: [view_type, tree, readonly_lookup]
    inherits: region_category
    fields:
      - children:
          type: region_category_tree[]
          references: [region_category.parent_region_id, region_category.parent_category_id]
```

A graph uses a `many_to_many` junction whose foreign keys both point at the node.

```yaml
- node_graph:
    tags: [view_type, graph, readonly_lookup]
    inherits: node
    fields:
      - edges:
          type: node_graph[]
          references: node_edge.from_node_id
```

`generate all` registers a tree with {{TreeReadonlyEnrichmentStrategy|TreeReadonlyEnrichmentStrategy|TreeReadonlyEnrichmentStrategy}} and a flat {{ReadonlyRepository|ReadonlyRepository|ReadonlyRepository}} via. A graph adds the junction child and {{GraphReadonlyEnrichmentStrategy|GraphReadonlyEnrichmentStrategy|GraphReadonlyEnrichmentStrategy}}. An id already on the walk path is dropped, so a cycle still returns.

::: dialects
```csharp
AddEnriched(
    new("Children", "Id", "ParentCategoryId", Type: EnrichmentPropertyType.List),
    new ReadonlyRepository<CategoryTree>(connection, Options()),
    new TreeReadonlyEnrichmentStrategy<CategoryTree>());
```
```rust
repo.add_enriched(
    Enrichment::new("Children", "Id", "ParentCategoryId", None, EnrichmentPropertyType::List),
    ReadonlyRepository::<CategoryTree>::new(connection, options())?,
    Some(Arc::new(TreeReadonlyEnrichmentStrategy)),
);
```
```typescript
this.addEnriched(
  { destinationProperty: "children", sourceProperty: "id", relatedKey: "parentCategoryId", type: "list" },
  new ReadonlyRepository<CategoryTree>(connection, readonlyOptions(connection)),
  new TreeReadonlyEnrichmentStrategy(),
);
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
npx @deterministic-code/datasources-cli generate all --dialect csharp
```
```rust
npx @deterministic-code/datasources-cli generate all --dialect rust
```
```typescript
npx @deterministic-code/datasources-cli generate all --dialect typescript
```
:::

`generate all` writes the SQL migrations and runs migrate-up against `DATASOURCE_URL`. `--no-migrations` skips the files. `--no-auto-migrate-up` skips applying them. See [Migrations](./cli.md#migrations) to write, apply, or roll back a migration on its own.

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
npx @deterministic-code/datasources-cli generate all --dialect csharp
```
```rust
npx @deterministic-code/datasources-cli generate all --dialect rust
```
```typescript
npx @deterministic-code/datasources-cli generate all --dialect typescript
```
:::

`generate all` writes the SQL migrations and runs migrate-up against `DATASOURCE_URL`. `--no-migrations` skips the files. `--no-auto-migrate-up` skips applying them. See [Migrations](./cli.md#migrations) to write, apply, or roll back a migration on its own.

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
import { Datasource } from "@deterministic-code/datasources";

const ds = Datasource.default;

const created = await ds.users.create({ id: 0, username: "ada", isEnabled: true });
const loaded = await ds.users.readSingle();
created.username = "ada-v2";
const updated = await ds.users.update(created);
const deleted = await ds.users.delete(created);
```
:::

## Seeds

Lookup rows live in `datasource_seeds.yaml`, beside `types.yaml` (default `deterministic/datasource_seeds.yaml`). The key under `seeds` is the type name. Each row key is `id` plus a number (`id1`, `id2`). That number is the inserted primary key. The cells are that type's field names. Leave `id` out of the row. A cell is a string, number, boolean, or null.

```yaml
version: 1.0.0
seeds:
  - gender:
      - id1:
          name: Male
      - id2:
          name: Female
```

`includes: - file:` pulls in another seed document. Every included `seeds` list combines with this file (includes first, this document last). There is no `types: { filter }` on seeds.

```yaml
version: 1.0.0
includes:
  - file: genders.yaml
seeds:
  - phone_type:
      - id1:
          name: Mobile
```

`gender` stays the type name when the stored table is `genders`. A mapped column stays the field name here (`email` stored as `email_address` is still `email`). A foreign key is the other row's number (`owner_id: 1` for that type's `id1`). An integer id is that number. A string id is those digits. A uuid id and a field named `uuid` are filled from the type name and the number; leave `uuid` out of the row.

Once any `version-*/diff.yaml` exists, `generate all` reads seed rows from those snapshots. Run `version` after editing the file. The next migration inserts each `idN` that was not in an earlier snapshot. An `idN` already snapshotted stays as inserted: editing its cells or deleting the key leaves the row.

```sh
npx @deterministic-code/datasources-cli version
npx @deterministic-code/datasources-cli generate all --dialect typescript
```
