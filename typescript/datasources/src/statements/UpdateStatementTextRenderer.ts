import type { Field } from "../builders/Field.ts";
import type { Scalar } from "../builders/Scalar.ts";
import type { Parameter } from "../Parameter.ts";
import type { IWhere } from "./IWhere.ts";
import type { StatementTextRenderer } from "./StatementTextRenderer.ts";
import type { UpdateStatement } from "./UpdateStatement.ts";

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

const setClause = (fields: Field[]): string =>
  fields.map((field) => `${field.name} = $${field.name}`).join(", ");

export class UpdateStatementTextRenderer {
  constructor(private readonly text: StatementTextRenderer) {}

  render(statement: UpdateStatement): string {
    return this.compose(
      `UPDATE ${statement.table} SET ${setClause(statement.fields)}`,
      statement.where,
      statement.fields,
    ).text;
  }

  compose(
    sql: string,
    where: IWhere | undefined,
    fields: Field[],
  ): { text: string; parameters: Parameter[] } {
    if (!where) return { text: sql, parameters: fields };
    const rendered = `${sql} WHERE ${this.text.render(where)}`;
    const split = rendered.lastIndexOf(" WHERE ");
    /* v8 ignore next 2 */
    const head = split < 0 ? rendered : rendered.slice(0, split);
    let tail = split < 0 ? "" : rendered.slice(split);
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
  }
}
