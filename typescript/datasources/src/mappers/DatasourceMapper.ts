export type Mapping = {
  source: string;
  destination: string;
  names: Record<string, string>;
  to: (name: string) => string;
  from: (name: string) => string;
};

export const DatasourceMapper = {
  mapping: ({
    source,
    destination,
    names = {},
  }: {
    source: string;
    destination: string;
    names?: Record<string, string>;
  }): Mapping => {
    const back = Object.fromEntries(
      Object.entries(names).map(([src, dest]) => [dest, src]),
    );
    return {
      source,
      destination,
      names,
      to: (name) => names[name] ?? name,
      from: (name) => back[name] ?? name,
    };
  },
};
