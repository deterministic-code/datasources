# Enrichment

One related fill. `destinationProperty` is the parent property that gets filled, `sourceProperty` is the parent key, and `relatedKey` is the key on the related row. `relatedProperty` copies one related property. `type` is [EnrichmentPropertyType](#enrichmentpropertytype). `association` names the junction columns for a link.

## Classes

::: reference
### typescript
#### Enrichment

```typescript
type Enrichment
```

One related fill.

**Properties**

##### destinationProperty

Parent property that gets filled.

```typescript
destinationProperty: string
```

##### sourceProperty

Parent key.

```typescript
sourceProperty: string
```

##### relatedKey

Key on the related row.

```typescript
relatedKey: string
```

##### relatedProperty

Optional related property to copy. A string or int stays on `Object`.

```typescript
relatedProperty?: string
```

##### type

[EnrichmentPropertyType](#enrichmentpropertytype). Omitted means `Object`.

```typescript
type?: EnrichmentPropertyType
```

##### association

Junction columns for a link. See [Association](#association).

```typescript
association?: Association
```

#### EnrichmentPropertyType

```typescript
const EnrichmentPropertyType
```

Shape of the filled property.

**Properties**

##### Object

One related row. `relatedProperty` copies one field.

```typescript
Object: "object"
```

##### List

Related rows. Written with the parent when the repository can persist.

```typescript
List: "list"
```

##### Dictionary

Rejected until its fill is defined.

```typescript
Dictionary: "dictionary"
```

##### Custom

Rejected until its fill is defined.

```typescript
Custom: "custom"
```

#### Association

```typescript
type Association
```

Parent and related columns on a junction table.

**Properties**

##### parent

Junction column that stores the parent key.

```typescript
parent: string
```

##### related

Junction column that stores the related key.

```typescript
related: string
```

### csharp
#### Enrichment

```csharp
public sealed record Enrichment
```

One related fill.

**Constructor**

##### Enrichment(destinationProperty, sourceProperty, relatedKey, relatedProperty, type, association)

Builds one related fill.

```csharp
public Enrichment(string destinationProperty, string sourceProperty, string relatedKey, string? relatedProperty = null, EnrichmentPropertyType type = EnrichmentPropertyType.Object, Association? association = null)
```

- `destinationProperty` — parent property that gets filled
- `sourceProperty` — parent key
- `relatedKey` — key on the related row
- `relatedProperty` — optional related property to copy
- `type` — [EnrichmentPropertyType](#enrichmentpropertytype)
- `association` — optional [Association](#association)

**Returns** [Enrichment](#enrichment)

**Properties**

##### DestinationProperty

Parent property that gets filled.

```csharp
string DestinationProperty { get; }
```

##### SourceProperty

Parent key.

```csharp
string SourceProperty { get; }
```

##### RelatedKey

Key on the related row.

```csharp
string RelatedKey { get; }
```

##### RelatedProperty

Optional related property to copy. A string or int stays on `Object`.

```csharp
string? RelatedProperty { get; }
```

##### Type

[EnrichmentPropertyType](#enrichmentpropertytype).

```csharp
EnrichmentPropertyType Type { get; }
```

##### Association

Junction columns for a link. See [Association](#association).

```csharp
Association? Association { get; }
```

#### EnrichmentPropertyType

```csharp
public enum EnrichmentPropertyType
```

Shape of the filled property.

**Properties**

##### Object

One related row. `RelatedProperty` copies one field.

```csharp
Object
```

##### List

Related rows. Written with the parent when the repository can persist.

```csharp
List
```

##### Dictionary

Rejected until its fill is defined.

```csharp
Dictionary
```

##### Custom

Rejected until its fill is defined.

```csharp
Custom
```

#### Association

```csharp
public sealed record Association
```

Parent and related columns on a junction table.

**Constructor**

##### Association(parent, related)

Builds junction column names.

```csharp
public Association(string parent, string related)
```

- `parent` — junction column that stores the parent key
- `related` — junction column that stores the related key

**Returns** [Association](#association)

**Properties**

##### Parent

Junction column that stores the parent key.

```csharp
string Parent { get; }
```

##### Related

Junction column that stores the related key.

```csharp
string Related { get; }
```

### rust
#### Enrichment

```rust
pub struct Enrichment
```

One related fill.

**Constructor**

##### new(destination_property, source_property, related_key, related_property, type)

Builds an enrichment. `association` starts unset.

```rust
pub fn new(destination_property: impl Into<String>, source_property: impl Into<String>, related_key: impl Into<String>, related_property: Option<String>, r#type: EnrichmentPropertyType) -> Self
```

- `destination_property` — parent field that gets filled
- `source_property` — parent key
- `related_key` — key on the related row
- `related_property` — optional related field to copy
- `type` — [EnrichmentPropertyType](#enrichmentpropertytype)

**Returns** [Enrichment](#enrichment)

##### linked(association)

Marks the fill as a `List` link and stores the junction columns.

```rust
pub fn linked(self, association: Association) -> Self
```

- `association` — [Association](#association)

**Returns** [Enrichment](#enrichment)

**Properties**

##### destination_property

Parent field that gets filled.

```rust
pub destination_property: String
```

##### source_property

Parent key.

```rust
pub source_property: String
```

##### related_key

Key on the related row.

```rust
pub related_key: String
```

##### related_property

Optional related field to copy. A string or int stays on `Object`.

```rust
pub related_property: Option<String>
```

##### type

[EnrichmentPropertyType](#enrichmentpropertytype).

```rust
pub r#type: EnrichmentPropertyType
```

##### association

Junction columns for a link. See [Association](#association).

```rust
pub association: Option<Association>
```

#### EnrichmentPropertyType

```rust
pub enum EnrichmentPropertyType
```

Shape of the filled property.

**Properties**

##### Object

One related row. `related_property` copies one field.

```rust
Object
```

##### List

Related rows. Written with the parent when the repository can persist.

```rust
List
```

##### Dictionary

Rejected until its fill is defined.

```rust
Dictionary
```

##### Custom

Rejected until its fill is defined.

```rust
Custom
```

#### Association

```rust
pub struct Association
```

Parent and related columns on a junction table.

**Constructor**

##### new(parent, related)

Builds junction column names.

```rust
pub fn new(parent: impl Into<String>, related: impl Into<String>) -> Self
```

- `parent` — junction column that stores the parent key
- `related` — junction column that stores the related key

**Returns** [Association](#association)

**Properties**

##### parent

Junction column that stores the parent key.

```rust
pub parent: String
```

##### related

Junction column that stores the related key.

```rust
pub related: String
```
:::

## Remarks

`Object` copies `relatedProperty`, or the related row when `relatedProperty` is omitted. `List` is walked and, on a CRUD repository, written with the parent. A `List` that sets `relatedProperty` is not written. `Dictionary` and `Custom` are rejected. [AddLinked](./crud-repository.md) sets `association` and writes the junction. Pass the value to [IEnriched](./i-enriched.md).

## Examples

### Fill a lookup

::: example
```csharp csharp/Datasources.Tests/ReadonlyRepositoryTests.cs#ReadFillsLookupSelectAndLeavesMissingDefault
```
```rust rust/datasources/tests/enriched.rs#enriched_fills_lookup_select_and_leaves_missing_default
```
```typescript typescript/datasources/src/ReadonlyRepository.test.ts#fills-lookup-select-and-leaves-missing-default
```
:::
