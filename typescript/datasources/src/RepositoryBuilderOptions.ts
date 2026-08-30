import type { IConnection } from "./IConnection.ts";
import type { Cells } from "./converters/Cell.ts";
import type { Constructable } from "./converters/Constructable.ts";
import type { TypeConverter } from "./converters/TypeConverter.ts";
import type { ValueConverter } from "./converters/ValueConverter.ts";

export type RepositoryBuilderOptions<T extends Cells> = {
  connection: IConnection;
  ids?: string[];
  assignIdentityOnAdd?: boolean;
  table?: string;
  mapping?: TypeConverter<T>;
  type?: Constructable<T>;
  converters?: Record<string, ValueConverter>;
  fields?: Record<string, ValueConverter>;
};
