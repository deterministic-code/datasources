import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { dirname, join } from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";
import { MESSAGE, blocks, isGeneratedPath, shellWritesGenerated } from "./block-generated.mjs";

const script = join(dirname(fileURLToPath(import.meta.url)), "block-generated.mjs");

const hook = (payload) =>
  spawnSync(process.execPath, [script], { input: JSON.stringify(payload), encoding: "utf8" });

test("a generated directory is any path segment named generated", () => {
  assert.equal(isGeneratedPath("generated/typescript/repositories/ContactRepository.cs"), true);
  assert.equal(isGeneratedPath("/proj/samples/app/generated/csharp/Contact.cs"), true);
  assert.equal(isGeneratedPath("C:\\proj\\generated\\Foo.cs"), true);
  assert.equal(isGeneratedPath("src/datasources-register.ts"), false);
  assert.equal(isGeneratedPath("src/generated.ts"), false);
});

test("edit tools under generated are blocked and other files are not", () => {
  assert.equal(blocks({ tool_name: "Write", tool_input: { path: "generated/Foo.cs", contents: "" } }), true);
  assert.equal(
    blocks({
      tool_name: "StrReplace",
      tool_input: { path: "samples/app/generated/csharp/Contact.cs", old_string: "a", new_string: "b" },
    }),
    true,
  );
  assert.equal(
    blocks({ tool_name: "Edit", tool_input: { file_path: "/proj/generated/Foo.cs", old_string: "a", new_string: "b" } }),
    true,
  );
  assert.equal(blocks({ tool_name: "Delete", tool_input: { path: "generated/Foo.cs" } }), true);
  assert.equal(
    blocks({ tool_name: "Write", tool_input: { path: "src/App.cs", contents: "see generated/Foo.cs" } }),
    false,
  );
  assert.equal(blocks({ tool_name: "Read", tool_input: { file_path: "generated/Foo.cs" } }), false);
});

test("shell writes under generated are blocked and the CLI is allowed", () => {
  assert.equal(shellWritesGenerated("echo x > generated/hack.cs"), true);
  assert.equal(shellWritesGenerated("rm -rf generated"), true);
  assert.equal(shellWritesGenerated("cp src/App.cs generated/App.cs"), true);
  assert.equal(shellWritesGenerated("cp generated/App.cs src/App.cs"), false);
  assert.equal(shellWritesGenerated("rg Contact generated"), false);
  assert.equal(shellWritesGenerated("cat generated/Foo.cs"), false);
  assert.equal(
    shellWritesGenerated(
      "npx @deterministic-code/datasources-cli generate all --dialect typescript --output ./generated",
    ),
    false,
  );
  assert.equal(
    shellWritesGenerated(
      "npx @deterministic-code/datasources-cli generate all --output ./generated && echo x > generated/hack.cs",
    ),
    true,
  );
  assert.equal(shellWritesGenerated("npm run cli -- generate all --dialect csharp"), false);
});

test("cursor and claude both return the generate instruction", () => {
  const cursor = hook({ tool_name: "Write", tool_input: { path: "generated/Foo.cs", contents: "x" } });
  assert.equal(cursor.status, 0);
  assert.deepEqual(JSON.parse(cursor.stdout), {
    permission: "deny",
    user_message: MESSAGE,
    agent_message: MESSAGE,
  });
  assert.equal(MESSAGE, "edit types.yaml or datasource.yaml and run generate all");

  const claude = hook({
    hook_event_name: "PreToolUse",
    tool_name: "Edit",
    tool_input: { file_path: "/proj/generated/Foo.cs", old_string: "a", new_string: "b" },
  });
  assert.equal(claude.status, 0);
  assert.deepEqual(JSON.parse(claude.stdout), {
    hookSpecificOutput: {
      hookEventName: "PreToolUse",
      permissionDecision: "deny",
      permissionDecisionReason: MESSAGE,
    },
  });

  const allow = hook({ tool_name: "Read", tool_input: { file_path: "src/App.cs" } });
  assert.equal(allow.status, 0);
  assert.deepEqual(JSON.parse(allow.stdout), { permission: "allow" });
});
