# Datasources

Datasources is a data-access and object-relational mapper library. It aims for parity across [Languages and Datasource Providers](./drivers.md).

The [Feature Stack:](./features.md#feature-stack) starts with commands, connections, and object mapping. Higher up, the API is more opinionated. Types and repositories are generated from a Types Specification, which can be the source of truth for the datasource schema and the generated code. The specification describes unions and inherited types. A type may contain referenced lists and dictionaries, and those references are generated as enriched repositories.

Thus, due to the well-defined [Feature stack](./features.md#feature-stack), the library can be used as a high-level repository builder or as a micro-ORM, alike.

The specification is declarative, so it suits generation by AI agents. Progressive skills keep that generation economical and repeatable. They use fewer tokens, and they keep generated code and mappings constant and deterministic. The library was written so TypeScript could be translated to Rust, and so data-access code written by an agent would remain coherent as an application scales.

<div class="quick-start">

[Quick Start](./quickstart.md)

[Install skill](./skill.md)

</div>
