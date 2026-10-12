<!-- Generated from docs/. Edit the markdown in docs/ and run npm run docs:readme. -->

# Datasources

Datasources is a data-access and object-relational mapper library. It aims for parity across [Languages and Datasource Providers](#languages-and-datasource-providers).

The [Feature Stack:](#feature-stack) starts with commands, connections, and object mapping. Higher up, the API is more opinionated. Types and repositories are generated from a Types Specification, which can be the source of truth for the datasource schema and the generated code. The specification describes unions and inherited types. A type may contain referenced lists and dictionaries, and those references are generated as enriched repositories.

Thus, due to the well-defined [Feature stack](#feature-stack), the library can be used as a high-level repository builder or as a micro-ORM, alike.

The specification is declarative, so it suits generation by AI agents. Progressive skills keep that generation economical and repeatable. They use fewer tokens, and they keep generated code and mappings constant and deterministic. The library was written so TypeScript could be translated to Rust, and so data-access code written by an agent would remain coherent as an application scales.

<div class="quick-start">

[Quick Start](#quick-start)

[Install skill](#installing-the-skill)

</div>

## Quick Start

### Installing the skill

```sh
npx skills add deterministic-code/datasources
```

### Installation from source

##### TypeScript

```typescript
npm install @datasources/core
```

##### C#

```csharp
dotnet add package Datasources
```

##### Rust

```rust
cargo add datasources
```

### Setup

<SkillPrompt text="Run setup and add sqlite and postgres for csharp and rust" />

The skill runs the CLI. `setup` writes `.env`. `add` installs provider packages. `--ecosystem` is one language per call.

```sh
npx @datasources/cli setup --yes --ecosystem csharp
npx @datasources/cli add sqlite postgres --ecosystem csharp
npx @datasources/cli add sqlite postgres --ecosystem rust
```

Run interactive setup when you want prompts instead of `--yes`.

```sh
npx @datasources/cli setup
```

### Generating code from specification

A Types Specification is the `deterministic` directory. `types.yaml` lists each entity. Tag a type `datasource_type` when it is a table.

```yaml
version: 1.0.0
types:
  - item:
      tags:
        - datasource_type
      fields:
        - id:
            type: integer
            is_id: true
        - name:
            type: string
            size: 128
```

`generate all` writes the types, the repositories, and SQL for one language. The default input directory is `./deterministic`. The default output directory is `./generated`.

##### C#

```csharp
npx @datasources/cli generate all --dialect csharp
```

##### Rust

```rust
npx @datasources/cli generate all --dialect rust
```

##### TypeScript

```typescript
npx @datasources/cli generate all --dialect typescript
```

See [Generate](docs/cli.md#generate). Walkthroughs for a User, an enriched users-and-roles spec, and a legacy table mapping are on [Repository Generation](#repository-generation).

### Using the generated code

Import the generated registry so `Datasource.Default` / `Datasource.default` / `default()` binds `Items` / `items` / `items()`. `Create` / `create` / `create` returns that entity. `ReadSingle` / `readSingle` / `read_single` returns the same entity.

##### C#

```csharp
using Backend.Types.Datasource;

dynamic ds = Datasource.Default;

Item created = await ds.Items.Create(new Item { Name = "Ada" });
Item? loaded = await ds.Items.ReadSingle();
created.Name = "Ada Lovelace";
Response updated = await ds.Items.Update(created);
Response deleted = await ds.Items.Delete(created);
```

##### Rust

```rust
#[path = "../generated/rust/repositories/mod.rs"]
mod repositories;

use datasources::{default, Response, Row, Value};
use repositories::repository_registry::Repositories;

let ds = default()?;
let items = ds.as_ref().items();
let mut created = Row::new();
created.insert("name".into(), Value::from("Ada"));
let mut created = items.create(&mut created).await?;
let loaded = items.read_single(None, None, None).await?;
created.insert("name".into(), Value::from("Ada Lovelace"));
let updated: Response = items.update(&created, None).await?;
let deleted: Response = items.delete(&created, None).await?;
```

##### TypeScript

```typescript
import { Datasource } from "@datasources/core";
import "./generated/typescript/repositories/repository-registry.ts";

const ds = Datasource.default as Datasource;

const created = await ds.items.create({ id: 0, name: "Ada" });
const loaded = await ds.items.readSingle();
created.name = "Ada Lovelace";
const updated = await ds.items.update(created);
const deleted = await ds.items.delete(created);
```

See [Datasource](docs/sections/datasource.md#create-using-dynamic-after-register).

### Running a simple read command

Start with `createCommand` and `executeReader`. The connection must be open.

##### C#

```csharp
var ds = Datasource.Default;
var connection = await ds.CreateConnection();
await connection.Open();

var command = await ds.CreateCommand(connection);
var reader = await command.ExecuteReader(
    "SELECT id, name FROM item WHERE id = $id",
    [new Parameter("id", 1)]);
while (await reader.Next())
    Console.WriteLine(reader.Current);
```

##### Rust

```rust
use datasources::{default, Parameter};

let ds = default()?;
let connection = ds.create_connection().await?;
connection.open().await?;

let command = ds.create_command(Some(connection)).await?;
let reader = command
    .execute_reader("SELECT id, name FROM item WHERE id = $id", Some(&[Parameter::new("id", 1)]))
    .await?;
while reader.next().await? {
    println!("{:?}", reader.current());
}
```

##### TypeScript

```typescript
import { Datasource } from "@datasources/core";

const ds = Datasource.default;
const connection = await ds.createConnection();
await connection.open();

const command = await ds.createCommand(connection);
const reader = await command.executeReader("SELECT id, name FROM item WHERE id = $id", [
  { name: "id", value: 1 },
]);
while (await reader.next()) console.log(reader.current);
```

See [Command](docs/sections/command.md) for middleware and `execute`.

### Create and using an Enriched Repository

A `CRUDRepository` fills related fields after `read` once enrichments are registered. When the related repository is also a CRUD repository, nested `List` children are written on create / update / delete. A readonly repository only fills.

##### C#

```csharp
var contacts = new CRUDRepository<Contact>(connection);
contacts.AddEnriched(new("contact_type_name", "contact_type_id", "id", "name"), types);
contacts.AddEnriched(new("addresses", "id", "contact_id", Type: EnrichmentPropertyType.List), addresses);

var loaded = await contacts.ReadSingle(new Where(new Operator.Equals("id", 10)));
await contacts.Create(new Contact { first_name = "Ada", addresses = [new Address { line1 = "1 Main" }] });
```

##### Rust

```rust
let mut contacts = CRUDRepository::from_type::<Contact>(connection)?;
contacts
    .add_enriched(Enrichment::new("contact_type_name", "contact_type_id", "id", Some("name".into()), EnrichmentPropertyType::Object), types, None)
    .add_enriched(Enrichment::new("addresses", "id", "contact_id", None, EnrichmentPropertyType::List), addresses, None);

let loaded = contacts.read_single(Some(&Where::new(Operator::equals("id", 10))), None).await?;
contacts.create(contact).await?;
```

##### TypeScript

```typescript
const contacts = new CRUDRepository(Contact, connection);
contacts.addEnriched({ destinationProperty: "contact_type_name", sourceProperty: "contact_type_id", relatedKey: "id", relatedProperty: "name" }, types);
contacts.addEnriched({ destinationProperty: "addresses", sourceProperty: "id", relatedKey: "contact_id", type: "list" }, addresses);

const loaded = await contacts.readSingle(new Where(new Operator.Equals("id", 10)));
await contacts.create({ first_name: "Ada", addresses: [{ line1: "1 Main" }] });
```

See [Repositories](docs/sections/repository.md) for readonly, CRUD, and bulk.

### Mappers

`TransformMapper` reshapes an already-converted object. Repository read is convert, then map.

##### C#

```csharp
var mapper = new TransformMapper<Nested, Flat>(
    n => new Flat { UserId = n.User.Id, UserName = n.User.Name },
    f => new Nested { User = new User { Id = f.UserId, Name = f.UserName } });
var flat = mapper.Map(nested);
var back = mapper.ReverseMap(flat);
```

##### Rust

```rust
let mapper = TransformMapper::new(
    |n: Nested| Flat { user_id: n.user.id, user_name: n.user.name },
    |f: Flat| Nested { user: User { id: f.user_id, name: f.user_name } },
);
let flat = mapper.map(nested);
let back = mapper.reverse_map(flat);
```

##### TypeScript

```typescript
const mapper = new TransformMapper<Nested, Flat>(
  (n) => ({ userId: n.user.id, userName: n.user.name }),
  (f) => ({ user: { id: f.userId, name: f.userName } }),
);
const flat = mapper.map(nested);
const back = mapper.reverseMap(flat);
```

See [Mappers](docs/sections/mappers.md) for `TransformMapper`, `FlatToNestedMapper`, and [DictionaryMapper](docs/sections/dictionary-mapper.md).

### Type Converters

`TypeConverter` is only `Convert` / `convert` / `convert` and `ReverseConvert` / `reverseConvert` / `reverse_convert`. Attach field converters on `MetaDataRowTypeConverter` when the native provider cannot map the type.

##### C#

```csharp
sealed class FlagConverter : TypeConverter<Value, Value>
{
    public override Value Convert(Value from) => from.ToObject() is long n && n == 1;
    public override Value ReverseConvert(Value to) => to;
}

var rows = new MetaDataRowTypeConverter<Item>(fields: new Dictionary<string, TypeConverter<Value, Value>>
{
    ["flag"] = new FlagConverter()
});
var item = rows.Convert(row);
```

##### Rust

```rust
struct FlagConverter;
impl TypeConverter<Value, Value> for FlagConverter {
    fn convert(&self, from: &Value) -> Result<Value> {
        Ok(match from {
            Value::Int(n) => Value::Bool(*n == 1),
            other => other.clone(),
        })
    }
    fn reverse_convert(&self, to: &Value) -> Result<Value> {
        Ok(to.clone())
    }
}

let rows = MetaDataRowTypeConverter::new(vec!["flag".into()]).with_fields(HashMap::from([(
    "flag".into(),
    Arc::new(FlagConverter) as Arc<dyn TypeConverter<Value, Value>>,
)]));
let item = rows.convert(&row)?;
```

##### TypeScript

```typescript
class FlagConverter extends TypeConverter {
  convert = (value: Cell): Cell => value === 1;
  reverseConvert = (value: Cell): Cell => value;
}

const rows = new MetaDataRowTypeConverter({ flag: new FlagConverter() });
const item = rows.convert(row);
```

See [Type Converters](docs/sections/type-converters.md).

## Using the skill

## Example prompts

### 1. Run setup and add providers

<SkillPrompt text="Run setup and add sqlite and postgres for csharp and rust" />

**Expected**

```sh
npx @datasources/cli setup --yes --ecosystem csharp
npx @datasources/cli add sqlite postgres --ecosystem csharp
npx @datasources/cli add sqlite postgres --ecosystem rust
```

### 2. Generate a user repository

<SkillPrompt text="Generate a user with field id: username, is_enabled in types.yaml and generate repositories for csharp" />

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

### 3. Generate users and roles

**Prompt:** The same `user` table, plus `role` and a `user_role` junction tagged `many_to_many`, with a `roles` list on `user`. Generate TypeScript and create a user that has a role.

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

### Run native SQL

**Prompt:** Insert an item and read it back with a command and SQL text. Do not use a repository.

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
    "INSERT INTO items (id, name) VALUES ($id, $name)",
    [new Parameter("id", 1), new Parameter("name", "a")]);
var converter = new MetaDataRowTypeConverter<Item>();
await using var createdReader = await command.ExecuteReader(
    "SELECT id, name FROM items WHERE id = $id",
    [new Parameter("id", 1)]);
var created = await createdReader.Read().Select(row => converter.Convert(row)).ToListAsync();
```

### Map a flat row into nested contacts

**Prompt:** Read users with a command, convert each row, then build a `contacts` list from `contact_name1` and `contact_name2`.

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

### Select a contact and its type

**Prompt:** Write a `SelectStatement` for `Contact.FirstName` and `ContactType.Name`, inner-joining `Contact` to `ContactType` on `Contact.ContactTypeId` = `ContactType.Id`.

**Expected**

```csharp
new SelectStatement(new()
{
    Table = "Contact",
    Fields = ["Contact.FirstName AS FirstName", "ContactType.Name AS ContactTypeName"],
    Joins = [Join.Inner("ContactType", "Contact.ContactTypeId", "ContactType.Id")],
});
```

### Map a legacy table and a unique key

**Prompt:** The type stays `user`, but the stored table is `OldUsersTbl`. `username` is unique and stored as `LoginNm`, `id` as `UsrID`, and `is_enabled` as `ActvFlg`. Add `datasource.yaml` mappings and generate.

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

## Features

Parity across C#, Rust, and TypeScript. Standard modules first; opinionated repositories above them.

### Feature stack

| Feature | C# | Rust | TypeScript |
| --- | :---: | :---: | :---: |
| [Repository Generation](#repository-generation) | ✅ | ✅ | ✅ |
| [Datasource Factory](docs/sections/datasource-factory.md) | ✅ | ✅ | ✅ |
| [Datasource](docs/sections/datasource.md) | ✅ | ✅ | ✅ |
| [Mappers](docs/sections/mappers.md) | ✅ | ✅ | ✅ |
| [Type Converters](docs/sections/type-converters.md) | ✅ | ✅ | ✅ |
| [Connection](docs/sections/connection.md) | ✅ | ✅ | ✅ |
| [Transaction](docs/sections/transaction.md) | ✅ | ✅ | ✅ |
| [Command](docs/sections/command.md) | ✅ | ✅ | ✅ |
| [Reader](docs/sections/reader.md) | ✅ | ✅ | ✅ |
| [Statements](docs/sections/statements.md) | ✅ | ✅ | ✅ |
| [Repositories](docs/sections/repository.md) | ✅ | ✅ | ✅ |
| [Readonly Repository](docs/sections/readonly-repository.md) | ✅ | ✅ | ✅ |
| [CRUD Repository](docs/sections/crud-repository.md) | ✅ | ✅ | ✅ |
| [Bulk Repository](docs/sections/bulk-repository.md) | ✅ | ✅ | ✅ |
| [Repository Factories](docs/sections/repository-factories.md) | ✅ | ✅ | ✅ |

## Repository Generation

`generate all` reads a Types Specification and writes the types, the repositories, and SQL for one language. Import the generated registry so `Datasource.Default` / `Datasource.default` / `default()` binds each plural name.

See [Generate](docs/cli.md#generate) for flags and output paths.

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
npx @datasources/cli generate all --dialect csharp
```

##### Rust

```rust
npx @datasources/cli generate all --dialect rust
```

##### TypeScript

```typescript
npx @datasources/cli generate all --dialect typescript
```

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
#[path = "../generated/rust/repositories/mod.rs"]
mod repositories;

use datasources::{default, Response, Row, Value};
use repositories::repository_registry::Repositories;

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
import { Datasource } from "@datasources/core";
import "./generated/typescript/repositories/repository-registry.ts";

const ds = Datasource.default as Datasource;

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
npx @datasources/cli generate all --dialect csharp
```

##### Rust

```rust
npx @datasources/cli generate all --dialect rust
```

##### TypeScript

```typescript
npx @datasources/cli generate all --dialect typescript
```

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
#[path = "../generated/rust/repositories/mod.rs"]
mod repositories;

use datasources::{default, encode_rows, Response, Row, Value};
use repositories::repository_registry::Repositories;

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
import { Datasource } from "@datasources/core";
import "./generated/typescript/repositories/repository-registry.ts";

const ds = Datasource.default as Datasource;

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
npx @datasources/cli generate all --dialect csharp
```

##### Rust

```rust
npx @datasources/cli generate all --dialect rust
```

##### TypeScript

```typescript
npx @datasources/cli generate all --dialect typescript
```

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
#[path = "../generated/rust/repositories/mod.rs"]
mod repositories;

use datasources::{default, Response, Row, Value};
use repositories::repository_registry::Repositories;

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
import { Datasource } from "@datasources/core";
import "./generated/typescript/repositories/repository-registry.ts";

const ds = Datasource.default as Datasource;

const created = await ds.users.create({ id: 0, username: "ada", isEnabled: true });
const loaded = await ds.users.readSingle();
created.username = "ada-v2";
const updated = await ds.users.update(created);
const deleted = await ds.users.delete(created);
```

<!-- Generated from docs/DriverSummary.yaml. Edit that file and run npm run docs:drivers. -->

## Languages and Datasource Providers {#languages}

Pick a language, then a driver. Each driver section lists `Information` / `information` / `information`, Overrides, and `RowTypeConverters` / `rowTypeConverters` / `row_type_converters` from that language's `IDriver` / `IDriver` / `Driver`.

<div class="driver-header">
  <a class="driver-tile" href="#csharp">
    <img src="/languages/csharp.svg" alt="" width="48" height="48" />
    <span>C#</span>
  </a>
  <a class="driver-tile" href="#rust">
    <img src="/languages/rust.svg" alt="" width="48" height="48" />
    <span>Rust</span>
  </a>
  <a class="driver-tile" href="#typescript">
    <img src="/languages/typescript.svg" alt="" width="48" height="48" />
    <span>TypeScript</span>
  </a>
</div>

### C# {#csharp}

<a href="#languages" class="driver-back">← Languages and Datasource Providers</a>

Drivers for C#. Click a tile for Information, Overrides, and RowTypeConverters from that `IDriver`.

<div class="driver-header">
  <a class="driver-tile" href="#csharp-sqlite">
    <img src="/drivers/sqlite.svg" alt="" width="48" height="48" />
    <span>SQLite</span>
  </a>
  <a class="driver-tile" href="#csharp-mysql">
    <img src="/drivers/mysql.svg" alt="" width="48" height="48" />
    <span>MySQL</span>
  </a>
  <a class="driver-tile" href="#csharp-postgres">
    <img src="/drivers/postgres.svg" alt="" width="48" height="48" />
    <span>PostgreSQL</span>
  </a>
</div>

#### SQLite {#csharp-sqlite}

<a href="#csharp" class="driver-back">← C# drivers</a>

##### Information

- Embedded SQL database for local development and tests.
- Full connection, command, transaction, and repository support.

##### Overrides

- Overrides SelectInsertedIdStatement to SELECT last_insert_rowid() AS {column}
- Overrides CreateTableStatement type mapping (int→INTEGER, bool→BOOLEAN, uuid/string/datetime→TEXT, decimal→NUMERIC, float→REAL, binary→BLOB)

##### RowTypeConverters

- `bool`
- `datetime`
- `uuid`
- `decimal`
- `binary`

##### ChangeLogs

_None._

#### MySQL {#csharp-mysql}

<a href="#csharp" class="driver-back">← C# drivers</a>

##### Information

- MySQL / MariaDB server driver.
- Full connection, command, transaction, and repository support.

##### Overrides

- Overrides SelectInsertedIdStatement to SELECT LAST_INSERT_ID() AS {column}
- Overrides CreateTableStatement identity to AUTO_INCREMENT
- Overrides CreateTableStatement type mapping (int→INT, uuid→CHAR(36), string/datetime→TEXT, bool→TINYINT, decimal→DECIMAL(18,4), float→DOUBLE, binary→BLOB)

##### RowTypeConverters

- `bool`
- `datetime`
- `uuid`
- `decimal`
- `binary`

##### ChangeLogs

_None._

#### PostgreSQL {#csharp-postgres}

<a href="#csharp" class="driver-back">← C# drivers</a>

##### Information

- PostgreSQL server driver.
- Full connection, command, transaction, and repository support.

##### Overrides

- Overrides SelectInsertedIdStatement to SELECT lastval() AS {column}
- Overrides CreateTableStatement identity to GENERATED BY DEFAULT AS IDENTITY
- Overrides CreateTableStatement boolean defaults (0/1 → TRUE/FALSE)
- Overrides CreateTableStatement type mapping (int→INTEGER, uuid→UUID, string→TEXT, bool→BOOLEAN, decimal→NUMERIC, float→DOUBLE PRECISION, datetime→TIMESTAMPTZ, binary→BYTEA)

##### RowTypeConverters

- `bool`
- `datetime`
- `uuid`
- `decimal`
- `binary`

##### ChangeLogs

_None._

### Rust {#rust}

<a href="#languages" class="driver-back">← Languages and Datasource Providers</a>

Drivers for Rust. Click a tile for information, Overrides, and row_type_converters from that `Driver`.

<div class="driver-header">
  <a class="driver-tile" href="#rust-sqlite">
    <img src="/drivers/sqlite.svg" alt="" width="48" height="48" />
    <span>SQLite</span>
  </a>
  <a class="driver-tile" href="#rust-mysql">
    <img src="/drivers/mysql.svg" alt="" width="48" height="48" />
    <span>MySQL</span>
  </a>
  <a class="driver-tile" href="#rust-postgres">
    <img src="/drivers/postgres.svg" alt="" width="48" height="48" />
    <span>PostgreSQL</span>
  </a>
</div>

#### SQLite {#rust-sqlite}

<a href="#rust" class="driver-back">← Rust drivers</a>

##### information

- Embedded SQL database for local development and tests.
- Full connection, command, transaction, and repository support.

##### Overrides

- Overrides SelectInsertedIdStatement to SELECT last_insert_rowid() AS {column}
- Overrides CreateTableStatement type mapping (int→INTEGER, bool→BOOLEAN, uuid/string/datetime→TEXT, decimal→NUMERIC, float→REAL, binary→BLOB)

##### row_type_converters

- `bool`
- `datetime`
- `uuid`
- `decimal`
- `binary`

##### change_logs

_None._

#### MySQL {#rust-mysql}

<a href="#rust" class="driver-back">← Rust drivers</a>

##### information

- MySQL / MariaDB server driver.
- Full connection, command, transaction, and repository support.

##### Overrides

- Overrides SelectInsertedIdStatement to SELECT LAST_INSERT_ID() AS {column}
- Overrides CreateTableStatement identity to AUTO_INCREMENT
- Overrides CreateTableStatement type mapping (int→INT, uuid→CHAR(36), string/datetime→TEXT, bool→TINYINT, decimal→DECIMAL(18,4), float→DOUBLE, binary→BLOB)

##### row_type_converters

- `bool`
- `datetime`
- `uuid`
- `decimal`
- `binary`

##### change_logs

_None._

#### PostgreSQL {#rust-postgres}

<a href="#rust" class="driver-back">← Rust drivers</a>

##### information

- PostgreSQL server driver.
- Full connection, command, transaction, and repository support.

##### Overrides

- Overrides SelectInsertedIdStatement to SELECT lastval() AS {column}
- Overrides CreateTableStatement identity to GENERATED BY DEFAULT AS IDENTITY
- Overrides CreateTableStatement boolean defaults (0/1 → TRUE/FALSE)
- Overrides CreateTableStatement type mapping (int→INTEGER, uuid→UUID, string→TEXT, bool→BOOLEAN, decimal→NUMERIC, float→DOUBLE PRECISION, datetime→TIMESTAMPTZ, binary→BYTEA)

##### row_type_converters

- `bool`
- `datetime`
- `uuid`
- `decimal`
- `binary`

##### change_logs

_None._

### TypeScript {#typescript}

<a href="#languages" class="driver-back">← Languages and Datasource Providers</a>

Drivers for TypeScript. Click a tile for information, Overrides, and rowTypeConverters from that `IDriver`.

<div class="driver-header">
  <a class="driver-tile" href="#typescript-sqlite">
    <img src="/drivers/sqlite.svg" alt="" width="48" height="48" />
    <span>SQLite</span>
  </a>
  <a class="driver-tile" href="#typescript-mysql">
    <img src="/drivers/mysql.svg" alt="" width="48" height="48" />
    <span>MySQL</span>
  </a>
  <a class="driver-tile" href="#typescript-postgres">
    <img src="/drivers/postgres.svg" alt="" width="48" height="48" />
    <span>PostgreSQL</span>
  </a>
</div>

#### SQLite {#typescript-sqlite}

<a href="#typescript" class="driver-back">← TypeScript drivers</a>

##### information

- Embedded SQL database for local development and tests.
- Full connection, command, transaction, and repository support.

##### Overrides

- Overrides SelectInsertedIdStatement to SELECT last_insert_rowid() AS {column}
- Overrides CreateTableStatement type mapping (int→INTEGER, bool→BOOLEAN, uuid/string/datetime→TEXT, decimal→NUMERIC, float→REAL, binary→BLOB)

##### rowTypeConverters

- `bool`
- `datetime`
- `uuid`
- `decimal`
- `binary`

##### changeLogs

_None._

#### MySQL {#typescript-mysql}

<a href="#typescript" class="driver-back">← TypeScript drivers</a>

##### information

- MySQL / MariaDB server driver.
- Full connection, command, transaction, and repository support.

##### Overrides

- Overrides SelectInsertedIdStatement to SELECT LAST_INSERT_ID() AS {column}
- Overrides CreateTableStatement identity to AUTO_INCREMENT
- Overrides CreateTableStatement type mapping (int→INT, uuid→CHAR(36), string/datetime→TEXT, bool→TINYINT, decimal→DECIMAL(18,4), float→DOUBLE, binary→BLOB)

##### rowTypeConverters

- `bool`
- `datetime`
- `uuid`
- `decimal`
- `binary`

##### changeLogs

_None._

#### PostgreSQL {#typescript-postgres}

<a href="#typescript" class="driver-back">← TypeScript drivers</a>

##### information

- PostgreSQL server driver.
- Full connection, command, transaction, and repository support.

##### Overrides

- Overrides SelectInsertedIdStatement to SELECT lastval() AS {column}
- Overrides CreateTableStatement identity to GENERATED BY DEFAULT AS IDENTITY
- Overrides CreateTableStatement boolean defaults (0/1 → TRUE/FALSE)
- Overrides CreateTableStatement type mapping (int→INTEGER, uuid→UUID, string→TEXT, bool→BOOLEAN, decimal→NUMERIC, float→DOUBLE PRECISION, datetime→TIMESTAMPTZ, binary→BYTEA)

##### rowTypeConverters

- `bool`
- `datetime`
- `uuid`
- `decimal`
- `binary`

##### changeLogs

_None._

## Documentation

- [Examples](docs/examples.md)

### Articles

- [The Mapping and Converting Pipelines](docs/tutorials/mapping-converting-pipelines.md)

### Code Reference

- [Datasource](docs/sections/datasource.md)
- [Connection](docs/sections/connection.md)
- [Transaction](docs/sections/transaction.md)
- [Command](docs/sections/command.md)
- [Reader](docs/sections/reader.md)
- [Repositories](docs/sections/repository.md)
  - [Readonly Repository](docs/sections/readonly-repository.md)
  - [CRUD Repository](docs/sections/crud-repository.md)
  - [Bulk Repository](docs/sections/bulk-repository.md)
  - [Repository Factories](docs/sections/repository-factories.md)
- [Mappers](docs/sections/mappers.md)
  - [Identity Mapper](docs/sections/identity-mapper.md)
  - [Dictionary Mapper](docs/sections/dictionary-mapper.md)
  - [Pluralizer Mapper](docs/sections/pluralizer-mapper.md)
  - [Change Casing Mapper](docs/sections/change-casing-mapper.md)
  - [Transform Mapper](docs/sections/transform-mapper.md)
  - [Flat To Nested Mapper](docs/sections/flat-to-nested-mapper.md)
- [Type Converters](docs/sections/type-converters.md)
  - [Metadata Row Type Converter](docs/sections/metadata-row-type-converter.md)
- [Statements](docs/sections/statements.md)
  - [Select Statement](docs/sections/select-statement.md)
  - [Insert Statement](docs/sections/insert-statement.md)
  - [Update Statement](docs/sections/update-statement.md)
  - [Delete Statement](docs/sections/delete-statement.md)
  - [Create Table Statement](docs/sections/create-table-statement.md)
  - [Create Index Statement](docs/sections/create-index-statement.md)
  - [Execute Stored Procedure Statement](docs/sections/execute-stored-procedure-statement.md)
  - [Return Statement](docs/sections/return-statement.md)
  - [Where](docs/sections/where-clause.md)
  - [Expressions](docs/sections/expressions.md)
  - [Expression Evaluator](docs/sections/expression-evaluator.md)
  - [Statement Evaluator](docs/sections/statement-evaluator.md)
- [Other](docs/sections/other.md)
  - [Datasource Factory](docs/sections/datasource-factory.md)
  - [Datasource URL](docs/sections/datasource-url.md)
- [Advanced](docs/sections/advanced.md)
  - [Enrichment](docs/sections/advanced.md#enrichment)
    - [IEnriched](docs/sections/i-enriched.md)
    - [Enrichment](docs/sections/enrichment.md)
    - [IReadonlyEnrichmentStrategy](docs/sections/i-readonly-enrichment-strategy.md)
    - [DefaultReadonlyEnrichmentStrategy](docs/sections/default-readonly-enrichment-strategy.md)
    - [HashsetOptimizedReadonlyEnrichmentStrategy](docs/sections/hashset-optimized-readonly-enrichment-strategy.md)
    - [NestedToObjectEnrichmentStrategy](docs/sections/nested-to-object-enrichment-strategy.md)
    - [IWriteEnrichmentStrategy](docs/sections/i-write-enrichment-strategy.md)
    - [DefaultWriteEnrichmentStrategy](docs/sections/default-write-enrichment-strategy.md)
    - [LazyWriteEnrichmentStrategy](docs/sections/lazy-write-enrichment-strategy.md)
    - [CreateEnrichmentStrategy](docs/sections/create-enrichment-strategy.md)
    - [DefaultCreateEnrichmentStrategy](docs/sections/default-create-enrichment-strategy.md)
    - [UpdateEnrichmentStrategy](docs/sections/update-enrichment-strategy.md)
    - [DefaultUpdateEnrichmentStrategy](docs/sections/default-update-enrichment-strategy.md)
    - [CascadeDeleteUpdateEnrichmentStrategy](docs/sections/cascade-delete-update-enrichment-strategy.md)
    - [DeleteEnrichmentStrategy](docs/sections/delete-enrichment-strategy.md)
    - [CascadeDeleteEnrichmentStrategy](docs/sections/cascade-delete-enrichment-strategy.md)
  - [Bulk](docs/sections/advanced.md#bulk)
    - [IBulkWriteStrategy](docs/sections/i-bulk-write-strategy.md)
    - [SingleCommandIBulkWriteStrategy](docs/sections/single-command-i-bulk-write-strategy.md)
    - [BatchCommandIBulkWriteStrategy](docs/sections/batch-command-i-bulk-write-strategy.md)
    - [StoredProcedureBulkWriteStrategy](docs/sections/stored-procedure-bulk-write-strategy.md)

### Command Line (CLI)

- [Setup](docs/cli.md#setup)
- [Add](docs/cli.md#add)
- [Remove](docs/cli.md#remove)
- [.env](docs/cli.md#env)

### Writing Providers

- [Driver](docs/writing-providers.md)

### Advanced

- [Testing Skills](docs/skill-queries.md)
