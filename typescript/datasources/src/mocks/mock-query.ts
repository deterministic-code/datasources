import type { IQuery } from "../IQuery.ts";

export class MockQuery implements IQuery {
  getTextCalls = 0;

  constructor(public text: string) {}

  getText(): string {
    this.getTextCalls += 1;
    return this.text;
  }
}
