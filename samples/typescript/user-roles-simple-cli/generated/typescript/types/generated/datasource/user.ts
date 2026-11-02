// schema-version: 1.0
/** Type User. */
export const user = (init: Partial<User>): User => ({ ...init } as User);
export class User {
  id!: number;
  uuid: string = crypto.randomUUID();
  created!: Date;
  updated: string = Buffer.from([0]).toString("base64");
  username!: string;
  password!: string;
  isEnabled: boolean = true;
}
