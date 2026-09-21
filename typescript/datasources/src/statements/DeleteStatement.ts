import type { IDeleteStatement } from "./IDeleteStatement.ts";
import type { IWhere } from "./IWhere.ts";

export class DeleteStatement implements IDeleteStatement {
  constructor(
    private readonly text: string,
    readonly where?: IWhere,
  ) {}
  getText = (): string => this.text;
}
