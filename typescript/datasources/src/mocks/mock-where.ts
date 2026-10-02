import type { IConditionVisitor } from "../statements/IConditionVisitor.ts";
import type { IWhere } from "../statements/IWhere.ts";

export class MockWhere implements IWhere {
  acceptCalls = 0;

  constructor(public text: string) {}

  accept(_visitor: IConditionVisitor): string {
    this.acceptCalls += 1;
    return this.text;
  }
}
