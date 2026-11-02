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
}
