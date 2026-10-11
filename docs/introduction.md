# Datasources

Datasources is a data-access and object-relational mapper library. It aims for parity across languages and datasource providers.

The feature stack starts with commands, connections, and object mapping. Higher up, the API is more opinionated. Types and repositories are generated from a Types Specification, which can be the source of truth for the datasource schema and the generated code. The specification describes unions and inherited types. A type may contain referenced lists and dictionaries, and those references are generated as enriched repositories.

The library works as a high-level repository builder or as a micro-ORM.

The specification is declarative, so it suits generation by AI agents. Progressive skills keep that generation economical and repeatable. They use fewer tokens, and they keep generated code and mappings constant and deterministic. The library was written so TypeScript could be translated to Rust, and so data-access code written by an agent would remain coherent as an application scales.

- [Quick Start](./quickstart.md)
- [Features](./features.md)
- [Languages](./drivers.md)
