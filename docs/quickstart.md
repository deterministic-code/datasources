# Quick Start

## Installation

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

Run interactive setup.

```sh
npx @datasources/cli setup
```

Add a new datasource provider.

```sh
npx @datasources/cli add sqlite
```

## Generate repositories from a Types Specification

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

See [Generate](./cli.md#generate).

## Dynamic registered datasource

Call the generated register, then use the plural name. {{Items|items|items()}} is the CRUD repository for `Item`. {{Create|create|create}} returns that entity. {{ReadSingle|readSingle|read_single}} returns the same entity.

::: dialects
```csharp
using Backend.Types.Datasource;

dynamic ds = Datasource.Default;
RepositoryRegistry.Register(ds.Repositories);

Item created = await ds.Items.Create(new Item { Name = "Ada" });
Item? loaded = await ds.Items.ReadSingle();
created.Name = "Ada Lovelace";
Response updated = await ds.Items.Update(created);
Response deleted = await ds.Items.Delete(created);
```
```rust
#[path = "../generated/rust/repositories/mod.rs"]
mod repositories;

use datasources::{default, Response, Row, Value};
use repositories::repository_registry::{register, Repositories};

let ds = default()?;
register(ds.repositories());
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
import { register } from "./generated/typescript/repositories/repository-registry.ts";

const ds = Datasource.default as Datasource;
register(ds.repositories);

const created = await ds.items.create({ id: 0, name: "Ada" });
const loaded = await ds.items.readSingle();
created.name = "Ada Lovelace";
const updated = await ds.items.update(created);
const deleted = await ds.items.delete(created);
```
:::

See [Datasource](./sections/datasource.md#create-using-dynamic-after-register).

## Readonly command

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

## Enriched CRUD repository

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
