# Repositories

Typed item marker and instance registry. {{Type|type|entity_type}} is the item CLR type, constructor, or `TypeId` the repository exposes, not the table `entity` name. Register a repository, then create by that type.

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

##### register(repository)

Registers a readonly repository by [type](#irepository-type).

```typescript
register(repository: IRepository): void
```

- `repository` — [IRepository](#irepository) to look up by `type`

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

Registers a CRUD repository by [type](#irepository-type).

```typescript
register(repository: IRepository): void
```

- `repository` — [IRepository](#irepository) to look up by `type`

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

- `repository` — [IRepository](#irepository) to look up by `type`

##### create(type)

Creates a CRUD repository.

```typescript
create<T>(type: Constructable<T>): ICRUDRepository<T>
```

- `type` — item constructor

**Returns** [ICRUDRepository](./crud-repository.md)`<T>`

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

##### Register(repository)

Registers a readonly repository by [Type](#irepository-type).

```csharp
void Register(IRepository repository)
```

- `repository` — [IRepository](#irepository) to look up by `Type`

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

Registers a CRUD repository by [Type](#irepository-type).

```csharp
void Register(IRepository repository)
```

- `repository` — [IRepository](#irepository) to look up by `Type`

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

- `repository` — [IRepository](#irepository) to look up by `Type`

##### Create()

Creates a CRUD repository by `T`.

```csharp
public ICRUDRepository<T> Create<T>()
```

**Returns** [ICRUDRepository](./crud-repository.md)`<T>`

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
pub fn as_readonly(repo: Arc<dyn ICRUDRepository>) -> Arc<dyn IReadonlyRepository>
```

**Methods**

##### as_readonly(repo)

Views a [ICRUDRepository](./crud-repository.md) as a [IReadonlyRepository](./readonly-repository.md).

```rust
pub fn as_readonly(repo: Arc<dyn ICRUDRepository>) -> Arc<dyn IReadonlyRepository>
```

- `repo` — [ICRUDRepository](./crud-repository.md)

**Returns** `Arc<dyn `[IReadonlyRepository](./readonly-repository.md)`>`

#### ReadonlyRepositoryFactory

```rust
pub trait ReadonlyRepositoryFactory
```

**Methods**

##### register(repository)

Registers a readonly repository by [entity_type](#irepository-entity_type).

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

Registers a CRUD repository by [entity_type](#irepository-entity_type).

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
:::

## Remarks

SQL provider default repositories report the row or dictionary type (`Row`, `Dictionary<string, Value>`, or `Object`), not a closed entity generic.

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

- [Readonly repository](./readonly-repository.md)
- [CRUD repository](./crud-repository.md)
- [Bulk repository](./bulk-repository.md)
