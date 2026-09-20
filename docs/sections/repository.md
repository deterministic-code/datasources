# Repository

Typed item marker and instance registry. `Type` / `type` / `entity_type` is the item CLR type, constructor, or `TypeId` the repository exposes, not the table `entity` name. Register a builder, then create by entity type name or by type. The callback receives an optional connection.

Registering CRUD also registers readonly. Registering readonly-only makes CRUD `create` throw.

## Classes

::: reference
### typescript
#### IRepository

```typescript
interface IRepository<T = unknown>
```

**Properties**

##### type

Item constructor the repository exposes.

```typescript
readonly type: Constructable<T>
```

#### IRepositoryFactory

```typescript
interface IRepositoryFactory
```

**Methods**

##### register(typeOrName, factory)

Registers a readonly factory by constructor or entity type name.

```typescript
register(typeOrName: Constructable<unknown> | string, factory: (connection?: IConnection) => IReadonlyRepository): void
```

- `typeOrName` — constructor or entity type name
- `factory` — builds an [IReadonlyRepository](./readonly-repository.md) from an optional [IConnection](./connection.md)

##### create(typeOrName, connection?)

Creates a readonly repository.

```typescript
create(typeOrName: Constructable<unknown> | string, connection?: IConnection): IReadonlyRepository
```

- `typeOrName` — constructor or entity type name
- `connection` — optional open [IConnection](./connection.md)

**Returns** [IReadonlyRepository](./readonly-repository.md)

#### ICRUDRepositoryFactory

```typescript
interface ICRUDRepositoryFactory
```

**Methods**

##### register(typeOrName, factory)

Registers a CRUD factory by constructor or entity type name.

```typescript
register(typeOrName: Constructable<unknown> | string, factory: (connection?: IConnection) => ICRUDRepository): void
```

- `typeOrName` — constructor or entity type name
- `factory` — builds an [ICRUDRepository](./crud-repository.md) from an optional [IConnection](./connection.md)

##### create(typeOrName, connection?)

Creates a CRUD repository.

```typescript
create(typeOrName: Constructable<unknown> | string, connection?: IConnection): ICRUDRepository
```

- `typeOrName` — constructor or entity type name
- `connection` — optional open [IConnection](./connection.md)

**Returns** [ICRUDRepository](./crud-repository.md)

#### RepositoryFactory

```typescript
class RepositoryFactory implements ICRUDRepositoryFactory
```

**Properties**

##### asReadonly

Readonly factory view. `register` / `create` call [registerReadonly](#repositoryfactory-registerreadonly-typeorname-factory) / [createReadonly](#repositoryfactory-createreadonly-typeorname-connection).

```typescript
readonly asReadonly: IRepositoryFactory
```

**Methods**

##### register(typeOrName, factory)

Registers CRUD and readonly for the same factory.

```typescript
register(typeOrName: Constructable<unknown> | string, factory: (connection?: IConnection) => unknown): void
```

- `typeOrName` — constructor or entity type name
- `factory` — builds a repository from an optional [IConnection](./connection.md)

##### registerReadonly(typeOrName, factory)

Registers readonly only. CRUD [create](#repositoryfactory-create-typeorname-connection) throws.

```typescript
registerReadonly(typeOrName: Constructable<unknown> | string, factory: (connection?: IConnection) => unknown): void
```

- `typeOrName` — constructor or entity type name
- `factory` — builds a readonly repository from an optional [IConnection](./connection.md)

##### create(typeOrName, connection?)

Creates a CRUD repository.

```typescript
create(typeOrName: Constructable<unknown> | string, connection?: IConnection): ICRUDRepository
```

- `typeOrName` — constructor or entity type name
- `connection` — optional open [IConnection](./connection.md)

**Returns** [ICRUDRepository](./crud-repository.md)

##### createReadonly(typeOrName, connection?)

Creates a readonly repository.

```typescript
createReadonly(typeOrName: Constructable<unknown> | string, connection?: IConnection): IReadonlyRepository
```

- `typeOrName` — constructor or entity type name
- `connection` — optional open [IConnection](./connection.md)

**Returns** [IReadonlyRepository](./readonly-repository.md)

#### RepositoryNotRegisteredError

```typescript
class RepositoryNotRegisteredError extends Error
```

**Constructor**

##### RepositoryNotRegisteredError(entityTypeName)

Thrown when [create](#repositoryfactory-create-typeorname-connection) cannot find a factory.

```typescript
constructor(entityTypeName: string)
```

- `entityTypeName` — missing entity type name

**Properties**

##### entityTypeName

Missing entity type name.

```typescript
readonly entityTypeName: string
```

### csharp
#### IRepository

```csharp
public interface IRepository
```

**Properties**

##### Type

Item CLR type the repository exposes.

```csharp
Type Type { get; }
```

#### IRepositoryFactory

```csharp
public interface IRepositoryFactory
```

**Methods**

##### Register(factory)

Registers a readonly factory by `T`.

```csharp
void Register<T>(Func<IConnection?, IReadonlyRepository<T>> factory)
```

- `factory` — builds an [IReadonlyRepository](./readonly-repository.md) from an optional [IConnection](./connection.md)

##### Register(entityTypeName, factory)

Registers a readonly factory by entity type name.

```csharp
void Register(string entityTypeName, Func<IConnection?, object> factory)
```

- `entityTypeName` — entity type name
- `factory` — builds a repository from an optional [IConnection](./connection.md)

##### Register(type, factory)

Registers a readonly factory by CLR type.

```csharp
void Register(Type type, Func<IConnection?, object> factory)
```

- `type` — item CLR type
- `factory` — builds a repository from an optional [IConnection](./connection.md)

##### Create()

Creates a readonly repository by `T`.

```csharp
IReadonlyRepository<T> Create<T>(IConnection? connection = null)
```

- `connection` — optional open [IConnection](./connection.md)

**Returns** [IReadonlyRepository](./readonly-repository.md)`<T>`

##### Create(entityTypeName, connection)

Creates a readonly repository by entity type name.

```csharp
object Create(string entityTypeName, IConnection? connection = null)
```

- `entityTypeName` — entity type name
- `connection` — optional open [IConnection](./connection.md)

**Returns** `object`

##### Create(type, connection)

Creates a readonly repository by CLR type.

```csharp
object Create(Type type, IConnection? connection = null)
```

- `type` — item CLR type
- `connection` — optional open [IConnection](./connection.md)

**Returns** `object`

#### ICRUDRepositoryFactory

```csharp
public interface ICRUDRepositoryFactory
```

**Methods**

##### Register(factory)

Registers a CRUD factory by `T`.

```csharp
void Register<T>(Func<IConnection?, ICRUDRepository<T>> factory)
```

- `factory` — builds an [ICRUDRepository](./crud-repository.md) from an optional [IConnection](./connection.md)

##### Register(entityTypeName, factory)

Registers a CRUD factory by entity type name.

```csharp
void Register(string entityTypeName, Func<IConnection?, object> factory)
```

- `entityTypeName` — entity type name
- `factory` — builds a repository from an optional [IConnection](./connection.md)

##### Register(type, factory)

Registers a CRUD factory by CLR type.

```csharp
void Register(Type type, Func<IConnection?, object> factory)
```

- `type` — item CLR type
- `factory` — builds a repository from an optional [IConnection](./connection.md)

##### Create()

Creates a CRUD repository by `T`.

```csharp
ICRUDRepository<T> Create<T>(IConnection? connection = null)
```

- `connection` — optional open [IConnection](./connection.md)

**Returns** [ICRUDRepository](./crud-repository.md)`<T>`

##### Create(entityTypeName, connection)

Creates a CRUD repository by entity type name.

```csharp
object Create(string entityTypeName, IConnection? connection = null)
```

- `entityTypeName` — entity type name
- `connection` — optional open [IConnection](./connection.md)

**Returns** `object`

##### Create(type, connection)

Creates a CRUD repository by CLR type.

```csharp
object Create(Type type, IConnection? connection = null)
```

- `type` — item CLR type
- `connection` — optional open [IConnection](./connection.md)

**Returns** `object`

#### RepositoryFactory

```csharp
public sealed class RepositoryFactory : IRepositoryFactory, ICRUDRepositoryFactory
```

**Methods**

##### Register(factory)

Registers CRUD and readonly for `T`.

```csharp
public void Register<T>(Func<IConnection?, ICRUDRepository<T>> factory)
```

- `factory` — builds an [ICRUDRepository](./crud-repository.md) from an optional [IConnection](./connection.md)

##### Register(entityTypeName, factory)

Registers CRUD and readonly by entity type name.

```csharp
public void Register(string entityTypeName, Func<IConnection?, object> factory)
```

- `entityTypeName` — entity type name
- `factory` — builds a repository from an optional [IConnection](./connection.md)

##### Register(type, factory)

Registers CRUD and readonly by CLR type.

```csharp
public void Register(Type type, Func<IConnection?, object> factory)
```

- `type` — item CLR type
- `factory` — builds a repository from an optional [IConnection](./connection.md)

##### Create()

Creates a CRUD repository by `T`.

```csharp
public ICRUDRepository<T> Create<T>(IConnection? connection = null)
```

- `connection` — optional open [IConnection](./connection.md)

**Returns** [ICRUDRepository](./crud-repository.md)`<T>`

##### Create(entityTypeName, connection)

Creates a readonly repository by entity type name. Use [ICRUDRepositoryFactory](#icrudrepositoryfactory) for the CRUD map.

```csharp
public object Create(string entityTypeName, IConnection? connection = null)
```

- `entityTypeName` — entity type name
- `connection` — optional open [IConnection](./connection.md)

**Returns** `object`

##### Create(type, connection)

Creates a readonly repository by CLR type. Use [ICRUDRepositoryFactory](#icrudrepositoryfactory) for the CRUD map.

```csharp
public object Create(Type type, IConnection? connection = null)
```

- `type` — item CLR type
- `connection` — optional open [IConnection](./connection.md)

**Returns** `object`

#### RepositoryNotRegisteredException

```csharp
public sealed class RepositoryNotRegisteredException : DatasourceException
```

**Constructor**

##### RepositoryNotRegisteredException(entityTypeName)

Thrown when [Create](#repositoryfactory-create) cannot find a factory.

```csharp
public RepositoryNotRegisteredException(string entityTypeName)
```

- `entityTypeName` — missing entity type name

**Properties**

##### EntityTypeName

Missing entity type name.

```csharp
string EntityTypeName { get; }
```

### rust
#### IRepository

```rust
pub trait IRepository
```

**Methods**

##### entity_type()

Item `TypeId` the repository exposes. Defaults to `Row`.

```rust
fn entity_type(&self) -> TypeId
```

**Returns** `TypeId`

#### datasources

```rust
pub fn as_readonly(repo: Arc<dyn CRUDRepository>) -> Arc<dyn ReadonlyRepository>
```

**Methods**

##### as_readonly(repo)

Views a [CRUDRepository](./crud-repository.md) as a [ReadonlyRepository](./readonly-repository.md).

```rust
pub fn as_readonly(repo: Arc<dyn CRUDRepository>) -> Arc<dyn ReadonlyRepository>
```

- `repo` — [CRUDRepository](./crud-repository.md)

**Returns** `Arc<dyn `[ReadonlyRepository](./readonly-repository.md)`>`

#### ReadonlyRepositoryFactory

```rust
pub trait ReadonlyRepositoryFactory
```

**Methods**

##### register(name, factory)

Registers a readonly factory by entity type name.

```rust
fn register(&self, name: &str, factory: Arc<dyn Fn(Option<Arc<dyn Connection>>) -> Arc<dyn ReadonlyRepository> + Send + Sync>)
```

- `name` — entity type name
- `factory` — builds a [ReadonlyRepository](./readonly-repository.md) from an optional [Connection](./connection.md)

##### register_type(factory)

Registers a readonly factory by `T`.

```rust
fn register_type<T: 'static>(&self, factory: Arc<dyn Fn(Option<Arc<dyn Connection>>) -> Arc<dyn ReadonlyRepository> + Send + Sync>)
```

- `factory` — builds a [ReadonlyRepository](./readonly-repository.md) from an optional [Connection](./connection.md)

##### create(name, connection)

Creates a readonly repository by entity type name.

```rust
fn create(&self, name: &str, connection: Option<Arc<dyn Connection>>) -> Result<Arc<dyn ReadonlyRepository>>
```

- `name` — entity type name
- `connection` — optional open [Connection](./connection.md)

**Returns** `Result<Arc<dyn `[ReadonlyRepository](./readonly-repository.md)`>>`

##### create_type(connection)

Creates a readonly repository by `T`.

```rust
fn create_type<T: 'static>(&self, connection: Option<Arc<dyn Connection>>) -> Result<Arc<dyn ReadonlyRepository>>
```

- `connection` — optional open [Connection](./connection.md)

**Returns** `Result<Arc<dyn `[ReadonlyRepository](./readonly-repository.md)`>>`

#### CRUDRepositoryFactory

```rust
pub trait CRUDRepositoryFactory
```

**Methods**

##### register(name, factory)

Registers a CRUD factory by entity type name.

```rust
fn register(&self, name: &str, factory: Arc<dyn Fn(Option<Arc<dyn Connection>>) -> Arc<dyn CRUDRepository> + Send + Sync>)
```

- `name` — entity type name
- `factory` — builds a [CRUDRepository](./crud-repository.md) from an optional [Connection](./connection.md)

##### register_type(factory)

Registers a CRUD factory by `T`.

```rust
fn register_type<T: 'static>(&self, factory: Arc<dyn Fn(Option<Arc<dyn Connection>>) -> Arc<dyn CRUDRepository> + Send + Sync>)
```

- `factory` — builds a [CRUDRepository](./crud-repository.md) from an optional [Connection](./connection.md)

##### create(name, connection)

Creates a CRUD repository by entity type name.

```rust
fn create(&self, name: &str, connection: Option<Arc<dyn Connection>>) -> Result<Arc<dyn CRUDRepository>>
```

- `name` — entity type name
- `connection` — optional open [Connection](./connection.md)

**Returns** `Result<Arc<dyn `[CRUDRepository](./crud-repository.md)`>>`

##### create_type(connection)

Creates a CRUD repository by `T`.

```rust
fn create_type<T: 'static>(&self, connection: Option<Arc<dyn Connection>>) -> Result<Arc<dyn CRUDRepository>>
```

- `connection` — optional open [Connection](./connection.md)

**Returns** `Result<Arc<dyn `[CRUDRepository](./crud-repository.md)`>>`

#### RepositoryFactory

```rust
pub struct RepositoryFactory
```

**Constructor**

##### new()

Empty registry.

```rust
pub fn new() -> Self
```

**Returns** [RepositoryFactory](#repositoryfactory)

**Methods**

##### register(name, factory)

Registers CRUD and readonly by entity type name.

```rust
pub fn register(&self, name: &str, factory: impl Fn(Option<Arc<dyn Connection>>) -> Arc<dyn CRUDRepository> + Send + Sync + 'static)
```

- `name` — entity type name
- `factory` — builds a [CRUDRepository](./crud-repository.md) from an optional [Connection](./connection.md)

##### register_type(factory)

Registers CRUD and readonly by `T`.

```rust
pub fn register_type<T: 'static>(&self, factory: impl Fn(Option<Arc<dyn Connection>>) -> Arc<dyn CRUDRepository> + Send + Sync + 'static)
```

- `factory` — builds a [CRUDRepository](./crud-repository.md) from an optional [Connection](./connection.md)

##### register_readonly(name, factory)

Registers readonly only. CRUD [create](#repositoryfactory-create-name-connection) fails.

```rust
pub fn register_readonly(&self, name: &str, factory: impl Fn(Option<Arc<dyn Connection>>) -> Arc<dyn ReadonlyRepository> + Send + Sync + 'static)
```

- `name` — entity type name
- `factory` — builds a [ReadonlyRepository](./readonly-repository.md) from an optional [Connection](./connection.md)

##### register_readonly_type(factory)

Registers readonly only by `T`.

```rust
pub fn register_readonly_type<T: 'static>(&self, factory: impl Fn(Option<Arc<dyn Connection>>) -> Arc<dyn ReadonlyRepository> + Send + Sync + 'static)
```

- `factory` — builds a [ReadonlyRepository](./readonly-repository.md) from an optional [Connection](./connection.md)

##### create(name, connection)

Creates a CRUD repository by entity type name.

```rust
pub fn create(&self, name: &str, connection: Option<Arc<dyn Connection>>) -> Result<Arc<dyn CRUDRepository>>
```

- `name` — entity type name
- `connection` — optional open [Connection](./connection.md)

**Returns** `Result<Arc<dyn `[CRUDRepository](./crud-repository.md)`>>`

##### create_type(connection)

Creates a CRUD repository by `T`.

```rust
pub fn create_type<T: 'static>(&self, connection: Option<Arc<dyn Connection>>) -> Result<Arc<dyn CRUDRepository>>
```

- `connection` — optional open [Connection](./connection.md)

**Returns** `Result<Arc<dyn `[CRUDRepository](./crud-repository.md)`>>`

##### create_readonly(name, connection)

Creates a readonly repository by entity type name.

```rust
pub fn create_readonly(&self, name: &str, connection: Option<Arc<dyn Connection>>) -> Result<Arc<dyn ReadonlyRepository>>
```

- `name` — entity type name
- `connection` — optional open [Connection](./connection.md)

**Returns** `Result<Arc<dyn `[ReadonlyRepository](./readonly-repository.md)`>>`

##### create_readonly_type(connection)

Creates a readonly repository by `T`.

```rust
pub fn create_readonly_type<T: 'static>(&self, connection: Option<Arc<dyn Connection>>) -> Result<Arc<dyn ReadonlyRepository>>
```

- `connection` — optional open [Connection](./connection.md)

**Returns** `Result<Arc<dyn `[ReadonlyRepository](./readonly-repository.md)`>>`
:::

## Remarks

SQL provider default repositories report the row or dictionary type (`Row`, `Dictionary<string, Value>`, or `Object`), not a closed entity generic.

A missing factory throws [RepositoryNotRegisteredException](#repositorynotregisteredexception) / [RepositoryNotRegisteredError](#repositorynotregisterederror) / `DatasourceError`.

See [Datasource](./datasource.md) for `CreateRepository` / `createRepository`.

## See also

- [Readonly repository](./readonly-repository.md)
- [CRUD repository](./crud-repository.md)
- [Enriched repository](./enriched-repository.md)
- [Bulk repository](./bulk-repository.md)
