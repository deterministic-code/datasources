# Fix contacts CLI quick starts

A quick start is copy files, then run **scripts**. Dest is a new folder. The harness is `node quick_starts/run.mjs`. It already walks `quick_start.yaml` and `spawnSync`s each line. That is the only loop.

**Do not write a runner.** No `cli/src/run.ts`, no `deps.run`, no `file:` packages, no dialect `if` that stands in for dest, no second command list in TypeScript. A runner is how this suite gets cheated.

**Do not edit `quick_starts/run.mjs`.** Do not point dest at this repo’s CLI, add timeouts, kill `tsx`, rewrite commands, or filter languages to hide a failure. Fix the published library, generator, CLI `setup`/`generate`, or the files that get copied. Then run `node quick_starts/run.mjs`.

C# is the reference. Call-site shape must match across C#, Rust, and TypeScript (casing only). Do not add a language-only extra argument or helper to paper over a missing C# member.

## Dest is scripts

After copy, dest’s `package.json` `scripts` **are** the consumer commands. `quick_start.yaml` and the markdown only call those names. The only dest-specific line is the `cp` from this repo.

```json
{
  "scripts": {
    "setup": "npx @deterministic-code/datasources-cli setup --yes sqlite",
    "generate": "npx @deterministic-code/datasources-cli generate all --dialect typescript",
    "start": "npx tsx src/main.ts"
  }
}
```

Rust and C# dest get the same idea: `setup` / `generate` are the published CLI; `start` is `cargo run` or `dotnet build && dotnet run`. Provider names stay on the script (or a second script like `setup:mysql`) — not in a TypeScript dispatcher.

Sqlite suite after copy:

```text
export PROVIDER=sqlite
export DATASOURCE_URL=sqlite:///./app.sqlite
<init + cp>
<install published core>
npm run setup
npm run generate
npm start
```

mysql-postgres is the same scripts (or `setup` with `mysql postgres`), then `export` + `npm run generate` + `npm start` again. Docker stays as compose lines in the yaml, not a runner feature.

`run.mjs` runs those lines as written. Published packages only.

## Goal

A person (and `run.mjs`) should:

1. `mkdir dest && cd dest` from the clone
2. Copy one language folder onto `./` (and shared `deterministic/` if that is a `cp` line)
3. Install the published core package
4. `npm run setup`
5. `npm run generate`
6. `npm start`

Copied files are the program, compose, expected output, authored spec, and `package.json` scripts. Nothing in the copy exists to fake compile paths, register providers, exclude `node_modules`, or remap generated modules.

## Do not start from

These are already done or out of scope for this plan:

- Rust `read_single(&where)` / `update(item)` / `delete(item)` (no extra `None`s). Keep that shape.
- Rust `create_repository<T>` → `Arc<dyn ICRUDRepository<T>>`. Do not bring back `create_typed_repository`.
- Re-exporting `tokio` from `datasources`. Consumers still `cargo add tokio`.
- A TypeScript command runner in `cli/` that replays dest. Delete `run.ts` / `run.test.ts` if they come back. Setup and generate stay as real CLI verbs; they are not a dest harness.

## Current hacks (what is wrong)

### 1. Rust path shims for `crate::types::generated`

Generate emits `crate::types::generated::views::…` / `crate::types::generated::datasource::…` (see `samples/rust/contacts-crud-enriched-medium-rest/generated/rust/*.rs`).

The quick start does not have that module. It copies:

- `quick_starts/contacts_cli/rust/src/datasources_register.rs` — `#[path = "../generated/rust/mod.rs"] pub mod generated;`
- `quick_starts/contacts_cli/rust/src/types.rs` — aliases `crate::types::generated` onto that path
- `main.rs` — `mod types;` plus `use datasources_register::{Address, Contact, …, Repositories}`

**Fix in generate / the documented module layout**, not in the copied dest. After generate, `cargo run` / `npm start` must compile with a consumer `main.rs` that only `use`s generated names the same way a real project would (one `mod` / one `use`, no `#[path]` shim, no dummy `types` module).

Likely direction (pick the one that matches how generate is supposed to own names; do not invent a second layout):

- Generate into `src/types/generated/…` so `crate::types::generated` is real, **or**
- Generate paths relative to the output crate module (`crate::generated::…`) and document that `mod generated;` is the only consumer line.

Commit `3df2df65` (“Own generated Rust names…”) is the reason the shim exists. Do not go back to rewriting the consumer project. Change the emitted paths or the output directory so the consumer does not invent aliases.

Delete `src/types.rs` and `src/datasources_register.rs` from the quick start once generate stands alone.

### 2. Copied `datasources_providers` / `register()` in Rust `main.rs`

`cli/src/wireRustProviders.ts` already writes `datasources_providers.rs` and injects:

```text
mod datasources_providers;
datasources_providers::register();
```

into `src/main.rs` during `setup`.

The copied `main.rs` already contains those lines so dest compiles even if setup’s rewrite is treated as optional. That is a harness lie.

**Fix:** copied `main.rs` has no `mod datasources_providers` and no `register()` call. `setup` injects them. Do not copy a `datasources_providers.rs`. Keep `wireRustProviders` / `attachedProviders` tests as the source of truth for the inject.

C# and TypeScript already rely on setup to bind providers. Rust must too.

### 3. TypeScript `as` casts

`quick_starts/contacts_cli/typescript/src/main.ts` does:

```ts
const contactsRepo = ds.createRepository("contacts") as ICRUDRepository<Contact>;
await contactsRepo.create({ … } as Contact);
// plus `as Email` / `as Phone` / `as Address` on every nested object
```

C# is `ds.CreateRepository<Contact>("Contacts")` and `new Contact() { … }` with no casts. Rust is `ds.contacts()` and struct literals.

**Fix the TypeScript public / generated API**, not the sample with more casts.

Preferred (match C# first, then generated accessors if C# has them):

- `createRepository<Contact>(Contact)` or `createRepository<Contact>("contacts")` already typed — no `as ICRUDRepository<Contact>`
- Generated types that are constructable (or factory functions) so create/update payloads are `Contact` without `as Contact`
- Or a generated `ds.contacts()` like Rust `Repositories`, if that is what generate already emits on the other languages

Side-effect-only `import "../generated/typescript/repositories/repositoryRegistry.ts"` is not the consumer API. Import the typed accessor or pass the type into `createRepository`.

`no-any-unknown` still applies. Do not “fix” this with `as any`.

### 4. Document `cargo add tokio`

`quick_starts/contacts_cli/rust/quick_start.yaml` already has:

```text
cargo add tokio --features macros,rt-multi-thread
```

These docs omit it, so a human following the markdown fails on `#[tokio::main]`:

- `quick_starts/contacts_cli/rust/quick_start.md`
- `docs/quickstarts/contacts-cli.md`

**Fix:** add the same `cargo add tokio --features macros,rt-multi-thread` line next to `cargo add datasources` in both docs. Do not re-export tokio from the `datasources` crate.

### 5. One shared spec

`types.yaml`, `datasource.yaml`, `datasource_seeds.yaml`, and `settings.yaml` are copied under:

- `quick_starts/contacts_cli/csharp/deterministic/`
- `quick_starts/contacts_cli/rust/deterministic/`
- `quick_starts/contacts_cli/typescript/deterministic/`

They are the same product spec. Three trees drift.

**Fix:** one shared folder (same idea as `samples/user-roles-simple-cli-shared/deterministic/`). Each language yaml copies that folder into dest, or the language folder has no `deterministic/` and the suite command copies the shared tree.

`run.mjs` runs commands as written. A shared copy is a `cp` line in `quick_start.yaml` **and** the same line in `quick_start.md` / `docs/quickstarts/contacts-cli.md`. Do not teach the runner a new “shared spec” feature.

Keep per-language only what is language-specific: `src/main.rs` / `Program.cs` / `src/main.ts`, `compose.yml`, `expected.txt`, `package.json` scripts / `tsconfig.json` if required, mysql grant SQL if compose needs it.

### 6. `setup` writes C# `Directory.Build.props`

`quick_starts/contacts_cli/csharp/Directory.Build.props` exists only so `dotnet build` ignores `node_modules/**` after `npx` / npm drop files next to the csproj.

A real consumer who follows setup + generate hits the same `node_modules` compile noise. Copying a props file is a dest hack.

**Fix in `setup` for csharp:** write (or merge) `Directory.Build.props` with:

```xml
<Project>
  <PropertyGroup>
    <DefaultItemExcludes>$(DefaultItemExcludes);node_modules/**</DefaultItemExcludes>
  </PropertyGroup>
</Project>
```

Then delete the copied props from the quick start. Cover it in `cli` setup tests the same way Rust bind is covered in `wireRustProviders.test.ts`.

If generate already writes under a path that `dotnet` should not compile, prefer generate output that is already excluded — but still write the props from setup so `npx`’s `node_modules` is not part of the project.

## Language parity for the program

After the hacks are gone, the three `main` files should be the same story:

| Step | C# | Rust | TypeScript |
| --- | --- | --- | --- |
| Default ds | `Datasource.Default` | `default()?` | `Datasource.default` |
| Contacts repo | `CreateRepository<Contact>("Contacts")` | typed `create_repository` or generated `contacts()` | typed `createRepository` / generated `contacts()` — **no cast** |
| Write | `Create` / `Update` / `Delete(item)` | `create` / `update` / `delete(item)` | `create` / `update` / `delete(item)` |
| Read | `ReadSingle(where)` | `read_single(&where)` | `readSingle(where)` |
| Close | `await using` | `ds.close().await` | `await using` |

Do not put `read_single(where, None, None)` or extra update/delete args back.

`expected.txt` stays the golden stdout. If a real API change changes printed fields, update all three `expected.txt` files together.

## Files to change (expected)

| Area | Where |
| --- | --- |
| Dest scripts | each language `package.json` `scripts` (`setup`, `generate`, `start`) |
| yaml / md | call `npm run setup`, `npm run generate`, `npm start` after copy + install |
| Rust generate paths | rust/ts code-generators that emit `crate::types::generated` |
| Rust setup bind | already `cli/src/wireRustProviders.ts` — stop pre-seeding `main.rs` |
| TS repo typing | `typescript/datasources` + generate `repositoryRegistry` |
| Tokio docs | `quick_starts/contacts_cli/rust/quick_start.md`, `docs/quickstarts/contacts-cli.md` |
| Shared spec | `quick_starts/contacts_cli/shared/` + yaml/md `cp` lines |
| C# setup props | setup (csharp wire next to `wireRustProviders`) |
| Copied dest | delete shims, casts, `Directory.Build.props`, duplicate `deterministic/` |
| No runner | do not recreate `cli/src/run.ts` |

Do not add sqlite/mysql/postgres packages to the dest `package.json` / `Cargo.toml` / csproj to skip `setup`. Setup installs providers and writes `.env` (`INSTALLED_PROVIDERS`, `DATASOURCE_URL`).

## How to verify

1. Unit: CLI tests for setup writing `Directory.Build.props` and still injecting Rust `datasources_providers`. Generator tests that a dest crate compiles **without** `types.rs` / `datasources_register.rs`.
2. Do **not** put this suite back into `npm test`. Quick starts are e2e.
3. Do **not** verify dest by calling `cli/src` or a fake `run()`. From repo root, after publishing or using the same published flow `run.mjs` already uses: `node quick_starts/run.mjs` (or `npm run test:e2e:quick-starts`, which is that).
4. Optionally one language: follow `quick_start.md` by hand in a fresh `dest` (not this repo’s `cli/` as cwd). `npm run setup`, `npm run generate`, `npm start`.

If e2e fails, fix generate / setup / the copied program / the dest scripts. Do not change `run.mjs`. Do not add a runner.
