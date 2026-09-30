# Repository factories

Register a repository, then create by that type. Registering CRUD also registers readonly. Registering readonly-only makes CRUD `create` throw.

{{Type|type|entity_type}} is the item CLR type, constructor, or `TypeId` the repository exposes, not the table `entity` name.

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
