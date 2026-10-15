export type FieldSize = number | number[] | "unlimited" | null;

export type SpecField = {
  type: string;
  name?: string;
  size?: FieldSize;
  defaultValue?: string | boolean | number | null;
  isOptimisticConcurrency?: boolean;
  useNativeRowVersion?: boolean;
};

export const usesNativeRowVersion = (field: SpecField): boolean =>
  field.useNativeRowVersion === true && field.isOptimisticConcurrency === true;

export const emitsNativeRowVersion = (field: SpecField, dialect: string): boolean =>
  dialect === "sqlserver" && usesNativeRowVersion(field);

export const isOccBinaryColumn = (field: SpecField): boolean =>
  field.type === "binary" && field.isOptimisticConcurrency === true;
