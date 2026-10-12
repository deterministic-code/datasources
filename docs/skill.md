# Using the skill

## Run setup and add providers

::: skill-prompt
Run setup and add sqlite and postgres for csharp and rust
:::

**Expected**

```sh
npx @datasources/cli setup --yes --ecosystem csharp
npx @datasources/cli add sqlite postgres --ecosystem csharp
npx @datasources/cli add sqlite postgres --ecosystem rust
```

# Example prompts

## Generate a user repository

::: skill-prompt
Generate a user with field id: username, is_enabled in types.yaml and generate repositories for csharp
:::

**Expected**

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
npx @datasources/cli generate all --dialect csharp
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

## Generate users and roles

::: skill-prompt
The same `user` table, plus `role` and a `user_role` junction tagged `many_to_many`, with a `roles` list on `user`. Generate TypeScript and create a user that has a role.
:::

**Expected**

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

## Run native SQL

::: skill-prompt
Insert an item and read it back with a command and SQL text. Do not use a repository.
:::

**Expected**

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
    Native.N("INSERT INTO items (id, name) VALUES ($id, $name)"),
    [new Parameter("id", 1), new Parameter("name", "a")]);
var converter = new MetaDataRowTypeConverter<Item>();
await using var createdReader = await command.ExecuteReader(
    Native.N("SELECT id, name FROM items WHERE id = $id"),
    [new Parameter("id", 1)]);
var created = await createdReader.Read().Select(row => converter.Convert(row)).ToListAsync();
```

## Map a flat row into nested contacts

::: skill-prompt
Read users with a command, convert each row, then build a `contacts` list from `contact_name1` and `contact_name2`.
:::

**Expected**

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

## Select a contact and its type

::: skill-prompt
Write a `SelectStatement` for `Contact.FirstName` and `ContactType.Name`, inner-joining `Contact` to `ContactType` on `Contact.ContactTypeId` = `ContactType.Id`.
:::

**Expected**

```csharp
new SelectStatement(new()
{
    Table = "Contact",
    Fields = ["Contact.FirstName AS FirstName", "ContactType.Name AS ContactTypeName"],
    Joins = [Join.Inner("ContactType", "Contact.ContactTypeId", "ContactType.Id")],
});
```

## Map a legacy table and a unique key

::: skill-prompt
The type stays `user`, but the stored table is `OldUsersTbl`. `username` is unique and stored as `LoginNm`, `id` as `UsrID`, and `is_enabled` as `ActvFlg`. Add `datasource.yaml` mappings and generate.
:::

**Expected**

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
