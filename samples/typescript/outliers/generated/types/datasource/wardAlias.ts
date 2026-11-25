// schema-version: 1.0
/** Type WardAlias. */
export const wardAlias = (init: Partial<WardAlias>): WardAlias => Object.assign(new WardAlias(), init);
export class WardAlias {
  id!: number;
  parentRegionId!: number;
  parentWardId!: number;
  name: string = "";
}
