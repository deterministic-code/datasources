// schema-version: 1.0
import type { Category } from "../datasource/category";
/** View Game. */
type Dictionary<K extends PropertyKey, V> = Record<K, V>;
export const game = (init: Partial<Game>): Game => Object.assign(new Game(), init);
export class Game {
  id!: number;
  uuid: string = crypto.randomUUID();
  created: Date = new Date();
  updated: Date = new Date();
  version: string = Buffer.from([0]).toString("base64");
  name: string = "";
  description: string = "";
  location: string = "";
  isEnabled: boolean = true;
  isDeleted: boolean = false;
  settings: Dictionary<string, string> = {};
  categories: Category[] = [];
}
