import type { Field } from "../builders/Field.ts";
import type { Scalar } from "../builders/Scalar.ts";
import type { Parameter } from "../Parameter.ts";
import type { IUpdateStatement } from "./IUpdateStatement.ts";
import type { IWhere } from "./IWhere.ts";

const rename = (text: string, name: string, alias: string): string => {
  const token = `$${name}`;
  const next = `$${alias}`;
  let index = 0;
  let result = "";
  while (text.indexOf(token, index) >= 0) {
    const at = text.indexOf(token, index);
    const end = at + token.length;
    const nextChar = text.charAt(end);
    const boundary = nextChar === "" || !/[A-Za-z0-9_]/.test(nextChar);
    result += text.slice(index, at) + (boundary ? next : token);
    index = end;
  }
  return result + text.slice(index);
};

const compose = (
  sql: string,
  fields: Field[],
  where?: IWhere,
): { text: string; parameters: Parameter[] } => {
  if (!where) return { text: sql, parameters: fields };
  const text = `${sql} WHERE ${where.getText()}`;
  const split = text.lastIndexOf(" WHERE ");
  const head = text.slice(0, split);
  let tail = text.slice(split);
  const parameters: Parameter[] = [...fields];
  const values = new Map<string, Scalar>();
  for (const field of fields) values.set(field.name, field.value);
  for (const parameter of where.parameters ?? []) {
    if (parameter.value === undefined) continue;
    const current = values.get(parameter.name);
    if (values.has(parameter.name) && current !== parameter.value) {
      const alias = `${parameter.name}__where`;
      tail = rename(tail, parameter.name, alias);
      parameters.push({ name: alias, value: parameter.value });
    } else parameters.push({ name: parameter.name, value: parameter.value });
  }
  return { text: head + tail, parameters };
};

export class UpdateStatement implements IUpdateStatement {
  readonly where?: IWhere;
  private readonly text: string;

  constructor(text: string, where?: IWhere);
  constructor(table: string, fields: Field[], where?: IWhere);
  constructor(tableOrText: string, fieldsOrWhere?: Field[] | IWhere, where?: IWhere) {
    if (Array.isArray(fieldsOrWhere)) {
      const bound = compose(
        `UPDATE ${tableOrText} SET ${fieldsOrWhere.map((field) => `${field.name} = $${field.name}`).join(", ")}`,
        fieldsOrWhere,
        where,
      );
      this.text = bound.text;
      this.where = where;
      return;
    }
    this.text = tableOrText;
    this.where = fieldsOrWhere;
  }

  getText = (): string => this.text;

  static parameterList = (fields: Field[], where?: IWhere): Parameter[] =>
    compose("", fields, where).parameters;
}
