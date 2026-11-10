import type { IConnection } from "./IConnection.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { Cells } from "./converters/Value.ts";

export class RepositoryAccess<T = Cells> {
  constructor(
    private readonly createRepo: (connection: IConnection) => IReadonlyRepository<T>,
    private readonly createReadonlyRepo: (connection: IConnection) => IReadonlyRepository<T>,
    private readonly connection: IConnection,
  ) {}

  create = (): ICRUDRepository<T> => this.createRepo(this.connection) as ICRUDRepository<T>;
  createReadonly = (): IReadonlyRepository<T> => this.createReadonlyRepo(this.connection);
}
