// schema-version: 1.0
/** Type WardNode. */
export const wardNode = (init: Partial<WardNode>): WardNode => Object.assign(new WardNode(), init);
export class WardNode {
  id!: number;
  parentRegionId!: number;
  parentWardId!: number;
  name: string = "";
}
