import type { ISelectStatement } from "../statements/ISelectStatement.ts";
import type { IWhere } from "../statements/IWhere.ts";

export class MockSelectStatement implements ISelectStatement {
  getTextCalls = 0;

  constructor(
    public text: string,
    public where?: IWhere,
  ) {}

  getText(): string {
    this.getTextCalls += 1;
    return this.text;
  }
}
