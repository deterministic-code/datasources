import type { MiddlewareHook } from "./MiddlewareHook.ts";

export interface IMiddlewareHost {
  use(before?: MiddlewareHook, after?: MiddlewareHook): this;
}
