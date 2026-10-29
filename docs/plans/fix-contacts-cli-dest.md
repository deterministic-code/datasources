# Fix contacts CLI dest (underlying only)

Dest is a consumer folder. The yaml is the script. [`quick_starts/run.mjs`](../../quick_starts/run.mjs) already runs it.

**Do not edit `run.mjs`.** Do not add a runner, dest shims, copied bind files, `file:` packages, or yaml/md that paper over a missing generate or setup behavior.

If dest cannot `mod generated`, call `ds.contacts`, or `dotnet build` after setup, that is a **published generate / setup / API bug**. Fix that product. Dest only copies the program and the spec.

The script (yaml = md). Only dest-specific line is the `cp` from this repo:

```sh
export PROVIDER=sqlite
export DATASOURCE_URL=sqlite:///./app.sqlite
cargo init
cp -R ../quick_starts/contacts_cli/rust/. ./
cp -R ../quick_starts/contacts_cli/shared/deterministic ./deterministic
cargo add datasources
cargo add tokio --features macros,rt-multi-thread
npx @deterministic-code/datasources-cli setup --yes sqlite
npx @deterministic-code/datasources-cli generate all --dialect rust
cargo run
```

Published packages only.

## Underlying problems

### 1. Rust generate does not own the crate module

Generate emits `crate::types::generated::…` and leaves the consumer to invent `#[path]`, `types.rs`, and `datasources_register.rs`. That is generate lying about the module layout.

**Product fix:** generate writes names the crate already has. After `generate all --dialect rust`, `src/generated.rs` exists and `mod generated; use generated::{…}` compiles. Remap or place types so there is no `crate::types::generated`. Re-export `Repositories` from that module.

`setup` already injects `datasources_providers` / `register()`. That stays in `wireRustProviders`. Copied `main.rs` must not pre-seed it — if dest needs those lines before setup, setup is broken, not dest.

Delete dest shims only after generate stands alone.

### 2. TypeScript create/repository is untyped

`IDatasource.createRepository(name: string)` is untyped. Generate does not give `Datasource` a `contacts` member or constructable types. Dest then uses `as`. That is a missing public / generated API. C# already has typed create and object initializers.

**Product fix:** generated registry augments `Datasource` with `readonly contacts: ContactRepository` and exports `contact` / `email` / `phone` / `address` factories (or the same typed construction C# has). `RepositoryFactory.bind` stays how C#/Rust bind. No TS-only extra argument on `createRepository`.

Dest `main.ts` then has no `as`. Do not “fix” dest with more casts.

### 3. C# setup does not exclude `node_modules`

`npx` drops `node_modules` next to the csproj. `dotnet build` compiles it. A copied `Directory.Build.props` is dest covering a setup hole.

**Product fix:** the same setup path that calls `wireRustProviders` writes (or merges) `Directory.Build.props` with `DefaultItemExcludes` including `node_modules/**`. `wireCsharpBuildProps` next to the Rust wire. Tests on that helper, same as `wireRustProviders.test.ts`.

Delete the copied props after setup writes it.

### 4. Spec and tokio are the script, not a harness feature

Three copies of the same yaml drift. Docs omit `cargo add tokio` so a human fails while yaml passes. That is spec/docs drift, not a runner gap.

**Product / sample fix:** one `shared/deterministic`. Each language script `cp`s it. Same line in md. Tokio line next to `cargo add datasources` in rust yaml **and** md. Do not add a “shared spec” flag to `run.mjs`. Language folders must not keep a leftover `deterministic/` that `cp language/.` also copies.

## Verify

Unit: generate emits `src/generated.rs` and no `crate::types::generated`; TS registry has `contacts` + factories; setup writes props and still injects Rust bind.

Do not put dest in `npm test`. Do not invoke `cli/src` as dest.

`node quick_starts/run.mjs` — published packages, the yaml script. Failures go back to generate, setup, or the copied program. Not the runner.
