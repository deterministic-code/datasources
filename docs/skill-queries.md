# Example queries

Six prompts that exercise the skill. C# is the reference. Rust and TypeScript keep the same call shape (`Read` / `read` / `read`). An agent that has the skill opens one topic file and copies the closest sample, including every type that sample defines.

These prompts and the expected answers stay on this page and in `evals/evals.json`. They are not generated into `SKILL.md`.

## Simulate

Simulation checks the code an agent writes. It does not run that code against a database.

1. Spawn an agent with skill path `SKILL.md` and only the prompt. Write the answer under `evals/workspace/iteration-1/<id>/with_skill/`.
2. Do not point the agent at this page or `evals/evals.json`.
3. Grade the written files against that eval’s assertions.
4. A second agent with the same prompt and no skill is optional. Skip it on the first pass.

Iteration output stays untracked in `evals/workspace/`.

## Queries

### Generate a user repository

**Prompt:** `types.yaml` has one `user` datasource type (`id`, `username`, `is_enabled`). Generate the TypeScript repository and create, read, update, and delete a user through it.

**Open:** [generation.md](./repository-generation.md#user) (skill file `generation.md`, heading User).

**Expected (C#):** the `user` yaml from that sample, `npx @datasources/cli generate all --dialect typescript`, then create, read, update, and delete through the generated repository. No hand-written repository class.

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

```sh
npx @datasources/cli generate all --dialect typescript
```

```csharp
using Backend.Types.Datasource;

dynamic ds = Datasource.Default;

User created = await ds.Users.Create(new User { Username = "ada", IsEnabled = true });
User? loaded = await ds.Users.ReadSingle();
created.Username = "ada-v2";
Response updated = await ds.Users.Update(created);
Response deleted = await ds.Users.Delete(created);
```

TypeScript matches `ds.users.create`, `readSingle`, `update`, and `delete`. Rust matches `users().create`, `read_single`, `update`, and `delete`.

### Generate users and roles

**Prompt:** The same `user` table, plus `role` and a `user_role` junction tagged `many_to_many`, with a `roles` list on `user`. Generate TypeScript and create a user that has a role.

**Open:** [generation.md](./repository-generation.md#users-and-roles) (skill file `generation.md`, heading Users and roles).

**Expected (C#):** that yaml, the same `generate all --dialect typescript` command, then create a role and a user whose `Roles` list is that role. `ReadSingle` fills `Roles`. Delete removes the user and the junction row. The link is the generated repository, not a hand-written `AddLinked`.

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

```sh
npx @datasources/cli generate all --dialect typescript
```

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

TypeScript matches `ds.roles.create` and `ds.users.create` with `roles: [admin]`. Rust matches `roles().create` and `users().create` with the encoded `roles` list.

### Run native SQL

**Prompt:** Insert an item and read it back with a command and SQL text. Do not use a repository.

**Open:** [command.md](./sections/command.md#execute-insert-read-update-and-delete) (skill file `command.md`, heading Execute, Insert, Read, Update, and Delete).

**Expected (C#):** `connection.CreateCommand()`, `command.Execute` with `INSERT INTO items` and `Parameter`s, `ExecuteReader` with `SELECT id, name FROM items WHERE id = $id`, and `MetaDataRowTypeConverter<Item>.Convert`. Include the `Item` type. The sample also updates and deletes; the insert and the select are enough.

```csharp
sealed class Item
{
    public long id { get; set; }
    public string name { get; set; } = "";
}

var ds = Datasource.Default;
await using var connection = await ds.CreateConnection();
await connection.Open();
var command = connection.CreateCommand();
await command.Execute(
    "INSERT INTO items (id, name) VALUES ($id, $name)",
    [new Parameter("id", 1), new Parameter("name", "a")]);
var converter = new MetaDataRowTypeConverter<Item>();
await using var createdReader = await command.ExecuteReader(
    "SELECT id, name FROM items WHERE id = $id",
    [new Parameter("id", 1)]);
var created = await createdReader.Read().Select(row => converter.Convert(row)).ToListAsync();
```

TypeScript and Rust match `createCommand` / `create_command`, `execute`, `executeReader` / `execute_reader`, and convert after read.

### Map a flat row into nested contacts

**Prompt:** Read users with a command, convert each row, then build a `contacts` list from `contact_name1` and `contact_name2`.

**Open:** [command.md](./sections/command.md#map-flat-contacts) (skill file `command.md`, heading Map flat contacts).

**Expected (C#):** `ExecuteReader` of those columns, `MetaDataRowTypeConverter<User>`, then a `TransformMapper<User, User>` that fills `contacts` and reverses it. The read is `mapper.Map(converter.Convert(row))`. Include `User` and `User.Contact`. The converter does not build the list.

```csharp
record User
{
    public int id { get; set; }
    public string username { get; set; } = "";
    public string email { get; set; } = "";
    public string contact_name1 { get; set; } = "";
    public string contact_name2 { get; set; } = "";
    public List<Contact> contacts { get; set; } = [];

    public record Contact
    {
        public string name { get; set; } = "";
    }
}

var ds = Datasource.Default;
var command = await ds.CreateCommand();
await using var reader = await command.ExecuteReader(
    "SELECT contact_name1, contact_name2, email, id, username FROM users");
var converter = new MetaDataRowTypeConverter<User>();
TransformMapper<User, User> UserContactsMapper() =>
    new(
        row =>
        {
            row.contacts =
            [
                new User.Contact { name = row.contact_name1 },
                new User.Contact { name = row.contact_name2 }
            ];
            return row;
        },
        user =>
        {
            user.contact_name1 = user.contacts.ElementAtOrDefault(0)?.name ?? "";
            user.contact_name2 = user.contacts.ElementAtOrDefault(1)?.name ?? "";
            return user;
        });
var mapper = UserContactsMapper();
var users = await reader.Read().Select(row => mapper.Map(converter.Convert(row))).ToListAsync();
```

TypeScript and Rust match convert, then map.

### Select a contact and its type

**Prompt:** Write a `SelectStatement` for `Contact.FirstName` and `ContactType.Name`, inner-joining `Contact` to `ContactType` on `Contact.ContactTypeId` = `ContactType.Id`.

**Open:** [select-statement.md](./sections/select-statement.md) (skill file `select-statement.md`).

**Expected (C#):** PascalCase entity types and properties. The statement does not name tables or columns.

```csharp
new SelectStatement(new()
{
    Table = "Contact",
    Fields = ["Contact.FirstName AS FirstName", "ContactType.Name AS ContactTypeName"],
    Joins = [Join.Inner("ContactType", "Contact.ContactTypeId", "ContactType.Id")],
});
```

TypeScript matches `table`, `fields`, and `Join.inner`. Rust matches `SelectStatement::new` and `Join::inner`.

### Map a legacy table and a unique key

**Prompt:** The type stays `user`, but the stored table is `OldUsersTbl`. `username` is unique and stored as `LoginNm`, `id` as `UsrID`, and `is_enabled` as `ActvFlg`. Add `datasource.yaml` mappings and generate.

**Open:** [generation.md](./repository-generation.md#legacy-tables) (skill file `generation.md`, heading Legacy tables).

**Expected (C#):** `types.yaml` unchanged, plus `datasource.yaml` with `mapping: OldUsersTbl` on `user`, field mappings `UsrID` / `LoginNm` / `ActvFlg`, and `is_unique: true` on `username`. Then `npx @datasources/cli generate all --dialect typescript`. CRUD still uses `Username`. No hand-written column rename.

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

```sh
npx @datasources/cli generate all --dialect typescript
```

```csharp
using Backend.Types.Datasource;

dynamic ds = Datasource.Default;

User created = await ds.Users.Create(new User { Username = "ada", IsEnabled = true });
User? loaded = await ds.Users.ReadSingle();
created.Username = "ada-v2";
Response updated = await ds.Users.Update(created);
Response deleted = await ds.Users.Delete(created);
```
