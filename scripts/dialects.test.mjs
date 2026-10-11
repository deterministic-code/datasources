import assert from "node:assert/strict";
import test from "node:test";
import { transformDialects } from "./dialects.mjs";

const command = `# Command

The connection must be open before {{Execute|execute|execute}} or {{ExecuteReader|executeReader|execute_reader}}.

## Classes

::: reference
### typescript
#### ICommand
:::

## Remarks

{{AddMiddleware|addMiddleware|add_middleware}} runs {{BeforeCommand|beforeCommand|before_command}}.

::: prose
### rust
The \`Command\` trait has no connection getter.
:::
`;

test("vitepress swaps each name in place and toggles a one-dialect section", () => {
  const page = transformDialects(command, "vitepress");
  assert.match(
    page,
    /<DialectName csharp="Execute" typescript="execute" rust="execute"><\/DialectName>/,
  );
  assert.match(
    page,
    /<DialectName csharp="ExecuteReader" typescript="executeReader" rust="execute_reader"><\/DialectName>/,
  );
  assert.match(page, /<DialectSelector \/>\n\n## Classes/);
  assert.match(
    page,
    /<DialectPanel name="rust">\n\nThe `Command` trait has no connection getter\.\n\n<\/DialectPanel>/,
  );
  assert.doesNotMatch(page, /\{\{/);
});

test("readme keeps every name in the sentence and expands a one-dialect section", () => {
  const page = transformDialects(command, "readme");
  assert.match(page, /`Execute` \/ `execute` \/ `execute`/);
  assert.match(page, /`ExecuteReader` \/ `executeReader` \/ `execute_reader`/);
  assert.match(page, /> \*\*Rust\*\*\n>\n> The `Command` trait has no connection getter\./);
  assert.doesNotMatch(page, /<Dialect/);
});

test("a page with only names gets the selector after the title", () => {
  const page = transformDialects("# CLI\n\n{{Default|default|default()}} reads the env.\n", "vitepress");
  assert.match(page, /^# CLI\n\n<DialectSelector \/>\n/);
  assert.match(page, /csharp="Default" typescript="default" rust="default\(\)"/);
});

test("a tutorial with names shows one dialect sample at a time", () => {
  const source = `# The Mapping and Converting Pipelines

{{Read|read|read}} calls {{ExecuteReader|executeReader|execute_reader}}.

### Example

::: example
\`\`\`csharp csharp/Providers/Sqlite.Tests/ReadPipelineIntegrationTests.cs#User
\`\`\`
\`\`\`csharp csharp/Providers/Sqlite.Tests/ReadPipelineIntegrationTests.cs#MapsAndConvertsUsersThroughTheReadPipeline
\`\`\`
\`\`\`rust rust/providers/sqlite/tests/read_pipeline.rs#ConvertSchemaForUser
\`\`\`
\`\`\`rust rust/providers/sqlite/tests/read_pipeline.rs#maps_and_converts_users_through_the_read_pipeline
\`\`\`
\`\`\`typescript typescript/providers/sqlite/tests/read-pipeline.integration.test.ts#User
\`\`\`
\`\`\`typescript typescript/providers/sqlite/tests/read-pipeline.integration.test.ts#maps-and-converts-users-through-the-read-pipeline
\`\`\`
:::
`;
  const page = transformDialects(source, "vitepress");
  assert.match(page, /<DialectSelector \/>/);
  assert.match(page, /<DialectPanel name="csharp">/);
  assert.match(page, /<DialectPanel name="rust">/);
  assert.match(page, /<DialectPanel name="typescript">/);
  assert.doesNotMatch(page, /::: code-group/);
});

test("a type and its usage render as one sample", () => {
  const source = `# Command

## Classes

::: reference
### typescript
#### ICommand
:::

### Run ExecuteReader and return NameValues

::: example
\`\`\`csharp csharp/Providers/Sqlite.Tests/SqliteCommandIntegrationTests.cs#NameValue
\`\`\`
\`\`\`csharp csharp/Providers/Sqlite.Tests/SqliteCommandIntegrationTests.cs#RunExecuteReaderAndReturnNameValues
\`\`\`
:::
`;
  const page = transformDialects(source, "vitepress");
  assert.match(page, /href="\/view-source\?path=csharp%2FProviders%2FSqlite\.Tests%2FSqliteCommandIntegrationTests\.cs&amp;from=\d+&amp;to=\d+"/);
  assert.doesNotMatch(page, /github\.com/);
  assert.equal(page.match(/<DialectPanel name="csharp">/g).length, 1);
  const sample = page.slice(page.indexOf("sealed class NameValue"), page.indexOf("Datasource.Default"));
  assert.match(sample, /public string value/);
  const readme = transformDialects(source, "readme");
  assert.match(readme, /github\.com\/deterministic-code\/datasources\/blob\/main\//);
  assert.equal(readme.match(/#### C#/g).length, 1);
  assert.match(readme, /sealed class NameValue[\s\S]*Datasource\.Default/);
});

test("an example after a one-dialect note keeps its sample heading", () => {
  const source = `# Connection

## Classes

::: reference
### typescript
#### IConnection
:::

### Close with using

::: prose
### rust
There is no \`using\`.
:::

::: example
\`\`\`csharp csharp/Providers/Memory.Tests/ConnectionLifecycleTests.cs#CloseWithUsing
\`\`\`
\`\`\`rust rust/providers/memory/src/lib.rs#open_and_close
\`\`\`
\`\`\`typescript typescript/providers/memory/src/index.test.ts#closes-with-using
\`\`\`
:::
`;
  const page = transformDialects(source, "vitepress");
  assert.match(page, /await using/);
  assert.match(page, /<DialectPanel name="rust">/);
});

test("a marker slot may contain an escaped pipe", () => {
  const source = "# Datasource\n\n{{ReadSingle|readSingle|read_single}} returns ({{Contact?|Contact \\| undefined|Option<Row>}}). {{Create|create|create}} returns that entity.\n";
  const page = transformDialects(source, "vitepress");
  assert.match(
    page,
    /<DialectName csharp="ReadSingle" typescript="readSingle" rust="read_single"><\/DialectName>/,
  );
  assert.match(
    page,
    /<DialectName csharp="Contact\?" typescript="Contact \| undefined" rust="Option&lt;Row&gt;"><\/DialectName>/,
  );
  assert.match(page, /<DialectName csharp="Create" typescript="create" rust="create"><\/DialectName>/);
  const readme = transformDialects(source, "readme");
  assert.match(readme, /`ReadSingle` \/ `readSingle` \/ `read_single`/);
  assert.match(readme, /`Contact\?` \/ `Contact \| undefined` \/ `Option<Row>`/);
  assert.match(readme, /`Create` \/ `create` \/ `create`/);
});

test("leaves a short marker and fenced text unchanged", () => {
  const source = `# Notes

{{Execute|execute}}

\`\`\`
{{Execute|execute|execute}}
\`\`\`

~~~~markdown
{{Execute|execute|execute}}

## Classes
~~~~
`;
  const page = transformDialects(source, "vitepress");
  assert.match(page, /\{\{Execute\|execute\}\}/);
  assert.match(page, /```\n\{\{Execute\|execute\|execute\}\}\n```/);
  assert.match(page, /~~~~markdown\n\{\{Execute\|execute\|execute\}\}/);
  assert.doesNotMatch(page, /DialectSelector/);
  assert.doesNotMatch(page, /DialectName/);
});
