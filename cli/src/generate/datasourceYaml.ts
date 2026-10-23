import type { IDeterministicReader } from "@deterministic-code/generators-common/deterministic-reader";

const known = ["sqlite", "mysql", "postgres", "sqlserver", "oracle"];

export const overlayDatasource = (reader: IDeterministicReader, yaml: string): IDeterministicReader => ({
  read: (name) => (name === "datasource.yaml" ? Promise.resolve(yaml) : reader.read(name)),
  exists: (name) => (name === "datasource.yaml" ? Promise.resolve(true) : reader.exists(name)),
});

export const engineDatasourceOf = async (
  reader: IDeterministicReader,
  engine: string,
): Promise<string | undefined> =>
  (await reader.exists(`datasource.${engine}.yaml`)) ? reader.read(`datasource.${engine}.yaml`) : undefined;

export const datasourceEnginesOf = async (
  reader: IDeterministicReader,
  settings: Record<string, string>,
): Promise<string[]> => {
  const engines = new Set<string>();
  for (const name of (settings["backend.datasources"] ?? "").split(",")) {
    const key = name.trim();
    if (key !== "") engines.add(key);
  }
  for (const name of known) {
    if (await reader.exists(`datasource.${name}.yaml`)) engines.add(name);
  }
  return [...engines];
};
