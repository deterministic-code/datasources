import type { IConnection } from "./IConnection.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";

export class RepositoryAccess {
  constructor(
    private readonly createRepo: (connection: IConnection) => ICRUDRepository,
    private readonly createReadonlyRepo: (connection: IConnection) => IReadonlyRepository,
    private readonly connection: IConnection,
  ) {}

  create = (): ICRUDRepository => this.createRepo(this.connection);
  createReadonly = (): IReadonlyRepository => this.createReadonlyRepo(this.connection);
}
