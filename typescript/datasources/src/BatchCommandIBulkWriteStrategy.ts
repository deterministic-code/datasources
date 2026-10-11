import type { CRUDRepository } from "./CRUDRepository.ts";
import type { CRUDRepositoryOptions } from "./CRUDRepositoryOptions.ts";
import type { Response } from "./Response.ts";
import type { IWhere } from "./statements/IWhere.ts";
import type { BulkCommand } from "./BulkCommand.ts";
import { Native } from "./statements/Native.ts";
import type { IBulkWriteStrategy } from "./IBulkWriteStrategy.ts";
import type { ICommand } from "./ICommand.ts";
import type { Cell, Cells } from "./converters/Cell.ts";
import type { Field } from "./builders/Field.ts";
import type { Scalar } from "./builders/Scalar.ts";
import type { IExpression } from "./statements/IExpression.ts";
import { DefaultStatementTextRenderer, Literal, Operator, Where } from "./statements/index.ts";
import { DatasourceError } from "./DatasourceError.ts";
import { repositoryParts } from "./CRUDRepositoryBuilder.ts";
import { MetaDataRowTypeConverter } from "./converters/MetaDataRowTypeConverter.ts";
import type { Parameter } from "./Parameter.ts";
import { afterBulkWrite } from "./BulkWriteEnrichment.ts";

const empty = (): Response => ({ result: null, rowsAffected: 0 });
const budget = 8000;
const maxRows = 250;

const assigned = (value: Cell): boolean =>
  value !== null && value !== "" && value !== 0 && value !== 0n;

const columnsOf = (rows: readonly { name: string }[][]): string[] => {
  const seen = new Set<string>();
  const columns: string[] = [];
  for (const row of rows)
    for (const field of row)
      if (!seen.has(field.name)) {
        seen.add(field.name);
        columns.push(field.name);
      }
  return columns;
};

const lookup = (fields: readonly { name: string; value?: Cell }[]): Map<string, Cell> => {
  const map = new Map<string, Cell>();
  for (const field of fields) map.set(field.name, (field.value ?? null) as Cell);
  return map;
};

const valueOf = (fields: Map<string, Cell>, column: string): Cell => fields.get(column) ?? null;

type Inserted = { table: string; fields: Parameter[] };
type Updated = { table: string; fields: Parameter[]; ids: Parameter[] };
type Deleted = { table: string; ids: Parameter[] };

export class BatchCommandIBulkWriteStrategy<T extends object> implements IBulkWriteStrategy<T> {
  repo!: CRUDRepository<T>;
  command!: ICommand;
  options!: CRUDRepositoryOptions<T>;
  private cached?: ReturnType<typeof repositoryParts>["statements"];

  attach = (command: ICommand, options: CRUDRepositoryOptions<T>): void => {
    this.command = command;
    this.options = options;
  };

  create = (items: T[]): Promise<Response> => this.then(this.writeCreates(items), items, "create");

  update = (items: T[], byWhere: IWhere | undefined): Promise<Response> =>
    this.then(
      byWhere ? this.sendEach(items.map((item) => this.updateCommand(item, byWhere))) : this.writeUpdates(items),
      items,
      "update",
    );

  delete = (items: T[], byWhere: IWhere | undefined): Promise<Response> =>
    this.then(
      byWhere ? this.sendEach(items.map((item) => this.deleteCommand(item, byWhere))) : this.writeDeletes(items),
      items,
      "delete",
    );

  private then = async (write: Promise<Response>, items: T[], mode: "create" | "update" | "delete"): Promise<Response> => {
    const response = await write;
    await afterBulkWrite(this.repo, this.options, items, mode);
    return response;
  };

  private writeCreates = async (items: T[]): Promise<Response> => {
    if (items.length === 0) return empty();
    const rows = items.map(this.mappedInsert);
    const columns = columnsOf(rows.map((row) => row.fields));
    return this.chunked(rows, columns.length, (chunk) => this.execute(this.insertSql(rows[0]!.table, columns, chunk)));
  };

  private writeUpdates = async (items: T[]): Promise<Response> => {
    if (items.length === 0) return empty();
    const rows = items.map(this.mappedUpdate);
    const set = columnsOf(rows.map((row) => row.fields));
    const ids = columnsOf(rows.map((row) => row.ids));
    return this.chunked(rows, set.length + ids.length, (chunk) =>
      this.execute(this.updateSql(rows[0]!.table, set, ids, chunk)),
    );
  };

  private writeDeletes = async (items: T[]): Promise<Response> => {
    if (items.length === 0) return empty();
    const rows = items.map(this.mappedDelete);
    const ids = columnsOf(rows.map((row) => row.ids));
    return this.chunked(rows, ids.length, (chunk) => this.execute(this.deleteSql(rows[0]!.table, ids, chunk)));
  };

  private chunked = async <TRow>(
    rows: TRow[],
    width: number,
    write: (chunk: TRow[]) => Promise<Response>,
  ): Promise<Response> => {
    const size = Math.min(maxRows, Math.max(1, Math.floor(budget / Math.max(1, width))));
    let rowsAffected = 0;
    for (let at = 0; at < rows.length; at += size) rowsAffected += (await write(rows.slice(at, at + size))).rowsAffected;
    return { result: null, rowsAffected };
  };

  private sendEach = async (commands: BulkCommand[]): Promise<Response> => {
    let rowsAffected = 0;
    for (const command of commands)
      rowsAffected += (await this.command.executeNative(new Native(command.text), command.parameters)).rowsAffected;
    return { result: null, rowsAffected };
  };

  private execute = ({ text, parameters }: BulkCommand): Promise<Response> =>
    this.command.executeNative(new Native(text), parameters);

  private mappedInsert = (item: T): Inserted => {
    const record = this.reverse(item);
    const statement = this.statements().insert(
      this.fields(record, (col) => !this.ids.includes(col) || assigned(record[col] ?? null)),
    );
    return { table: statement.table, fields: statement.fields };
  };

  private mappedUpdate = (item: T): Updated => {
    const record = this.reverse(item);
    const fields = this.fields(record, (col) => !this.ids.includes(col));
    if (fields.length === 0) throw new DatasourceError("Update requires at least one non-Id field");
    const statement = this.statements().update(fields, this.idsWhere(record), (name, value) =>
      this.cell(name, value, true),
    );
    return { table: statement.table, fields: statement.fields, ids: statement.where?.parameters ?? [] };
  };

  private mappedDelete = (item: T): Deleted => {
    const record = this.reverse(item);
    const statement = this.statements().delete(this.idsWhere(record), (name, value) => this.cell(name, value, true));
    return { table: statement.table, ids: statement.where?.parameters ?? [] };
  };

  private insertSql = (table: string, columns: string[], rows: Inserted[]): BulkCommand => {
    const parameters: Parameter[] = [];
    const values = rows.map((row, index) => {
      const byName = lookup(row.fields);
      return `(${columns.map((column) => {
        parameters.push({ name: `${column}__${index}`, value: valueOf(byName, column) as Scalar });
        return `$${column}__${index}`;
      }).join(", ")})`;
    });
    return {
      text: `INSERT INTO ${table} (${columns.join(", ")}) VALUES ${values.join(", ")}`,
      parameters,
    };
  };

  private updateSql = (table: string, set: string[], ids: string[], rows: Updated[]): BulkCommand => {
    const parameters: Parameter[] = [];
    const columns = [...set, ...ids];
    const source = `(${rows
      .map((row, index) => {
        const byName = lookup([...row.fields, ...row.ids]);
        return `SELECT ${columns
          .map((column) => {
            parameters.push({ name: `${column}__${index}`, value: valueOf(byName, column) as Scalar });
            return `$${column}__${index} AS ${column}`;
          })
          .join(", ")}`;
      })
      .join(" UNION ALL ")}) AS data`;
    const assigns = set
      .map((column) => (this.mysql ? `${table}.${column} = data.${column}` : `${column} = data.${column}`))
      .join(", ");
    const on = ids.map((column) => `${table}.${column} = data.${column}`).join(" AND ");
    return {
      text: this.mysql
        ? `UPDATE ${table} JOIN ${source} ON ${on} SET ${assigns}`
        : `UPDATE ${table} SET ${assigns} FROM ${source} WHERE ${on}`,
      parameters,
    };
  };

  private deleteSql = (table: string, ids: string[], rows: Deleted[]): BulkCommand => {
    if (ids.length === 0) return { text: `DELETE FROM ${table} WHERE 1 = 1`, parameters: [] };
    const parameters: Parameter[] = [];
    if (ids.length === 1) {
      const column = ids[0]!;
      const names = rows.map((row, index) => {
        parameters.push({ name: `${column}__${index}`, value: valueOf(lookup(row.ids), column) as Scalar });
        return `$${column}__${index}`;
      });
      return { text: `DELETE FROM ${table} WHERE ${column} IN (${names.join(", ")})`, parameters };
    }
    const tuples = rows.map((row, index) => {
      const byName = lookup(row.ids);
      return `(${ids
        .map((column) => {
          parameters.push({ name: `${column}__${index}`, value: valueOf(byName, column) as Scalar });
          return `$${column}__${index}`;
        })
        .join(", ")})`;
    });
    return { text: `DELETE FROM ${table} WHERE (${ids.join(", ")}) IN (${tuples.join(", ")})`, parameters };
  };

  private updateCommand = (item: T, byWhere?: IWhere): BulkCommand => {
    const record = this.reverse(item);
    const fields = this.fields(record, (col) => !this.ids.includes(col));
    if (fields.length === 0) throw new DatasourceError("Update requires at least one non-Id field");
    const statement = this.statements().update(fields, byWhere ?? this.idsWhere(record), (name, value) =>
      this.cell(name, value, byWhere === undefined),
    );
    return { text: DefaultStatementTextRenderer.instance.render(statement), parameters: statement.parameters };
  };

  private deleteCommand = (item: T, byWhere?: IWhere): BulkCommand => {
    const record = this.reverse(item);
    const statement = this.statements().delete(byWhere ?? this.idsWhere(record), (name, value) =>
      this.cell(name, value, byWhere === undefined),
    );
    const listed = statement.where?.parameters ?? [];
    const parameters: Parameter[] = listed.length
      ? listed
      : DefaultStatementTextRenderer.instance.render(statement).includes("$")
        ? this.fields(record, () => true)
        : [];
    return { text: DefaultStatementTextRenderer.instance.render(statement), parameters };
  };

  private statements = () =>
    (this.cached ??= repositoryParts(this.command.connection, this.options).statements);

  private get ids(): readonly string[] {
    return this.options.ids ?? this.repo.ids;
  }

  private get mysql(): boolean {
    return this.command.connection.provider === "mysql";
  }

  private reverse = (item: T): Cells => {
    const objects = this.options.objectConverter ?? new MetaDataRowTypeConverter<T>();
    const row = objects.reverseConvert(item);
    return this.options.rowConverter ? this.options.rowConverter.reverseConvert(row) : row;
  };

  private fields = (record: Cells, include: (col: string) => boolean): Field[] =>
    Object.entries(record)
      .filter(([key, value]) => include(key) && value !== null && value !== undefined)
      .sort(([left], [right]) => left.localeCompare(right))
      .map(([name, value]) => ({ name, value: this.cell(name, value, true) as Scalar }));

  private cell = (property: string, value: Cell, fieldsApplied: boolean): Cell =>
    fieldsApplied
      ? value
      : this.options.objectConverter instanceof MetaDataRowTypeConverter &&
          this.options.objectConverter.fields[property]
        ? this.options.objectConverter.fields[property]!.reverseConvert(value)
        : value;

  private idsWhere = (record: Cells): IWhere => {
    const parts: IExpression[] = this.ids.map((col) => {
      if (!Object.prototype.hasOwnProperty.call(record, col))
        throw new DatasourceError(`Missing Id value "${col}"`);
      return new Operator.Equals(col, record[col] as Scalar);
    });
    return new Where(
      parts.length === 0
        ? new Operator.Equals(new Literal(1), new Literal(1))
        : parts.reduce((left, right) => new Operator.And(left, right)),
    );
  };
}
