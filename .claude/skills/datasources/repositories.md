<!-- Generated from docs/. Edit the markdown in docs/ and run npm run docs:skill. -->

# Repositories

Repositories map datasource commands to a domain object. They mediate reading and modifying objects in a datasource, and they can enrich those objects into view objects with extra properties. That abstraction removes boilerplate datasource code.

## Classes

#### TypeScript

#### IRepository {#typescript-irepository}

```typescript
interface IRepository<T = unknown>
```

**Properties**

| Name | Description |
| --- | --- |
| type | Item constructor the repository exposes. |

#### C#

#### IRepository {#csharp-irepository}

```csharp
public interface IRepository
```

**Properties**

| Name | Description |
| --- | --- |
| Type | Item CLR type the repository exposes. |

#### Rust

#### IRepository {#rust-irepository}

```rust
pub trait IRepository
```

**Methods**

| Name | Description |
| --- | --- |
| [entity_type()](#rust-irepository-entity_type) | Item `TypeId` the repository exposes. Defaults to `Row`. |

##### entity_type() {#rust-irepository-entity_type}

Item `TypeId` the repository exposes. Defaults to `Row`.

**Definition**

```rust
fn entity_type(&self) -> TypeId
```

**Returns**

`TypeId`

#### datasources {#rust-datasources}

```rust
pub fn as_readonly(repo: Arc<dyn ICRUDRepository>) -> Arc<dyn IReadonlyRepository>
```

**Methods**

| Name | Description |
| --- | --- |
| [as_readonly(repo)](#rust-datasources-as_readonly-repo) | Views a [ICRUDRepository](crud-repository.md) as a [IReadonlyRepository](readonly-repository.md). |

##### as_readonly(repo) {#rust-datasources-as_readonly-repo}

Views a [ICRUDRepository](crud-repository.md) as a [IReadonlyRepository](readonly-repository.md).

**Definition**

```rust
pub fn as_readonly(repo: Arc<dyn ICRUDRepository>) -> Arc<dyn IReadonlyRepository>
```

**Parameters**

| Name | Description |
| --- | --- |
| `repo` | [ICRUDRepository](crud-repository.md) |

**Returns**

`Arc<dyn `[IReadonlyRepository](readonly-repository.md)`>`

## Remarks

SQL provider default repositories report the row or dictionary type (`Row`, `Dictionary<string, Value>`, or `Object`), not a closed entity generic.

## Examples

### Simple readonly

`ReadSingle` / `readSingle` / `read_single` returns the first matching user. `Read` / `read` / `read` yields every user that matches the where.

#### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#L22-L27) [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#L34-L39) [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#L29-L32) [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#L49-L72)

```csharp
sealed class User
{
    public long Id { get; set; }
    public string Username { get; set; } = "";
    public bool IsEnabled { get; set; }
}

static IMapper<string, string> SnakeThenPlural()
{
    return new TransformMapper<string, string>(
        name => new PluralizerMapper().Map(Snake().Map(name)),
        name => Snake().ReverseMap(new PluralizerMapper().ReverseMap(name)));
}

static IMapper<string, string> Snake()
{
    return new ChangeCasingMapper(Casing.Pascal, Casing.Snake);
}

var ds = Datasource.Default;
await Execute(connection, "INSERT INTO users (id, username, is_enabled) VALUES (1, 'ada', 1)");
await Execute(connection, "INSERT INTO users (id, username, is_enabled) VALUES (2, 'grace', 0)");
await Execute(connection, "INSERT INTO users (id, username, is_enabled) VALUES (3, 'bea', 1)");
var repo = ds.CreateReadonlyRepository<User>(connection, new ReadonlyRepositoryMappingOptions<User>
{
    EntityMapper = SnakeThenPlural(),
    PropertyMappings = Snake(),
});

var byId = await repo.ReadSingle(new Where("Id", eq, 1));
var row = await repo.Read(new Where("Username", eq, "grace").And("IsEnabled", eq, 0)).SingleAsync();
```

**Row**

```
{
  Id: 2,
  Username: 'grace',
  IsEnabled: false
}
```

#### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/readonly_repository.rs#L18-L26) [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/readonly_repository.rs#L45-L55) [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/readonly_repository.rs#L41-L43) [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/readonly_repository.rs#L68-L113)

```rust
impl ConvertSchema for User {
    fn schema() -> Vec<(&'static str, &'static str)> {
        vec![
            ("Id", "INTEGER"),
            ("Username", "TEXT"),
            ("IsEnabled", "INTEGER"),
        ]
    }
}

fn snake_then_plural() -> TransformMapper<String, String> {
    let snake = Arc::new(snake());
    let forward = snake.clone();
    let reverse = snake;
    TransformMapper::new(
        move |name: String| PluralizerMapper::new().map(Mapper::map(forward.as_ref(), name)),
        move |name: String| {
            Mapper::reverse_map(reverse.as_ref(), PluralizerMapper::new().reverse_map(name))
        },
    )
}

fn snake() -> ChangeCasingMapper {
    ChangeCasingMapper::new(Casing::Pascal, Casing::Snake)
}

let ds = default().unwrap();
execute_sql(
    &connection,
    "INSERT INTO users (id, username, is_enabled) VALUES (1, 'ada', 1)",
)
.await;
execute_sql(
    &connection,
    "INSERT INTO users (id, username, is_enabled) VALUES (2, 'grace', 0)",
)
.await;
execute_sql(
    &connection,
    "INSERT INTO users (id, username, is_enabled) VALUES (3, 'bea', 1)",
)
.await;
let mut options = create_readonly_repository_mapping_options::<User>();
options.entity_mapper = Some(std::sync::Arc::new(snake_then_plural()));
options.property_mappings = Some(std::sync::Arc::new(snake()));
let repo = ds.create_readonly_repository::<User>(Some(connection), Some(options));
let by_id = repo
    .read_single(Some(&Where::binary("Id", eq, 1_i64)), None, None)
    .await
    .unwrap()
    .unwrap();
let row = repo
    .read(
        Some(&Where::binary("Username", eq, "grace").and("IsEnabled", eq, 0_i64)),
        None, None
    )
    .await
    .unwrap()
    .into_iter()
    .next()
    .unwrap();
```

**Row**

```
{
  Id: 2,
  Username: 'grace',
  IsEnabled: false
}
```

#### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/readonly.repository.integration.test.ts#L27-L31) [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/readonly.repository.integration.test.ts#L37-L42) [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/readonly.repository.integration.test.ts#L33-L35) [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/readonly.repository.integration.test.ts#L76-L94)

```typescript
class User {
  Id = 0;
  Username = "";
  IsEnabled = false;
}

function snakeThenPlural() {
  return new TransformMapper(
    (name: string) => new PluralizerMapper().map(snake().map(name)),
    (name: string) => snake().reverseMap(new PluralizerMapper().reverseMap(name)),
  );
}

function snake() {
  return new ChangeCasingMapper("pascalCase", "snakeCase");
}

const ds = Datasource.default;
await executeSql(connection, "INSERT INTO users (id, username, is_enabled) VALUES (1, 'ada', 1)");
await executeSql(connection, "INSERT INTO users (id, username, is_enabled) VALUES (2, 'grace', 0)");
await executeSql(connection, "INSERT INTO users (id, username, is_enabled) VALUES (3, 'bea', 1)");
const repo = ds.createReadonlyRepository(User, connection, {
  entityMapper: snakeThenPlural(),
  propertyMappings: snake(),
});
const byId = await repo.readSingle(new Where("Id", eq, 1));
const [row] = await Array.fromAsync(repo.read(new Where("Username", eq, "grace").and("IsEnabled", eq, 0)));
```

**Row**

```
{
  Id: 2,
  Username: 'grace',
  IsEnabled: false
}
```

Schema: [minimal-users](docs/examples.md#minimal-users)

```sql
CREATE TABLE users (id int, username string NOT NULL, is_enabled bool NOT NULL, CONSTRAINT pk_users PRIMARY KEY (id))
```

### Create, Read, Update, and Delete

Create omits `id`, then writes the inserted id onto the user. Update and delete use that id.

#### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#L22-L27) [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#L621-L653)

```csharp
sealed class User
{
    public long Id { get; set; }
    public string Username { get; set; } = "";
    public bool IsEnabled { get; set; }
}

var ds = Datasource.Default;
var casing = new ChangeCasingMapper(Casing.Pascal, Casing.Snake);
var usersRepo = ds.CreateRepository<User>(connection, new CRUDRepositoryOptions<User>
{
    PropertyMappings = casing,
    Ids = ["Id"],
});
var created = await usersRepo.Create(new User { Username = "ada", IsEnabled = true });
var byId = await usersRepo.ReadSingleById(created.Id);
var matches = new List<User>();
await foreach (var row in usersRepo.Read(new Where("Username", eq, "ada").And("IsEnabled", eq, true)))
    matches.Add(row);
created.Username = "ada-v2";
var updated = await usersRepo.Update(created);
var renamed = await usersRepo.ReadSingleById(created.Id);
var removed = await usersRepo.Delete(created);
var gone = await usersRepo.ReadSingleById(created.Id);
```

**After update**

```
{
  Id: 1,
  Username: 'ada-v2',
  IsEnabled: true
}
```

#### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/readonly_repository.rs#L18-L26) [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/readonly_repository.rs#L653-L705)

```rust
impl ConvertSchema for User {
    fn schema() -> Vec<(&'static str, &'static str)> {
        vec![
            ("Id", "INTEGER"),
            ("Username", "TEXT"),
            ("IsEnabled", "INTEGER"),
        ]
    }
}

let ds = default().unwrap();
let mut options = create_crud_repository_options::<User>();
options.property_mappings = Some(std::sync::Arc::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake)));
options.ids = vec!["Id".into()];
let users_repo = ds.create_repository::<User>(Some(connection), Some(options));
let mut created = datasources::Row::new();
created.insert("Username".into(), Value::from("ada"));
created.insert("IsEnabled".into(), Value::Bool(true));
let mut created = users_repo.create(&mut created).await.unwrap();
let by_id = users_repo
    .read_single_by_id(created.get("Id").unwrap())
    .await
    .unwrap()
    .unwrap();
let matches = users_repo
    .read(
        Some(&Where::binary("Username", eq, "ada").and(
            "IsEnabled",
            eq,
            Value::Bool(true),
        )),
        None, None
)
    .await
    .unwrap();
created.insert("Username".into(), Value::from("ada-v2"));
let updated = users_repo.update(&created, None).await.unwrap();
let renamed = users_repo
    .read_single_by_id(created.get("Id").unwrap())
    .await
    .unwrap()
    .unwrap();
let removed = users_repo.delete(&created, None).await.unwrap();
let gone = users_repo
    .read_single_by_id(created.get("Id").unwrap())
    .await
    .unwrap();
```

**After update**

```
{
  Id: 1,
  Username: 'ada-v2',
  IsEnabled: true
}
```

#### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/readonly.repository.integration.test.ts#L27-L31) [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/readonly.repository.integration.test.ts#L229-L256)

```typescript
class User {
  Id = 0;
  Username = "";
  IsEnabled = false;
}

const ds = Datasource.default;
const casing = new ChangeCasingMapper("pascalCase", "snakeCase");
const usersRepo = ds.createRepository(User, connection, {
  ...createCRUDRepositoryOptions(User),
  propertyMappings: casing,
  ids: ["Id"],
});
const created = await usersRepo.create(Object.assign(new User(), { Username: "ada", IsEnabled: true }));
const byId = await usersRepo.readSingleById(created.Id);
const matches: User[] = [];
for await (const row of usersRepo.read(new Where("Username", eq, "ada").and("IsEnabled", eq, true)))
  matches.push(row);
created.Username = "ada-v2";
const updated = await usersRepo.update(created);
const renamed = await usersRepo.readSingleById(created.Id);
const removed = await usersRepo.delete(created);
const gone = await usersRepo.readSingleById(created.Id);
```

**After update**

```
{
  Id: 1,
  Username: 'ada-v2',
  IsEnabled: true
}
```

Schema: [user-pascal](docs/examples.md#user-pascal)

```sql
CREATE TABLE User (id int, is_enabled bool NOT NULL, username string NOT NULL, CONSTRAINT pk_User PRIMARY KEY (id))
```

### Enriched Create, Read, Update, and Delete

`User` maps onto `user`. Create writes each user and its roles. Enrichment fills `UserType` from that row. Update renames ada, switches the type to member, and adds the owner role when the version matches.

#### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#L195-L204) [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#L183-L187) [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#L189-L193) [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#L206-L293)

```csharp
record User
{
    public long Key { get; set; }
    public string Login { get; set; } = "";
    public bool Active { get; set; }
    public DateTime Opened { get; set; }
    public UserType UserType { get; set; } = new();
    public long Version { get; set; }
    public List<UserRole> Roles { get; set; } = [];
}

record UserType
{
    public long Id { get; set; }
    public string Name { get; set; } = "";
}

record UserRole
{
    public long UserId { get; set; }
    public string Name { get; set; } = "";
}

var ds = Datasource.Default;
var casing = new ChangeCasingMapper(Casing.Pascal, Casing.Snake);
var usersRepo = new CRUDRepository<User>(connection, new CRUDRepositoryOptions<User>
{
    Ids = ["Key"],
    EntityMapper = casing,
    PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
    {
        ["Key"] = "id",
        ["Login"] = "username",
        ["Active"] = "is_enabled",
        ["Opened"] = "created",
        ["UserType"] = "user_type_id",
        ["Version"] = "version",
    }),
});
var userTypesRepo = new CRUDRepository<UserType>(connection, new CRUDRepositoryOptions<UserType>
{
    Ids = ["Id"],
    EntityMapper = casing,
    PropertyMappings = casing,
});
var userRolesRepo = new CRUDRepository<UserRole>(connection, new CRUDRepositoryOptions<UserRole>
{
    Ids = [],
    EntityMapper = casing,
    PropertyMappings = casing,
});
usersRepo.AddEnriched(new("UserType", "UserType", "Id"), userTypesRepo);
usersRepo.AddEnriched(new("Roles", "Key", "UserId", Type: EnrichmentPropertyType.List), userRolesRepo);
var admin = await userTypesRepo.Create(new UserType { Name = "admin" });
var user = new User
{
    Login = "ada",
    Active = true,
    Opened = new DateTime(2020, 1, 1, 0, 0, 0, DateTimeKind.Utc),
    UserType = admin,
    Version = 1,
    Roles = [new UserRole { Name = "admin" }, new UserRole { Name = "editor" }],
};
var created = await usersRepo.Create(user);
var grace = await usersRepo.Create(new User
{
    Login = "grace",
    Active = true,
    Opened = new DateTime(2024, 6, 1, 0, 0, 0, DateTimeKind.Utc),
    UserType = admin,
    Version = 1,
    Roles = [new UserRole { Name = "viewer" }],
});
var loaded = await usersRepo.ReadSingle(new Where("Key", eq, user.Key));
var version = loaded!.Version;
var member = await userTypesRepo.Create(new UserType { Name = "member" });
loaded.Login = "ada-v2";
loaded.UserType = member;
loaded.Roles.Add(new UserRole { Name = "owner" });
loaded.Version = version + 1;
var updated = await usersRepo.Update(loaded, new Where("Key", eq, loaded.Key).And("Version", eq, version));
var current = await usersRepo.ReadSingle(new Where("Key", eq, loaded.Key));
var stale = await usersRepo.Update(loaded, new Where("Key", eq, loaded.Key).And("Version", eq, version));
var removed = await usersRepo.Delete(loaded);
var gone = await usersRepo.ReadSingleById(loaded.Key);
```

**Loaded**

```
{
  Key: 1,
  Login: 'ada',
  Active: true,
  UserType: {
    Id: 1,
    Name: 'admin'
  },
  Roles: ['admin', 'editor'],
  Version: 1
}
```

**Updated**

```
{
  rowsAffected: 1
}
```

**Current**

```
{
  Key: 1,
  Login: 'ada-v2',
  Active: true,
  UserType: {
    Id: 2,
    Name: 'member'
  },
  Roles: ['admin', 'editor', 'owner'],
  Version: 2
}
```

**Stale**

```
{
  rowsAffected: 0
}
```

**Removed**

```
{
  rowsAffected: 1
}
```

**Gone**

```
no row
```

#### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/readonly_repository.rs#L718-L729) [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/readonly_repository.rs#L733-L737) [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/readonly_repository.rs#L741-L745) [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/readonly_repository.rs#L760-L913)

```rust
impl ConvertSchema for User {
    fn schema() -> Vec<(&'static str, &'static str)> {
        vec![
            ("Key", "INTEGER"),
            ("Login", "TEXT"),
            ("Active", "INTEGER"),
            ("Opened", "TEXT"),
            ("UserType", "INTEGER"),
            ("Version", "INTEGER"),
        ]
    }
}

impl ConvertSchema for UserType {
    fn schema() -> Vec<(&'static str, &'static str)> {
        vec![("Id", "INTEGER"), ("Name", "TEXT")]
    }
}

impl ConvertSchema for UserRole {
    fn schema() -> Vec<(&'static str, &'static str)> {
        vec![("UserId", "INTEGER"), ("Name", "TEXT")]
    }
}

let ds = default().unwrap();
let mut users_options = create_crud_repository_options::<User>();
users_options.ids = vec!["Key".into()];
users_options.entity_mapper =
    Some(std::sync::Arc::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake)));
users_options.property_mappings = Some(std::sync::Arc::new(DictionaryMapper::new([
    ("Key".to_string(), "id".to_string()),
    ("Login".to_string(), "username".to_string()),
    ("Active".to_string(), "is_enabled".to_string()),
    ("Opened".to_string(), "created".to_string()),
    ("UserType".to_string(), "user_type_id".to_string()),
    ("Version".to_string(), "version".to_string()),
])));
users_options.row_converter = Some(RowTypeConverter::of::<User>());
users_options.object_converter = Some(Box::new(
    MetaDataRowTypeConverter::of::<User>().with_references(
        vec!["UserType".into()],
        Box::new(DictionaryMapper::new([
            ("Key".to_string(), "id".to_string()),
            ("Login".to_string(), "username".to_string()),
            ("Active".to_string(), "is_enabled".to_string()),
            ("Opened".to_string(), "created".to_string()),
            ("UserType".to_string(), "user_type_id".to_string()),
            ("Version".to_string(), "version".to_string()),
        ])),
    ),
));
let casing = ChangeCasingMapper::new(Casing::Pascal, Casing::Snake);
let mut type_options = create_crud_repository_options::<UserType>();
type_options.entity_mapper =
    Some(std::sync::Arc::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake)));
type_options.property_mappings = Some(std::sync::Arc::new(casing));
type_options.row_converter = Some(RowTypeConverter::of::<UserType>());
let mut role_options = create_crud_repository_options::<UserRole>();
role_options.entity_mapper =
    Some(std::sync::Arc::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake)));
role_options.property_mappings =
    Some(std::sync::Arc::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake)));
role_options.row_converter = Some(RowTypeConverter::of::<UserRole>());
let mut users_repo = CRUDRepository::new(connection.clone(), users_options).unwrap();
let user_types_repo =
    Arc::new(CRUDRepository::new(connection.clone(), type_options).unwrap());
let user_roles_repo = Arc::new(CRUDRepository::new(connection, role_options).unwrap());
users_repo.add_enriched(
    datasources::Enrichment::new(
        "UserType",
        "UserType",
        "Id",
        None,
        datasources::EnrichmentPropertyType::Object,
    ),
    user_types_repo.clone(),
    None,
);
users_repo.add_enriched(
    datasources::Enrichment::new("Roles", "Key", "UserId", None, datasources::EnrichmentPropertyType::List),
    user_roles_repo.clone(),
    None,
);
let mut admin = datasources::Row::new();
admin.insert("Name".into(), Value::from("admin"));
let admin = user_types_repo.create(&mut admin).await.unwrap();
let mut admin_role = datasources::Row::new();
admin_role.insert("Name".into(), Value::from("admin"));
let mut editor_role = datasources::Row::new();
editor_role.insert("Name".into(), Value::from("editor"));
let mut user = datasources::Row::new();
user.insert("Login".into(), Value::from("ada"));
user.insert("Active".into(), Value::Bool(true));
user.insert("Opened".into(), Value::from("2020-01-01T00:00:00.000Z"));
user.insert("UserType".into(), admin.get("Id").cloned().unwrap());
user.insert("Version".into(), Value::Int(1));
user.insert("Roles".into(), encode_rows(&[admin_role, editor_role]));
let user = users_repo.create(&mut user).await.unwrap();
let mut viewer_role = datasources::Row::new();
viewer_role.insert("Name".into(), Value::from("viewer"));
let mut grace = datasources::Row::new();
grace.insert("Login".into(), Value::from("grace"));
grace.insert("Active".into(), Value::Bool(true));
grace.insert("Opened".into(), Value::from("2024-06-01T00:00:00.000Z"));
grace.insert("UserType".into(), admin.get("Id").cloned().unwrap());
grace.insert("Version".into(), Value::Int(1));
grace.insert("Roles".into(), encode_rows(&[viewer_role]));
let grace = users_repo.create(&mut grace).await.unwrap();
let key = user.get("Key").cloned().unwrap();
let mut loaded = users_repo
    .read_single(Some(&Where::binary("Key", eq, key.clone())), None, None)
    .await
    .unwrap()
    .unwrap();
let version = loaded.get("Version").cloned().unwrap();
let mut member = datasources::Row::new();
member.insert("Name".into(), Value::from("member"));
let member = user_types_repo.create(&mut member).await.unwrap();
let mut owner_role = datasources::Row::new();
owner_role.insert("Name".into(), Value::from("owner"));
let mut kept_admin = datasources::Row::new();
kept_admin.insert("Name".into(), Value::from("admin"));
let mut kept_editor = datasources::Row::new();
kept_editor.insert("Name".into(), Value::from("editor"));
loaded.insert("Login".into(), Value::from("ada-v2"));
loaded.insert("UserType".into(), member.get("Id").cloned().unwrap());
loaded.insert(
    "Roles".into(),
    encode_rows(&[kept_admin, kept_editor, owner_role]),
);
loaded.insert("Version".into(), Value::Int(2));
let updated = users_repo
    .update(
        &loaded,
        Some(&Where::binary("Key", eq, key.clone()).and("Version", eq, version.clone())),
    )
    .await
    .unwrap();
let current = users_repo
    .read_single(Some(&Where::binary("Key", eq, key.clone())), None, None)
    .await
    .unwrap()
    .unwrap();
let stale = users_repo
    .update(
        &loaded,
        Some(&Where::binary("Key", eq, key.clone()).and("Version", eq, version)),
    )
    .await
    .unwrap();
let removed = users_repo.delete(&loaded, None).await.unwrap();
let gone = users_repo.read_single_by_id(&key).await.unwrap();
```

**Loaded**

```
{
  Key: 1,
  Login: 'ada',
  Active: true,
  UserType: {
    Id: 1,
    Name: 'admin'
  },
  Roles: ['admin', 'editor'],
  Version: 1
}
```

**Updated**

```
{
  rowsAffected: 1
}
```

**Current**

```
{
  Key: 1,
  Login: 'ada-v2',
  Active: true,
  UserType: {
    Id: 2,
    Name: 'member'
  },
  Roles: ['admin', 'editor', 'owner'],
  Version: 2
}
```

**Stale**

```
{
  rowsAffected: 0
}
```

**Removed**

```
{
  rowsAffected: 1
}
```

**Gone**

```
no row
```

#### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/readonly.repository.integration.test.ts#L278-L286) [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/readonly.repository.integration.test.ts#L268-L271) [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/readonly.repository.integration.test.ts#L273-L276) [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/readonly.repository.integration.test.ts#L288-L369)

```typescript
class User {
  Key = 0;
  Login = "";
  Active = false;
  Opened = "";
  UserType = new UserType();
  Version = 0;
  Roles: UserRole[] = [];
}

class UserType {
  Id = 0;
  Name = "";
}

class UserRole {
  UserId = 0;
  Name = "";
}

const ds = Datasource.default;
const casing = new ChangeCasingMapper("pascalCase", "snakeCase");
const usersRepo = new CRUDRepository(connection, {
  ...createCRUDRepositoryOptions(User),
  ids: ["Key"],
  entity: "User",
  entityMapper: casing,
  propertyMappings: new DictionaryMapper([
    ["Key", "id"],
    ["Login", "username"],
    ["Active", "is_enabled"],
    ["Opened", "created"],
    ["UserType", "user_type_id"],
    ["Version", "version"],
  ]),
});
const userTypesRepo = new CRUDRepository(connection, {
  ...createCRUDRepositoryOptions(UserType),
  entity: "UserType",
  entityMapper: casing,
  propertyMappings: casing,
});
const userRolesRepo = new CRUDRepository(connection, {
  ...createCRUDRepositoryOptions(UserRole),
  entity: "UserRole",
  entityMapper: casing,
  propertyMappings: casing,
});
usersRepo.addEnriched({ destinationProperty: "UserType", sourceProperty: "UserType", relatedKey: "Id" }, userTypesRepo);
usersRepo.addEnriched({ destinationProperty: "Roles", sourceProperty: "Key", relatedKey: "UserId", type: "list" }, userRolesRepo);
const admin = await userTypesRepo.create(Object.assign(new UserType(), { Name: "admin" }));
const user = Object.assign(new User(), {
  Login: "ada",
  Active: true,
  Opened: "2020-01-01T00:00:00.000Z",
  UserType: admin,
  Version: 1,
  Roles: [Object.assign(new UserRole(), { Name: "admin" }), Object.assign(new UserRole(), { Name: "editor" })],
});
const created = await usersRepo.create(user);
const grace = await usersRepo.create(Object.assign(new User(), {
  Login: "grace",
  Active: true,
  Opened: "2024-06-01T00:00:00.000Z",
  UserType: admin,
  Version: 1,
  Roles: [Object.assign(new UserRole(), { Name: "viewer" })],
}));
const loaded = await usersRepo.readSingle(new Where("Key", eq, user.Key));
const version = loaded!.Version;
const member = await userTypesRepo.create(Object.assign(new UserType(), { Name: "member" }));
loaded!.Login = "ada-v2";
loaded!.UserType = member;
loaded!.Roles.push(Object.assign(new UserRole(), { Name: "owner" }));
loaded!.Version = version + 1;
const updated = await usersRepo.update(loaded!, new Where("Key", eq, loaded!.Key).and("Version", eq, version));
const current = await usersRepo.readSingle(new Where("Key", eq, loaded!.Key));
const stale = await usersRepo.update(loaded!, new Where("Key", eq, loaded!.Key).and("Version", eq, version));
const removed = await usersRepo.delete(loaded!);
const gone = await usersRepo.readSingleById(loaded!.Key);
```

**Loaded**

```
{
  Key: 1,
  Login: 'ada',
  Active: true,
  UserType: {
    Id: 1,
    Name: 'admin'
  },
  Roles: ['admin', 'editor'],
  Version: 1
}
```

**Updated**

```
{
  rowsAffected: 1
}
```

**Current**

```
{
  Key: 1,
  Login: 'ada-v2',
  Active: true,
  UserType: {
    Id: 2,
    Name: 'member'
  },
  Roles: ['admin', 'editor', 'owner'],
  Version: 2
}
```

**Stale**

```
{
  rowsAffected: 0
}
```

**Removed**

```
{
  rowsAffected: 1
}
```

**Gone**

```
no row
```

Schema: [user-view-version](docs/examples.md#user-view-version)

```sql
CREATE TABLE user_type (id int, name string NOT NULL, CONSTRAINT pk_user_type PRIMARY KEY (id));
CREATE TABLE user (id int, username string NOT NULL, is_enabled bool NOT NULL, created datetime NOT NULL, user_type_id int NOT NULL, version int NOT NULL, CONSTRAINT pk_user PRIMARY KEY (id));
CREATE TABLE user_role (user_id int NOT NULL, name string NOT NULL)
```

### Bulk

Each parent is created with its nested children. Responses sum `rowsAffected`.

#### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/BulkCRUDRepositoryTests.cs#L29-L47)

```csharp
var connection = await Setup();
var parentsRepo = new BulkCRUDRepository<Parent>(connection);
var childrenRepo = new BulkCRUDRepository<Child>(connection);
var ids = await Repo(parentsRepo, childrenRepo).Create(
[
    new Parent { children = [new Child { label = "a" }] },
    new Parent { children = [new Child { label = "b" }, new Child { label = "c" }] }
]);
var parents = await parentsRepo.Read().ToListAsync();
var children = await childrenRepo.Read().ToListAsync();
```

#### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/tests/enriched.rs#L904-L942)

```rust
let connection = setup(&mut []).await;
let mut options = CRUDRepositoryOptions::default();
options.entity = "Parent".into();
let mut parents = BulkCRUDRepository::new(connection.clone(), options).unwrap();
let mut child_options = CRUDRepositoryOptions::default();
child_options.entity = "Child".into();
let children = Arc::new(BulkCRUDRepository::new(connection, child_options).unwrap());
parents.add_enriched(
    Enrichment::new("children", "id", "parent_id", None, datasources::EnrichmentPropertyType::List),
    children.clone(),
    None,
);
let ids = parents
    .create_bulk(&mut [
        record(&[(
            "children",
            encode_rows(&[record(&[("label", Value::from("a"))])]),
        )]),
        record(&[(
            "children",
            encode_rows(&[
                record(&[("label", Value::from("b"))]),
                record(&[("label", Value::from("c"))]),
            ]),
        )]),
    ])
    .await
    .unwrap();
let stored_parents = parents.read(None, None, None).await.unwrap();
```

#### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/BulkCRUDRepository.test.ts#L33-L49)

```typescript
const connection = await setup();
const parentsRepo = new BulkCRUDRepository(Parent, connection);
const childrenRepo = new BulkCRUDRepository(Child, connection);
const ids = await relate(parentsRepo, childrenRepo).createBulk([
  Object.assign(new Parent(), { children: [Object.assign(new Child(), { label: "a" })] }),
  Object.assign(new Parent(), {
    children: [Object.assign(new Child(), { label: "b" }), Object.assign(new Child(), { label: "c" })],
  }),
]);
const parents = await collectAsync(parentsRepo.read());
const children = await collectAsync(childrenRepo.read());
expect(parents).toHaveLength(2);
```

## See also

- [Readonly repository](readonly-repository.md)
- [CRUD repository](crud-repository.md)
- [Bulk repository](bulk-repository.md)
- [Repository factories](repository-factories.md)
