import type { ICommand } from "../ICommand.ts";
import type { IConnection } from "../IConnection.ts";
import type { IDatasource } from "../IDatasource.ts";
import type { IRepository } from "../IRepository.ts";
import type { RepositoryOptions } from "../RepositoryOptions.ts";
import { StubCommand } from "./StubCommand.ts";
import { StubConnection } from "./StubConnection.ts";
import { StubRepository } from "./StubRepository.ts";

export class StubDatasource implements IDatasource {
  constructor(readonly provider: string) {}

  create(_options?: unknown): IDatasource {
    return new StubDatasource(this.provider);
  }

  createConnection(): IConnection {
    return new StubConnection(this.provider);
  }

  createCommand(connection?: IConnection): ICommand {
    return new StubCommand(
      this.provider,
      connection ?? this.createConnection(),
    );
  }

  createRepository<T = unknown>(
    connection?: IConnection,
    _options?: RepositoryOptions,
  ): IRepository<T> {
    return new StubRepository<T>(
      this.provider,
      connection ?? this.createConnection(),
    );
  }
}
