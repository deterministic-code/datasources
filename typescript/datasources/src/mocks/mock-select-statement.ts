import type { ISelectStatement } from "../statements/ISelectStatement.ts";
import type { IStatementVisitor } from "../statements/IStatementVisitor.ts";
import type { IWhere } from "../statements/IWhere.ts";

export class MockSelectStatement implements ISelectStatement {
  acceptCalls = 0;

  constructor(
    public text: string,
    public where?: IWhere,
  ) {}

  accept(_visitor: IStatementVisitor): string {
    this.acceptCalls += 1;
    return this.text;
  }
}
