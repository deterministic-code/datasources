# Code reference format

Authoring contract for Code Reference pages. Pages live under `docs/sections/` and generate both the VitePress site and `README.md`. Examples are copied from named tests into `docs/examples/`; keep this structure parseable.

Not published in the sidebar (`toc.yaml`).

Layout follows MSDN-style API pages: title, summary, language selector, member tables, then a definition and parameters for each Constructor and Methods member. Properties render as a table only.

## Page shape

Required sections, in order:

1. `# {MajorClass}` — one major type per page
2. Summary paragraph(s) — dialect-neutral
3. Dialect selector — injected before `## Classes`; switches class lists and examples
4. `## Classes` plus one `::: reference` block
5. `## Remarks` — extra usage advice (always present; can be short)
6. `## Examples` — omit the heading when there are zero samples

Associated types belong on the same page (Transaction covers Transaction and IsolationLevel). Extra types are extra `####` headings in each dialect panel.

Sidebar nesting: a child sets `parent` to an existing page title. That page is the clickable group. There are no label-only groups.

After the major class and its instance type, document any **page-local** type that appears in a parameter or return and does not have its own Code Reference page (for example `DatasourceOptions` on Datasource). Language primitives (`string`, `void`, `HashMap`) are not listed as classes. Types with their own page (`IConnection`, `ICommand`, repositories, `IDatasourceFactory`) stay links. Parameter and return text links to the type. If another page uses the same type, document it once and link from the other page. C# and Rust may still use the conceptual heading when the runtime type is a dictionary.

## Class list (`::: reference`)

Reserved panel markers (stripped by the transform): `### typescript`, `### csharp`, `### rust`. Aliases: `ts`, `cs`, `rs`.

Each class has an optional definition fence, then `**Constructor**` / `**Properties**` / `**Methods**` groups. Each member is a `#####` heading: short description, definition fence, parameter list, optional `**Returns**`. The transform builds the Name / Description tables from those members. Properties stay as that table; Constructor and Methods also get a definition and parameters.

~~~~markdown
## Classes

::: reference
### typescript
#### Datasource

```typescript
class Datasource
```

**Methods**

##### create(options?)

Creates a datasource from the registry.

```typescript
static create(options?: DatasourceOptions): IDatasource
```

- `options` — [DatasourceOptions](#datasourceoptions); omitted loads `.env`, then process `INSTALLED_PROVIDERS`, `PROVIDER`, and `DATASOURCE_URL`

**Returns** `IDatasource`
:::
~~~~

Rules:

- Document each member once. Do not hand-write the top member tables.
- Use `**Constructor**`, `**Properties**`, and `**Methods**` as the only group labels. Omit a group when the type has none.
- The first paragraph under a member is the table description. Constructor and Methods also use it as the detail lead.
- Parameter lines are `- \`name\` — description`. Link page-local and cross-page types (`[DatasourceOptions](#datasourceoptions)`, `[IDatasourceFactory](../sections/datasource-factory.md)`).
- Rust may use a module heading (`#### datasources`) for free functions when there is no registry class.

VitePress uses one page-level dialect selector for classes and examples. README expands each panel under `#### TypeScript` / `#### C#` / `#### Rust`.

## Dialect names

A summary or remark names an API once per sentence. The marker has three slots, in order C#, TypeScript, Rust. Repeat a slot when two dialects share a spelling. The site shows the slot for the selected language. README keeps all three in the sentence.

A `::: prose` block is for a sentence that is not the same fact under another name. Use `### typescript`, `### csharp`, and `### rust`. Omit a heading when that dialect has nothing to say. Shared sentences stay outside the block. README quotes that sentence under the dialect name so the following remarks stay shared.

~~~~markdown
The connection must be open before {{Execute|execute|execute}} or {{ExecuteReader|executeReader|execute_reader}}.

::: prose
### rust
The `Command` trait has no connection getter.
:::
~~~~

## Examples

When present, one `###` heading per sample, optional prose, then `::: example`. Each fence points at a test or type by name (`path#Name`), not a line range. `npm run docs:examples` writes the usage sample — not the test method wrapper or equality asserts — into `docs/examples/<heading>/<File>.<Name>.<lang>`. The test path and line range live in `docs/examples/lines`. The transform renders the sample file. Do not paste a second fictional fence. A source icon opens the recorded line range from the repo. README links that range on GitHub. On a reference page those fences follow the page-level selector; they do not get their own tabs.

A new fence may use `path#Lstart-Lend` once. The command rewrites it to the region name and stores that slice. After that, `path#Name` is the pointer. Run `npm run docs:examples` after adding a sample. A Cursor hook runs it when a cited `.cs`, `.rs`, or `.ts` file changes.

Optional `output` fences after the sample fences become titled textboxes under the sample. Use a shared fence for every dialect, or `output:typescript` / `output:csharp` / `output:rust` to replace a same-title box for one language.

`::: dialects` stays for install or CLI snippets that are not tests.

~~~~markdown
## Examples

### Create from environment

::: example
```csharp csharp/Datasources.Tests/DatasourceFactoryTests.cs#DefaultFromEnvironment
```
```rust rust/datasources/src/tests/datasource_factory.rs#default_from_environment
```
```typescript typescript/datasources/src/DatasourceFactory.test.ts#default-from-environment
```
:::
~~~~

~~~~markdown
::: example
```csharp csharp/Datasources.Tests/TransformMapperTests.cs#FlattensAndUnflattensUserAndContacts
```
```rust rust/datasources/src/mappers/mod.rs#flattens_and_unflattens_user_and_contacts
```
```typescript typescript/datasources/src/mappers/transformMapper.test.ts#flattens-and-unflattens-user-and-contacts
```

```output [Before: nested value]
{
  user: { id: 1, name: 'Ada' },
  contacts: { email: 'ada@x.dev', phone: '555' }
}
```

```output [After: flat value]
{
  userId: 1,
  userName: 'Ada',
  contactEmail: 'ada@x.dev',
  contactPhone: '555'
}
```
:::
~~~~

## Checklist

- [ ] Title is the major class name
- [ ] Summary keeps one sentence; dialect API names use the three-slot marker
- [ ] `## Classes` wraps one `::: reference` with all three panels
- [ ] Each member has a description; Constructor and Methods also have a definition and parameters (Properties render as a table only)
- [ ] Associated types and page-local argument/return types are extra `####` headings, not a second page
- [ ] `## Remarks` is present
- [ ] Examples use `::: example` name pointers (`path#Name`) or the Examples heading is omitted
- [ ] Nested sidebar items set `parent` in `docs/toc.yaml` to the group label
- [ ] Page is listed in `docs/toc.yaml`
