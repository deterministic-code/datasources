import type { IUpdateStatement } from "./IUpdateStatement.ts";
import type { IWhere } from "./IWhere.ts";

export class UpdateStatement implements IUpdateStatement {
  constructor(
    private readonly text: string,
    readonly where?: IWhere,
  ) {}
  getText = (): string => this.text;
}
