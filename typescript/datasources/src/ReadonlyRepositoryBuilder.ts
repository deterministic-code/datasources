import type { IConnection } from "./IConnection.ts";
import type { Constructable } from "./converters/Constructable.ts";
import { MetaDataRowTypeConverter } from "./converters/MetaDataRowTypeConverter.ts";
import { fieldNamesOf } from "./createCRUDRepositoryOptions.ts";
import type { BuiltRepository } from "./BuiltRepository.ts";
import type { ReadonlyRepositoryOptions } from "./ReadonlyRepositoryOptions.ts";
import { ReadonlyRepositoryStatements } from "./ReadonlyRepositoryStatements.ts";
import { IdentityMapper } from "./mappers/IdentityMapper.ts";

export class ReadonlyRepositoryBuilder<T = object> {
  parts = (connection: IConnection, options: ReadonlyRepositoryOptions<T>): BuiltRepository<T> => {
    const rowConverter = options.rowConverter;
    const objectConverter = options.objectConverter ?? new MetaDataRowTypeConverter<T>();
    const entityMapper = options.entityMapper ?? new IdentityMapper<string>();
    const propertyMappings = options.propertyMappings ?? new IdentityMapper<string>();
    const entity = entityMapper.map(options.entity ?? options.type?.name ?? "");
    const names = options.readProperties ?? (options.type ? fieldNamesOf(options.type) : []);
    if (objectConverter instanceof MetaDataRowTypeConverter && options.type) {
      const instance = new options.type() as Record<string, string | number | boolean | bigint | null | object>;
      objectConverter.references.clear();
      objectConverter.referenceColumns = propertyMappings;
      for (const name of names) {
        const value = instance[name];
        if (value !== null && typeof value === "object" && !Array.isArray(value))
          objectConverter.references.add(name);
      }
    }
    const columns = [...names]
      .map((name) => propertyMappings.map(name))
      .sort((left, right) => left.localeCompare(right));
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

  protected ids = (_options: ReadonlyRepositoryOptions<T>): string[] => [];

  protected statements = (table: string, columns: string[]): ReadonlyRepositoryStatements =>
    new ReadonlyRepositoryStatements(table, columns);
}
