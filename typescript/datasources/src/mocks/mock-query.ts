import type { IQuery } from "../IQuery.ts";
import type { IWhere } from "../IWhere.ts";

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
