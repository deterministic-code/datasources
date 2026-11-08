// schema-version: 1.0
/** Type Review. */
export const review = (init: Partial<Review>): Review => ({ ...init } as Review);
export class Review {
  id!: number;
  uuid: string = crypto.randomUUID();
  created: Date = new Date();
  updated: Date = new Date();
  version: string = Buffer.from([0]).toString("base64");
  gameId!: number;
  userId!: number;
  description!: string;
  rating!: string;
  isApproved: boolean = false;
}
