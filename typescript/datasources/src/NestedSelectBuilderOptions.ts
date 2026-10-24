import type { IWhere } from "./statements/IWhere.ts";

export type NestedSelectBuilderOptions = {
  where?: IWhere;
  stopAt?: readonly string[];
};
