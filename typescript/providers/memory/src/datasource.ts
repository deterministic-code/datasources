import {
  type ICommand,
  type IConnection,
  type IDatasource,
  type IRepository,
} from "@datasources/core";
import {
  MemoryConnection,
  type MemoryConnectionOptions,
} from "./connection.ts";
import { MemoryCommand } from "./command.ts";
import { MemoryRepository } from "./repository.ts";
import { MemoryStore } from "./store.ts";

export class MemoryDatasource implements IDatasource {
  static readonly factory = new MemoryDatasource();
  static readonly provider = "memory";
  readonly provider = "memory";
  private readonly options: MemoryConnectionOptions;

  constructor(options: MemoryConnectionOptions = {}) {
    this.options = {
      ...options,
      store: options.store ?? new MemoryStore(),
    };
  }

  static create(options?: unknown): IDatasource {
    return new MemoryDatasource((options ?? {}) as MemoryConnectionOptions);
  }

  create(options?: unknown): IDatasource {
    return MemoryDatasource.create(options);
  }

  private newConnection(): MemoryConnection {
    return new MemoryConnection(this.options);
  }

  createConnection(): Promise<IConnection> {
    return Promise.resolve(this.newConnection());
  }

  createCommand(connection?: IConnection): ICommand {
    return new MemoryCommand(
      (connection as MemoryConnection | undefined) ?? this.newConnection(),
    );
  }

  createRepository<T = unknown>(connection?: IConnection): IRepository<T> {
    return new MemoryRepository<T>(
      (connection as MemoryConnection | undefined) ?? this.newConnection(),
    );
  }
}
