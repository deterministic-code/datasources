---
name: datasources
description: >-
  Uses Datasources across C#, Rust, and TypeScript: install packages, run setup
  and add providers, generate repositories from types.yaml and datasource.yaml,
  write mappers and type converters, run commands and readers, build
  repositories, call AddEnriched and AddLinked, and write Where and other
  statements from the documented examples. Use when the user mentions
  datasources, repository generation, types.yaml, datasource.yaml, mapper,
  type converter, command, reader, AddEnriched, AddLinked, Where, or
  SelectStatement.
---

<!-- Generated from docs/. Edit the markdown in docs/ and run npm run docs:skill. -->

# Datasources

C# is the reference. Rust and TypeScript match its call shape (`Read` / `read` / `read`).

Read one file below. Do not load the rest.

## Rules

- A select names PascalCase entity types and properties (`Contact.FirstName`), not table or column names.
- Read is convert, then map. Do not fold mapping into the converter.
- When writing code, find the closest row in [examples.md](examples.md), open that language's snippet, and use the same objects the sample uses (connection, repository, enrichment). Include every type the sample defines.
- `AddEnriched` is a read fill. A `List` on a CRUD repository is also written with the parent. `AddLinked` reads the related repository and writes the association through the link repository. Register a child repository's own fills on that child, then register the child on the parent. Read [enrichment.md](enrichment.md) before either call.
- Types, repositories, and SQL come from `types.yaml` (and `datasource.yaml` when it exists). After you change either file, run `npx @deterministic-code/datasources-cli version`, then `npx @deterministic-code/datasources-cli generate all --dialect csharp|typescript|rust`. `version` writes the next `version-*/diff.yaml`. When it prints `unchanged`, stop.
- Do not create, edit, or delete generator output (a directory named `generated`, Rust `src/generated.rs`, or the SQL migrations `generate all` just wrote). If `generate all` fails, stop immediately. Report the command's error unchanged. Do not patch generated code. Change `types.yaml` or `datasource.yaml` and run `generate all` again. In a consumer project, if `.cursor/hooks/block-generated.mjs` is missing, offer to install that hook and wait for a yes. generation.md lists the files to write. Skip that offer when the project root contains `scripts/generate-skill.mjs`.

## Install and setup

Install the core package for the project language.

```sh
npm install @deterministic-code/datasources
dotnet add package Datasources
cargo add datasources
```

Then install dialects and write `.env`:

```sh
npx @deterministic-code/datasources-cli setup --yes --languages ts|rust|csharp --datasources sqlite postgres
```

`--languages` is the project language. `--datasources` is the engine (`sqlite`, `postgres`, `mysql`, `sqlserver`, `oracle`, and the other catalog names). `add` installs more engines later. Checked SQL dialects also install `@deterministic-code/migraters` and run `migrate-init`. Run `setup` without `--yes` for prompts. Provider package names are in [downloading.md](downloading.md). Flags and `.env` are in [setup.md](setup.md).

## Open one file

| Task | File |
| --- | --- |
| Install packages | [downloading.md](downloading.md) |
| setup / add / remove / .env | [setup.md](setup.md) |
| Generate from types.yaml and datasource.yaml | [generation.md](generation.md) |
| Mappers and the read pipeline | [mappings.md](mappings.md) |
| Type converters | [type-converters.md](type-converters.md) |
| Command | [command.md](command.md) |
| Reader | [reader.md](reader.md) |
| Repositories overview | [repositories.md](repositories.md) |
| ReadonlyRepository | [readonly-repository.md](readonly-repository.md) |
| CRUDRepository | [crud-repository.md](crud-repository.md) |
| BulkRepository | [bulk-repository.md](bulk-repository.md) |
| Repository factories | [repository-factories.md](repository-factories.md) |
| AddEnriched / AddLinked | [enrichment.md](enrichment.md) |
| Statements | [statements.md](statements.md) |
| Where | [where.md](where.md) |
| Expressions | [expressions.md](expressions.md) |
| SelectStatement | [select-statement.md](select-statement.md) |
| InsertStatement | [insert-statement.md](insert-statement.md) |
| UpdateStatement | [update-statement.md](update-statement.md) |
| DeleteStatement | [delete-statement.md](delete-statement.md) |
| Other statements | [other-statements.md](other-statements.md) |
| Example index | [examples.md](examples.md) |
