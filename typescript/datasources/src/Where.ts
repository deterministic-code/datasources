import type { IWhere } from "./IWhere.ts";

export class Where implements IWhere {
  constructor(private readonly text: string) {}

  getText(): string {
    return this.text;
  }
}
