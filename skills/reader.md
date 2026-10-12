<!-- Generated from docs/. Edit the markdown in docs/ and run npm run docs:skill. -->

# Reader

Streams rows from [Command](command.md) `ExecuteReader` / `executeReader` / `execute_reader`.

## Classes

#### TypeScript

#### IReader {#typescript-ireader}

```typescript
interface IReader<T>
```

**Properties**

| Name | Description |
| --- | --- |
| current | The row from the last successful [next](#typescript-ireader-next). |

**Methods**

| Name | Description |
| --- | --- |
| [next()](#typescript-ireader-next) | Advances to the next row. |
| [read()](#typescript-ireader-read) | Reads the current result set. Skips a step whose [current](#typescript-ireader-current) is undefined. |
| [nextResult()](#typescript-ireader-nextresult) | Moves to the next result set and clears [current](#typescript-ireader-current). Returns false when there is no further set. [next](#typescript-ireader-next) then reads that set. Calling only [next](#typescript-ireader-next) leaves later sets unread. |
| [dispose()](#typescript-ireader-dispose) | Releases the reader. |
| [`Symbol.asyncDispose`](#typescript-ireader-symbol-asyncdispose) | `await using` calls [dispose](#typescript-ireader-dispose). |

##### next() {#typescript-ireader-next}

Advances to the next row.

**Definition**

```typescript
next(): Promise<boolean>
```

**Returns**

`Promise<boolean>`

##### read() {#typescript-ireader-read}

Reads the current result set. Skips a step whose [current](#typescript-ireader-current) is undefined.

**Definition**

```typescript
read(): AsyncIterable<T>
```

**Returns**

`AsyncIterable<T>`

##### nextResult() {#typescript-ireader-nextresult}

Moves to the next result set and clears [current](#typescript-ireader-current). Returns false when there is no further set. [next](#typescript-ireader-next) then reads that set. Calling only [next](#typescript-ireader-next) leaves later sets unread.

**Definition**

```typescript
nextResult(): Promise<boolean>
```

**Returns**

`Promise<boolean>`

##### dispose() {#typescript-ireader-dispose}

Releases the reader.

**Definition**

```typescript
dispose(): Promise<void>
```

**Returns**

`Promise<void>`

##### `Symbol.asyncDispose` {#typescript-ireader-symbol-asyncdispose}

`await using` calls [dispose](#typescript-ireader-dispose).

**Definition**

```typescript
[Symbol.asyncDispose](): Promise<void>
```

**Returns**

`Promise<void>`

#### C#

#### IReader {#csharp-ireader}

```csharp
public interface IReader<out T>
```

**Properties**

| Name | Description |
| --- | --- |
| Current | The row from the last successful [Next](#csharp-ireader-next). |

**Methods**

| Name | Description |
| --- | --- |
| [Next()](#csharp-ireader-next) | Advances to the next row. |
| [Read()](#csharp-ireader-read) | Reads the current result set. Skips a step whose [Current](#csharp-ireader-current) is null. `ToListAsync` materializes the sequence. |
| [NextResult()](#csharp-ireader-nextresult) | Moves to the next result set and clears [Current](#csharp-ireader-current). Returns false when there is no further set. [Next](#csharp-ireader-next) then reads that set. A reader with one set returns false. Calling only [Next](#csharp-ireader-next) leaves later sets unread. |
| [DisposeAsync()](#csharp-ireader-disposeasync) | Releases the reader. `await using` calls this. |

##### Next() {#csharp-ireader-next}

Advances to the next row.

**Definition**

```csharp
Task<bool> Next()
```

**Returns**

`Task<bool>`

##### Read() {#csharp-ireader-read}

Reads the current result set. Skips a step whose [Current](#csharp-ireader-current) is null. `ToListAsync` materializes the sequence.

**Definition**

```csharp
IAsyncEnumerable<T> Read()
```

**Returns**

`IAsyncEnumerable<T>`

##### NextResult() {#csharp-ireader-nextresult}

Moves to the next result set and clears [Current](#csharp-ireader-current). Returns false when there is no further set. [Next](#csharp-ireader-next) then reads that set. A reader with one set returns false. Calling only [Next](#csharp-ireader-next) leaves later sets unread.

**Definition**

```csharp
Task<bool> NextResult()
```

**Returns**

`Task<bool>`

##### DisposeAsync() {#csharp-ireader-disposeasync}

Releases the reader. `await using` calls this.

**Definition**

```csharp
ValueTask DisposeAsync()
```

**Returns**

`ValueTask`

#### Rust

#### Reader {#rust-reader}

```rust
pub trait Reader<T: Send = Row>: Send + Sync
```

**Methods**

| Name | Description |
| --- | --- |
| [current()](#rust-reader-current) | The row from the last successful [next](#rust-reader-next). |
| [next()](#rust-reader-next) | Advances to the next row. |
| [read()](#rust-reader-read) | Reads the current result set. Skips a step whose [current](#rust-reader-current) is `None`. `try_collect` materializes the stream. |
| [next_result()](#rust-reader-next_result) | Moves to the next result set and clears [current](#rust-reader-current). Returns false when there is no further set. [next](#rust-reader-next) then reads that set. The default returns false. Calling only [next](#rust-reader-next) leaves later sets unread. |

##### current() {#rust-reader-current}

The row from the last successful [next](#rust-reader-next).

**Definition**

```rust
fn current(&self) -> Option<T>
```

**Returns**

`Option<T>`

##### next() {#rust-reader-next}

Advances to the next row.

**Definition**

```rust
async fn next(&self) -> Result<bool>
```

**Returns**

`Result<bool>`

##### read() {#rust-reader-read}

Reads the current result set. Skips a step whose [current](#rust-reader-current) is `None`. `try_collect` materializes the stream.

**Definition**

```rust
fn read(&self) -> Pin<Box<dyn Stream<Item = Result<T>> + Send + '_>>
```

**Returns**

`Pin<Box<dyn Stream<Item = Result<T>> + Send + '_>>`

##### next_result() {#rust-reader-next_result}

Moves to the next result set and clears [current](#rust-reader-current). Returns false when there is no further set. [next](#rust-reader-next) then reads that set. The default returns false. Calling only [next](#rust-reader-next) leaves later sets unread.

**Definition**

```rust
async fn next_result(&self) -> Result<bool>
```

**Returns**

`Result<bool>`

## Remarks

`ExecuteReader` / `executeReader` / `execute_reader` returns a reader for the first result set. `Next` / `next` / `next` reads that set. `NextResult` / `nextResult` / `next_result` advances to the next set. Rows stay unread until `Next` / `next` / `next`.

Dispose the reader with `await using`. Rust drops it.

## Examples

### Read a row

#### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/SqliteCommandIntegrationTests.cs#L72-L90)

```csharp
var ds = Datasource.Default;
var command = await ds.CreateCommand();
await command.Execute(Native.N("INSERT INTO items (id, name) VALUES (1, 'ada')"));
await command.Execute(Native.N("INSERT INTO items (id, name) VALUES (2, 'grace')"));
await command.Execute(Native.N("INSERT INTO items (id, name) VALUES (3, 'bea')"));
await using var reader = await command.ExecuteReader("SELECT id, name FROM items");
var rows = await reader.Read().ToListAsync();
```

**Rows**

```
[
  { id: 1, name: 'ada' },
  { id: 2, name: 'grace' },
  { id: 3, name: 'bea' }
]
```

#### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/command.rs#L77-L107)

```rust
let ds = default().unwrap();
let command = ds.create_command(None).await.unwrap();
command
    .execute_native(&Native::new("INSERT INTO items (id, name) VALUES (1, 'ada')"), None)
    .await
    .unwrap();
command
    .execute_native(&Native::new("INSERT INTO items (id, name) VALUES (2, 'grace')"), None)
    .await
    .unwrap();
command
    .execute_native(&Native::new("INSERT INTO items (id, name) VALUES (3, 'bea')"), None)
    .await
    .unwrap();
let reader = command
    .execute_reader("SELECT id, name FROM items", None)
    .await
    .unwrap();
let rows: Vec<_> = reader.read().try_collect().await.unwrap();
```

**Rows**

```
[
  { id: 1, name: 'ada' },
  { id: 2, name: 'grace' },
  { id: 3, name: 'bea' }
]
```

#### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/sqlite.integration.test.ts#L91-L105)

```typescript
const ds = Datasource.default;
const command = await ds.createCommand();
await command.executeNative(new Native("INSERT INTO items (id, name) VALUES (1, 'ada')"));
await command.executeNative(new Native("INSERT INTO items (id, name) VALUES (2, 'grace')"));
await command.executeNative(new Native("INSERT INTO items (id, name) VALUES (3, 'bea')"));
await using reader = await command.executeReader("SELECT id, name FROM items");
const rows = await Array.fromAsync(reader.read());
```

**Rows**

```
[
  { id: 1, name: 'ada' },
  { id: 2, name: 'grace' },
  { id: 3, name: 'bea' }
]
```

Schema: [items](docs/examples.md#items)

```sql
CREATE TABLE items (id int, name string, CONSTRAINT pk_items PRIMARY KEY (id))
```

### Read and apply case mappings

#### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/SqliteCommandIntegrationTests.cs#L92-L105)

```csharp
var ds = Datasource.Default;
var command = await ds.CreateCommand();
await command.Execute(Native.N("INSERT INTO contacts (user_id, first_name) VALUES (1, 'Ada')"));
await using var reader = await command.ExecuteReader("SELECT user_id, first_name FROM contacts");
var casing = new ChangeCasingMapper(Casing.Pascal, Casing.Snake);
var mapped = await reader.Read().Select(source =>
    new Row(source.ToDictionary(cell => casing.ReverseMap(cell.Key), cell => cell.Value))).ToListAsync();
```

**Rows**

```
[
  { UserId: 1, FirstName: 'Ada' }
]
```

#### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/command.rs#L109-L140)

```rust
let ds = default().unwrap();
let command = ds.create_command(None).await.unwrap();
command
    .execute_native(
        &Native::new("INSERT INTO contacts (user_id, first_name) VALUES (1, 'Ada')"),
        None,
    )
    .await
    .unwrap();
let reader = command
    .execute_reader("SELECT user_id, first_name FROM contacts", None)
    .await
    .unwrap();
let casing = ChangeCasingMapper::new(Casing::Pascal, Casing::Snake);
let mapped: Vec<Row> = reader
    .read()
    .map_ok(|source| {
        source
            .into_iter()
            .map(|(key, value)| (Mapper::reverse_map(&casing, key), value))
            .collect()
    })
    .try_collect()
    .await
    .unwrap();
```

**Rows**

```
[
  { UserId: 1, FirstName: 'Ada' }
]
```

#### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/sqlite.integration.test.ts#L107-L118)

```typescript
const ds = Datasource.default;
const command = await ds.createCommand();
await command.executeNative(new Native("INSERT INTO contacts (user_id, first_name) VALUES (1, 'Ada')"));
await using reader = await command.executeReader("SELECT user_id, first_name FROM contacts");
const casing = new ChangeCasingMapper("pascalCase", "snakeCase");
const mapped = await Array.fromAsync(reader.read(), (source) =>
  Object.fromEntries(Object.entries(source).map(([key, value]) => [casing.reverseMap(key), value])),
);
```

**Rows**

```
[
  { UserId: 1, FirstName: 'Ada' }
]
```

Schema: [case-contacts](docs/examples.md#case-contacts)

```sql
CREATE TABLE contacts (user_id int, first_name string, CONSTRAINT pk_contacts PRIMARY KEY (user_id))
```

### ExecuteReader, then apply flat to nested mapper.

#### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/SqliteCommandIntegrationTests.cs#L107-L136)

```csharp
var ds = Datasource.Default;
var command = await ds.CreateCommand();
await command.Execute(Native.N("INSERT INTO users (id, username, contact_name) VALUES (1, 'bobby', 'bob')"));
await command.Execute(Native.N("INSERT INTO users (id, username, contact_name) VALUES (1, 'bobby', 'roberta')"));
await using var reader = await command.ExecuteReader("SELECT id, username, contact_name FROM users");
var rows = await reader.Read().ToListAsync();
var mapper = new FlatToNestedMapper(new TypeMap
{
    Key = "id",
    Fields = new Dictionary<string, TypeMapField>
    {
        ["id"] = "id",
        ["username"] = "username",
        ["contacts"] = new TypeMapGroup
        {
            Key = "contact_name",
            Fields = new Dictionary<string, TypeMapField> { ["name"] = "contact_name" },
        },
    },
});
var nested = mapper.Map(rows);
```

**Contact**

```
{
  id: 1,
  username: 'bobby',
  contacts: [
    { name: 'bob' },
    { name: 'roberta' }
  ]
}
```

#### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/command.rs#L142-L195)

```rust
let ds = default().unwrap();
let command = ds.create_command(None).await.unwrap();
command
    .execute_native(
        &Native::new("INSERT INTO users (id, username, contact_name) VALUES (1, 'bobby', 'bob')"),
        None,
    )
    .await
    .unwrap();
command
    .execute_native(
        &Native::new("INSERT INTO users (id, username, contact_name) VALUES (1, 'bobby', 'roberta')"),
        None,
    )
    .await
    .unwrap();
let reader = command
    .execute_reader("SELECT id, username, contact_name FROM users", None)
    .await
    .unwrap();
let rows: Vec<_> = reader.read().try_collect().await.unwrap();
let mapper = FlatToNestedMapper::new(TypeMap::new(
    "id",
    vec![
        ("id".into(), TypeMapField::column("id")),
        ("username".into(), TypeMapField::column("username")),
        (
            "contacts".into(),
            TypeMapField::Group(TypeMapGroup::new(
                "contact_name",
                vec![("name".into(), TypeMapField::column("contact_name"))],
            )),
        ),
    ],
));
let nested = Mapper::map(&mapper, rows);
```

**Contact**

```
{
  id: 1,
  username: 'bobby',
  contacts: [
    { name: 'bob' },
    { name: 'roberta' }
  ]
}
```

#### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/sqlite.integration.test.ts#L120-L144)

```typescript
const ds = Datasource.default;
const command = await ds.createCommand();
await command.executeNative(new Native("INSERT INTO users (id, username, contact_name) VALUES (1, 'bobby', 'bob')"));
await command.executeNative(new Native("INSERT INTO users (id, username, contact_name) VALUES (1, 'bobby', 'roberta')"));
await using reader = await command.executeReader("SELECT id, username, contact_name FROM users");
const rows = await Array.fromAsync(reader.read());
const mapper = new FlatToNestedMapper<{
  id: number;
  username: string;
  contacts: { name: string }[];
}>({
  key: "id",
  fields: {
    id: "id",
    username: "username",
    contacts: { key: "contact_name", fields: { name: "contact_name" } },
  },
});
const nested = mapper.map(rows);
```

**Contact**

```
{
  id: 1,
  username: 'bobby',
  contacts: [
    { name: 'bob' },
    { name: 'roberta' }
  ]
}
```

Schema: [flat-users](docs/examples.md#flat-users)

```sql
CREATE TABLE users (id int, username string, contact_name string)
```

### ExecuteReader maps flat contacts

#### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/SqliteCommandIntegrationTests.cs#L29-L42) [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/SqliteCommandIntegrationTests.cs#L196-L229)

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
await command.Execute(Native.N(
    "INSERT INTO users (id, username, email, contact_name1, contact_name2) VALUES (1, 'bobby', 'bob@asdf.com', 'bob', 'roberta')"));
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

**User**

```
{
  id: 1,
  username: 'bobby',
  email: 'bob@asdf.com',
  contacts: [
    { name: 'bob' },
    { name: 'roberta' }
  ]
}
```

#### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/command.rs#L26-L28) [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/command.rs#L30-L37) [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/command.rs#L324-L419)

```rust
struct Contact {
    name: String,
}

struct User {
    id: i64,
    username: String,
    email: String,
    contact_name1: String,
    contact_name2: String,
    contacts: Vec<Contact>,
}

let ds = default().unwrap();
let command = ds.create_command(None).await.unwrap();
command
    .execute_native(
        &Native::new("INSERT INTO users (id, username, email, contact_name1, contact_name2)
         VALUES (1, 'bobby', 'bob@asdf.com', 'bob', 'roberta')"),
        None,
    )
    .await
    .unwrap();
let reader = command
    .execute_reader(
        "SELECT contact_name1, contact_name2, email, id, username FROM users",
        None,
    )
    .await
    .unwrap();
let converter = RowTypeConverter::of::<User>();
fn user_contacts_mapper() -> TransformMapper<Row, User> {
    fn text_of(row: &Row, key: &str) -> String {
        match row.get(key) {
            Some(Value::Text(text)) => text.clone(),
            _ => String::new(),
        }
    }
    TransformMapper::new(
        |row| {
            let contact_name1 = text_of(&row, "contact_name1");
            let contact_name2 = text_of(&row, "contact_name2");
            User {
                id: match row.get("id") {
                    Some(Value::Int(id)) => *id,
                    _ => 0,
                },
                username: text_of(&row, "username"),
                email: text_of(&row, "email"),
                contacts: vec![
                    Contact {
                        name: contact_name1.clone(),
                    },
                    Contact {
                        name: contact_name2.clone(),
                    },
                ],
                contact_name1,
                contact_name2,
            }
        },
        |mut user| {
            user.contact_name1 = user
                .contacts
                .first()
                .map(|contact| contact.name.clone())
                .unwrap_or_default();
            user.contact_name2 = user
                .contacts
                .get(1)
                .map(|contact| contact.name.clone())
                .unwrap_or_default();
            let mut row = Row::new();
            row.insert("id".into(), Value::Int(user.id));
            row.insert("username".into(), Value::from(user.username));
            row.insert("email".into(), Value::from(user.email));
            row.insert("contact_name1".into(), Value::from(user.contact_name1));
            row.insert("contact_name2".into(), Value::from(user.contact_name2));
            row
        },
    )
}
let mapper = user_contacts_mapper();
let users: Vec<_> = reader
    .read()
    .map_ok(|row| mapper.map(converter.convert(&row).unwrap()))
    .try_collect()
    .await
    .unwrap();
```

**User**

```
{
  id: 1,
  username: 'bobby',
  email: 'bob@asdf.com',
  contacts: [
    { name: 'bob' },
    { name: 'roberta' }
  ]
}
```

#### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/sqlite.integration.test.ts#L40-L42) [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/sqlite.integration.test.ts#L44-L51) [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/sqlite.integration.test.ts#L199-L232)

```typescript
class Contact {
  name = "";
}

class User {
  id = 0;
  username = "";
  email = "";
  contact_name1 = "";
  contact_name2 = "";
  contacts: Contact[] = [];
}

const ds = Datasource.default;
const command = await ds.createCommand();
await command.executeNative(
  new Native(
    "INSERT INTO users (id, username, email, contact_name1, contact_name2) VALUES (1, 'bobby', 'bob@asdf.com', 'bob', 'roberta')",
  ),
);
await using reader = await command.executeReader(
  "SELECT contact_name1, contact_name2, email, id, username FROM users",
);
const converter = new MetaDataRowTypeConverter<User>();
const userContactsMapper = () =>
  new TransformMapper<User, User>(
    (row) => {
      row.contacts = [
        Object.assign(new Contact(), { name: row.contact_name1 }),
        Object.assign(new Contact(), { name: row.contact_name2 }),
      ];
      return row;
    },
    (user) => {
      user.contact_name1 = user.contacts[0]?.name ?? "";
      user.contact_name2 = user.contacts[1]?.name ?? "";
      return user;
    },
  );
const mapper = userContactsMapper();
const users = await Array.fromAsync(reader.read(), (row) => mapper.map(converter.convert(row, User)));
```

**User**

```
{
  id: 1,
  username: 'bobby',
  email: 'bob@asdf.com',
  contacts: [
    { name: 'bob' },
    { name: 'roberta' }
  ]
}
```

Schema: [flat-contact-users](docs/examples.md#flat-contact-users)

```sql
CREATE TABLE users (id int, username string, email string, contact_name1 string, contact_name2 string, CONSTRAINT pk_users PRIMARY KEY (id))
```

### Read multiple results returned in a single command

#### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/SqliteCommandIntegrationTests.cs#L231-L249)

```csharp
var ds = Datasource.Default;
var command = await ds.CreateCommand();
await using var reader = await command.ExecuteReader(Native.N("SELECT 1 AS id; SELECT 2 AS id"));
var first = new List<Row>();
while (await reader.Next())
    first.Add(reader.Current!);
var moved = await reader.NextResult();
var second = new List<Row>();
while (await reader.Next())
    second.Add(reader.Current!);
var again = await reader.NextResult();
```

**First result**

```
{
  id: 1
}
```

**Second result**

```
{
  id: 2
}
```

#### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/command.rs#L421-L444)

```rust
let ds = default().unwrap();
let command = ds.create_command(None).await.unwrap();
let reader = command
    .execute_reader_native(&Native::new("SELECT 1 AS id; SELECT 2 AS id"), None)
    .await
    .unwrap();
let mut first = Vec::new();
while reader.next().await.unwrap() {
    first.push(reader.current());
}
let moved = reader.next_result().await.unwrap();
let mut second = Vec::new();
while reader.next().await.unwrap() {
    second.push(reader.current());
}
let again = reader.next_result().await.unwrap();
assert!(moved);
assert!(!again);
```

**First result**

```
{
  id: 1
}
```

**Second result**

```
{
  id: 2
}
```

#### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/sqlite.integration.test.ts#L234-L248)

```typescript
const ds = Datasource.default;
const command = await ds.createCommand();
await using reader = await command.executeReaderNative(new Native("SELECT 1 AS id; SELECT 2 AS id"));
const first: { id: number }[] = [];
while (await reader.next()) first.push(reader.current as { id: number });
const moved = await reader.nextResult();
const second: { id: number }[] = [];
while (await reader.next()) second.push(reader.current as { id: number });
const again = await reader.nextResult();
```

**First result**

```
{
  id: 1
}
```

**Second result**

```
{
  id: 2
}
```

> **Rust**
>
> There is no `using`. The reader is dropped at the end of the scope.

### Call a stored procedure that returns two results

#### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Mysql.Tests/MysqlCommandIntegrationTests.cs#L82-L104)

```csharp
await command.Execute(Native.N("""
    CREATE PROCEDURE two_ids()
    BEGIN
      SELECT 1 AS id;
      SELECT 2 AS id;
    END
    """));
await using var reader = await command.ExecuteReader(new ExecuteStoredProcedureStatement("two_ids", []));
var first = new List<Row>();
while (await reader.Next())
    first.Add(reader.Current!);
var moved = await reader.NextResult();
var second = new List<Row>();
while (await reader.Next())
    second.Add(reader.Current!);
var again = await reader.NextResult();
```

**First result**

```
{
  id: 1
}
```

**Second result**

```
{
  id: 2
}
```

#### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/mysql/tests/integration.rs#L372-L399)

```rust
async fn call_procedure_returns_two_results(command: &dyn Command) {
    use datasources::ExecuteStoredProcedureStatement;
    command
        .execute_native(
            &Native::new("CREATE PROCEDURE two_ids() BEGIN SELECT 1 AS id; SELECT 2 AS id; END"),
            None,
        )
        .await
        .unwrap();
    let reader = command
        .execute_reader_statement(&ExecuteStoredProcedureStatement::new("two_ids", &[]), None)
        .await
        .unwrap();
    let mut first = Vec::new();
    while reader.next().await.unwrap() {
        first.push(reader.current());
    }
    let moved = reader.next_result().await.unwrap();
    let mut second = Vec::new();
    while reader.next().await.unwrap() {
        second.push(reader.current());
    }
    let again = reader.next_result().await.unwrap();
```

**First result**

```
{
  id: 1
}
```

**Second result**

```
{
  id: 2
}
```

#### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/mysql/tests/mysql.integration.test.ts#L45-L64)

```typescript
async function callProcedureReturnsTwoResults(command: MysqlCommand) {
  await command.executeNative(new Native(`
    CREATE PROCEDURE two_ids()
    BEGIN
      SELECT 1 AS id;
      SELECT 2 AS id;
    END
  `));
  await using reader = await command.executeReaderStatement(new ExecuteStoredProcedureStatement("two_ids", []));
  const first: { id: number }[] = [];
  while (await reader.next()) first.push(reader.current as { id: number });
  const moved = await reader.nextResult();
  const second: { id: number }[] = [];
  while (await reader.next()) second.push(reader.current as { id: number });
  const again = await reader.nextResult();
```

**First result**

```
{
  id: 1
}
```

**Second result**

```
{
  id: 2
}
```
