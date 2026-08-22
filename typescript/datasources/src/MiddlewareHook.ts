import type { MiddlewareContext } from "./MiddlewareContext.ts";

export type MiddlewareHook = (ctx: MiddlewareContext) => void | Promise<void>;
