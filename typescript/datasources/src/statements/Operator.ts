import type { Cell } from "../converters/Cell.ts";
import { ArrayExpression } from "./ArrayExpression.ts";
import { Identifier } from "./Identifier.ts";
import type { IExpression } from "./IExpression.ts";
import { Literal } from "./Literal.ts";

const pair = (
  left: string | IExpression,
  right: IExpression | Cell,
): [IExpression, IExpression] =>
  typeof left === "string"
    ? [new Identifier(left), new Literal(right as Cell)]
    : [left, right as IExpression];

export abstract class Operator implements IExpression {}

export namespace Operator {
  export class Equals extends Operator {
    readonly left: IExpression;
    readonly right: IExpression;
    constructor(field: string, value: Cell);
    constructor(left: IExpression, right: IExpression);
    constructor(left: string | IExpression, right: IExpression | Cell) {
      super();
      [this.left, this.right] = pair(left, right);
    }
  }

  export class NotEquals extends Operator {
    readonly left: IExpression;
    readonly right: IExpression;
    constructor(field: string, value: Cell);
    constructor(left: IExpression, right: IExpression);
    constructor(left: string | IExpression, right: IExpression | Cell) {
      super();
      [this.left, this.right] = pair(left, right);
    }
  }

  export class GreaterThan extends Operator {
    readonly left: IExpression;
    readonly right: IExpression;
    constructor(field: string, value: Cell);
    constructor(left: IExpression, right: IExpression);
    constructor(left: string | IExpression, right: IExpression | Cell) {
      super();
      [this.left, this.right] = pair(left, right);
    }
  }

  export class LessThan extends Operator {
    readonly left: IExpression;
    readonly right: IExpression;
    constructor(field: string, value: Cell);
    constructor(left: IExpression, right: IExpression);
    constructor(left: string | IExpression, right: IExpression | Cell) {
      super();
      [this.left, this.right] = pair(left, right);
    }
  }

  export class GreaterThanOrEqual extends Operator {
    readonly left: IExpression;
    readonly right: IExpression;
    constructor(field: string, value: Cell);
    constructor(left: IExpression, right: IExpression);
    constructor(left: string | IExpression, right: IExpression | Cell) {
      super();
      [this.left, this.right] = pair(left, right);
    }
  }

  export class LessThanOrEqual extends Operator {
    readonly left: IExpression;
    readonly right: IExpression;
    constructor(field: string, value: Cell);
    constructor(left: IExpression, right: IExpression);
    constructor(left: string | IExpression, right: IExpression | Cell) {
      super();
      [this.left, this.right] = pair(left, right);
    }
  }

  export class And extends Operator {
    constructor(
      readonly left: IExpression,
      readonly right: IExpression,
    ) {
      super();
    }
  }

  export class Or extends Operator {
    constructor(
      readonly left: IExpression,
      readonly right: IExpression,
    ) {
      super();
    }
  }

  export class In extends Operator {
    readonly left: IExpression;
    readonly right: IExpression;
    constructor(field: string, values: Cell[]);
    constructor(left: IExpression, right: IExpression);
    constructor(left: string | IExpression, right: Cell[] | IExpression) {
      super();
      if (typeof left === "string") {
        this.left = new Identifier(left);
        this.right = new ArrayExpression(
          (right as Cell[]).map((value) => new Literal(value)),
        );
      } else {
        this.left = left;
        this.right = right as IExpression;
      }
    }
  }

  /* v8 ignore next 6 */
  export class Not extends Operator {
    readonly argument: IExpression;
    constructor(argument: IExpression) {
      super();
      this.argument = argument;
    }
  }
}
