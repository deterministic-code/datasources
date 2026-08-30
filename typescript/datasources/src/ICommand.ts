import type { ICommandMiddleware } from "./ICommandMiddleware.ts";
import type { IConnection } from "./IConnection.ts";
import type { IReader } from "./IReader.ts";
import type { IResponse } from "./IResponse.ts";
import type { Parameter } from "./Parameter.ts";

export interface ICommand {
  readonly connection: IConnection;
  use(middleware: ICommandMiddleware): this;
  executeReader(text: string, parameters?: Parameter[]): Promise<IReader>;
  execute(text: string, parameters?: Parameter[]): Promise<IResponse>;
}
