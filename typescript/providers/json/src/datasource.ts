import {
  type ICommand,
  type IConnection,
  type IDatasource,
  type IRepository,
  type RepositoryOptions,
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

  createConnection(): IConnection {
    return new JsonConnection(this.options);
  }

  createCommand(connection?: IConnection): ICommand {
    return new JsonCommand(
      (connection as JsonConnection | undefined) ??
        (this.createConnection() as JsonConnection),
    );
  }

  createRepository<T = unknown>(
    connection?: IConnection,
    options?: RepositoryOptions,
  ): IRepository<T> {
    const conn =
      (connection as JsonConnection | undefined) ??
      (this.createConnection() as JsonConnection);
    if (options?.collection) {
      conn.collection = options.collection;
    }
    return new JsonRepository<T>(conn);
  }
}
