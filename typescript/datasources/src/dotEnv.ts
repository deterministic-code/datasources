import { existsSync, readFileSync } from "node:fs";
import type { DatasourceOptions } from "./DatasourceOptions.ts";

export const DotEnv = {
  parse: (text: string): DatasourceOptions => {
    const options: DatasourceOptions = {};
    for (const raw of text.split(/\r?\n/)) {
      const line = raw.trimStart();
      if (!line || line.startsWith("#") || !line.includes("=")) continue;
      const i = line.indexOf("=");
      const key = line.slice(0, i).trim();
      if (key) options[key] = line.slice(i + 1);
    }
    return options;
  },
  load: (path = ".env"): DatasourceOptions =>
    existsSync(path) ? DotEnv.parse(readFileSync(path, "utf8")) : {},
};
