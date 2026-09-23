import type { IMapper } from "./IMapper.ts";

export class DictionaryMapper<T extends string | number> implements IMapper<T, T> {
  private readonly forward = new Map<T, T>();
  private readonly reverse = new Map<T, T>();

  constructor(map: readonly (readonly [T, T])[]) {
    for (const [from, to] of map) {
      this.forward.set(from, to);
      if (!this.reverse.has(to)) this.reverse.set(to, from);
    }
  }

  map = (value: T): T => this.forward.get(value) ?? value;
  reverseMap = (value: T): T => this.reverse.get(value) ?? value;
}
