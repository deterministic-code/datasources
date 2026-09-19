# Code reference format

Authoring contract for Code Reference pages. Pages live under `docs/sections/` and generate both the VitePress site and `README.md`. Integrity against source is a later pass; keep this structure parseable.

Not published in the sidebar (`toc.yaml`).

Layout follows MSDN-style API pages: title, summary, language selector, member tables, then a definition and parameters for each member.

## Page shape

Required sections, in order:

1. `# {MajorClass}` — one major type per page
2. Summary paragraph(s) — dialect-neutral
3. Dialect selector — injected before `## Classes`; switches class lists and examples
4. `## Classes` plus one `::: reference` block
5. `## Remarks` — extra usage advice (always present; can be short)
6. `## Examples` — omit the heading when there are zero samples

Associated types belong on the same page (Transaction covers Transaction and IsolationLevel). Extra types are extra `####` headings in each dialect panel.

After the major class and its instance type, document any **page-local** type that appears in a parameter or return and does not have its own Code Reference page (for example `DatasourceOptions` on Datasource). Language primitives (`string`, `void`, `HashMap`) are not listed as classes. Types with their own page (`IConnection`, `ICommand`, repositories, `IDatasourceFactory`) stay links. Parameter and return text links to the type. If another page uses the same type, document it once and link from the other page. C# and Rust may still use the conceptual heading when the runtime type is a dictionary.

## Class list (`::: reference`)

Reserved panel markers (stripped by the transform): `### typescript`, `### csharp`, `### rust`. Aliases: `ts`, `cs`, `rs`.

Each class has an optional definition fence, then `**Constructor**` / `**Properties**` / `**Methods**` groups. Each member is a `#####` heading: short description, definition fence, parameter list, optional `**Returns**`. The transform builds the Name / Description tables from those members.

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

- `options` — [DatasourceOptions](#datasourceoptions); omitted uses `DATABASE_URL` / `DATABASE_SCHEMA`

**Returns** `IDatasource`
:::
~~~~

Rules:

- Document each member once. Do not hand-write the top member tables.
- Use `**Constructor**`, `**Properties**`, and `**Methods**` as the only group labels. Omit a group when the type has none.
- The first paragraph under a member is the table description and the detail lead.
- Parameter lines are `- \`name\` — description`. Link page-local and cross-page types (`[DatasourceOptions](#datasourceoptions)`, `[IDatasourceFactory](./datasource-factory.md)`).
- Rust may use a module heading (`#### datasources`) for free functions when there is no registry class.
- Omit internal hooks (`bindInstall` / `bind_install`) from the class list.

VitePress uses one page-level dialect selector for classes and examples. README expands each panel under `#### TypeScript` / `#### C#` / `#### Rust`.

## Examples

When present, one `###` heading per sample, optional prose, then `::: dialects`. On a reference page those fences follow the page-level selector; they do not get their own tabs.

~~~~markdown
## Examples

### Create from environment

::: dialects
```csharp
var ds = DatasourceFactory.Create();
```
```rust
let ds = create_default()?;
```
```typescript
const ds = Datasource.create();
```
:::
~~~~

## Checklist

- [ ] Title is the major class name
- [ ] Summary is dialect-neutral
- [ ] `## Classes` wraps one `::: reference` with all three panels
- [ ] Each member has a description, definition, and parameters
- [ ] Associated types and page-local argument/return types are extra `####` headings, not a second page
- [ ] `## Remarks` is present
- [ ] Examples use `::: dialects` or the Examples heading is omitted
- [ ] Page is listed in `docs/toc.yaml`
