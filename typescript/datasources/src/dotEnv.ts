import { existsSync, readFileSync } from "node:fs";
import type { DatasourceOptions } from "./DatasourceOptions.ts";

export const DotEnv = {
  parse: (text: string): DatasourceOptions =>
    Object.fromEntries(
      text
        .split(/\r?\n/)
        .map((raw) => raw.trimStart().split("=", 2))
        .filter((parts): parts is [string, string] =>
          parts.length === 2 && !parts[0].startsWith("#") && parts[0].trim() !== "")
        .map(([key, value]) => [key.trim(), value]),
    ),
  load: (path = ".env"): DatasourceOptions =>
    existsSync(path) ? DotEnv.parse(readFileSync(path, "utf8")) : {},
};
