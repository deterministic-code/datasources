export type MemoryRecord = Record<string, unknown>;

export class MemoryStore {
  readonly collections = new Map<string, MemoryRecord[]>();

  hasCollection(name: string): boolean {
    return this.collections.has(name);
  }

  getCollection(name: string): MemoryRecord[] {
    let rows = this.collections.get(name);
    if (!rows) {
      rows = [];
      this.collections.set(name, rows);
    }
    return rows;
  }

  snapshot(): Map<string, MemoryRecord[]> {
    return new Map(
      [...this.collections].map(([name, rows]) => [
        name,
        rows.map((row) => ({ ...row })),
      ]),
    );
  }

  restore(snapshot: Map<string, MemoryRecord[]>): void {
    this.collections.clear();
    for (const [name, rows] of snapshot) {
      this.collections.set(
        name,
        rows.map((row) => ({ ...row })),
      );
    }
  }

  seed(data: MemoryRecord[] | Record<string, MemoryRecord[]>, collection: string): void {
    if (Array.isArray(data)) {
      if (!this.hasCollection(collection)) {
        this.collections.set(
          collection,
          data.map((row) => ({ ...row })),
        );
      }
      return;
    }

    for (const [name, rows] of Object.entries(data)) {
      if (!this.hasCollection(name)) {
        this.collections.set(
          name,
          rows.map((row) => ({ ...row })),
        );
      }
    }
  }
}
