# Statements

Shared statement construction used by SQL providers. `StatementBuilder` builds query / insert / update / delete text. `WhereBuilder` composes predicates. Dialect packages may wrap `StatementBuilder` for quoting and native column types, including `CreateStoredProcedure` the same way as `CreateInsert`.

## StatementBuilder

| C# | Rust | TypeScript |
| --- | --- | --- |
| `StatementBuilder` | `StatementBuilder` | `StatementBuilder` |
| `CreateQuery` | `create_query` | `createQuery` |
| `CreateInsert` | `create_insert` | `createInsert` |
| `CreateUpdate` | `create_update` | `createUpdate` |
| `CreateDelete` | `create_delete` | `createDelete` |
| `CreateTable` | `create_table` | `createTable` |
| `CreateStoredProcedure` | `create_stored_procedure` | `createStoredProcedure` |

## WhereBuilder

| C# | Rust | TypeScript |
| --- | --- | --- |
| `WhereBuilder` | `WhereBuilder` | `WhereBuilder` |
| `Equals` | `equals` | `equals` |
| `And` / `Or` / `Not` | `and` / `or` / `not` | `and` / `or` / `not` |

## Examples

TODO: query / insert / where samples as the builder API is documented.
