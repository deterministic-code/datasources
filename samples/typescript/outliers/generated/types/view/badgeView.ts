// schema-version: 1.0
import type { BadgeAlias } from "../datasource/badgeAlias";
import type { BadgeNode } from "../datasource/badgeNode";
/** View BadgeView. */
type Dictionary<K extends PropertyKey, V> = Record<K, V>;
export const badgeView = (init: Partial<BadgeView>): BadgeView => Object.assign(new BadgeView(), init);
export class BadgeView {
  badgeId: string = "";
  code: string = "";
  name: string = "";
  aliases: BadgeAlias[] = [];
  settings: Dictionary<string, string> = {};
  nodes: BadgeNode[] = [];
}
