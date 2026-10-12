<!-- Generated from docs/. Edit the markdown in docs/ and run npm run docs:skill. -->

# IEnriched

Registers a related fill on a repository. Fluent for siblings. A related repository fills its own fields. Does not write.

## Classes

#### TypeScript

#### IEnriched {#typescript-ienriched}

```typescript
interface IEnriched<T>
```

**Methods**

| Name | Description |
| --- | --- |
| [addEnriched(enrichment, via, strategy?)](#typescript-ienriched-addenriched-enrichment-via-strategy) | Adds a related fill. Fluent for siblings. A related repository fills its own fields. Does not write. |

##### addEnriched(enrichment, via, strategy?) {#typescript-ienriched-addenriched-enrichment-via-strategy}

Adds a related fill. Fluent for siblings. A related repository fills its own fields. Does not write.

**Definition**

```typescript
addEnriched<TRelated extends object>(enrichment: Enrichment, via: IReadonlyRepository<TRelated>, strategy?: IReadonlyEnrichmentStrategy<T & object>): this
```

**Parameters**

| Name | Description |
| --- | --- |
| `enrichment` | [Enrichment](#) |
| `via` | related [IReadonlyRepository](readonly-repository.md) |
| `strategy` | optional [IReadonlyEnrichmentStrategy](docs/sections/i-readonly-enrichment-strategy.md) for this edge. Omitted uses [DefaultReadonlyEnrichmentStrategy](docs/sections/default-readonly-enrichment-strategy.md) |

**Returns**

`this`

#### C#

#### IEnriched {#csharp-ienriched}

```csharp
public interface IEnriched<T>
```

**Methods**

| Name | Description |
| --- | --- |
| [AddEnriched(enrichment, via, strategy)](#csharp-ienriched-addenriched-enrichment-via-strategy) | Adds a related fill. Fluent for siblings. A related repository fills its own fields. Does not write. |

##### AddEnriched(enrichment, via, strategy) {#csharp-ienriched-addenriched-enrichment-via-strategy}

Adds a related fill. Fluent for siblings. A related repository fills its own fields. Does not write.

**Definition**

```csharp
IEnriched<T> AddEnriched<TRelated>(Enrichment enrichment, IReadonlyRepository<TRelated> via, IReadonlyEnrichmentStrategy<T>? strategy = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `enrichment` | [Enrichment](#) |
| `via` | related [IReadonlyRepository](readonly-repository.md) |
| `strategy` | optional [IReadonlyEnrichmentStrategy](docs/sections/i-readonly-enrichment-strategy.md) for this edge. Omitted uses [DefaultReadonlyEnrichmentStrategy](docs/sections/default-readonly-enrichment-strategy.md) |

**Returns**

[IEnriched](#csharp-ienriched)`<T>`

#### Rust

#### ReadonlyRepository {#rust-readonlyrepository}

```rust
pub struct ReadonlyRepository
```

**Methods**

| Name | Description |
| --- | --- |
| [add_enriched(enrichment, via, strategy)](#rust-readonlyrepository-add_enriched-enrichment-via-strategy) | Adds a related fill. A related repository fills its own fields. Does not write. |

##### add_enriched(enrichment, via, strategy) {#rust-readonlyrepository-add_enriched-enrichment-via-strategy}

Adds a related fill. A related repository fills its own fields. Does not write.

**Definition**

```rust
pub fn add_enriched(&mut self, enrichment: Enrichment, via: impl EnrichmentVia, strategy: Option<Arc<dyn ReadonlyEnrichmentStrategy>>) -> &mut Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `enrichment` | [Enrichment](#) |
| `via` | related [IReadonlyRepository](readonly-repository.md). The readonly path keeps the read repository only |
| `strategy` | optional [ReadonlyEnrichmentStrategy](docs/sections/i-readonly-enrichment-strategy.md) for this edge. Omitted uses [DefaultReadonlyEnrichmentStrategy](docs/sections/default-readonly-enrichment-strategy.md) |

**Returns**

`&mut Self`

## Remarks

Register the child repository's own fills on that repository:

```
childrenRepo.AddEnriched(new("type_name", "type_id", "id", "name"), childTypesRepo);
parentsRepo.AddEnriched(new("children", "id", "parent_id", Type: EnrichmentPropertyType.List), childrenRepo);
```

Rust has no separate trait. `AddEnriched` / `addEnriched` / `add_enriched` is a method on [ReadonlyRepository](readonly-repository.md) and [CRUDRepository](crud-repository.md). With no enrichments, `Read` / `read` / `read` stays streaming. With enrichments, the read is collected, filled, then yielded. Writes, including `List` children and associations, are [CRUDRepository](crud-repository.md).

## Examples

### Register a fill

#### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/ReadonlyRepositoryTests.cs#L72-L88)

```csharp
var person = new Row { ["name"] = "person" };
var gone = new Row { ["name"] = "gone" };
var typesConnection = await Setup(person, gone);
await typesConnection.CreateCommand().Execute("delete", [new Parameter("id", gone["id"])]);
var connection = await Setup(
    new Row { ["type_id"] = person["id"] },
    new Row { ["type_id"] = gone["id"] });
var parentsRepo = Open<Parent>(connection);
var typesRepo = Open<UserType>(typesConnection);
parentsRepo.AddEnriched(new("TypeName", "Type", "Id", "Name"), typesRepo);
var items = await parentsRepo.Read().ToListAsync();
```

#### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/tests/enriched.rs#L136-L161)

```rust
let mut seeds = [
    record(&[("name", Value::from("person"))]),
    record(&[("name", Value::from("gone"))]),
];
let types_connection = setup(&mut seeds).await;
let person = seeds[0].clone();
let gone = seeds[1].clone();
drop_row(&types_connection, &gone).await;
let mut parents = [
    record(&[("type_id", person.get("id").unwrap().clone())]),
    record(&[("type_id", gone.get("id").unwrap().clone())]),
];
let connection = setup(&mut parents).await;
let mut parents_repo = mapped(connection, "Parent");
let types_repo = Arc::new(mapped(types_connection, "UserType"));
parents_repo.add_enriched(
    Enrichment::new("TypeName", "Type", "Id", Some("Name".into()), datasources::EnrichmentPropertyType::Object),
    types_repo,
    None,
);
let items = parents_repo.read(None, None, None).await.unwrap();
```

#### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/ReadonlyRepository.test.ts#L94-L106)

```typescript
const person: Seed = { name: "person" };
const gone: Seed = { name: "gone" };
const typesConnection = await setup([person, gone]);
await remove(typesConnection, gone);
const connection = await setup([{ type_id: person.id }, { type_id: gone.id }]);
const parentsRepo = repo(Parent, connection);
const typesRepo = repo(UserType, typesConnection);
parentsRepo.addEnriched({ destinationProperty: "TypeName", sourceProperty: "Type", relatedKey: "Id", relatedProperty: "Name" }, typesRepo);
const items = await collectAsync(parentsRepo.read());
```

## Enrichment

One related fill. `destinationProperty` is the parent property that gets filled, `sourceProperty` is the parent key, and `relatedKey` is the key on the related row. `relatedProperty` copies one related property. `type` is [EnrichmentPropertyType](#enrichmentpropertytype). `association` names the junction columns for a link.

### Classes

##### TypeScript

##### Enrichment {#typescript-enrichment}

```typescript
type Enrichment
```

**Properties**

| Name | Description |
| --- | --- |
| destinationProperty | Parent property that gets filled. |
| sourceProperty | Parent key. |
| relatedKey | Key on the related row. |
| relatedProperty | Optional related property to copy. A string or int stays on `Object`. |
| type | [EnrichmentPropertyType](#typescript-enrichmentpropertytype). Omitted means `Object`. |
| association | Junction columns for a link. See [Association](#typescript-association). |

##### EnrichmentPropertyType {#typescript-enrichmentpropertytype}

```typescript
const EnrichmentPropertyType
```

**Properties**

| Name | Description |
| --- | --- |
| Object | One related row. `relatedProperty` copies one field. |
| List | Related rows. Written with the parent when the repository can persist. |
| Dictionary | Related rows keyed by `key`. Written with the parent when the repository can persist. `relatedProperty` copies one related field as the map value; otherwise remaining fields form the value. |
| Custom | Rejected until its fill is defined. |

##### Association {#typescript-association}

```typescript
type Association
```

**Properties**

| Name | Description |
| --- | --- |
| parent | Junction column that stores the parent key. |
| related | Junction column that stores the related key. |

##### C#

##### Enrichment {#csharp-enrichment}

```csharp
public sealed record Enrichment
```

**Constructor**

| Name | Description |
| --- | --- |
| [Enrichment(destinationProperty, sourceProperty, relatedKey, relatedProperty, type, association)](#csharp-enrichment-enrichment-destinationproperty-sourceproperty-relatedkey-relatedproperty-type-association) | Builds one related fill. |

**Properties**

| Name | Description |
| --- | --- |
| DestinationProperty | Parent property that gets filled. |
| SourceProperty | Parent key. |
| RelatedKey | Key on the related row. |
| RelatedProperty | Optional related property to copy. A string or int stays on `Object`. |
| Type | [EnrichmentPropertyType](#csharp-enrichmentpropertytype). |
| Association | Junction columns for a link. See [Association](#csharp-association). |

###### Enrichment(destinationProperty, sourceProperty, relatedKey, relatedProperty, type, association) {#csharp-enrichment-enrichment-destinationproperty-sourceproperty-relatedkey-relatedproperty-type-association}

Builds one related fill.

**Definition**

```csharp
public Enrichment(string destinationProperty, string sourceProperty, string relatedKey, string? relatedProperty = null, EnrichmentPropertyType type = EnrichmentPropertyType.Object, Association? association = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `destinationProperty` | parent property that gets filled |
| `sourceProperty` | parent key |
| `relatedKey` | key on the related row |
| `relatedProperty` | optional related property to copy |
| `type` | [EnrichmentPropertyType](#csharp-enrichmentpropertytype) |
| `association` | optional [Association](#csharp-association) |

**Returns**

[Enrichment](#csharp-enrichment)

##### EnrichmentPropertyType {#csharp-enrichmentpropertytype}

```csharp
public enum EnrichmentPropertyType
```

**Properties**

| Name | Description |
| --- | --- |
| Object | One related row. `RelatedProperty` copies one field. |
| List | Related rows. Written with the parent when the repository can persist. |
| Dictionary | Related rows keyed by `key`. Written with the parent when the repository can persist. `relatedProperty` copies one related field as the map value; otherwise remaining fields form the value. |
| Custom | Rejected until its fill is defined. |

##### Association {#csharp-association}

```csharp
public sealed record Association
```

**Constructor**

| Name | Description |
| --- | --- |
| [Association(parent, related)](#csharp-association-association-parent-related) | Builds junction column names. |

**Properties**

| Name | Description |
| --- | --- |
| Parent | Junction column that stores the parent key. |
| Related | Junction column that stores the related key. |

###### Association(parent, related) {#csharp-association-association-parent-related}

Builds junction column names.

**Definition**

```csharp
public Association(string parent, string related)
```

**Parameters**

| Name | Description |
| --- | --- |
| `parent` | junction column that stores the parent key |
| `related` | junction column that stores the related key |

**Returns**

[Association](#csharp-association)

##### Rust

##### Enrichment {#rust-enrichment}

```rust
pub struct Enrichment
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(destination_property, source_property, related_key, related_property, type)](#rust-enrichment-new-destination_property-source_property-related_key-related_property-type) | Builds an enrichment. `association` starts unset. |
| [linked(association)](#rust-enrichment-linked-association) | Marks the fill as a `List` link and stores the junction columns. |

**Properties**

| Name | Description |
| --- | --- |
| destination_property | Parent field that gets filled. |
| source_property | Parent key. |
| related_key | Key on the related row. |
| related_property | Optional related field to copy. A string or int stays on `Object`. |
| type | [EnrichmentPropertyType](#rust-enrichmentpropertytype). |
| association | Junction columns for a link. See [Association](#rust-association). |

###### new(destination_property, source_property, related_key, related_property, type) {#rust-enrichment-new-destination_property-source_property-related_key-related_property-type}

Builds an enrichment. `association` starts unset.

**Definition**

```rust
pub fn new(destination_property: impl Into<String>, source_property: impl Into<String>, related_key: impl Into<String>, related_property: Option<String>, r#type: EnrichmentPropertyType) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `destination_property` | parent field that gets filled |
| `source_property` | parent key |
| `related_key` | key on the related row |
| `related_property` | optional related field to copy |
| `type` | [EnrichmentPropertyType](#rust-enrichmentpropertytype) |

**Returns**

[Enrichment](#rust-enrichment)

###### linked(association) {#rust-enrichment-linked-association}

Marks the fill as a `List` link and stores the junction columns.

**Definition**

```rust
pub fn linked(self, association: Association) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `association` | [Association](#rust-association) |

**Returns**

[Enrichment](#rust-enrichment)

##### EnrichmentPropertyType {#rust-enrichmentpropertytype}

```rust
pub enum EnrichmentPropertyType
```

**Properties**

| Name | Description |
| --- | --- |
| Object | One related row. `related_property` copies one field. |
| List | Related rows. Written with the parent when the repository can persist. |
| Dictionary | Related rows keyed by `key`. Written with the parent when the repository can persist. `relatedProperty` copies one related field as the map value; otherwise remaining fields form the value. |
| Custom | Rejected until its fill is defined. |

##### Association {#rust-association}

```rust
pub struct Association
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(parent, related)](#rust-association-new-parent-related) | Builds junction column names. |

**Properties**

| Name | Description |
| --- | --- |
| parent | Junction column that stores the parent key. |
| related | Junction column that stores the related key. |

###### new(parent, related) {#rust-association-new-parent-related}

Builds junction column names.

**Definition**

```rust
pub fn new(parent: impl Into<String>, related: impl Into<String>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `parent` | junction column that stores the parent key |
| `related` | junction column that stores the related key |

**Returns**

[Association](#rust-association)

### Remarks

`Object` copies `relatedProperty`, or the related row when `relatedProperty` is omitted. `List` is walked and, on a CRUD repository, written with the parent. A `List` that sets `relatedProperty` is not written. `Dictionary` loads child rows into a map keyed by `key` and writes one row per entry. `Custom` is rejected. [AddLinked](crud-repository.md) sets `association` and writes the junction. Pass the value to [IEnriched](#).

### Examples

#### Fill a lookup

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/ReadonlyRepositoryTests.cs#L72-L88)

```csharp
var person = new Row { ["name"] = "person" };
var gone = new Row { ["name"] = "gone" };
var typesConnection = await Setup(person, gone);
await typesConnection.CreateCommand().Execute("delete", [new Parameter("id", gone["id"])]);
var connection = await Setup(
    new Row { ["type_id"] = person["id"] },
    new Row { ["type_id"] = gone["id"] });
var parentsRepo = Open<Parent>(connection);
var typesRepo = Open<UserType>(typesConnection);
parentsRepo.AddEnriched(new("TypeName", "Type", "Id", "Name"), typesRepo);
var items = await parentsRepo.Read().ToListAsync();
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/tests/enriched.rs#L151-L157)

```rust
let mut parents_repo = mapped(connection, "Parent");
let types_repo = Arc::new(mapped(types_connection, "UserType"));
parents_repo.add_enriched(
    Enrichment::new("TypeName", "Type", "Id", Some("Name".into()), datasources::EnrichmentPropertyType::Object),
    types_repo,
    None,
);
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/ReadonlyRepository.test.ts#L94-L106)

```typescript
const person: Seed = { name: "person" };
const gone: Seed = { name: "gone" };
const typesConnection = await setup([person, gone]);
await remove(typesConnection, gone);
const connection = await setup([{ type_id: person.id }, { type_id: gone.id }]);
const parentsRepo = repo(Parent, connection);
const typesRepo = repo(UserType, typesConnection);
parentsRepo.addEnriched({ destinationProperty: "TypeName", sourceProperty: "Type", relatedKey: "Id", relatedProperty: "Name" }, typesRepo);
const items = await collectAsync(parentsRepo.read());
```
