export type Association = {
  parent: string;
  related: string;
};

export type Enrichment = {
  into: string;
  from: string;
  by: string;
  select?: string;
  many?: boolean;
  association?: Association;
};
