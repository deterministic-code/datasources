# Repository factories

Register a repository, then create by that type. Registering CRUD also registers readonly. Registering readonly-only makes CRUD `create` throw.

## Classes

::: reference
### typescript
#### IRepositoryFactory

```typescript
interface IRepositoryFactory
```

**Methods**

##### register(repository)

Registers a readonly repository by [type](./repository.md#irepository-type).

```typescript
register(repository: IRepository): void
```

- `repository` — [IRepository](./repository.md#irepository) to look up by `type`

##### create(type)

Creates a readonly repository.

```typescript
create<T>(type: Constructable<T>): IReadonlyRepository<T>
```

- `type` — item constructor

**Returns** [IReadonlyRepository](./readonly-repository.md)`<T>`

#### ICRUDRepositoryFactory

```typescript
interface ICRUDRepositoryFactory
```

**Methods**

##### register(repository)

Registers a CRUD repository by [type](./repository.md#irepository-type).

```typescript
register(repository: IRepository): void
```

- `repository` — [IRepository](./repository.md#irepository) to look up by `type`

##### create(type)

Creates a CRUD repository.

```typescript
create<T>(type: Constructable<T>): ICRUDRepository<T>
```

- `type` — item constructor

**Returns** [ICRUDRepository](./crud-repository.md)`<T>`

#### RepositoryFactory

```typescript
class RepositoryFactory implements ICRUDRepositoryFactory
```

**Properties**

##### asReadonly

Readonly factory view. `register` / `create` call [IRepositoryFactory](#irepositoryfactory).

```typescript
readonly asReadonly: IRepositoryFactory
```

**Methods**

##### register(repository)

Registers CRUD and readonly for the same repository.

```typescript
register(repository: IRepository): void
```

- `repository` — [IRepository](./repository.md#irepository) to look up by `type`

##### create(type)

Creates a CRUD repository.

```typescript
create<T>(type: Constructable<T>): ICRUDRepository<T>
```

- `type` — item constructor

**Returns** [ICRUDRepository](./crud-repository.md)`<T>`

##### register(typeName, pluralName, create, createReadonly)

Registers CRUD and readonly factories by entity name and plural name.

```typescript
register(typeName: string, pluralName: string, create: (connection: IConnection) => ICRUDRepository, createReadonly: (connection: IConnection) => IReadonlyRepository): void
```

- `typeName` — entity name (`Contact`)
- `pluralName` — plural name (`contacts`)
- `create` — builds the CRUD repository from a [IConnection](./connection.md)
- `createReadonly` — builds the readonly repository from a [IConnection](./connection.md)

##### create(name, connection)

Creates a registered CRUD repository by entity or plural name.

```typescript
create(name: string, connection: IConnection): ICRUDRepository
```

- `name` — entity name or plural name
- `connection` — [IConnection](./connection.md) passed to the factory

**Returns** [ICRUDRepository](./crud-repository.md)

##### createReadonly(name, connection)

Creates a registered read-only repository by entity or plural name.

```typescript
createReadonly(name: string, connection: IConnection): IReadonlyRepository
```

- `name` — entity name or plural name
- `connection` — [IConnection](./connection.md) passed to the factory

**Returns** [IReadonlyRepository](./readonly-repository.md)

##### access(name, connection)

Returns a [RepositoryAccess](./datasource.md#repositoryaccess) for a registered name.

```typescript
access(name: string, connection: IConnection): RepositoryAccess
```

- `name` — entity name or plural name
- `connection` — [IConnection](./connection.md) used by `create` / `createReadonly`

**Returns** [RepositoryAccess](./datasource.md#repositoryaccess)

#### RepositoryNotRegisteredError

```typescript
class RepositoryNotRegisteredError extends Error
```

**Constructor**

##### RepositoryNotRegisteredError(entityTypeName)

Thrown when [create](#repositoryfactory-create-type) cannot find a repository.

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
#### IRepositoryFactory

```csharp
public interface IRepositoryFactory
```

**Methods**

##### Register(repository)

Registers a readonly repository by [Type](./repository.md#irepository-type).

```csharp
void Register(IRepository repository)
```

- `repository` — [IRepository](./repository.md#irepository) to look up by `Type`

##### Create()

Creates a readonly repository by `T`.

```csharp
IReadonlyRepository<T> Create<T>()
```

- `connection` — optional open [IConnection](./connection.md)

**Returns** [IReadonlyRepository](./readonly-repository.md)`<T>`

#### ICRUDRepositoryFactory

```csharp
public interface ICRUDRepositoryFactory
```

**Methods**

##### Register(repository)

Registers a CRUD repository by [Type](./repository.md#irepository-type).

```csharp
void Register(IRepository repository)
```

- `repository` — [IRepository](./repository.md#irepository) to look up by `Type`

##### Create()

Creates a CRUD repository by `T`.

```csharp
ICRUDRepository<T> Create<T>()
```

**Returns** [ICRUDRepository](./crud-repository.md)`<T>`

#### RepositoryFactory

```csharp
public sealed class RepositoryFactory : IRepositoryFactory, ICRUDRepositoryFactory
```

**Methods**

##### Register(repository)

Registers CRUD and readonly for the same repository.

```csharp
public void Register(IRepository repository)
```

- `repository` — [IRepository](./repository.md#irepository) to look up by `Type`

##### Create()

Creates a CRUD repository by `T`.

```csharp
public ICRUDRepository<T> Create<T>()
```

**Returns** [ICRUDRepository](./crud-repository.md)`<T>`

##### Register(typeName, pluralName, create, createReadonly)

Registers CRUD and readonly factories by entity name and plural name.

```csharp
public void Register(string typeName, string pluralName, Func<IConnection, object> create, Func<IConnection, object> createReadonly)
```

- `typeName` — entity name (`Contact`)
- `pluralName` — plural name (`Contacts`)
- `create` — builds the CRUD repository from an [IConnection](./connection.md)
- `createReadonly` — builds the readonly repository from an [IConnection](./connection.md)

##### Create(name, connection)

Creates a registered CRUD repository by entity or plural name.

```csharp
public TRepository Create<TRepository>(string name, IConnection connection) where TRepository : class
```

- `name` — entity name or plural name
- `connection` — [IConnection](./connection.md) passed to the factory

**Returns** `TRepository`

##### CreateReadonly(name, connection)

Creates a registered read-only repository by entity or plural name.

```csharp
public TRepository CreateReadonly<TRepository>(string name, IConnection connection) where TRepository : class
```

- `name` — entity name or plural name
- `connection` — [IConnection](./connection.md) passed to the factory

**Returns** `TRepository`

##### Access(name, connection)

Returns a [RepositoryAccess](./datasource.md#repositoryaccess) for a registered name.

```csharp
public RepositoryAccess Access(string name, IConnection connection)
```

- `name` — entity name or plural name
- `connection` — [IConnection](./connection.md) used by `Create` / `CreateReadonly`

**Returns** [RepositoryAccess](./datasource.md#repositoryaccess)

#### RepositoryNotRegisteredException

```csharp
public sealed class RepositoryNotRegisteredException : DatasourceException
```

**Constructor**

##### RepositoryNotRegisteredException(entityTypeName)

Thrown when [Create](#repositoryfactory-create) cannot find a repository.

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
#### ReadonlyRepositoryFactory

```rust
pub trait ReadonlyRepositoryFactory
```

**Methods**

##### register(repository)

Registers a readonly repository by [entity_type](./repository.md#irepository-entity_type).

```rust
fn register(&self, repository: Arc<dyn IReadonlyRepository>)
```

- `repository` — [IReadonlyRepository](./readonly-repository.md) to look up by `entity_type`

##### create()

Creates a readonly repository by `T`.

```rust
fn create<T: 'static>(&self) -> Result<Arc<dyn IReadonlyRepository>>
```

**Returns** `Result<Arc<dyn `[IReadonlyRepository](./readonly-repository.md)`>>`

#### CRUDRepositoryFactory

```rust
pub trait CRUDRepositoryFactory
```

**Methods**

##### register(repository)

Registers a CRUD repository by [entity_type](./repository.md#irepository-entity_type).

```rust
fn register(&self, repository: Arc<dyn ICRUDRepository>)
```

- `repository` — [ICRUDRepository](./crud-repository.md) to look up by `entity_type`

##### create()

Creates a CRUD repository by `T`.

```rust
fn create<T: 'static>(&self) -> Result<Arc<dyn ICRUDRepository>>
```

**Returns** `Result<Arc<dyn `[ICRUDRepository](./crud-repository.md)`>>`

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

##### register(repository)

Registers CRUD and readonly for the same repository.

```rust
pub fn register(&self, repository: Arc<dyn ICRUDRepository>)
```

- `repository` — [ICRUDRepository](./crud-repository.md) to look up by `entity_type`

##### create()

Creates a CRUD repository by `T`.

```rust
pub fn create<T: 'static>(&self) -> Result<Arc<dyn ICRUDRepository>>
```

**Returns** `Result<Arc<dyn `[ICRUDRepository](./crud-repository.md)`>>`

##### register_named(type_name, plural_name, create, create_readonly)

Registers CRUD and readonly factories by entity name and plural name.

```rust
pub fn register_named(&self, type_name: impl Into<String>, plural_name: impl Into<String>, create: impl Fn(Arc<dyn Connection>) -> Arc<dyn ICRUDRepository> + Send + Sync + 'static, create_readonly: impl Fn(Arc<dyn Connection>) -> Arc<dyn IReadonlyRepository> + Send + Sync + 'static)
```

- `type_name` — entity name (`Contact`)
- `plural_name` — plural name (`contacts`)
- `create` — builds the CRUD repository from a [Connection](./connection.md)
- `create_readonly` — builds the readonly repository from a [Connection](./connection.md)

##### create_named(name, connection)

Creates a registered CRUD repository by entity or plural name.

```rust
pub fn create_named(&self, name: &str, connection: Arc<dyn Connection>) -> Result<Arc<dyn ICRUDRepository>>
```

- `name` — entity name or plural name
- `connection` — [Connection](./connection.md) passed to the factory

**Returns** `Result<Arc<dyn `[ICRUDRepository](./crud-repository.md)`>>`

##### create_readonly_named(name, connection)

Creates a registered read-only repository by entity or plural name.

```rust
pub fn create_readonly_named(&self, name: &str, connection: Arc<dyn Connection>) -> Result<Arc<dyn IReadonlyRepository>>
```

- `name` — entity name or plural name
- `connection` — [Connection](./connection.md) passed to the factory

**Returns** `Result<Arc<dyn `[IReadonlyRepository](./readonly-repository.md)`>>`

##### access(name, connection)

Returns a [RepositoryAccess](./datasource.md#repositoryaccess) for a registered name.

```rust
pub fn access(&self, name: &str, connection: Arc<dyn Connection>) -> RepositoryAccess
```

- `name` — entity name or plural name
- `connection` — [Connection](./connection.md) used by `create` / `create_readonly`

**Returns** [RepositoryAccess](./datasource.md#repositoryaccess)
:::

## Remarks

::: prose
### typescript
A missing repository throws [RepositoryNotRegisteredError](#repositorynotregisterederror).
### csharp
A missing repository throws [RepositoryNotRegisteredException](#repositorynotregisteredexception).
### rust
A missing repository throws `DatasourceError`.
:::

See [Datasource](./datasource.md) for {{CreateRepository|createRepository|create_repository}}.

## Examples

### Register and create

::: example
```csharp csharp/Datasources.Tests/RepositoryFactoryTests.cs#RegistersAndCreates
```
```rust rust/datasources/src/repository_factory.rs#registers_and_creates
```
```typescript typescript/datasources/src/RepositoryFactory.test.ts#registers-and-creates
```
:::

## See also

- [Repositories](./repository.md)
- [Readonly repository](./readonly-repository.md)
- [CRUD repository](./crud-repository.md)
