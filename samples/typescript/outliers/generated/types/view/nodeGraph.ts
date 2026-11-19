// schema-version: 1.0
/** View NodeGraph. */
export const nodeGraph = (init: Partial<NodeGraph>): NodeGraph => Object.assign(new NodeGraph(), init);
export class NodeGraph {
  id!: number;
  value: string = "";
  edges: NodeGraph[] = [];
}
