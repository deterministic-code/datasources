/** Whole-line batch separator migraters splits on. */
export const defaultBatchSeparator = "GO";

const requireSeparator = (value: string): string => {
  if (value.includes("\n") || value.includes("\r"))
    throw new Error("datasource.batch_separator must be a single line");
  return value;
};

export const batchSeparatorOf = (settings: Record<string, string>): string =>
  requireSeparator(settings["datasource.batch_separator"] ?? defaultBatchSeparator);

/** Join executable SQL chunks. An empty separator keeps a newline between them. */
export const joinBatches = (parts: readonly string[], separator: string): string => {
  const chunks = parts.filter((part) => part !== "");
  if (chunks.length === 0) return "";
  return chunks.join(separator.trim() === "" ? "\n" : `\n${separator}\n`);
};
