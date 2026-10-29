import { join } from "node:path";
import { describe, expect, it } from "vitest";
import type { Fs } from "./io.ts";
import { csharpBuildProps, wireCsharpBuildProps } from "./wireCsharpBuildProps.ts";

const written = `<Project>
  <PropertyGroup>
    <DefaultItemExcludes>$(DefaultItemExcludes);node_modules/**</DefaultItemExcludes>
  </PropertyGroup>
</Project>
`;

const mem = (files: Record<string, string> = {}): { files: Record<string, string>; fs: Fs } => {
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
      readdir: async () => [],
    },
  };
};

describe("wireCsharpBuildProps", () => {
  it("writes Directory.Build.props when missing and skips when present", async () => {
    expect(csharpBuildProps()).toBe(written);
    expect(csharpBuildProps(written)).toBeUndefined();
    const cwd = "app";
    const { files, fs } = mem();
    const logs: string[] = [];
    await wireCsharpBuildProps(cwd, false, { fs, log: (line) => logs.push(line) });
    expect(files[join(cwd, "Directory.Build.props")]).toBe(written);
    expect(logs).toEqual([join(cwd, "Directory.Build.props")]);
    await wireCsharpBuildProps(cwd, false, { fs, log: () => {} });
    expect(files[join(cwd, "Directory.Build.props")]).toBe(written);
  });

  it("merges node_modules exclude into an existing props file", async () => {
    const existing = `<Project>
  <PropertyGroup>
    <Nullable>enable</Nullable>
  </PropertyGroup>
</Project>
`;
    expect(csharpBuildProps(existing)).toContain("node_modules/**");
    expect(csharpBuildProps(existing)).toContain("<Nullable>enable</Nullable>");
    const cwd = "app";
    const { files, fs } = mem({ [join(cwd, "Directory.Build.props")]: existing });
    await wireCsharpBuildProps(cwd, false, { fs, log: () => {} });
    expect(files[join(cwd, "Directory.Build.props")]).toContain("<Nullable>enable</Nullable>");
    expect(files[join(cwd, "Directory.Build.props")]).toContain("node_modules/**");
  });

  it("logs the path on dry-run without writing", async () => {
    const cwd = "app";
    const { files, fs } = mem();
    const logs: string[] = [];
    await wireCsharpBuildProps(cwd, true, { fs, log: (line) => logs.push(line) });
    expect(files[join(cwd, "Directory.Build.props")]).toBeUndefined();
    expect(logs).toEqual([join(cwd, "Directory.Build.props")]);
  });
});
