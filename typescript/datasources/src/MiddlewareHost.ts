import type { IMiddlewareHost } from "./IMiddlewareHost.ts";
import type { MiddlewareContext } from "./MiddlewareContext.ts";
import type { MiddlewareHook } from "./MiddlewareHook.ts";
import type { Parameter } from "./Parameter.ts";

export class MiddlewareHost implements IMiddlewareHost {
  private readonly befores: MiddlewareHook[] = [];
  private readonly afters: MiddlewareHook[] = [];

  use(before?: MiddlewareHook, after?: MiddlewareHook): this {
    if (before) {
      this.befores.push(before);
    }
    if (after) {
      this.afters.push(after);
    }
    return this;
  }

  async run<T>(
    operation: string,
    fn: () => Promise<T>,
    extras: { text?: string | null; parameters?: Parameter[] } = {},
  ): Promise<T> {
    const ctx: MiddlewareContext = {
      operation,
      host: this,
      text: extras.text,
      parameters: extras.parameters,
    };

    for (const before of this.befores) {
      await before(ctx);
    }

    try {
      const result = await fn();
      for (const after of this.afters) {
        await after(ctx);
      }
      return result;
    } catch (error) {
      ctx.error = error;
      for (const after of this.afters) {
        await after(ctx);
      }
      throw error;
    }
  }
}
