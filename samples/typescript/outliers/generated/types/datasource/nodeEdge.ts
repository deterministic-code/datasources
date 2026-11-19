// schema-version: 1.0
/** Type NodeEdge. */
export const nodeEdge = (init: Partial<NodeEdge>): NodeEdge => Object.assign(new NodeEdge(), init);
export class NodeEdge {
  fromNodeId!: number;
  toNodeId!: number;
}
