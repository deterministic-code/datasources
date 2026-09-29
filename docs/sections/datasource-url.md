# Datasource URL

A connection URL split into provider, host, credentials, datasource, schema, and options. A provider may parse `url` or `database_url` with `DatasourceUrlParser`. The call is optional.

## Classes

::: reference
### typescript
#### DatasourceUrlParser

```typescript
class DatasourceUrlParser
```

**Methods**

##### parse(url?)

Parses a connection URL. A blank string or a string that is not a URL returns `undefined`.

```typescript
static parse(url?: string): DatasourceUrl | undefined
```

- `url` — connection URL, such as `postgres://ds:ds@127.0.0.1:5432/datasources?schema=public`

**Returns** [DatasourceUrl](#datasourceurl) or `undefined`

#### DatasourceUrl

```typescript
class DatasourceUrl
```

**Properties**

##### provider

URL scheme, lowercased and unchanged.

```typescript
readonly provider: string
```

##### username

User from the authority. Absent when the URL has no user.

```typescript
readonly username: string | undefined
```

##### password

Password from the authority, percent-decoded. Absent when the URL has no password. Split from the user on the first `:`.

```typescript
readonly password: string | undefined
```

##### host

Authority host. Absent when the URL has no host.

```typescript
readonly host: string | undefined
```

##### port

Authority port. Absent when the URL omits it.

```typescript
readonly port: number | undefined
```

##### path

Path after the authority, including a leading `/`. An opaque URL uses the remainder after the scheme.

```typescript
readonly path: string
```

##### datasource

`path` with one leading `/` removed. Absent when that leaves nothing.

```typescript
get datasource(): string | undefined
```

##### schema

The `schema` query parameter, percent-decoded. Absent when that parameter is missing. Not copied into `options`.

```typescript
readonly schema: string | undefined
```

##### options

The other query parameters. A flag with no `=` has an empty value. A repeated key keeps the last value.

```typescript
readonly options: Readonly<Record<string, string>>
```

### csharp
#### DatasourceUrlParser

```csharp
public static class DatasourceUrlParser
```

**Methods**

##### Parse(url)

Parses a connection URL. A blank string or a string that is not a URL returns null.

```csharp
public static DatasourceUrl? Parse(string? url)
```

- `url` — connection URL, such as `postgres://ds:ds@127.0.0.1:5432/datasources?schema=public`

**Returns** [DatasourceUrl](#datasourceurl) or null

#### DatasourceUrl

```csharp
public sealed record DatasourceUrl(string Provider, string? Username, string? Password, string? Host, int? Port, string Path, string? Schema, IReadOnlyDictionary<string, string> Options)
```

**Properties**

##### Provider

URL scheme, lowercased and unchanged.

```csharp
string Provider { get; }
```

##### Username

User from the authority. Absent when the URL has no user.

```csharp
string? Username { get; }
```

##### Password

Password from the authority, percent-decoded. Absent when the URL has no password. Split from the user on the first `:`.

```csharp
string? Password { get; }
```

##### Host

Authority host. Absent when the URL has no host.

```csharp
string? Host { get; }
```

##### Port

Authority port. Absent when the URL omits it.

```csharp
int? Port { get; }
```

##### Path

Path after the authority, including a leading `/`. An opaque URL uses the remainder after the scheme.

```csharp
string Path { get; }
```

##### Datasource

`Path` with one leading `/` removed. Absent when that leaves nothing.

```csharp
string? Datasource { get; }
```

##### Schema

The `schema` query parameter, percent-decoded. Absent when that parameter is missing. Not copied into `Options`.

```csharp
string? Schema { get; }
```

##### Options

The other query parameters. A flag with no `=` has an empty value. A repeated key keeps the last value.

```csharp
IReadOnlyDictionary<string, string> Options { get; }
```

### rust
#### DatasourceUrlParser

```rust
pub struct DatasourceUrlParser
```

**Methods**

##### parse(url)

Parses a connection URL. A blank string or a string that is not a URL returns `None`.

```rust
pub fn parse(url: &str) -> Option<DatasourceUrl>
```

- `url` — connection URL, such as `postgres://ds:ds@127.0.0.1:5432/datasources?schema=public`

**Returns** `Option<`[DatasourceUrl](#datasourceurl)`>`

#### DatasourceUrl

```rust
pub struct DatasourceUrl
```

**Properties**

##### provider

URL scheme, lowercased and unchanged.

```rust
pub provider: String
```

##### username

User from the authority. Absent when the URL has no user.

```rust
pub username: Option<String>
```

##### password

Password from the authority, percent-decoded. Absent when the URL has no password. Split from the user on the first `:`.

```rust
pub password: Option<String>
```

##### host

Authority host. Absent when the URL has no host.

```rust
pub host: Option<String>
```

##### port

Authority port. Absent when the URL omits it.

```rust
pub port: Option<i32>
```

##### path

Path after the authority, including a leading `/`. An opaque URL uses the remainder after the scheme.

```rust
pub path: String
```

##### schema

The `schema` query parameter, percent-decoded. Absent when that parameter is missing. Not copied into `options`.

```rust
pub schema: Option<String>
```

##### options

The other query parameters. A flag with no `=` has an empty value. A repeated key keeps the last value.

```rust
pub options: HashMap<String, String>
```

**Methods**

##### datasource()

`path` with one leading `/` removed. Absent when that leaves nothing.

```rust
pub fn datasource(&self) -> Option<&str>
```

**Returns** `Option<&str>`
:::

## Remarks

The provider is the scheme as written, lowercased. `postgres://` stays `postgres`. The parser does not rewrite a scheme to another name.

`sqlite:////tmp/app.db` keeps the extra slash: `path` is `//tmp/app.db` and `datasource` is `/tmp/app.db`. `sqlite://:memory:` (no third slash) is a host of `:memory:`, so `datasource` is absent. SQLite still applies its own filename default.

A raw connection string such as `Host=db;Port=1` is not a URL. `Parse` returns null and the provider can keep that string.

## Examples

### Postgres URL

::: example
```csharp csharp/Datasources.Tests/DatasourceUrlParserTests.cs#PostgresUrl
```
```rust rust/datasources/src/tests/datasource_url_parser.rs#postgres_url
```
```typescript typescript/datasources/src/DatasourceUrlParser.test.ts#postgres-url
```
:::

### SQLite file

::: example
```csharp csharp/Datasources.Tests/DatasourceUrlParserTests.cs#SqliteRelativeFile
```
```rust rust/datasources/src/tests/datasource_url_parser.rs#sqlite_relative_file
```
```typescript typescript/datasources/src/DatasourceUrlParser.test.ts#sqlite-relative-file
```
:::

### Not a URL

::: example
```csharp csharp/Datasources.Tests/DatasourceUrlParserTests.cs#NotAUrl
```
```rust rust/datasources/src/tests/datasource_url_parser.rs#not_a_url
```
```typescript typescript/datasources/src/DatasourceUrlParser.test.ts#not-a-url
```
:::
