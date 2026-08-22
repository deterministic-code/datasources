import type { IReader } from "../IReader.ts";
import { notImplemented } from "./notImplemented.ts";

export class StubReader<T = unknown> implements IReader<T> {
  current: T | undefined = undefined;

  constructor(readonly provider: string) {}

  async next(): Promise<boolean> {
    notImplemented(this.provider);
  }
}
