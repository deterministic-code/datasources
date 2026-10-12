<!-- Generated from docs/. Edit the markdown in docs/ and run npm run docs:skill. -->

# Repository factories

Register a repository, then create by that type. Registering CRUD also registers readonly. Registering readonly-only makes CRUD `create` throw.

## Classes

#### TypeScript

#### IRepositoryFactory {#typescript-irepositoryfactory}

```typescript
interface IRepositoryFactory
```

**Methods**

| Name | Description |
| --- | --- |
| [register(repository)](#typescript-irepositoryfactory-register-repository) | Registers a readonly repository by [type](repositories.md#typescript-irepository-type). |
| [create(type)](#typescript-irepositoryfactory-create-type) | Creates a readonly repository. |

##### register(repository) {#typescript-irepositoryfactory-register-repository}

Registers a readonly repository by [type](repositories.md#typescript-irepository-type).

**Definition**

```typescript
register(repository: IRepository): void
```

**Parameters**

| Name | Description |
| --- | --- |
| `repository` | [IRepository](repositories.md#typescript-irepository) to look up by `type` |

##### create(type) {#typescript-irepositoryfactory-create-type}

Creates a readonly repository.

**Definition**

```typescript
create<T>(type: Constructable<T>): IReadonlyRepository<T>
```

**Parameters**

| Name | Description |
| --- | --- |
| `type` | item constructor |

**Returns**

[IReadonlyRepository](readonly-repository.md)`<T>`

#### ICRUDRepositoryFactory {#typescript-icrudrepositoryfactory}

```typescript
interface ICRUDRepositoryFactory
```

**Methods**

| Name | Description |
| --- | --- |
| [register(repository)](#typescript-icrudrepositoryfactory-register-repository) | Registers a CRUD repository by [type](repositories.md#typescript-irepository-type). |
| [create(type)](#typescript-icrudrepositoryfactory-create-type) | Creates a CRUD repository. |

##### register(repository) {#typescript-icrudrepositoryfactory-register-repository}

Registers a CRUD repository by [type](repositories.md#typescript-irepository-type).

**Definition**

```typescript
register(repository: IRepository): void
```

**Parameters**

| Name | Description |
| --- | --- |
| `repository` | [IRepository](repositories.md#typescript-irepository) to look up by `type` |

##### create(type) {#typescript-icrudrepositoryfactory-create-type}

Creates a CRUD repository.

**Definition**

```typescript
create<T>(type: Constructable<T>): ICRUDRepository<T>
```

**Parameters**

| Name | Description |
| --- | --- |
| `type` | item constructor |

**Returns**

[ICRUDRepository](crud-repository.md)`<T>`

#### RepositoryFactory {#typescript-repositoryfactory}

```typescript
class RepositoryFactory implements ICRUDRepositoryFactory
```

**Properties**

| Name | Description |
| --- | --- |
| asReadonly | Readonly factory view. `register` / `create` call [IRepositoryFactory](#typescript-irepositoryfactory). |

**Methods**

| Name | Description |
| --- | --- |
| [register(repository)](#typescript-repositoryfactory-register-repository) | Registers CRUD and readonly for the same repository. |
| [create(type)](#typescript-repositoryfactory-create-type) | Creates a CRUD repository. |
| [register(typeName, pluralName, create, createReadonly)](#typescript-repositoryfactory-register-typename-pluralname-create-createreadonly) | Registers CRUD and readonly factories by entity name and plural name. |
| [create(name, connection)](#typescript-repositoryfactory-create-name-connection) | Creates a registered CRUD repository by entity or plural name. |
| [createReadonly(name, connection)](#typescript-repositoryfactory-createreadonly-name-connection) | Creates a registered read-only repository by entity or plural name. |
| [access(name, connection)](#typescript-repositoryfactory-access-name-connection) | Returns a [RepositoryAccess](docs/sections/datasource.md#typescript-repositoryaccess) for a registered name. |

##### register(repository) {#typescript-repositoryfactory-register-repository}

Registers CRUD and readonly for the same repository.

**Definition**

```typescript
register(repository: IRepository): void
```

**Parameters**

| Name | Description |
| --- | --- |
| `repository` | [IRepository](repositories.md#typescript-irepository) to look up by `type` |

##### create(type) {#typescript-repositoryfactory-create-type}

Creates a CRUD repository.

**Definition**

```typescript
create<T>(type: Constructable<T>): ICRUDRepository<T>
```

**Parameters**

| Name | Description |
| --- | --- |
| `type` | item constructor |

**Returns**

[ICRUDRepository](crud-repository.md)`<T>`

##### register(typeName, pluralName, create, createReadonly) {#typescript-repositoryfactory-register-typename-pluralname-create-createreadonly}

Registers CRUD and readonly factories by entity name and plural name.

**Definition**

```typescript
register(typeName: string, pluralName: string, create: (connection: IConnection) => ICRUDRepository, createReadonly: (connection: IConnection) => IReadonlyRepository): void
```

**Parameters**

| Name | Description |
| --- | --- |
| `typeName` | entity name (`Contact`) |
| `pluralName` | plural name (`contacts`) |
| `create` | builds the CRUD repository from a [IConnection](docs/sections/connection.md) |
| `createReadonly` | builds the readonly repository from a [IConnection](docs/sections/connection.md) |

##### create(name, connection) {#typescript-repositoryfactory-create-name-connection}

Creates a registered CRUD repository by entity or plural name.

**Definition**

```typescript
create(name: string, connection: IConnection): ICRUDRepository
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | entity name or plural name |
| `connection` | [IConnection](docs/sections/connection.md) passed to the factory |

**Returns**

[ICRUDRepository](crud-repository.md)

##### createReadonly(name, connection) {#typescript-repositoryfactory-createreadonly-name-connection}

Creates a registered read-only repository by entity or plural name.

**Definition**

```typescript
createReadonly(name: string, connection: IConnection): IReadonlyRepository
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | entity name or plural name |
| `connection` | [IConnection](docs/sections/connection.md) passed to the factory |

**Returns**

[IReadonlyRepository](readonly-repository.md)

##### access(name, connection) {#typescript-repositoryfactory-access-name-connection}

Returns a [RepositoryAccess](docs/sections/datasource.md#typescript-repositoryaccess) for a registered name.

**Definition**

```typescript
access(name: string, connection: IConnection): RepositoryAccess
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | entity name or plural name |
| `connection` | [IConnection](docs/sections/connection.md) used by `create` / `createReadonly` |

**Returns**

[RepositoryAccess](docs/sections/datasource.md#typescript-repositoryaccess)

#### RepositoryNotRegisteredError {#typescript-repositorynotregisterederror}

```typescript
class RepositoryNotRegisteredError extends Error
```

**Constructor**

| Name | Description |
| --- | --- |
| [RepositoryNotRegisteredError(entityTypeName)](#typescript-repositorynotregisterederror-repositorynotregisterederror-entitytypename) | Thrown when [create](#typescript-repositoryfactory-create-type) cannot find a repository. |

**Properties**

| Name | Description |
| --- | --- |
| entityTypeName | Missing entity type name. |

##### RepositoryNotRegisteredError(entityTypeName) {#typescript-repositorynotregisterederror-repositorynotregisterederror-entitytypename}

Thrown when [create](#typescript-repositoryfactory-create-type) cannot find a repository.

**Definition**

```typescript
constructor(entityTypeName: string)
```

**Parameters**

| Name | Description |
| --- | --- |
| `entityTypeName` | missing entity type name |

#### C#

#### IRepositoryFactory {#csharp-irepositoryfactory}

```csharp
public interface IRepositoryFactory
```

**Methods**

| Name | Description |
| --- | --- |
| [Register(repository)](#csharp-irepositoryfactory-register-repository) | Registers a readonly repository by [Type](repositories.md#csharp-irepository-type). |
| [Create()](#csharp-irepositoryfactory-create) | Creates a readonly repository by `T`. |

##### Register(repository) {#csharp-irepositoryfactory-register-repository}

Registers a readonly repository by [Type](repositories.md#csharp-irepository-type).

**Definition**

```csharp
void Register(IRepository repository)
```

**Parameters**

| Name | Description |
| --- | --- |
| `repository` | [IRepository](repositories.md#csharp-irepository) to look up by `Type` |

##### Create() {#csharp-irepositoryfactory-create}

Creates a readonly repository by `T`.

**Definition**

```csharp
IReadonlyRepository<T> Create<T>()
```

**Parameters**

| Name | Description |
| --- | --- |
| `connection` | optional open [IConnection](docs/sections/connection.md) |

**Returns**

[IReadonlyRepository](readonly-repository.md)`<T>`

#### ICRUDRepositoryFactory {#csharp-icrudrepositoryfactory}

```csharp
public interface ICRUDRepositoryFactory
```

**Methods**

| Name | Description |
| --- | --- |
| [Register(repository)](#csharp-icrudrepositoryfactory-register-repository) | Registers a CRUD repository by [Type](repositories.md#csharp-irepository-type). |
| [Create()](#csharp-icrudrepositoryfactory-create) | Creates a CRUD repository by `T`. |

##### Register(repository) {#csharp-icrudrepositoryfactory-register-repository}

Registers a CRUD repository by [Type](repositories.md#csharp-irepository-type).

**Definition**

```csharp
void Register(IRepository repository)
```

**Parameters**

| Name | Description |
| --- | --- |
| `repository` | [IRepository](repositories.md#csharp-irepository) to look up by `Type` |

##### Create() {#csharp-icrudrepositoryfactory-create}

Creates a CRUD repository by `T`.

**Definition**

```csharp
ICRUDRepository<T> Create<T>()
```

**Returns**

[ICRUDRepository](crud-repository.md)`<T>`

#### RepositoryFactory {#csharp-repositoryfactory}

```csharp
public sealed class RepositoryFactory : IRepositoryFactory, ICRUDRepositoryFactory
```

**Methods**

| Name | Description |
| --- | --- |
| [Register(repository)](#csharp-repositoryfactory-register-repository) | Registers CRUD and readonly for the same repository. |
| [Create()](#csharp-repositoryfactory-create) | Creates a CRUD repository by `T`. |
| [Register(typeName, pluralName, create, createReadonly)](#csharp-repositoryfactory-register-typename-pluralname-create-createreadonly) | Registers CRUD and readonly factories by entity name and plural name. |
| [Create(name, connection)](#csharp-repositoryfactory-create-name-connection) | Creates a registered CRUD repository by entity or plural name. |
| [CreateReadonly(name, connection)](#csharp-repositoryfactory-createreadonly-name-connection) | Creates a registered read-only repository by entity or plural name. |
| [Access(name, connection)](#csharp-repositoryfactory-access-name-connection) | Returns a [RepositoryAccess](docs/sections/datasource.md#csharp-repositoryaccess) for a registered name. |

##### Register(repository) {#csharp-repositoryfactory-register-repository}

Registers CRUD and readonly for the same repository.

**Definition**

```csharp
public void Register(IRepository repository)
```

**Parameters**

| Name | Description |
| --- | --- |
| `repository` | [IRepository](repositories.md#csharp-irepository) to look up by `Type` |

##### Create() {#csharp-repositoryfactory-create}

Creates a CRUD repository by `T`.

**Definition**

```csharp
public ICRUDRepository<T> Create<T>()
```

**Returns**

[ICRUDRepository](crud-repository.md)`<T>`

##### Register(typeName, pluralName, create, createReadonly) {#csharp-repositoryfactory-register-typename-pluralname-create-createreadonly}

Registers CRUD and readonly factories by entity name and plural name.

**Definition**

```csharp
public void Register(string typeName, string pluralName, Func<IConnection, object> create, Func<IConnection, object> createReadonly)
```

**Parameters**

| Name | Description |
| --- | --- |
| `typeName` | entity name (`Contact`) |
| `pluralName` | plural name (`Contacts`) |
| `create` | builds the CRUD repository from an [IConnection](docs/sections/connection.md) |
| `createReadonly` | builds the readonly repository from an [IConnection](docs/sections/connection.md) |

##### Create(name, connection) {#csharp-repositoryfactory-create-name-connection}

Creates a registered CRUD repository by entity or plural name.

**Definition**

```csharp
public TRepository Create<TRepository>(string name, IConnection connection) where TRepository : class
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | entity name or plural name |
| `connection` | [IConnection](docs/sections/connection.md) passed to the factory |

**Returns**

`TRepository`

##### CreateReadonly(name, connection) {#csharp-repositoryfactory-createreadonly-name-connection}

Creates a registered read-only repository by entity or plural name.

**Definition**

```csharp
public TRepository CreateReadonly<TRepository>(string name, IConnection connection) where TRepository : class
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | entity name or plural name |
| `connection` | [IConnection](docs/sections/connection.md) passed to the factory |

**Returns**

`TRepository`

##### Access(name, connection) {#csharp-repositoryfactory-access-name-connection}

Returns a [RepositoryAccess](docs/sections/datasource.md#csharp-repositoryaccess) for a registered name.

**Definition**

```csharp
public RepositoryAccess Access(string name, IConnection connection)
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | entity name or plural name |
| `connection` | [IConnection](docs/sections/connection.md) used by `Create` / `CreateReadonly` |

**Returns**

[RepositoryAccess](docs/sections/datasource.md#csharp-repositoryaccess)

#### RepositoryNotRegisteredException {#csharp-repositorynotregisteredexception}

```csharp
public sealed class RepositoryNotRegisteredException : DatasourceException
```

**Constructor**

| Name | Description |
| --- | --- |
| [RepositoryNotRegisteredException(entityTypeName)](#csharp-repositorynotregisteredexception-repositorynotregisteredexception-entitytypename) | Thrown when [Create](#csharp-repositoryfactory-create) cannot find a repository. |

**Properties**

| Name | Description |
| --- | --- |
| EntityTypeName | Missing entity type name. |

##### RepositoryNotRegisteredException(entityTypeName) {#csharp-repositorynotregisteredexception-repositorynotregisteredexception-entitytypename}

Thrown when [Create](#csharp-repositoryfactory-create) cannot find a repository.

**Definition**

```csharp
public RepositoryNotRegisteredException(string entityTypeName)
```

**Parameters**

| Name | Description |
| --- | --- |
| `entityTypeName` | missing entity type name |

#### Rust

#### ReadonlyRepositoryFactory {#rust-readonlyrepositoryfactory}

```rust
pub trait ReadonlyRepositoryFactory
```

**Methods**

| Name | Description |
| --- | --- |
| [register(repository)](#rust-readonlyrepositoryfactory-register-repository) | Registers a readonly repository by [entity_type](repositories.md#rust-irepository-entity_type). |
| [create()](#rust-readonlyrepositoryfactory-create) | Creates a readonly repository by `T`. |

##### register(repository) {#rust-readonlyrepositoryfactory-register-repository}

Registers a readonly repository by [entity_type](repositories.md#rust-irepository-entity_type).

**Definition**

```rust
fn register(&self, repository: Arc<dyn IReadonlyRepository>)
```

**Parameters**

| Name | Description |
| --- | --- |
| `repository` | [IReadonlyRepository](readonly-repository.md) to look up by `entity_type` |

##### create() {#rust-readonlyrepositoryfactory-create}

Creates a readonly repository by `T`.

**Definition**

```rust
fn create<T: 'static>(&self) -> Result<Arc<dyn IReadonlyRepository>>
```

**Returns**

`Result<Arc<dyn `[IReadonlyRepository](readonly-repository.md)`>>`

#### CRUDRepositoryFactory {#rust-crudrepositoryfactory}

```rust
pub trait CRUDRepositoryFactory
```

**Methods**

| Name | Description |
| --- | --- |
| [register(repository)](#rust-crudrepositoryfactory-register-repository) | Registers a CRUD repository by [entity_type](repositories.md#rust-irepository-entity_type). |
| [create()](#rust-crudrepositoryfactory-create) | Creates a CRUD repository by `T`. |

##### register(repository) {#rust-crudrepositoryfactory-register-repository}

Registers a CRUD repository by [entity_type](repositories.md#rust-irepository-entity_type).

**Definition**

```rust
fn register(&self, repository: Arc<dyn ICRUDRepository>)
```

**Parameters**

| Name | Description |
| --- | --- |
| `repository` | [ICRUDRepository](crud-repository.md) to look up by `entity_type` |

##### create() {#rust-crudrepositoryfactory-create}

Creates a CRUD repository by `T`.

**Definition**

```rust
fn create<T: 'static>(&self) -> Result<Arc<dyn ICRUDRepository>>
```

**Returns**

`Result<Arc<dyn `[ICRUDRepository](crud-repository.md)`>>`

#### RepositoryFactory {#rust-repositoryfactory}

```rust
pub struct RepositoryFactory
```

**Constructor**

| Name | Description |
| --- | --- |
| [new()](#rust-repositoryfactory-new) | Empty registry. |

**Methods**

| Name | Description |
| --- | --- |
| [register(repository)](#rust-repositoryfactory-register-repository) | Registers CRUD and readonly for the same repository. |
| [create()](#rust-repositoryfactory-create) | Creates a CRUD repository by `T`. |
| [register_named(type_name, plural_name, create, create_readonly)](#rust-repositoryfactory-register_named-type_name-plural_name-create-create_readonly) | Registers CRUD and readonly factories by entity name and plural name. |
| [create_named(name, connection)](#rust-repositoryfactory-create_named-name-connection) | Creates a registered CRUD repository by entity or plural name. |
| [create_readonly_named(name, connection)](#rust-repositoryfactory-create_readonly_named-name-connection) | Creates a registered read-only repository by entity or plural name. |
| [access(name, connection)](#rust-repositoryfactory-access-name-connection) | Returns a [RepositoryAccess](docs/sections/datasource.md#rust-repositoryaccess) for a registered name. |

##### new() {#rust-repositoryfactory-new}

Empty registry.

**Definition**

```rust
pub fn new() -> Self
```

**Returns**

[RepositoryFactory](#rust-repositoryfactory)

##### register(repository) {#rust-repositoryfactory-register-repository}

Registers CRUD and readonly for the same repository.

**Definition**

```rust
pub fn register(&self, repository: Arc<dyn ICRUDRepository>)
```

**Parameters**

| Name | Description |
| --- | --- |
| `repository` | [ICRUDRepository](crud-repository.md) to look up by `entity_type` |

##### create() {#rust-repositoryfactory-create}

Creates a CRUD repository by `T`.

**Definition**

```rust
pub fn create<T: 'static>(&self) -> Result<Arc<dyn ICRUDRepository>>
```

**Returns**

`Result<Arc<dyn `[ICRUDRepository](crud-repository.md)`>>`

##### register_named(type_name, plural_name, create, create_readonly) {#rust-repositoryfactory-register_named-type_name-plural_name-create-create_readonly}

Registers CRUD and readonly factories by entity name and plural name.

**Definition**

```rust
pub fn register_named(&self, type_name: impl Into<String>, plural_name: impl Into<String>, create: impl Fn(Arc<dyn Connection>) -> Arc<dyn ICRUDRepository> + Send + Sync + 'static, create_readonly: impl Fn(Arc<dyn Connection>) -> Arc<dyn IReadonlyRepository> + Send + Sync + 'static)
```

**Parameters**

| Name | Description |
| --- | --- |
| `type_name` | entity name (`Contact`) |
| `plural_name` | plural name (`contacts`) |
| `create` | builds the CRUD repository from a [Connection](docs/sections/connection.md) |
| `create_readonly` | builds the readonly repository from a [Connection](docs/sections/connection.md) |

##### create_named(name, connection) {#rust-repositoryfactory-create_named-name-connection}

Creates a registered CRUD repository by entity or plural name.

**Definition**

```rust
pub fn create_named(&self, name: &str, connection: Arc<dyn Connection>) -> Result<Arc<dyn ICRUDRepository>>
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | entity name or plural name |
| `connection` | [Connection](docs/sections/connection.md) passed to the factory |

**Returns**

`Result<Arc<dyn `[ICRUDRepository](crud-repository.md)`>>`

##### create_readonly_named(name, connection) {#rust-repositoryfactory-create_readonly_named-name-connection}

Creates a registered read-only repository by entity or plural name.

**Definition**

```rust
pub fn create_readonly_named(&self, name: &str, connection: Arc<dyn Connection>) -> Result<Arc<dyn IReadonlyRepository>>
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | entity name or plural name |
| `connection` | [Connection](docs/sections/connection.md) passed to the factory |

**Returns**

`Result<Arc<dyn `[IReadonlyRepository](readonly-repository.md)`>>`

##### access(name, connection) {#rust-repositoryfactory-access-name-connection}

Returns a [RepositoryAccess](docs/sections/datasource.md#rust-repositoryaccess) for a registered name.

**Definition**

```rust
pub fn access(&self, name: &str, connection: Arc<dyn Connection>) -> RepositoryAccess
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | entity name or plural name |
| `connection` | [Connection](docs/sections/connection.md) used by `create` / `create_readonly` |

**Returns**

[RepositoryAccess](docs/sections/datasource.md#rust-repositoryaccess)

## Remarks

> **TypeScript**
>
> A missing repository throws [RepositoryNotRegisteredError](#repositorynotregisterederror).

> **C#**
>
> A missing repository throws [RepositoryNotRegisteredException](#repositorynotregisteredexception).

> **Rust**
>
> A missing repository throws `DatasourceError`.

See [Datasource](docs/sections/datasource.md) for `CreateRepository` / `createRepository` / `create_repository`.

## Examples

### Register and create

#### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/RepositoryFactoryTests.cs#L12-L19)

```csharp
{
    var factory = new RepositoryFactory();
    var repo = new StubRepository<Item>("memory", new StubConnection("memory"));
    factory.Register(repo);
    var created = factory.Create<Item>();
```

#### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/repository_factory.rs#L212-L216)

```rust
let factory = RepositoryFactory::new();
factory.register(Arc::new(StubRepository::typed::<Item>("memory")));
let created = factory.create::<Item>().unwrap();
```

#### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/RepositoryFactory.test.ts#L18-L24)

```typescript
const factory = new RepositoryFactory();
const repo = new MockRepository<Item>(new MockConnection(), Item);
factory.register(repo);
const created = factory.create(Item);
```

## See also

- [Repositories](repositories.md)
- [Readonly repository](readonly-repository.md)
- [CRUD repository](crud-repository.md)
