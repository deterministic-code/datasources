import { parse } from "yaml";
import { describe, expect, it } from "vitest";
import { apply, applyFile, delta, type Json } from "./delta.ts";
import { applyDocument, asDocument, render } from "./render.ts";

const commentBefore: Json = {
  types: [
    {
      comment: {
        fields: [
          { author_id: { type: "number", references: "user.id" } },
          { body: { type: "string", size: "unlimited" } },
          { old_field: { type: "string", size: 32 } },
        ],
      },
    },
  ],
};

const commentAfter: Json = {
  types: [
    {
      comment: {
        fields: [
          { author_id: { type: "number", references: "user.id" } },
          { body: { type: "string", size: 1024 } },
          { is_pinned: { type: "boolean", default_value: false } },
        ],
      },
    },
  ],
};

describe("delta", () => {
  it("renders the comment.fields example and round-trips through apply", () => {
    const yaml = render([{ path: "ds.yaml", before: commentBefore, after: commentAfter }]);
    const parsed = parse(yaml) as { [path: string]: Json };
    const fields = (parsed["ds.yaml"] as { types: { changed: { comment: { fields: Json } }[] } }).types
      .changed[0]!.comment.fields;
    expect(fields).toEqual({
      added: [{ is_pinned: { type: "boolean", default_value: false } }],
      changed: [{ body: { size: 1024 } }],
      removed: ["old_field"],
    });
    expect(applyFile(commentBefore, parsed["ds.yaml"]!)).toEqual(commentAfter);
  });

  it("returns no document when values are identical or only reordered", () => {
    expect(render([{ path: "a.yaml", before: commentAfter, after: commentAfter }])).toBe("");
    expect(
      render([
        {
          path: "x.yaml",
          before: [{ a: 1 }, { b: 2 }],
          after: [{ b: 2 }, { a: 1 }],
        },
      ]),
    ).toBe("");
    expect(
      render([
        {
          path: "x.yaml",
          before: { z: 1, a: 2 },
          after: { a: 2, z: 1 },
        },
      ]),
    ).toBe("");
    expect(delta({ list: [{ a: 1 }, { b: 2 }] }, { list: [{ b: 2 }, { a: 1 }] })).toBeUndefined();
  });

  it("wraps a new file and a deleted file", () => {
    const added = render([{ path: "new.yaml", before: undefined, after: { types: [] } }]);
    const addedDoc = parse(added) as { [path: string]: Json };
    expect(addedDoc["new.yaml"]).toEqual({ added: true, content: { types: [] } });
    expect(applyFile(undefined, addedDoc["new.yaml"]!)).toEqual({ types: [] });
    const removed = render([{ path: "gone.yaml", before: { types: [] }, after: undefined }]);
    const removedDoc = parse(removed) as { [path: string]: Json };
    expect(removedDoc["gone.yaml"]).toEqual({ removed: true });
    expect(applyFile({ types: [] }, removedDoc["gone.yaml"]!)).toBeUndefined();
  });

  it("replaces scalars and primitive arrays wholesale", () => {
    expect(delta("a", "b")).toBe("b");
    expect(delta([1, 2], [1, 2, 3])).toEqual([1, 2, 3]);
    expect(apply([1, 2], [1, 2, 3])).toEqual([1, 2, 3]);
  });

  it("leaves the previous value when the delta is null", () => {
    expect(apply({ a: 5 }, null)).toEqual({ a: 5 });
    expect(apply({ a: 5 }, { a: null })).toEqual({ a: 5 });
  });

  it("keeps the last value when a keyed identity is repeated", () => {
    expect(delta([{ user: { n: 1 } }, { user: { n: 2 } }], [{ user: { n: 2 } }])).toBeUndefined();
    expect(delta([{ user: { n: 1 } }, { user: { n: 2 } }], [{ user: { n: 3 } }])).toEqual({
      changed: [{ user: { n: 3 } }],
    });
  });

  it("adds object keys, drops removed names, and applies keyed splits", () => {
    expect(delta({ a: 1 }, { a: 1, b: 2 })).toEqual({ b: 2 });
    expect(delta({ a: 1, gone: true }, { a: 1 })).toEqual({ removed: ["gone"] });
    expect(apply({ a: 1 }, { b: 2 })).toEqual({ a: 1, b: 2 });
    expect(apply([{ a: 1 }], "replaced")).toBe("replaced");
    expect(apply([{ a: 1 }], { added: [{ b: 2 }], extra: 1 })).toEqual({ added: [{ b: 2 }], extra: 1 });
    expect(apply({ a: 1, gone: true }, { removed: ["gone"] })).toEqual({ a: 1 });
    expect(apply({ a: 1 }, { removed: true })).toEqual({ a: 1, removed: true });
    expect(apply([{ a: 1 }, { b: 2 }], { removed: ["a"], added: [{ c: 3 }] })).toEqual([{ b: 2 }, { c: 3 }]);
    expect(apply([{ a: { n: 1 } }], { changed: [{ a: { n: 2 } }] })).toEqual([{ a: { n: 2 } }]);
  });

  it("skips empty pairs, null added content, and document apply of removals", () => {
    expect(render([])).toBe("");
    expect(render([{ path: "x.yaml" }])).toBe("");
    expect(parse(render([{ path: "x.yaml", after: null }]))).toEqual({ "x.yaml": { added: true, content: null } });
    expect(applyFile({ a: 1 }, null)).toEqual({ a: 1 });
    expect(applyFile(undefined, 4)).toBe(4);
    expect(applyDocument({ "a.yaml": { n: 1 }, "b.yaml": { n: 2 } }, { "a.yaml": { removed: true } })).toEqual({
      "b.yaml": { n: 2 },
    });
    expect(() => asDocument([], "diff.yaml")).toThrow(/diff.yaml: expected a mapping/);
  });
});
