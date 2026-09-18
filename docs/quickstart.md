# QuickStart

## Installation

::: dialects
```typescript
npm install @datasources/core
---
npm install @datasources/sqlite
npm install @datasources/mysql
npm install @datasources/postgres
npm install @datasources/memory
npm install @datasources/json
```
```csharp
dotnet add package Datasources
---
dotnet add package Datasources.Sqlite
dotnet add package Datasources.Sqlite.Microsoft
dotnet add package Datasources.Mysql
dotnet add package Datasources.Postgres
dotnet add package Datasources.Memory
dotnet add package Datasources.Json
```
```rust
cargo add datasources
---
cargo add datasources-sqlite
cargo add datasources-mysql
cargo add datasources-postgres
cargo add datasources-memory
cargo add datasources-json
```
:::

## Setup

Add dialect packages and write a local `.env` with [`@datasources/cli`](./cli.md). Core does not read `.env`; the app loads it, then registers providers and defaults.

```sh
npx @datasources/cli add sqlite
```

Pass `--ecosystem ts|rust|csharp` when the directory has more than one project type. From this repo: `npm run setup` or `npm run cli -- add sqlite`.

The optional install helper (`Datasources.Hosting` / `datasources-register` / `@datasources/register`) registers each name in `providers` / `INSTALLED_PROVIDERS`, always includes the `DATABASE_URL` scheme, then sets defaults. Connections are constructed **closed**; open on first use.

::: dialects
```csharp
var options = new Dictionary<string, string?>
{
    ["providers"] = Environment.GetEnvironmentVariable("INSTALLED_PROVIDERS"),
    ["url"] = Environment.GetEnvironmentVariable("DATABASE_URL") ?? "sqlite:///:memory:"
};
DatasourceInstall.From(options);

var datasource = Datasource.Create();
var connection = await datasource.CreateConnection();
```
```rust
use std::collections::HashMap;
use datasources::create_default;
use datasources_register::register_from;

let options = HashMap::from([
    ("providers".into(), std::env::var("INSTALLED_PROVIDERS").unwrap_or_default()),
    ("url".into(), std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:///:memory:".into())),
]);
register_from(options)?;

let datasource = create_default()?;
let connection = datasource.create_connection().await?;
```
```typescript
import { Datasource } from "@datasources/core";
import { install } from "@datasources/register";

await install({
  providers: process.env.INSTALLED_PROVIDERS,
  url: process.env.DATABASE_URL ?? "sqlite:///:memory:",
});

const datasource = Datasource.create();
const connection = await datasource.createConnection();
```
:::

Do not commit URLs with credentials. Production uses the real environment or a secrets manager.

## Readonly command

Start with `createCommand` and `executeReader`. The connection must be open.

::: dialects
```csharp
Datasource.Register(SqliteDatasource.Instance);

var ds = Datasource.Create("sqlite", new SqliteConnectionOptions { Filename = ":memory:" });
var connection = await ds.CreateConnection();
await connection.Open();

var command = ds.CreateCommand(connection);
var reader = await command.ExecuteReader(
    "SELECT id, name FROM item WHERE id = $id",
    [new Parameter("id", 1)]);
while (await reader.Next())
    Console.WriteLine(reader.Current);
```
```rust
use datasources::{create, register, Parameter};
use datasources_sqlite::SqliteDatasource;

register(SqliteDatasource::instance());

let ds = create("sqlite", None)?;
let connection = ds.create_connection().await?;
connection.open().await?;

let command = ds.create_command(Some(connection));
let reader = command
    .execute_reader("SELECT id, name FROM item WHERE id = $id", Some(&[Parameter::new("id", 1)]))
    .await?;
while reader.next().await? {
    println!("{:?}", reader.current());
}
```
```typescript
import { Datasource } from "@datasources/core";
import { SqliteDatasource } from "@datasources/sqlite";

Datasource.register(SqliteDatasource.instance);

const ds = Datasource.create("sqlite", { filename: ":memory:" });
const connection = await ds.createConnection();
await connection.open();

const command = ds.createCommand(connection);
const reader = await command.executeReader("SELECT id, name FROM item WHERE id = $id", [
  { name: "id", value: 1 },
]);
while (await reader.next()) console.log(reader.current);
```
:::

See [Command](./sections/command.md) for middleware and `execute`.

## Enriched CRUD repository

A `DeepCRUDRepository` wraps a CRUD repository, fills related fields after `get`, and writes nested `many` children on create / update / delete.

::: dialects
```csharp
var contacts = new DeepCRUDRepository<Contact>(contactRepo);
contacts.AddEnriched("contact_type_name", "contact_type_id", types, "id", "name");
contacts.AddEnriched("addresses", "id", addresses, "contact_id", many: true,
    nest: node => node.Include(addressTypes, "address_type_name", "address_type_id", "id", "name"));

var loaded = await contacts.GetSingle(WhereBuilder.Equals("id", 10));
await contacts.Create(new Contact { first_name = "Ada", addresses = [new Address { line1 = "1 Main" }] });
```
```rust
let mut contacts = DeepCRUDRepository::new(contact_repo);
contacts
    .add_enriched("contact_type_name", "contact_type_id", types, "id", Some("name".into()), false)
    .add_enriched_repo_with("addresses", "id", addresses, "contact_id", None, true, |node| {
        node.include(address_types, "address_type_name", "address_type_id", "id", Some("name".into()), false);
    });

let loaded = contacts.get_single_where(&WhereBuilder::equals("id", 10)).await?;
contacts.create(contact).await?;
```
```typescript
const contacts = new DeepCRUDRepository(contactRepo);
contacts.addEnriched("contact_type_name", "contact_type_id", types, "id", "name");
contacts.addEnriched("addresses", "id", addresses, "contact_id", undefined, true, (node) =>
  node.include(addressTypes, "address_type_name", "address_type_id", "id", "name"),
);

const loaded = await contacts.getSingle(WhereBuilder.equals("id", 10));
await contacts.create({ first_name: "Ada", addresses: [{ line1: "1 Main" }] });
```
:::

See [Repositories](./sections/repositories.md) for readonly, CRUD, and bulk.

## Mappers

`TransformMapper` reshapes an already-converted object. Repository get is convert, then map.

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

See [Mappers](./sections/mappers.md) for `NameMapper`, `FlatToTypeMapper`, and `StatementMapper`.

## Type Converter

`TypeConverter` is only `Convert` / `ReverseConvert`. Attach field converters on `RowTypeConverter` when the native provider cannot map the type.

::: dialects
```csharp
sealed class FlagConverter : TypeConverter<Value, Value>
{
    public override Value Convert(Value from) => from.ToObject() is long n && n == 1;
    public override Value ReverseConvert(Value to) => to;
}

var rows = new RowTypeConverter<Item>(fields: new Dictionary<string, TypeConverter<Value, Value>>
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

let rows = RowTypeConverter::new(vec!["flag".into()]).with_fields(HashMap::from([(
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

const rows = new RowTypeConverter({}, { flag: new FlagConverter() });
const item = rows.convert(row);
```
:::

See [TypeConverters](./sections/type-converters.md).
