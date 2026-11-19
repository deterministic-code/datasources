// schema-version: 1.0
/** Type Node. */
export const node = (init: Partial<Node>): Node => Object.assign(new Node(), init);
export class Node {
  id!: number;
  value: string = "";
}
