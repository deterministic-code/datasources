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
```csharp csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#User
```
```csharp csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#SnakeThenPlural
```
```csharp csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#Snake
```
```csharp csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#ReadsUsersByIdAndByUsernameAndEnabled
```
```rust rust/providers/sqlite/tests/readonly_repository.rs#snake_then_plural
```
```rust rust/providers/sqlite/tests/readonly_repository.rs#snake
```
```rust rust/providers/sqlite/tests/readonly_repository.rs#reads_users_by_id_and_by_username_and_enabled
```
```typescript typescript/providers/sqlite/tests/readonly.repository.integration.test.ts#User-1
```
```typescript typescript/providers/sqlite/tests/readonly.repository.integration.test.ts#snakeThenPlural
```
```typescript typescript/providers/sqlite/tests/readonly.repository.integration.test.ts#snake
```
```typescript typescript/providers/sqlite/tests/readonly.repository.integration.test.ts#reads-users-by-id-and-by-username-and-enabled
```

```output [Row]
{
  Id: 2,
  Username: 'grace',
  IsEnabled: false
}
```
:::

### Create, Read, Update, and Delete

Create omits `id`, then writes the inserted id onto the user. Update and delete use that id.

::: example
```csharp csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#User
```
```csharp csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#WritesUserFromTheType
```
```rust rust/providers/sqlite/tests/readonly_repository.rs#writes_user_from_the_type
```
```typescript typescript/providers/sqlite/tests/readonly.repository.integration.test.ts#User-1
```
```typescript typescript/providers/sqlite/tests/readonly.repository.integration.test.ts#writes-a-user-from-the-type
```

```output [After update]
{
  Id: 1,
  Username: 'ada-v2',
  IsEnabled: true
}
```
:::

### Enriched Create, Read, Update, and Delete

`User` maps onto `user`. Create writes each user and its roles. Enrichment fills `UserType` from that row. Update renames ada, switches the type to member, and adds the owner role when the version matches.

::: example
```csharp csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#usersviewcrudtests-User
```
```csharp csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#UserType
```
```csharp csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#UserRole
```
```csharp csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#UpdatesUsersViewWhenTheVersionMatches
```
```rust rust/providers/sqlite/tests/readonly_repository.rs#updates_users_view_when_the_version_matches
```
```typescript typescript/providers/sqlite/tests/readonly.repository.integration.test.ts#users-view-crud-User
```
```typescript typescript/providers/sqlite/tests/readonly.repository.integration.test.ts#users-view-crud-UserType
```
```typescript typescript/providers/sqlite/tests/readonly.repository.integration.test.ts#UserRole
```
```typescript typescript/providers/sqlite/tests/readonly.repository.integration.test.ts#updates-a-user-view-when-the-version-matches
```

```output [Loaded]
{
  Key: 1,
  Login: 'ada',
  Active: true,
  UserType: {
    Id: 1,
    Name: 'admin'
  },
  Roles: ['admin', 'editor'],
  Version: 1
}
```

```output [Updated]
{
  rowsAffected: 1
}
```

```output [Current]
{
  Key: 1,
  Login: 'ada-v2',
  Active: true,
  UserType: {
    Id: 2,
    Name: 'member'
  },
  Roles: ['admin', 'editor', 'owner'],
  Version: 2
}
```

```output [Stale]
{
  rowsAffected: 0
}
```

```output [Removed]
{
  rowsAffected: 1
}
```

```output [Gone]
no row
```
:::

### Bulk

Each parent is created with its nested children. Responses sum `rowsAffected`.

::: example
```csharp csharp/Datasources.Tests/BulkCRUDRepositoryTests.cs#CreateWritesParentsAndChildren
```
```rust rust/datasources/tests/enriched.rs#bulk_create_writes_parents_and_children
```
```typescript typescript/datasources/src/BulkCRUDRepository.test.ts#creates-parents-and-children
```
:::

## See also

- [Readonly repository](./readonly-repository.md)
- [CRUD repository](./crud-repository.md)
- [Bulk repository](./bulk-repository.md)
- [Repository factories](./repository-factories.md)
