import type { IQuery } from "../statements/IQuery.ts";
import type { IWhere } from "../statements/IWhere.ts";

export class MockQuery implements IQuery {
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
