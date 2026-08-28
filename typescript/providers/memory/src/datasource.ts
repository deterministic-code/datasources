import {
  type ICommand,
  type IConnection,
  type IDatasource,
  type IReadonlyRepository,
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
    this.options = { ...options, store: options.store ?? new MemoryStore() };
  }

  static create = (options?: unknown): IDatasource =>
    new MemoryDatasource((options ?? {}) as MemoryConnectionOptions);

  create = (options?: unknown): IDatasource => MemoryDatasource.create(options);

  private newConnection = (): MemoryConnection =>
    new MemoryConnection(this.options);

  createConnection = (): Promise<IConnection> =>
    Promise.resolve(this.newConnection());

  createCommand = (connection?: IConnection): ICommand =>
    new MemoryCommand(
      (connection as MemoryConnection | undefined) ?? this.newConnection(),
    );

  createRepository = <T = unknown>(connection?: IConnection): IRepository<T> =>
    new MemoryRepository<T>(
      (connection as MemoryConnection | undefined) ?? this.newConnection(),
    );

  createReadonlyRepository = <T = unknown>(
    connection?: IConnection,
  ): IReadonlyRepository<T> => this.createRepository(connection);
}
