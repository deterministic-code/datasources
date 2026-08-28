import type { IWhere } from "../IWhere.ts";
import type { Parameter } from "../Parameter.ts";
import type { Scalar } from "./Scalar.ts";

type CompareNode = { kind: "compare"; field: string; op: string; value: Scalar };
type GroupNode = {
  kind: "group";
  join: "AND" | "OR";
  empty: string;
  children: WhereBuilder[];
};
type NotNode = { kind: "not"; child: WhereBuilder };
type Node = CompareNode | GroupNode | NotNode;

export class WhereBuilder implements IWhere {
  private constructor(private readonly node: Node) {}

  static equals = (field: string, value: Scalar): WhereBuilder =>
    WhereBuilder.compare(field, "=", value);
  static ne = (field: string, value: Scalar): WhereBuilder =>
    WhereBuilder.compare(field, "<>", value);
  static greaterThan = (field: string, value: Scalar): WhereBuilder =>
    WhereBuilder.compare(field, ">", value);
  static lt = (field: string, value: Scalar): WhereBuilder =>
    WhereBuilder.compare(field, "<", value);
  static gte = (field: string, value: Scalar): WhereBuilder =>
    WhereBuilder.compare(field, ">=", value);
  static lte = (field: string, value: Scalar): WhereBuilder =>
    WhereBuilder.compare(field, "<=", value);
  static and = (...children: WhereBuilder[]): WhereBuilder =>
    new WhereBuilder({ kind: "group", join: "AND", empty: "1 = 1", children });
  static or = (...children: WhereBuilder[]): WhereBuilder =>
    new WhereBuilder({ kind: "group", join: "OR", empty: "1 = 0", children });
  static not = (child: WhereBuilder): WhereBuilder =>
    new WhereBuilder({ kind: "not", child });

  private static compare = (
    field: string,
    op: string,
    value: Scalar,
  ): WhereBuilder => new WhereBuilder({ kind: "compare", field, op, value });

  getText = (): string => {
    const { node } = this;
    if (node.kind === "compare") return `${node.field} ${node.op} $${node.field}`;
    if (node.kind === "not") return `NOT (${node.child.getText()})`;
    const [only] = node.children;
    if (!only) return node.empty;
    if (node.children.length === 1) return only.getText();
    return node.children
      .map((child) =>
        child.node.kind === "compare"
          ? child.getText()
          : `(${child.getText()})`,
      )
      .join(` ${node.join} `);
  };

  get parameters(): Parameter[] {
    const { node } = this;
    if (node.kind === "compare") return [{ name: node.field, value: node.value }];
    if (node.kind === "not") return node.child.parameters;
    return node.children.flatMap((child) => child.parameters);
  }
}
