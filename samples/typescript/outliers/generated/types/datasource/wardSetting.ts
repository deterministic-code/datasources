// schema-version: 1.0
/** Type WardSetting. */
export const wardSetting = (init: Partial<WardSetting>): WardSetting => Object.assign(new WardSetting(), init);
export class WardSetting {
  id!: number;
  parentRegionId!: number;
  parentWardId!: number;
  name: string = "";
  value: string = "";
}
