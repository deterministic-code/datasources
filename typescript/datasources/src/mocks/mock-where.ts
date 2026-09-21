import type { IWhere } from "../statements/IWhere.ts";

export class MockWhere implements IWhere {
  getTextCalls = 0;

  constructor(public text: string) {}

  getText(): string {
    this.getTextCalls += 1;
    return this.text;
  }
}
