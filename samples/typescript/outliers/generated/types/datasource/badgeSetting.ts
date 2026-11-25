// schema-version: 1.0
/** Type BadgeSetting. */
export const badgeSetting = (init: Partial<BadgeSetting>): BadgeSetting => Object.assign(new BadgeSetting(), init);
export class BadgeSetting {
  id!: number;
  parentBadgeId: string = "";
  parentCode: string = "";
  name: string = "";
  value: string = "";
}
