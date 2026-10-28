/** Whole-line batch separator migraters splits on. */
export const defaultBatchSeparator = "GO";

export const batchSeparatorOf = (settings: Record<string, string>): string =>
  settings["datasource.batch_separator"] ?? defaultBatchSeparator;

/** Join executable SQL chunks. An empty separator keeps a newline between them. */
export const joinBatches = (parts: readonly string[], separator: string): string => {
  const chunks = parts.map((part) => part.trim()).filter((part) => part !== "");
  if (chunks.length === 0) return "";
  return chunks.join(separator.trim() === "" ? "\n" : `\n${separator}\n`);
};
