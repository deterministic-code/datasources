import {
  type DatasourceOptions,
  type ICommand,
  type IConnection,
  type IDatasource,
  type IReadonlyRepository,
  type IRepository,
} from "@datasources/core";
import type { MemoryConnectionOptions } from "./connection-options.ts";
import { MemoryConnection } from "./connection.ts";
import { MemoryCommand } from "./command.ts";
import { MemoryRepository } from "./repository.ts";
import { MemoryStore } from "./store.ts";

export class MemoryDatasource implements IDatasource {
  static readonly instance = new MemoryDatasource();
  static readonly provider = "memory";
  readonly provider = "memory";
  private readonly options: MemoryConnectionOptions;

  constructor(options: MemoryConnectionOptions = {}) {
    this.options = { ...options, store: options.store ?? new MemoryStore() };
  }

  private connection?: MemoryConnection;

  static create = (options?: DatasourceOptions): IDatasource =>
    new MemoryDatasource((options ?? {}) as MemoryConnectionOptions);

  create = (options?: DatasourceOptions): IDatasource =>
    MemoryDatasource.create(options);

  private newConnection = (): MemoryConnection =>
    new MemoryConnection(this.options);

  private defaultConnection = (): MemoryConnection =>
    (this.connection ??= this.newConnection());

  createConnection = (): Promise<IConnection> =>
    Promise.resolve(this.defaultConnection());

  createCommand = (connection?: IConnection): ICommand =>
    new MemoryCommand(
      (connection as MemoryConnection | undefined) ?? this.defaultConnection(),
    );

  createRepository = <T>(connection?: IConnection): IRepository<T> =>
    new MemoryRepository<T>(
      (connection as MemoryConnection | undefined) ?? this.defaultConnection(),
    );

  createReadonlyRepository = <T>(
    connection?: IConnection,
  ): IReadonlyRepository<T> => this.createRepository(connection);
}
