const key = (name: string): symbol => Symbol.for(`@deterministic-code/datasources/${name}`);

export const brand = (type: { name: string; prototype: object }): void => {
  Object.defineProperty(type.prototype, key(type.name), { value: true });
};

export const is = <T extends object>(
  value: object,
  type: abstract new (...args: never[]) => T,
): value is T => value instanceof type || Boolean((value as Record<symbol, boolean>)[key(type.name)]);
