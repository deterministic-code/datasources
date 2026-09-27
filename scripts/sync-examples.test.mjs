import assert from "node:assert/strict";
import test from "node:test";
import { applyEdits, findRegions, locateSlice, regionForRange } from "./sync-examples.mjs";

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

test("applies an edit inside the stored slice", () => {
  const slice = "        var x = 1;\n        var y = 2;";
  const next = applyEdits(slice, [{ old_string: "var x = 1;", new_string: "var x = 3;" }]);
  assert.equal(next, "        var x = 3;\n        var y = 2;");
});
