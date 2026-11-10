# Using the skill

## Run setup and add providers

::: skill-prompt
Run setup and add sqlite and postgres for csharp and rust
:::

**Expected**

```sh
npx @deterministic-code/datasources-cli setup --yes --languages csharp rust --datasources sqlite postgres
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
npx @deterministic-code/datasources-cli generate all --dialect csharp
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
npx @deterministic-code/datasources-cli generate all --dialect typescript
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
    Fields = ["Contact.FirstName", "ContactType.Name"],
    Aliases = ["FirstName", "ContactTypeName"],
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
npx @deterministic-code/datasources-cli generate all --dialect typescript
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

## Create datasource seeds

::: skill-prompt
Add `datasource_seeds.yaml` that seeds `gender` with Male as `id1` and Female as `id2`. Snapshot the spec, then generate for typescript.
:::

**Expected**

```yaml
version: 1.0.0
seeds:
  - gender:
      - id1:
          name: Male
      - id2:
          name: Female
```

```sh
npx @deterministic-code/datasources-cli version
npx @deterministic-code/datasources-cli generate all --dialect typescript
```

## Add a read method that joins tables

::: skill-prompt
Add a read method that joins Game, GameParent, GameCategory, and Category into one GameSummary, with categories nested on the game. One endpoint instead of several calls.
:::

**Expected**

One `SelectStatement` with joins. Fold the flat rows with `NestedToObjectEnrichmentStrategy`. Do not group the rows by hand. Fields are paths. Aliases are the parallel list. A field that contains `AS` is rejected.

```typescript
const ENTITIES = [["Game", "games"], ["GameParent", "game_parents"], ["PuzzleGame", "puzzle_games"],
                  ["Resource", "resources"], ["GameCategory", "game_categories"], ["Category", "categories"]]
const PROPERTIES = [["gameParentId", "game_parent_id"], ["gameId", "game_id"], ["imageId", "image_id"],
                    ["categoryId", "category_id"], ["isDeleted", "is_deleted"], ["isEnabled", "is_enabled"]]

const joined = new SelectStatement({
  table: "Game",
  fields: ["Game.id", "Game.name", "GameParent.name", "Resource.location", "GameCategory.categoryId", "Category.name"],
  aliases: ["id", "name", "parentName", "imageLocation", "categoryId", "categoryName"],
  joins: [
    Join.inner("GameParent", "GameParent.id", "Game.gameParentId"),
    Join.left("GameCategory", "GameCategory.gameId", "Game.id"),
    Join.left("Category", "Category.id", "GameCategory.categoryId"),
  ],
  orderBy: [OrderBy.asc("Game.id"), OrderBy.asc("GameCategory.categoryId")],
})

class GameSummariesService extends ReadonlyRepository<GameSummary> {
  constructor(connection: IConnection) {
    super(connection, { type: GameSummary, entity: "Game",
      entityMapper: new DictionaryMapper(ENTITIES), propertyMappings: new DictionaryMapper(PROPERTIES) })
    this.addEnriched(
      { destinationProperty: "categories", sourceProperty: "id", relatedKey: "categoryId", type: "list" }, this,
      new NestedToObjectEnrichmentStrategy<GameSummary & { imageId: number | null }>({ key: "id", fields: {
        id: "id", name: "name", parentName: "parentName", thumbnailLocation: "imageLocation",
        categories: { key: "categoryId", fields: { id: "categoryId", name: "categoryName" } } } }))
  }
  override read() { return super.read(joined) }
}
```

Open the connection with `await ds.createConnection()` in TypeScript, `CreateConnection()` in C#, and `ds.bind_connection(None)` in Rust.

C# is the same shape: `Aliases = [...]`, `new Join(JoinKind.Left, ...)`, `new NestedToObjectEnrichmentStrategy<T>(new TypeMap { Key, Fields })`, and `OrderBy = [OrderBy.Asc(...)]`. Extra join columns that the generated view lacks go on a private subclass used as the row type.

Rust uses the same statement (`aliases: &[Some("id"), ...]`, `OrderBy::asc`). `NestedToObjectEnrichmentStrategy` returns a nested group as one text string, so group the rows by hand in Rust.

Property mappings stay plain column names. Mapping `gameId` to `games.id` qualifies every path that uses it and can emit `JOIN ... ON games.id = games.id`. A where on a joined column names the parameter `$games.is_deleted`, and sqlite fails with `near "."`. Filter that column in code. A join matches two columns, so a computed key such as `name + '_thumbnail'` is a second read filled in code. A view with no table has no generated repository; build the `ReadonlyRepository` yourself, and the view needs `is_id: true`.

Print the SQL with `repo.addMiddleware({ beforeCommand: (t) => console.log(t), afterCommand: () => {} })`. Both functions are required. Compare the new endpoint with the existing one row by row, and check that categories are non-empty. A scratch script with top-level await under tsx needs the `.mts` extension.
