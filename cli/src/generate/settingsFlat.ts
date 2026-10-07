import { parse } from "yaml";

type SettingNode = string | number | boolean | null | SettingNode[] | { [key: string]: SettingNode };

const isMap = (node: SettingNode): node is { [key: string]: SettingNode } =>
  node !== null && typeof node === "object" && !Array.isArray(node);

const write = (node: SettingNode, prefix: string, out: Record<string, string>): void => {
  if (Array.isArray(node)) {
    if (prefix) out[prefix] = node.map((item) => String(item)).join(",");
    return;
  }
  if (isMap(node)) {
    for (const [key, value] of Object.entries(node)) write(value, prefix ? `${prefix}.${key}` : key, out);
    return;
  }
  if (prefix) out[prefix] = String(node);
};

export const flattenSettings = (text: string): Record<string, string> => {
  const parsed = parse(text) as SettingNode;
  const root = isMap(parsed) && isMap(parsed.settings) ? parsed.settings : parsed;
  const out: Record<string, string> = {};
  write(root, "", out);
  return out;
};

export const settingsFrom = (text: string | undefined): Record<string, string> => ({
  "backend.datasources": "sqlite",
  ...(text === undefined ? {} : flattenSettings(text)),
});
