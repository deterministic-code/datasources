import type { IFileStore } from "./i-file-store.ts";

export class MemoryFileStore implements IFileStore {
  constructor(public files: Map<string, string> = new Map()) {}

  async read(path: string): Promise<string> {
    const contents = this.files.get(path);
    if (contents === undefined) throw new Error(`File not found: ${path}`);
    return contents;
  }

  async write(path: string, contents: string): Promise<void> {
    this.files.set(path, contents);
  }

  async exists(path: string): Promise<boolean> {
    return this.files.has(path);
  }
}
