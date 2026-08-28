import type { IConnection } from "./IConnection.ts";
import type { Cells } from "./converters/Cell.ts";
import type { Constructable } from "./converters/Constructable.ts";
import type { FieldConverter } from "./converters/FieldConverter.ts";
import type { TypeMapping } from "./converters/TypeMapping.ts";

export type RepositoryBuilderOptions<T extends Cells> = {
  connection: IConnection;
  ids?: string[];
  assignIdentityOnAdd?: boolean;
  converters?: Record<string, FieldConverter>;
  mapping?: TypeMapping<T>;
  type?: Constructable<T>;
};
