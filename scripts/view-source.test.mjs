import assert from "node:assert/strict";
import test from "node:test";
import { sourcePage } from "./view-source.mjs";

test("view source renders the recorded line range", () => {
  const html = sourcePage("/view-source?path=package.json&from=1&to=1");
  assert.match(html, /<h1>package\.json<\/h1>/);
  assert.match(html, /<tr id="L1" class="hit">/);
  assert.match(html, /datasources/);
});

test("view source rejects a path that leaves the repo", () => {
  assert.equal(sourcePage("/view-source?path=../package.json"), null);
  assert.equal(sourcePage("/view-source?path=docs/../../package.json"), null);
  assert.equal(sourcePage("/view-source?path=/etc/passwd"), null);
});
