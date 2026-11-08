# Same inferred spec for types and repositories

## Bug

`references` with no `type` parses as `string`. Parent column type is copied only on the repository path.

- Repos: `planRepositories` → `inferredSpec` → `schemaFields` (`cli/src/generate/RepositoryPlan.ts`, `schemaFields.ts`). `ContactId` is `integer`.
- Types: `deterministic.generateTypes` → `language.types.datasource(ctx)` / `views(ctx)` (`cli/src/generate/DeterministicProvider.ts`). Those parse the reader again. Rust view `Address` emits `pub contact_id: String`. C# sample still has `public string ContactId`.

`#510` on `main` CI failed:

```
FAIL cli/src/generate/generate.test.ts > generate > owns generated Rust module names without crate::types::generated
AssertionError: expected '…' to match /pub contact_id: i32/
    pub contact_id: String,
```

Scheduled e2e on that SHA is a separate issue (`npm warn deprecated` in `scripts/e2e-fail-fast.sh`). Local working tree already allowlists that. Do not mix it into this change unless it is still missing on the branch you cut.

## Proof (keep these; add C#/TS)

Same contacts fixture (`contactsFiles()` in `cli/src/generate/generate.test.ts`):

| Path | Assertion | On main |
| --- | --- | --- |
| `planRepositories` | `schema` `ContactId` type `integer` (~line 710) | pass |
| `generateTypes` rust | `address.rs` `/pub contact_id: i32/` (~line 1214) | fail (`String`) |

Fix is proven when **one** `generateTypes` + `planRepositories` on that fixture agree in all languages:

- C# view/datasource: `int ContactId` (integer id), not `string`.
- TypeScript: `number` (or the same mapping C# uses for `integer`).
- Rust: `i32`.

Complex-game `contacts_base.id` is bigint → C# `long`, Rust `i64`. Do not force every FK to `i32`.

Do not “fix” by weakening the rust assertion to `String`.

## What to change

C# is the reference. One inferred spec. No extra `generateTypes` argument that C# generators lack. No YAML round-trip reader hack.

`inferredSpec` already calls `inferReferenceFieldTypes` (`typescript/code-generators/src/sql/common/infer-reference-types.ts`). SQL generate already infers. Repos already infer. Types do not.

Preferred in this repo:

1. `generateTypes` parses with `DeterministicParser` like `generateRepositories`.
2. Run `inferredSpec(spec)` (same helper as `planRepositories`).
3. Emit datasource + view types from **that** spec, not a second parse inside `views(ctx)`.

If `generate-view-types` / `generate-datasource-types` (packages `@deterministic-code/{csharp,typescript,rust}-generators`) only take `GenerateContext` `{ reader, settings }`, do **not** invent a second CLI signature. Change those generators (C# first) to infer after parse with the same `inferReferenceFieldTypes` (or the existing Rust `infer_reference_types` in `rust/code-generators/src/spec.rs` for the Rust crate — that is SQL spec, not CLI view emit). CLI still must not parse twice with two different specs.

Do not special-case Rust. Do not stamp FKs in samples. Do not edit `generated/` by hand; `generate all` after the emit is correct.

## Halt

- No new converter type. No provider dialect in core.
- No extra C#/TS method that C# lacks. Languages without overloading name C# overloads; no `ReadInput`.
- Generated `*Repository` stays the class, not `ICRUDRepository` / `CRUDRepository` child fields.
- Locked signatures: `.cursor/hooks/locked-classes.txt`.
- No custom e2e runner under `scripts/`.

## Out of scope

- Sample `json_object.rs` / HTTP JSON Default overlay (uuid UNIQUE).
- Publishing / version stamps.
- Weakening e2e-fail-fast except the existing `npm warn deprecated` allowlist if the branch still treats that as fatal.

## Order

1. C# type emit uses inferred parent types. Test C# `ContactId` on the contacts fixture.
2. TypeScript and Rust match that fixture (`number` / `i32`).
3. Existing rust `pub contact_id: i32` test passes.
4. `npx` / `datasources generate all --dialect csharp|typescript|rust` on samples. Complex-game Address `ContactId` follows bigint parent.
5. `npm test --prefix cli` (covers generate tests). Do not start from a custom e2e runner.
