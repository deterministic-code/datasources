import * as nodeFs from "node:fs/promises";
import type { FileSystemApi, IFileStore } from "./i-file-store.ts";

const defaultFs: FileSystemApi = {
  readFile: (path, encoding) => nodeFs.readFile(path, encoding),
  writeFile: (path, contents, encoding) =>
    nodeFs.writeFile(path, contents, encoding),
  access: (path) => nodeFs.access(path),
};

export class NodeFileStore implements IFileStore {
  constructor(private readonly fs: FileSystemApi = defaultFs) {}

  async read(path: string): Promise<string> {
    return this.fs.readFile(path, "utf8");
  }

  async write(path: string, contents: string): Promise<void> {
    await this.fs.writeFile(path, contents, "utf8");
  }

  async exists(path: string): Promise<boolean> {
    try {
      await this.fs.access(path);
      return true;
    } catch {
      return false;
    }
  }
}
