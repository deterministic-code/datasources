import type { IConnection } from "./IConnection.ts";
import type { Constructable } from "./converters/Constructable.ts";
import { MetaDataRowTypeConverter } from "./converters/MetaDataRowTypeConverter.ts";
import { RowTypeConverter } from "./converters/RowTypeConverter.ts";
import { fieldNamesOf } from "./createReadonlyRepositoryMappingOptions.ts";
import type { BuiltRepository } from "./BuiltRepository.ts";
import { DatasourceFactory } from "./DatasourceFactory.ts";
import type { IMapper } from "./mappers/IMapper.ts";
import type { ReadonlyRepositoryMappingOptions } from "./ReadonlyRepositoryMappingOptions.ts";
import { ReadonlyRepositoryStatements } from "./ReadonlyRepositoryStatements.ts";
import { IdentityMapper } from "./mappers/IdentityMapper.ts";

export class ReadonlyRepositoryBuilder<T = object> {
  protected entityMapper: IMapper<string, string> = new IdentityMapper();
  protected propertyMapper: IMapper<string, string> = new IdentityMapper();

  parts = (connection: IConnection, options: ReadonlyRepositoryMappingOptions<T>): BuiltRepository<T> => {
    const converters = options.rowTypeConverters ?? DatasourceFactory.rowTypeConverters(connection.provider);
    const rowConverter =
      options.rowConverter ??
      (options.schema && converters
        ? RowTypeConverter.from(converters, options.schema.types())
        : undefined);
    const objectConverter = options.objectConverter ?? new MetaDataRowTypeConverter<T>();
    const entityMapper = options.entityMapper ?? new IdentityMapper<string>();
    const propertyMappings = options.propertyMappings ?? new IdentityMapper<string>();
    this.entityMapper = entityMapper;
    this.propertyMapper = propertyMappings;
    const entity = options.entity ?? options.type?.name ?? "";
    const names = options.readProperties ?? (options.type ? fieldNamesOf(options.type) : []);
    if (objectConverter instanceof MetaDataRowTypeConverter && options.type) {
      const instance = new options.type() as Record<string, string | number | boolean | bigint | null | object>;
      objectConverter.references.clear();
      objectConverter.referenceColumns = propertyMappings;
      objectConverter.ids = options.schema?.ids().length ? options.schema.ids() : this.ids(options);
      for (const name of names) {
        const value = instance[name];
        if (
          value !== null &&
          typeof value === "object" &&
          !Array.isArray(value) &&
          !(value instanceof Date) &&
          !(value instanceof Uint8Array)
        )
          objectConverter.references.add(name);
      }
    }
    const columns = [...names].sort((left, right) =>
      propertyMappings.map(left).localeCompare(propertyMappings.map(right)));
    return {
      connection,
      rowConverter,
      objectConverter,
      propertyMappings,
      entityMapper,
      statements: this.statements(entity, columns),
      ids: this.ids(options),
      type: options.type ?? (Object as never as Constructable<T>),
    };
  };

  protected ids = (_options: ReadonlyRepositoryMappingOptions<T>): string[] => [];

  protected statements = (table: string, columns: string[]): ReadonlyRepositoryStatements =>
    new ReadonlyRepositoryStatements(table, columns);
}
