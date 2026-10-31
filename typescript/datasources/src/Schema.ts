export type SchemaColumn = {
  type: string;
  nullable: boolean;
  hasDefault: boolean;
};

export class Schema {
  constructor(readonly columns: Readonly<Record<string, SchemaColumn>>) {}

  types = (): Record<string, string> =>
    Object.fromEntries(Object.entries(this.columns).map(([name, column]) => [name, column.type]));
}
