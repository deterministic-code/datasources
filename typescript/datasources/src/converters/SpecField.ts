export type FieldSize = number | number[] | "unlimited" | null;

export type SpecField = {
  type: string;
  name?: string;
  size?: FieldSize;
  defaultValue?: string | boolean | number | null;
};
