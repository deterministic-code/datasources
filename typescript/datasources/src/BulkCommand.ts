import type { Parameter } from "./Parameter.ts";

export interface BulkCommand {
  text: string;
  parameters: Parameter[];
}
