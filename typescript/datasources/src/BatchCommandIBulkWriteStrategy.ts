import type { Response } from "./Response.ts";
import type { IWhere } from "./statements/IWhere.ts";
import type { BulkCommand } from "./BulkCommand.ts";
import type { IBulkWrite } from "./IBulkWrite.ts";
import type { IBulkWriteStrategy } from "./IBulkWriteStrategy.ts";

const empty = (): Response => ({ result: null, rowsAffected: 0 });

const send = <T>(write: IBulkWrite<T>, commands: BulkCommand[]): Promise<Response> => {
  if (commands.length === 0) return Promise.resolve(empty());
  const renamed = commands.map(rename);
  return write.command.execute(
    renamed.map((command) => command.text).join("; "),
    renamed.flatMap((command) => command.parameters),
  );
};

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
    const boundary = end >= text.length || !/[0-9A-Za-z_]/.test(text[end] ?? "");
    result += text.slice(index, at) + (boundary ? next : token);
    index = end;
  }
  return result + text.slice(index);
};

export class BatchCommandIBulkWriteStrategy<T extends object> implements IBulkWriteStrategy<T> {
  create = (items: T[], write: IBulkWrite<T>): Promise<Response> => send(write, write.inserts(items));

  update = (items: T[], byWhere: IWhere | undefined, write: IBulkWrite<T>): Promise<Response> =>
    send(write, write.updates(items, byWhere));

  delete = (items: T[], byWhere: IWhere | undefined, write: IBulkWrite<T>): Promise<Response> =>
    send(write, write.deletes(items, byWhere));
}
