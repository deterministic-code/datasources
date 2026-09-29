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
