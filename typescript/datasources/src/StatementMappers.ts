import type { Cell } from "./converters/Cell.ts";
import { DatasourceError } from "./DatasourceError.ts";
import type { IMapper } from "./mappers/IMapper.ts";
import { applySelectFieldAlias, asIndexOf, functionOf, segments, selectFieldAliases } from "./statements/SelectFieldAliases.ts";
import { DeleteStatement } from "./statements/DeleteStatement.ts";
import type { IStatement } from "./statements/IStatement.ts";
import type { IWhere } from "./statements/IWhere.ts";
import { InsertStatement } from "./statements/InsertStatement.ts";
import { SelectInsertedIdStatement } from "./statements/SelectInsertedIdStatement.ts";
import { SelectStatement } from "./statements/SelectStatement.ts";
import { UpdateStatement } from "./statements/UpdateStatement.ts";
import { Where } from "./statements/Where.ts";

export class StatementMappers implements IMapper<string, string> {
  constructor(
    private readonly entityMappers: IMapper<string, string>,
    private readonly propertyMappers: IMapper<string, string>,
  ) {}

  map = (property: string): string => {
    const parts = segments(property);
    if (parts.length < 2) return this.propertyMappers.map(property);
    const qualifier = parts[parts.length - 2];
    const name = parts[parts.length - 1];
    const mapped = this.propertyMappers.map(name);
    if (mapped !== name && mapped.includes(".")) return mapped;
    const table = this.entityMappers.map(qualifier);
    return table === qualifier && mapped === name ? `${qualifier}.${name}` : `${table}.${mapped}`;
  };

  reverseMap = (key: string): string => {
    const reversed = this.propertyMappers.reverseMap(key);
    if (reversed !== key && this.propertyMappers.map(reversed) === key) return reversed;
    const parts = segments(key);
    if (parts.length < 2) return reversed;
    const column = parts[parts.length - 1];
    const name = this.propertyMappers.reverseMap(column);
    return name !== column && this.propertyMappers.map(name) === column ? name : reversed;
  };

  column = (property: string): string => this.map(property);

  property = (key: string): string => this.reverseMap(key);

  apply = (
    query: IStatement | undefined,
    cell: (property: string, value: Cell) => Cell,
  ): IStatement | undefined => {
    if (query instanceof SelectStatement && query.table)
      return new SelectStatement({
        distinct: query.distinct,
        table: this.entityMappers.map(query.table),
        fields: this.mapSelectFields(query.table, query.fields, query.aliases),
        where: query.where instanceof Where ? this.applyWhere(query.where, cell) : query.where,
        joins: query.joins.map((join) => ({
          kind: join.kind,
          table: this.entityMappers.map(join.table),
          on: join.on ? new Where(join.on).map(this.column, cell).condition : undefined,
        })),
        groupBy: query.groupBy.map(this.column),
        having: query.having instanceof Where ? this.applyWhere(query.having, cell) : query.having,
        orderBy: query.orderBy.map((order) => ({ column: this.column(order.column), direction: order.direction })),
        limit: query.limit,
        offset: query.offset,
      });
    if (query instanceof InsertStatement && query.table !== undefined)
      return new InsertStatement(
        this.entityMappers.map(query.table),
        query.fields.map((field) => ({ name: this.column(field.name), value: field.value })),
      );
    if (query instanceof UpdateStatement && query.table !== undefined)
      return new UpdateStatement(
        this.entityMappers.map(query.table),
        query.fields.map((field) => ({ name: this.column(field.name), value: field.value })),
        query.where instanceof Where ? this.applyWhere(query.where, cell) : query.where,
      );
    if (query instanceof DeleteStatement && query.table !== undefined)
      return new DeleteStatement(
        this.entityMappers.map(query.table),
        query.where instanceof Where ? this.applyWhere(query.where, cell) : query.where,
      );
    if (query instanceof SelectInsertedIdStatement)
      return new SelectInsertedIdStatement(this.entityMappers.map(query.table), this.column(query.column));
    return query;
  };

  applyWhere = (
    where: IWhere | undefined,
    cell: (property: string, value: Cell) => Cell,
  ): IWhere | undefined =>
    where instanceof Where ? where.map(this.column, cell) : where;

  private mapSelectFields = (
    table: string,
    fields: readonly string[],
    aliases: readonly (string | undefined)[],
  ): string[] => {
    const lookup = selectFieldAliases(table, fields, aliases);
    return fields.map((field, i) => this.mapSelectField(field, lookup[i]));
  };

  private mapSelectField = (field: string, alias?: string): string => {
    if (asIndexOf(field) >= 0) throw new DatasourceError(`Select field "${field}" must be a path`);
    const functionCall = functionOf(field);
    const mapped = functionCall === undefined ? this.map(field) : `${functionCall.name}(${this.map(functionCall.inner)})`;
    return applySelectFieldAlias(mapped, alias);
  };
}
