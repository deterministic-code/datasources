import type { IWhere } from "./IWhere.ts";

export type NestedSelectBuilderOptions = {
  where?: IWhere;
  stopAt?: readonly string[];
};
