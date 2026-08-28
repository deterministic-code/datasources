export type Mapping = {
  name: string;
  source: string;
  destination: string;
  names: Record<string, string>;
  to: (name: string) => string;
  from: (name: string) => string;
};

export const DatasourceMapper = {
  mapping: ({
    name = "",
    source,
    destination,
    names = {},
  }: {
    name?: string;
    source: string;
    destination: string;
    names?: Record<string, string>;
  }): Mapping => {
    const back = Object.fromEntries(
      Object.entries(names).map(([src, dest]) => [dest, src]),
    );
    return {
      name,
      source,
      destination,
      names,
      to: (column) => names[column] ?? column,
      from: (column) => back[column] ?? column,
    };
  },
};
