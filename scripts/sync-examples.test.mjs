import assert from "node:assert/strict";
import test from "node:test";
import { exampleCode, exampleSchema } from "./example.mjs";
import {
  applyEdits,
  findRegions,
  formatLines,
  locateSlice,
  parseLines,
  regionForRange,
  renderExamplesPage,
} from "./sync-examples.mjs";

const csharp = `public class IdentityMapperTests
{
    [Fact]
    public void MapsAndReversesTheSameValue()
    {
        var mapper = new IdentityMapper<string>();
    }

    [Fact]
    public void Other()
    {
        var x = 1;
    }
}
`;

test("prefixes a later C# type when the name is already used", () => {
  const source = `public class First
{
    sealed class User
    {
        public int Id { get; set; }
    }
}
public class Second
{
    record User
    {
        public long Key { get; set; }
    }
}
`;
  const names = findRegions(source, "csharp").map((region) => region.name);
  assert.ok(names.includes("User"));
  assert.ok(names.includes("second-User"));
});

test("names a C# fact and the span of its method", () => {
  const regions = findRegions(csharp, "csharp");
  const fact = regions.find((region) => region.name === "MapsAndReversesTheSameValue");
  assert.equal(fact.kind, "test");
  assert.equal(fact.from, 3);
  assert.equal(fact.to, 7);
  assert.equal(regionForRange(regions, 5, 6).name, "MapsAndReversesTheSameValue");
});

test("rejects a range that crosses two tests", () => {
  const regions = findRegions(csharp, "csharp");
  assert.throws(() => regionForRange(regions, 5, 12), /span/);
});

test("names a Rust test and a TypeScript it() slug", () => {
  const rust = findRegions(
    `#[test]\nfn maps_and_reverses_the_same_value() {\n    let x = 1;\n}\n`,
    "rust",
  ).find((region) => region.kind === "test");
  assert.equal(rust.name, "maps_and_reverses_the_same_value");
  assert.equal(rust.from, 1);
  assert.equal(rust.to, 4);

  const ts = findRegions(
    `describe("IdentityMapper", () => {\n  it("maps and reverses the same value", () => {\n    const x = 1;\n  });\n});\n`,
    "typescript",
  ).find((region) => region.kind === "test");
  assert.equal(ts.name, "maps-and-reverses-the-same-value");
});

test("uses the enclosing type when the range is not a test", () => {
  const source = `public sealed class MockDriver : IDriver\n{\n    public IConnection Connection() => null;\n}\n`;
  const regions = findRegions(source, "csharp");
  const type = regions.find((region) => region.name === "MockDriver");
  assert.equal(regionForRange(regions, 1, type.to).name, "MockDriver");
});

test("finds a slice again after lines move above it", () => {
  const source = csharp.replace("{\n    [Fact]", "{\n    int pad;\n    [Fact]");
  const regions = findRegions(source, "csharp");
  const region = regions.find((item) => item.name === "MapsAndReversesTheSameValue");
  const found = locateSlice(source, region, "        var mapper = new IdentityMapper<string>();");
  assert.equal(found.from, 7);
  assert.equal(found.to, 7);
});

test("keeps the recorded lines when the slice text appears twice", () => {
  const source = "#[test]\nfn statements_and_builders() {\n    let x = 1;\n    let x = 1;\n}\n";
  const region = findRegions(source, "rust").find((item) => item.kind === "test");
  const found = locateSlice(source, region, "    let x = 1;", { from: 4, to: 4 });
  assert.equal(found.from, 4);
  assert.equal(found.ambiguous, undefined);
});

test("keeps source lines apart from the usage sample", () => {
  const raw = [
    "    [Fact]",
    "    public void M()",
    "    {",
    "        var ds = DatasourceFactory.Default;",
    "        Assert.Equal(1, ds);",
    "    }",
  ].join("\n");
  const file = "docs/examples/m/T.M.csharp";
  const text = formatLines([{ file, test: "a.cs#M", from: 1, to: 6, raw }]);
  const record = parseLines(text).get(file);
  assert.equal(record.from, 1);
  assert.equal(record.to, 6);
  assert.equal(record.raw, raw);
  assert.equal(exampleCode(record.raw), "var ds = DatasourceFactory.Default;");
});

test("drops open and apply and keeps the schema name", () => {
  const raw = [
    "public async Task ReadsUsers()",
    "{",
    '    var ds = Datasource.Default;',
    "    var connection = await Open(ds);",
    '    await SchemaSpec.Apply(connection, "minimal-users");',
    '    await Execute(connection, "INSERT INTO users (id) VALUES (1)");',
    "}",
  ].join("\n");
  assert.equal(
    exampleCode(raw),
    'var ds = Datasource.Default;\nawait Execute(connection, "INSERT INTO users (id) VALUES (1)");',
  );
  assert.equal(exampleSchema(raw), "minimal-users");
});

test("drops a leading env lock and arrange", () => {
  const raw = [
    "#[tokio::test]",
    "fn create_default_from_env_settings() {",
    "    let _guard = super::env_lock();",
    "    arrange();",
    "    let ds = default().expect(\"default\");",
    "    let command = ds.create_command(None);",
    "    assert_eq!(Datasource::provider(&*ds), \"sqlite\");",
    "}",
  ].join("\n");
  assert.equal(
    exampleCode(raw),
    "let ds = default().expect(\"default\");\nlet command = ds.create_command(None);",
  );
  const nested = [
    "#[tokio::test]",
    "fn writes_user_from_the_type() {",
    "    let _guard = super::super::env_lock();",
    "    let ds = default().unwrap();",
    "}",
  ].join("\n");
  assert.equal(exampleCode(nested), "let ds = default().unwrap();");
});

test("drops a truncated tokio test wrapper", () => {
  const raw = [
    "#[tokio::test]",
    "async fn open_and_close() {",
    "    let connection = MemoryConnection::from_options(None);",
    "    connection.open().await.unwrap();",
  ].join("\n");
  assert.equal(
    exampleCode(raw),
    "let connection = MemoryConnection::from_options(None);\nconnection.open().await.unwrap();",
  );
});

test("drops a public async Task wrapper", () => {
  const raw = ["public async Task CreateWritesParentsAndChildren()", "{", "    var connection = await Setup();", "}"].join(
    "\n",
  );
  assert.equal(exampleCode(raw), "var connection = await Setup();");
});

test("keeps an it() body that contains a template", () => {
  const raw = [
    'it("reads", async () => {',
    "  const sql = `",
    "    INSERT INTO user VALUES ('${ada}', 'ada');",
    "  `;",
    "  const repo = ds.createReadonlyRepository(User);",
    "});",
  ].join("\n");
  assert.equal(
    exampleCode(raw),
    "const sql = `\n  INSERT INTO user VALUES ('${ada}', 'ada');\n`;\nconst repo = ds.createReadonlyRepository(User);",
  );
});

test("keeps a rust helper function", () => {
  const raw = ["fn snake() -> ChangeCasingMapper {", "    ChangeCasingMapper::new(Casing::Pascal, Casing::Snake)", "}"].join(
    "\n",
  );
  assert.equal(exampleCode(raw), raw);
});

test("keeps a fn that appears after the sample starts", () => {
  const raw = "let x = 1;\nfn helper() {\n    let y = 2;\n}";
  assert.equal(exampleCode(raw), raw);
});

test("lists examples in toc order with heading links", () => {
  const pages = [
    { path: "sections/command.md", title: "Command" },
    { path: "sections/repository.md", title: "Repositories" },
    { path: "sections/connection.md", title: "Connection" },
  ];
  const headings = [
    { doc: "docs/sections/repository.md", title: "Simple readonly" },
    { doc: "docs/sections/command.md", title: "Run raw SQL" },
    { doc: "docs/sections/repository.md", title: "CRUD" },
  ];
  assert.equal(
    renderExamplesPage({ pages, headings, schemas: [] }),
    [
      "# Examples",
      "",
      "Samples from the code reference. Open a link for the explanation and the C#, Rust, and TypeScript code.",
      "",
      "## Command",
      "",
      "- [Run raw SQL](./sections/command.md#run-raw-sql)",
      "",
      "## Repositories",
      "",
      "- [Simple readonly](./sections/repository.md#simple-readonly)",
      "- [CRUD](./sections/repository.md#crud)",
      "",
    ].join("\n"),
  );
});

test("applies an edit inside the stored slice", () => {
  const slice = "        var x = 1;\n        var y = 2;";
  const next = applyEdits(slice, [{ old_string: "var x = 1;", new_string: "var x = 3;" }]);
  assert.equal(next, "        var x = 3;\n        var y = 2;");
});
