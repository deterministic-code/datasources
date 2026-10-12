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
- When writing code, find the closest row in [examples.md](skills/examples.md), open that language's snippet, and use the same objects the sample uses (connection, repository, enrichment). Include every type the sample defines.
- `AddEnriched` is a read fill. A `List` on a CRUD repository is also written with the parent. `AddLinked` reads the related repository and writes the association through the link repository. Register a child repository's own fills on that child, then register the child on the parent. Read [enrichment.md](skills/enrichment.md) before either call.

## Open one file

| Task | File |
| --- | --- |
| Install packages | [downloading.md](skills/downloading.md) |
| setup / add / remove / .env | [setup.md](skills/setup.md) |
| Generate from types.yaml and datasource.yaml | [generation.md](skills/generation.md) |
| Mappers and the read pipeline | [mappings.md](skills/mappings.md) |
| Type converters | [type-converters.md](skills/type-converters.md) |
| Command | [command.md](skills/command.md) |
| Reader | [reader.md](skills/reader.md) |
| Repositories overview | [repositories.md](skills/repositories.md) |
| ReadonlyRepository | [readonly-repository.md](skills/readonly-repository.md) |
| CRUDRepository | [crud-repository.md](skills/crud-repository.md) |
| BulkRepository | [bulk-repository.md](skills/bulk-repository.md) |
| Repository factories | [repository-factories.md](skills/repository-factories.md) |
| AddEnriched / AddLinked | [enrichment.md](skills/enrichment.md) |
| Statements | [statements.md](skills/statements.md) |
| Where | [where.md](skills/where.md) |
| Expressions | [expressions.md](skills/expressions.md) |
| SelectStatement | [select-statement.md](skills/select-statement.md) |
| InsertStatement | [insert-statement.md](skills/insert-statement.md) |
| UpdateStatement | [update-statement.md](skills/update-statement.md) |
| DeleteStatement | [delete-statement.md](skills/delete-statement.md) |
| Other statements | [other-statements.md](skills/other-statements.md) |
| Example index | [examples.md](skills/examples.md) |
