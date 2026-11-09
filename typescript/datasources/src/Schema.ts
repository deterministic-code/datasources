export type SchemaColumn = {
  type: string;
  nullable: boolean;
  hasDefault: boolean;
  isId?: boolean;
};

export class Schema {
  constructor(readonly columns: Readonly<Record<string, SchemaColumn>>) {}

  ids = (): string[] =>
    Object.entries(this.columns)
      .filter(([, column]) => column.isId === true)
      .map(([name]) => name);

  types = (): Record<string, string> =>
    Object.fromEntries(Object.entries(this.columns).map(([name, column]) => [name, column.type]));

  dictionaryKey = (): string => ("Key" in this.columns ? "Key" : "key" in this.columns ? "key" : "Key");

  dictionaryValueColumns = (relatedKey: string, mapKey: string): string[] =>
    Object.entries(this.columns)
      .filter(([name, column]) => column.isId !== true && name !== relatedKey && name !== mapKey)
      .map(([name]) => name);
}
