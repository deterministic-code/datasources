// schema-version: 1.0
import type { Category } from "../datasource/category";
import type { GamesBase } from "../datasource/gamesBase";
/** View Game. */
type Dictionary<K extends PropertyKey, V> = Record<K, V>;
export const game = (init: Partial<Game>): Game => ({ ...init } as Game);
export interface Game extends GamesBase {
  settings: Dictionary<string, string>;
  categories: Category[];
}
