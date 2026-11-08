// schema-version: 1.0
/** Type GamesCategory. */
export const gamesCategory = (init: Partial<GamesCategory>): GamesCategory => ({ ...init } as GamesCategory);
export class GamesCategory {
  id!: number;
  uuid: string = crypto.randomUUID();
  created: Date = new Date();
  updated: Date = new Date();
  version: string = Buffer.from([0]).toString("base64");
  gameId!: number;
  categoryId!: number;
}
