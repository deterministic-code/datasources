import type { IQuery } from "./IQuery.ts";

export class Query implements IQuery {
  constructor(private readonly text: string) {}
  getText = (): string => this.text;
}
