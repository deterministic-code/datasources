import { join } from "node:path";
import { parse } from "yaml";
import { describe, expect, it } from "vitest";
import type { Fs } from "../io.ts";
import type { Deps } from "../run.ts";
import type { Prompter } from "../setup.ts";
import { run } from "../run.ts";
import { applyVersion } from "./applyVersion.ts";
import type { Json } from "./delta.ts";

const silentPrompt: Prompter = {
  select: async ({ default: def }) => def,
  checkbox: async ({ choices }) => choices.filter((c) => c.checked).map((c) => c.value),
  input: async ({ default: def }) => def,
  confirm: async ({ default: def }) => def,
};

const tree = (
  files: Record<string, string>,
): { files: Record<string, string>; fs: Fs } => {
  const store = { ...files };
  return {
    files: store,
    fs: {
      readFile: async (path) => {
        const text = store[path];
        if (text === undefined) throw new Error("ENOENT");
        return text;
      },
      writeFile: async (path, text) => {
        store[path] = text;
      },
      readdir: async (path) => {
        const prefix = path.endsWith("/") ? path : `${path}/`;
        const names = new Set<string>();
        for (const file of Object.keys(store)) {
          if (!file.startsWith(prefix)) continue;
          const name = file.slice(prefix.length).split("/")[0];
          if (name) names.add(name);
        }
        return [...names];
      },
    },
  };
};

const deps = (fs: Fs): Deps & { logs: string[] } => {
  const logs: string[] = [];
  return {
    fs,
    run: async () => undefined,
    log: (line) => logs.push(line),
    err: () => undefined,
    prompt: silentPrompt,
    logs,
  };
};

const types = `types:
  - user:
      fields: []
`;

const typesPinned = `types:
  - user:
      fields: []
  - comment:
      fields:
        - is_pinned:
            type: boolean
`;

describe("applyVersion", () => {
  it("writes added envelopes, then a nested delta, then unchanged", async () => {
    const root = join("app", "deterministic");
    const { files, fs } = tree({ [join(root, "types.yaml")]: types });
    const io = deps(fs);
    const now = () => new Date("2026-10-16T08:34:00.000Z");
    const command = { kind: "version" as const, input: "deterministic" };
    const flags = { cwd: "app", languages: [], datasources: [], dryRun: false, yes: false };

    await applyVersion(command, flags, io, now);
    const first = join(root, "version-20261016T083400Z-001", "diff.yaml");
    expect(io.logs).toEqual([first]);
    expect((parse(files[first]!) as { [path: string]: Json })["types.yaml"]).toEqual({
      added: true,
      content: { types: [{ user: { fields: [] } }] },
    });

    files[join(root, "types.yaml")] = typesPinned;
    io.logs.length = 0;
    await applyVersion(command, flags, io, now);
    const second = join(root, "version-20261016T083400Z-002", "diff.yaml");
    expect(io.logs).toEqual([second]);
    expect((parse(files[second]!) as { [path: string]: Json })["types.yaml"]).toEqual({
      types: { added: [{ comment: { fields: [{ is_pinned: { type: "boolean" } }] } }] },
    });

    io.logs.length = 0;
    await applyVersion(command, flags, io, now);
    expect(io.logs).toEqual(["unchanged"]);
    expect(files[join(root, "version-20261016T083400Z-003", "diff.yaml")]).toBeUndefined();
  });

  it("logs the path on dry-run without writing", async () => {
    const root = join("app", "deterministic");
    const { files, fs } = tree({ [join(root, "types.yaml")]: types });
    const io = deps(fs);
    await applyVersion(
      { kind: "version", input: "deterministic" },
      { cwd: "app", dryRun: true, yes: false },
      io,
      () => new Date("2026-10-16T08:34:00.000Z"),
    );
    const path = join(root, "version-20261016T083400Z-001", "diff.yaml");
    expect(io.logs).toEqual([path]);
    expect(files[path]).toBeUndefined();
  });

  it("dispatches from run and skips empty or missing prior diffs", async () => {
    const root = join("app", "deterministic");
    const { files, fs } = tree({
      [join(root, "types.yaml")]: types,
      [join(root, "version-20261016T000000Z-001", "diff.yaml")]: "",
      [join(root, "version-20261016T000000Z-002", "other.yaml")]: "skip",
    });
    const io = deps(fs);
    expect(await run(["version", "--cwd", "app"], io)).toBe(0);
    expect(io.logs[0]).toMatch(/version-\d{8}T\d{6}Z-003\/diff\.yaml$/);
    expect(files[io.logs[0]!]).toMatch(/added: true/);
  });

  it("uses an absolute input, records removals, and refuses sequence 1000", async () => {
    const root = "/tmp/deterministic-spec";
    const now = () => new Date("2026-10-16T08:34:00.000Z");
    const flags = { cwd: "app", languages: [], datasources: [], dryRun: false, yes: false };
    const { files, fs } = tree({ [join(root, "types.yaml")]: types });
    const io = deps(fs);
    await applyVersion({ kind: "version", input: root }, flags, io, now);
    delete files[join(root, "types.yaml")];
    io.logs.length = 0;
    await applyVersion({ kind: "version", input: root }, flags, io, now);
    expect((parse(files[io.logs[0]!]!) as { [path: string]: Json })["types.yaml"]).toEqual({ removed: true });

    const overflow = deps(
      tree({
        [join(root, "types.yaml")]: types,
        [join(root, "version-20261016T000000Z-999", "diff.yaml")]: "types.yaml:\n  added: true\n  content: {}\n",
      }).fs,
    );
    await expect(applyVersion({ kind: "version", input: root }, flags, overflow, now)).rejects.toThrow(
      /version sequence exceeds 999/,
    );

    const bad = deps(
      tree({
        [join(root, "types.yaml")]: types,
        [join(root, "version-20261016T000000Z-001", "diff.yaml")]: "- not a mapping\n",
      }).fs,
    );
    await expect(applyVersion({ kind: "version", input: root }, flags, bad, now)).rejects.toThrow(/expected a mapping/);
  });
});
