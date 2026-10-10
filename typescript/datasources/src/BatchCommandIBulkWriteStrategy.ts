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

const empty = (): Response => ({ result: null, rowsAffected: 0 });

const assigned = (value: Cell): boolean =>
  value !== null && value !== "" && value !== 0 && value !== 0n;

const rename = (command: BulkCommand, index: number): BulkCommand => {
  let text = command.text;
  const parameters = command.parameters.map((parameter) => ({ ...parameter }));
  const names = [...new Set(parameters.map((parameter) => parameter.name))].sort(
    (left, right) => right.length - left.length,
  );
  for (const name of names) {
    const alias = `${name}__${index}`;
    text = renameToken(text, name, alias);
    for (let at = 0; at < parameters.length; at++) {
      if (parameters[at]?.name === name) parameters[at] = { ...parameters[at], name: alias };
    }
  }
  return { text, parameters };
};

const renameToken = (text: string, name: string, alias: string): string => {
  const token = `$${name}`;
  const next = `$${alias}`;
  let index = 0;
  let result = "";
  while (text.indexOf(token, index) >= 0) {
    const at = text.indexOf(token, index);
    const end = at + token.length;
    const nextChar = text[end];
    const boundary = nextChar === undefined || !/[0-9A-Za-z_]/.test(nextChar);
    result += text.slice(index, at) + (boundary ? next : token);
    index = end;
  }
  return result + text.slice(index);
};

export class BatchCommandIBulkWriteStrategy<T extends object> implements IBulkWriteStrategy<T> {
  repo!: CRUDRepository<T>;
  command!: ICommand;
  options!: CRUDRepositoryOptions<T>;

  attach = (command: ICommand, options: CRUDRepositoryOptions<T>): void => {
    this.command = command;
    this.options = options;
  };

  create = (items: T[]): Promise<Response> => this.send(items.map((item) => this.insert(item)));

  update = (items: T[], byWhere: IWhere | undefined): Promise<Response> =>
    this.send(items.map((item) => this.updateCommand(item, byWhere)));

  delete = (items: T[], byWhere: IWhere | undefined): Promise<Response> =>
    this.send(items.map((item) => this.deleteCommand(item, byWhere)));

  private send = (commands: BulkCommand[]): Promise<Response> => {
    if (commands.length === 0) return Promise.resolve(empty());
    const renamed = commands.map(rename);
    return this.command.executeNative(
      new Native(renamed.map((command) => command.text).join("; ")),
      renamed.flatMap((command) => command.parameters),
    );
  };

  private insert = (item: T): BulkCommand => {
    const record = this.reverse(item);
    const statement = this.statements().insert(
      this.fields(record, (col) => !this.ids.includes(col) || assigned(record[col] ?? null)),
    );
    return { text: DefaultStatementTextRenderer.instance.render(statement), parameters: statement.fields };
  };

  private updateCommand = (item: T, byWhere?: IWhere): BulkCommand => {
    const record = this.reverse(item);
    const fields = this.fields(record, (col) => !this.ids.includes(col));
    if (fields.length === 0) throw new DatasourceError("Update requires at least one non-Id field");
    const statement = this.statements().update(
      fields,
      byWhere ?? this.idsWhere(record),
      (name, value) => this.cell(name, value, byWhere === undefined),
    );
    return { text: DefaultStatementTextRenderer.instance.render(statement), parameters: statement.parameters };
  };

  private deleteCommand = (item: T, byWhere?: IWhere): BulkCommand => {
    const record = this.reverse(item);
    const statement = this.statements().delete(
      byWhere ?? this.idsWhere(record),
      (name, value) => this.cell(name, value, byWhere === undefined),
    );
    const listed = statement.where?.parameters ?? [];
    const parameters: Parameter[] = listed.length
      ? listed
      : DefaultStatementTextRenderer.instance.render(statement).includes("$")
        ? this.fields(record, () => true)
        : [];
    return { text: DefaultStatementTextRenderer.instance.render(statement), parameters };
  };

  private statements = () => repositoryParts(this.command.connection, this.options).statements;

  private get ids(): readonly string[] {
    return this.options.ids ?? this.repo.ids;
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
