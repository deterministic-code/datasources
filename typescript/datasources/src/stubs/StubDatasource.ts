import type { ICommand } from "../ICommand.ts";
import type { IConnection } from "../IConnection.ts";
import type { IDatasource } from "../IDatasource.ts";
import type { IRepository } from "../IRepository.ts";
import { StubCommand } from "./StubCommand.ts";
import { StubConnection } from "./StubConnection.ts";
import { StubRepository } from "./StubRepository.ts";

export class StubDatasource implements IDatasource {
  constructor(readonly provider: string) {}

  create(_options?: unknown): IDatasource {
    return new StubDatasource(this.provider);
  }

  private newConnection(): IConnection {
    return new StubConnection(this.provider);
  }

  createConnection(): Promise<IConnection> {
    return Promise.resolve(this.newConnection());
  }

  createCommand(connection?: IConnection): ICommand {
    return new StubCommand(this.provider, connection ?? this.newConnection());
  }

  createRepository<T = unknown>(connection?: IConnection): IRepository<T> {
    return new StubRepository<T>(
      this.provider,
      connection ?? this.newConnection(),
    );
  }
}
