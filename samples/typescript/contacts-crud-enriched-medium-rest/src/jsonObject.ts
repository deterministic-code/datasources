export const jsonObject = {
  toString(value: unknown): string {
    return JSON.stringify(value);
  },
  fromString<T>(body: string | undefined): T {
    if (body == null || body.trim() === "") return {} as T;
    const parsed: unknown = JSON.parse(body);
    if (parsed == null || typeof parsed !== "object") throw new SyntaxError("invalid json");
    return parsed as T;
  },
};
