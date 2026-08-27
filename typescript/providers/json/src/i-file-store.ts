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
