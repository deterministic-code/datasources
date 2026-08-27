import {
  type ICommand,
  type IConnection,
  type IDatasource,
  type IRepository,
} from "@datasources/core";
import {
  JsonConnection,
  type JsonConnectionOptions,
} from "./connection.ts";
import { JsonCommand } from "./command.ts";
import { JsonRepository } from "./repository.ts";

export class JsonDatasource implements IDatasource {
  static readonly factory = new JsonDatasource();
  static readonly provider = "json";
  readonly provider = "json";

  constructor(private readonly options: JsonConnectionOptions = {}) {}

  static create(options?: unknown): IDatasource {
    return new JsonDatasource((options ?? {}) as JsonConnectionOptions);
  }

  create(options?: unknown): IDatasource {
    return JsonDatasource.create(options);
  }

  private newConnection(): JsonConnection {
    return new JsonConnection(this.options);
  }

  createConnection(): Promise<IConnection> {
    return Promise.resolve(this.newConnection());
  }

  createCommand(connection?: IConnection): ICommand {
    return new JsonCommand(
      (connection as JsonConnection | undefined) ?? this.newConnection(),
    );
  }

  createRepository<T = unknown>(connection?: IConnection): IRepository<T> {
    return new JsonRepository<T>(
      (connection as JsonConnection | undefined) ?? this.newConnection(),
    );
  }
}
