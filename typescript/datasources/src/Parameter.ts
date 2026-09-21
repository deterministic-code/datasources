import type { Scalar } from "./builders/Scalar.ts";
import type { ParameterDirection } from "./ParameterDirection.ts";

export interface Parameter<T = Scalar> {
  name: string;
  value?: T;
  direction?: ParameterDirection;
}
