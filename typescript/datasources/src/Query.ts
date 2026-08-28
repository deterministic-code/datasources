import type { IQuery } from "./IQuery.ts";
import type { IWhere } from "./IWhere.ts";

export class Query implements IQuery {
  constructor(
    private readonly text: string,
    readonly where?: IWhere,
  ) {}
  getText = (): string => this.text;
}
