import type { IConnection } from "./IConnection.ts";
import type { Cells } from "./converters/Cell.ts";
import type { Constructable } from "./converters/Constructable.ts";
import type { TypeConverter } from "./converters/TypeConverter.ts";

export type RepositoryBuilderOptions<T extends Cells> = {
  connection: IConnection;
  ids?: string[];
  assignIdentityOnAdd?: boolean;
  mapping?: TypeConverter<T>;
  type?: Constructable<T>;
};
