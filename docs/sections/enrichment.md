# Enrichment

One related fill. `into` is the destination property, `from` is the parent key, and `by` is the related key. `select` copies one related property. `many` collects a list. `association` names the junction columns for a link.

## Classes

::: reference
### typescript
#### Enrichment

```typescript
type Enrichment
```

One related fill.

**Properties**

##### into

Destination property.

```typescript
into: string
```

##### from

Parent key.

```typescript
from: string
```

##### by

Related key.

```typescript
by: string
```

##### select

Optional related property to copy.

```typescript
select?: string
```

##### many

Collect a list.

```typescript
many?: boolean
```

##### association

Junction columns for a link. See [Association](#association).

```typescript
association?: Association
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

##### Enrichment(into, from, by, select, many, association)

Builds one related fill.

```csharp
public Enrichment(string into, string from, string by, string? select = null, bool many = false, Association? association = null)
```

- `into` — destination property
- `from` — parent key
- `by` — related key
- `select` — optional related property to copy
- `many` — collect a list
- `association` — optional [Association](#association)

**Returns** [Enrichment](#enrichment)

**Properties**

##### Into

Destination property.

```csharp
string Into { get; }
```

##### From

Parent key.

```csharp
string From { get; }
```

##### By

Related key.

```csharp
string By { get; }
```

##### Select

Optional related property to copy.

```csharp
string? Select { get; }
```

##### Many

Collect a list.

```csharp
bool Many { get; }
```

##### Association

Junction columns for a link. See [Association](#association).

```csharp
Association? Association { get; }
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

##### new(into, from, by, select, many)

Builds an enrichment. `association` starts unset.

```rust
pub fn new(into: impl Into<String>, from: impl Into<String>, by: impl Into<String>, select: Option<String>, many: bool) -> Self
```

- `into` — destination field
- `from` — parent key
- `by` — related key
- `select` — optional related field to copy
- `many` — collect a list

**Returns** [Enrichment](#enrichment)

##### linked(association)

Marks the fill as a many link and stores the junction columns.

```rust
pub fn linked(self, association: Association) -> Self
```

- `association` — [Association](#association)

**Returns** [Enrichment](#enrichment)

**Properties**

##### into

Destination field.

```rust
pub into: String
```

##### from

Parent key.

```rust
pub from: String
```

##### by

Related key.

```rust
pub by: String
```

##### select

Optional related field to copy.

```rust
pub select: Option<String>
```

##### many

Collect a list.

```rust
pub many: bool
```

##### association

Junction columns for a link. See [Association](#association).

```rust
pub association: Option<Association>
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

A lookup copies `select`, or the related row when `select` is omitted. A `many` list is walked. A `select` scalar is not. [AddLinked](./crud-repository.md) sets `association` and writes the junction. Pass the value to [IEnriched](./i-enriched.md).

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
