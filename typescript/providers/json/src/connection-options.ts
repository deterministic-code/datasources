import type { IFileStore } from "./file-store.ts";

export interface JsonConnectionOptions {
  path?: string;
  collection?: string;
  fileStore?: IFileStore;
  data?: unknown;
}
