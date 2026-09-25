import type { IWhere } from "./IWhere.ts";

export class WhereText implements IWhere {
  constructor(private readonly text: string) {}
  getText = (): string => this.text;
}
