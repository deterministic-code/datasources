import type { MemoryRecord, MemoryStore } from "./store.ts";

export interface MemoryConnectionOptions {
  collection?: string;
  store?: MemoryStore;
  data?: MemoryRecord[] | Record<string, MemoryRecord[]>;
}
