import type { ICommand } from "./ICommand.ts";
import type { IConnection } from "./IConnection.ts";
import type { IDeleteStatement } from "./builders/IDeleteStatement.ts";
import type { IInsertStatement } from "./builders/IInsertStatement.ts";
import type { IQuery } from "./IQuery.ts";
import type { IUpdateStatement } from "./builders/IUpdateStatement.ts";
import type { IWhere } from "./IWhere.ts";
import { StatementBuilder } from "./builders/index.ts";
import type { Field } from "./builders/Field.ts";
import type { Cells } from "./converters/Cell.ts";
import type { Constructable } from "./converters/Constructable.ts";
import { TypeConverter } from "./converters/TypeConverter.ts";
import type { TypeMapping } from "./converters/TypeMapping.ts";
import { Mapper } from "./mappers/Mapper.ts";
import { createRepositoryBuilderOptions } from "./createRepositoryBuilderOptions.ts";
import type { RepositoryBuilderOptions } from "./RepositoryBuilderOptions.ts";

export type RepositoryCommands = {
  get: ICommand;
  add: ICommand;
  update: ICommand;
  delete: ICommand;
};

export type RepositoryStatements = {
  query: (where?: IWhere) => IQuery;
  insert: (fields: Field[]) => IInsertStatement;
  update: (fields: Field[], where?: IWhere) => IUpdateStatement;
  delete: (where?: IWhere) => IDeleteStatement;
};

export type BuiltRepository<T extends Cells> = {
  commands: RepositoryCommands;
  converters: TypeMapping<T>;
  mapping: Mapper;
  statements: RepositoryStatements;
  ids: string[];
  assignIdentityOnAdd: boolean;
};

const commandsOf = (connection: IConnection): RepositoryCommands => ({
  get: connection.createCommand(),
  add: connection.createCommand(),
  update: connection.createCommand(),
  delete: connection.createCommand(),
});

const statementsOf = (table: string, columns: string[]): RepositoryStatements => ({
  query: (where) => StatementBuilder.createQuery(table, columns, where),
  insert: (fields) => StatementBuilder.createInsert(table, fields),
  update: (fields, where) => StatementBuilder.createUpdate(table, fields, where),
  delete: (where) => StatementBuilder.createDelete(table, where),
});

export const parts = <T extends Cells>(
  options: RepositoryBuilderOptions<T>,
): BuiltRepository<T> => {
  const converters =
    options.mapping ??
    TypeConverter.mapping<T>({
      source: options.connection.provider,
      type: options.type,
      converters: options.converters,
    });
  TypeConverter.table(options.connection.provider);
  const mapping = new Mapper(converters.name, options.connection.provider, "typescript");
  return {
    commands: commandsOf(options.connection),
    converters,
    mapping,
    statements: statementsOf(mapping.name, TypeConverter.keys(options.type).sort()),
    ids: options.ids ?? ["id"],
    assignIdentityOnAdd: options.assignIdentityOnAdd !== false,
  };
};

export class RepositoryBuilder<T extends Cells> {
  constructor(private readonly options: RepositoryBuilderOptions<T>) {}

  static from<T extends Cells>(type: Constructable<T>, connection: IConnection): RepositoryBuilder<T>;
  static from<T extends Cells>(options: RepositoryBuilderOptions<T>): RepositoryBuilder<T>;
  static from<T extends Cells>(
    typeOrOptions: Constructable<T> | RepositoryBuilderOptions<T>,
    connection?: IConnection,
  ): RepositoryBuilder<T> {
    return new RepositoryBuilder(
      connection === undefined
        ? (typeOrOptions as RepositoryBuilderOptions<T>)
        : createRepositoryBuilderOptions(typeOrOptions as Constructable<T>, connection),
    );
  }

  parts = (): BuiltRepository<T> => parts(this.options);
}
