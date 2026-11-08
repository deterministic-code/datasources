// schema-version: 1.0
/** Type GameState. */
export const gameState = (init: Partial<GameState>): GameState => ({ ...init } as GameState);
export class GameState {
  id!: number;
  uuid: string = crypto.randomUUID();
  created: Date = new Date();
  updated: Date = new Date();
  version: string = Buffer.from([0]).toString("base64");
  gameId!: number;
  userId!: number;
  state!: string;
}
