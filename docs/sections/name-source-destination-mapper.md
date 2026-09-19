# NameSourceDestinationMapper

[NameMapper](./name-mapper.md) that also holds source and destination labels. Names still pass through.

## Classes

::: reference
### typescript
#### NameSourceDestinationMapper

```typescript
class NameSourceDestinationMapper extends NameMapper implements ISourceDestinationMapper
```

**Constructor**

##### NameSourceDestinationMapper(source, destination)

Holds source and destination labels. Names still pass through.

```typescript
constructor(source: string, destination: string)
```

- `source` — source label
- `destination` — destination label

**Properties**

##### source

Dialect or provider label for the source side.

```typescript
readonly source: string
```

##### destination

Dialect or provider label for the destination side.

```typescript
readonly destination: string
```

#### ISourceDestinationMapper

```typescript
interface ISourceDestinationMapper
```

**Properties**

##### source

Dialect or provider label for the source side.

```typescript
readonly source: string
```

##### destination

Dialect or provider label for the destination side.

```typescript
readonly destination: string
```

### csharp
#### NameSourceDestinationMapper

```csharp
public sealed class NameSourceDestinationMapper : NameMapper, ISourceDestinationMapper
```

**Constructor**

##### NameSourceDestinationMapper(source, destination)

Holds source and destination labels. Names still pass through.

```csharp
public NameSourceDestinationMapper(string source, string destination)
```

- `source` — source label
- `destination` — destination label

**Properties**

##### Source

Dialect or provider label for the source side.

```csharp
string Source { get; }
```

##### Destination

Dialect or provider label for the destination side.

```csharp
string Destination { get; }
```

#### ISourceDestinationMapper

```csharp
public interface ISourceDestinationMapper
```

**Properties**

##### Source

Dialect or provider label for the source side.

```csharp
string Source { get; }
```

##### Destination

Dialect or provider label for the destination side.

```csharp
string Destination { get; }
```

### rust
#### NameSourceDestinationMapper

```rust
pub struct NameSourceDestinationMapper
```

**Constructor**

##### new(source, destination)

Holds source and destination labels. Names still pass through.

```rust
pub fn new(source: impl Into<String>, destination: impl Into<String>) -> Self
```

- `source` — source label
- `destination` — destination label

**Returns** [NameSourceDestinationMapper](#namesourcedestinationmapper)

**Properties**

##### source

Dialect or provider label for the source side.

```rust
pub source: String
```

##### destination

Dialect or provider label for the destination side.

```rust
pub destination: String
```

**Methods**

##### map(name)

Returns the name unchanged.

```rust
pub fn map(&self, name: &str) -> String
```

- `name` — column or entity name

**Returns** `String`

##### reverse_map(name)

Returns the name unchanged.

```rust
pub fn reverse_map(&self, name: &str) -> String
```

- `name` — column or entity name

**Returns** `String`

#### SourceDestinationMapper

```rust
pub trait SourceDestinationMapper
```

**Methods**

##### source()

Dialect or provider label for the source side.

```rust
fn source(&self) -> &str
```

**Returns** `&str`

##### destination()

Dialect or provider label for the destination side.

```rust
fn destination(&self) -> &str
```

**Returns** `&str`
:::

## Remarks

Labels are metadata. [NameMapper](./name-mapper.md) still returns names unchanged.
