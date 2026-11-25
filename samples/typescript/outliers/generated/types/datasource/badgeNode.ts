// schema-version: 1.0
/** Type BadgeNode. */
export const badgeNode = (init: Partial<BadgeNode>): BadgeNode => Object.assign(new BadgeNode(), init);
export class BadgeNode {
  id!: number;
  parentBadgeId: string = "";
  parentCode: string = "";
  name: string = "";
}
