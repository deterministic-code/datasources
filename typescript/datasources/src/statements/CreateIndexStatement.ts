import type { ICreateIndexStatement } from "./ICreateIndexStatement.ts";

export class CreateIndexStatement implements ICreateIndexStatement {
  private readonly text: string;

  constructor(text: string);
  constructor(name: string, table: string, columns: string[], unique?: boolean);
  constructor(nameOrText: string, table?: string, columns?: string[], unique = false) {
    this.text = columns
      ? `CREATE ${unique ? "UNIQUE " : ""}INDEX ${nameOrText} ON ${table} (${columns.join(", ")})`
      : nameOrText;
  }

  getText = (): string => this.text;
}
