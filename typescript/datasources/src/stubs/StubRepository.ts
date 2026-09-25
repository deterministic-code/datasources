import type { Scalar } from "../builders/Scalar.ts";
import type { Cells } from "../converters/Cell.ts";
import type { Constructable } from "../converters/Constructable.ts";
import type { IConnection } from "../IConnection.ts";
import type { Parameter } from "../Parameter.ts";
import type { IExecuteStoredProcedureStatement } from "../statements/IExecuteStoredProcedureStatement.ts";
import type { ISelectStatement } from "../statements/ISelectStatement.ts";
import type { ICRUDRepository } from "../ICRUDRepository.ts";
import { ReadonlyRepository, tracked, untyped, withoutSource, type Sourced } from "../ReadonlyRepository.ts";
import type { Response } from "../Response.ts";
import type { IWhere } from "../statements/IWhere.ts";
import { notImplemented } from "./notImplemented.ts";

export class StubRepository<T = unknown> extends ReadonlyRepository<T> implements ICRUDRepository<T> {
  readonly ids: readonly string[] = [];
  constructor(
    readonly provider: string,
    _connection: IConnection,
    type: Constructable<T> = untyped<T>(),
  ) {
    super(type);
  }

  async *read(
    _query?: ISelectStatement | IWhere | IExecuteStoredProcedureStatement,
    _parameters?: Parameter[],
  ): AsyncIterable<T> {
    notImplemented(this.provider);
  }

  async *[tracked](
    query?: ISelectStatement | IWhere | IExecuteStoredProcedureStatement,
    parameters?: Parameter[],
  ): AsyncIterable<Sourced<T>> {
    yield* withoutSource(this.read(query, parameters));
  }

  create = async (_item: T): Promise<T> => notImplemented(this.provider);

  update = async (_item: T, _byWhere?: IWhere): Promise<Response> =>
    notImplemented(this.provider);

  delete = async (_item: T, _byWhere?: IWhere): Promise<Response> =>
    notImplemented(this.provider);

  readSingleById = (_id: Scalar | Cells): Promise<T | undefined> => notImplemented(this.provider);
}
