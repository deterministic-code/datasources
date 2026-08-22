import type { Parameter } from "./Parameter.ts";

export interface MiddlewareContext {
  operation: string;
  host: unknown;
  text?: string | null;
  parameters?: Parameter[];
  error?: unknown;
}
