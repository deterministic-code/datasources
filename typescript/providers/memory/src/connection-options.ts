import type { MemoryEngine, MemoryRecord } from "./engine.ts";

export interface MemoryConnectionOptions {
  collection?: string;
  engine?: MemoryEngine;
  data?: MemoryRecord[] | Record<string, MemoryRecord[]>;
}
