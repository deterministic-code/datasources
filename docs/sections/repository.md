# Repositories

Repositories map datasource commands to a domain object. They mediate reading and modifying objects in a datasource, and they can enrich those objects into view objects with extra properties. That abstraction removes boilerplate datasource code.

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
:::

## Remarks

SQL provider default repositories report the row or dictionary type (`Row`, `Dictionary<string, Value>`, or `Object`), not a closed entity generic.

## Examples

### Simple readonly

{{ReadSingle|readSingle|read_single}} returns the first matching user. {{Read|read|read}} yields every user that matches the where.

::: example
```csharp csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#ReadsUsersByIdAndByUsernameAndEnabled
```
```rust rust/providers/sqlite/tests/readonly_repository.rs#reads_users_by_id_and_by_username_and_enabled
```
```typescript typescript/providers/sqlite/tests/readonly.repository.integration.test.ts#reads-users-by-id-and-by-username-and-enabled
```
:::

### CRUD

Create omits `id`, then writes the inserted id onto the user. Update and delete use that id.

::: example
```csharp csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#WritesUserFromTheType
```
```rust rust/providers/sqlite/tests/readonly_repository.rs#writes_user_from_the_type
```
```typescript typescript/providers/sqlite/tests/readonly.repository.integration.test.ts#writes-a-user-from-the-type
```
:::

### Enriched CRUD

`UsersView` maps onto `user`. Create writes the user and its roles. Read fills `UserType` and `Roles`.

::: example
```csharp csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#UpdatesUsersViewWhenTheVersionMatches
```
```rust rust/providers/sqlite/tests/readonly_repository.rs#updates_users_view_when_the_version_matches
```
```typescript typescript/providers/sqlite/tests/readonly.repository.integration.test.ts#updates-a-user-view-when-the-version-matches
```
:::

### Bulk

Each parent is created with its nested children. Responses sum `rowsAffected`.

::: example
```csharp csharp/Datasources.Tests/BulkCRUDRepositoryTests.cs#CreateWritesParentsAndChildren
```
```rust rust/datasources/src/tests/mod.rs#table_repository_maps_flat_row_into_contacts
```
```typescript typescript/datasources/src/BulkCRUDRepository.test.ts#creates-parents-and-children
```
:::

## See also

- [Readonly repository](./readonly-repository.md)
- [CRUD repository](./crud-repository.md)
- [Bulk repository](./bulk-repository.md)
- [Repository factories](./repository-factories.md)
