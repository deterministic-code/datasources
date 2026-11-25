// schema-version: 1.0
import type { WardAlias } from "../datasource/wardAlias";
import type { WardNode } from "../datasource/wardNode";
/** View WardView. */
type Dictionary<K extends PropertyKey, V> = Record<K, V>;
export const wardView = (init: Partial<WardView>): WardView => Object.assign(new WardView(), init);
export class WardView {
  regionId!: number;
  wardId!: number;
  name: string = "";
  aliases: WardAlias[] = [];
  settings: Dictionary<string, string> = {};
  nodes: WardNode[] = [];
}
