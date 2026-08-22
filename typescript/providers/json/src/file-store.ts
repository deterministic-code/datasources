import * as nodeFs from "node:fs/promises";

export interface IFileStore {
  read(path: string): Promise<string>;
  write(path: string, contents: string): Promise<void>;
  exists(path: string): Promise<boolean>;
}

export interface FileSystemApi {
  readFile(path: string, encoding: "utf8"): Promise<string>;
  writeFile(path: string, contents: string, encoding: "utf8"): Promise<void>;
  access(path: string): Promise<void>;
}

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

export class MemoryFileStore implements IFileStore {
  constructor(public files: Map<string, string> = new Map()) {}

  async read(path: string): Promise<string> {
    const contents = this.files.get(path);
    if (contents === undefined) {
      throw new Error(`File not found: ${path}`);
    }
    return contents;
  }

  async write(path: string, contents: string): Promise<void> {
    this.files.set(path, contents);
  }

  async exists(path: string): Promise<boolean> {
    return this.files.has(path);
  }
}
