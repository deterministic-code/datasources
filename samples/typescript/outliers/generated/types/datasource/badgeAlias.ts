// schema-version: 1.0
/** Type BadgeAlias. */
export const badgeAlias = (init: Partial<BadgeAlias>): BadgeAlias => Object.assign(new BadgeAlias(), init);
export class BadgeAlias {
  id!: number;
  parentBadgeId: string = "";
  parentCode: string = "";
  name: string = "";
}
