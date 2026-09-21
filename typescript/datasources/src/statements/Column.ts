import type { ColumnDefault } from "./ColumnDefault.ts";
import type { NotNull } from "./NotNull.ts";

export interface Column {
  name: string;
  type: string;
  default?: ColumnDefault;
  notNull?: NotNull;
}
