# Quick Start

## Installing the skill

```sh
npx skills add deterministic-code/datasources
```

## Installation from source

::: dialects
```typescript
npm install @datasources/core
```
```csharp
dotnet add package Datasources
```
```rust
cargo add datasources
```
:::

## Setup

::: skill-prompt
Run setup and add sqlite and postgres for csharp and rust
:::

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

## Generating code from specification

::: skill-prompt
Generate all types, repositories, and SQL for csharp, rust, and typescript
:::

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

`generate all` writes the types, the repositories, and SQL for one language. It also writes SQL migrations and runs migrate-up. The default input directory is `./deterministic`. The default output directory is `./generated`. See [Migrating](#migrating).

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

See [Generate](./cli.md#generate). Walkthroughs for a User, an enriched users-and-roles spec, and a legacy table mapping are on [Repository Generation](./repository-generation.md).

## Migrating

::: skill-prompt
Create migrations and migrate up for sqlite
:::

`generate all` writes SQL migrations under `sql/<dialect>/migrations` and runs migrate-up. The connection is `PROVIDER` and `DATASOURCE_URL` from `.env`. Process env overrides the file. An in-memory `DATASOURCE_URL` skips migrate-up.

```sh
PROVIDER=sqlite
DATASOURCE_URL=sqlite:///./app.sqlite
```

```sh
npx @datasources/cli generate all --dialect typescript
```

`--no-migrations` skips writing the migration files. `--no-migrate` skips migrate-up. `--dry-run` prints the migrate-up command and leaves the migration files unwritten.

```sh
npx @datasources/cli generate all --dialect typescript --no-migrations
npx @datasources/cli generate all --dialect typescript --no-migrate
```

See [Migrations](./cli.md#migrations).

## Adding migrations manually

`migrations create` writes the same SQL files as `generate all` and leaves the database unchanged. `migrate-up` applies every pending file. `migrate-down` rolls one step back and needs the token it prints, passed as `--confirm`.

```sh
npx @datasources/cli migrations create
npx migrate-up --provider sqlite --connection ./app.sqlite
npx migrate-down --provider sqlite --connection ./app.sqlite --confirm TOKEN
```

Add a migration that generate keeps. Put an up file and a down file under `deterministic/custom/<dialect>/`. The file name is `<order>_<name>_<up|down>.sql`. `order` is 1–999. `name` starts with a letter or digit. The next `generate all` copies the pair into `sql/<dialect>/migrations` and runs migrate-up.

```text
deterministic/custom/sqlite/0001_add_notes_up.sql
deterministic/custom/sqlite/0001_add_notes_down.sql
```

Scaffold an empty pair in the migrations directory with migraters:

```sh
npx migrate-create --provider sqlite --name add_notes
```

## Using the generated code

`setup` and `generate` wire the generated registry so {{Datasource.Default|Datasource.default|default()}} binds {{Items|items|items()}}. {{Create|create|create}} returns that entity. {{ReadSingle|readSingle|read_single}} returns the same entity.

::: dialects
```csharp
using Backend.Types.Datasource;

dynamic ds = Datasource.Default;

Item created = await ds.Items.Create(new Item { Name = "Ada" });
Item? loaded = await ds.Items.ReadSingle();
created.Name = "Ada Lovelace";
Response updated = await ds.Items.Update(created);
Response deleted = await ds.Items.Delete(created);
```
```rust
use datasources::{default, Response, Row, Value};

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
```typescript
import { Datasource } from "@datasources/core";

const ds = Datasource.default;

const created = await ds.items.create({ id: 0, name: "Ada" });
const loaded = await ds.items.readSingle();
created.name = "Ada Lovelace";
const updated = await ds.items.update(created);
const deleted = await ds.items.delete(created);
```
:::

See [Datasource](./sections/datasource.md#create-using-dynamic-after-register).

## Running a simple read command

Start with `createCommand` and `executeReader`. The connection must be open.

::: dialects
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
:::

See [Command](./sections/command.md) for middleware and `execute`.

## Create and using an Enriched Repository

A `CRUDRepository` fills related fields after `read` once enrichments are registered. When the related repository is also a CRUD repository, nested `List` children are written on create / update / delete. A readonly repository only fills.

::: dialects
```csharp
var contacts = new CRUDRepository<Contact>(connection);
contacts.AddEnriched(new("contact_type_name", "contact_type_id", "id", "name"), types);
contacts.AddEnriched(new("addresses", "id", "contact_id", Type: EnrichmentPropertyType.List), addresses);

var loaded = await contacts.ReadSingle(new Where(new Operator.Equals("id", 10)));
await contacts.Create(new Contact { first_name = "Ada", addresses = [new Address { line1 = "1 Main" }] });
```
```rust
let mut contacts = CRUDRepository::from_type::<Contact>(connection)?;
contacts
    .add_enriched(Enrichment::new("contact_type_name", "contact_type_id", "id", Some("name".into()), EnrichmentPropertyType::Object), types, None)
    .add_enriched(Enrichment::new("addresses", "id", "contact_id", None, EnrichmentPropertyType::List), addresses, None);

let loaded = contacts.read_single(Some(&Where::new(Operator::equals("id", 10))), None).await?;
contacts.create(contact).await?;
```
```typescript
const contacts = new CRUDRepository(Contact, connection);
contacts.addEnriched({ destinationProperty: "contact_type_name", sourceProperty: "contact_type_id", relatedKey: "id", relatedProperty: "name" }, types);
contacts.addEnriched({ destinationProperty: "addresses", sourceProperty: "id", relatedKey: "contact_id", type: "list" }, addresses);

const loaded = await contacts.readSingle(new Where(new Operator.Equals("id", 10)));
await contacts.create({ first_name: "Ada", addresses: [{ line1: "1 Main" }] });
```
:::

See [Repositories](./sections/repository.md) for readonly, CRUD, and bulk.

## Mappers

`TransformMapper` reshapes an already-converted object. Repository read is convert, then map.

::: dialects
```csharp
var mapper = new TransformMapper<Nested, Flat>(
    n => new Flat { UserId = n.User.Id, UserName = n.User.Name },
    f => new Nested { User = new User { Id = f.UserId, Name = f.UserName } });
var flat = mapper.Map(nested);
var back = mapper.ReverseMap(flat);
```
```rust
let mapper = TransformMapper::new(
    |n: Nested| Flat { user_id: n.user.id, user_name: n.user.name },
    |f: Flat| Nested { user: User { id: f.user_id, name: f.user_name } },
);
let flat = mapper.map(nested);
let back = mapper.reverse_map(flat);
```
```typescript
const mapper = new TransformMapper<Nested, Flat>(
  (n) => ({ userId: n.user.id, userName: n.user.name }),
  (f) => ({ user: { id: f.userId, name: f.userName } }),
);
const flat = mapper.map(nested);
const back = mapper.reverseMap(flat);
```
:::

See [Mappers](./sections/mappers.md) for `TransformMapper`, `FlatToNestedMapper`, and [DictionaryMapper](./sections/dictionary-mapper.md).

## Type Converters

`TypeConverter` is only {{Convert|convert|convert}} and {{ReverseConvert|reverseConvert|reverse_convert}}. Attach field converters on `MetaDataRowTypeConverter` when the native provider cannot map the type.

::: dialects
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
```typescript
class FlagConverter extends TypeConverter {
  convert = (value: Cell): Cell => value === 1;
  reverseConvert = (value: Cell): Cell => value;
}

const rows = new MetaDataRowTypeConverter({ flag: new FlagConverter() });
const item = rows.convert(row);
```
:::

See [Type Converters](./sections/type-converters.md).
