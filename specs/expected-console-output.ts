export type CommandOp = { sql?: string };

export type CommandSpec = {
  steps: Array<{
    execute?: CommandOp;
    executeReader?: CommandOp;
  }>;
};

export type ExpectedConsoleOptions = {
  start?: Date | string;
  elapsedMs?: number | ((index: number, query: string) => number);
};

export const formatElapsed = (elapsedMs: number): string =>
  elapsedMs < 1
    ? `${Math.round(elapsedMs * 1000)}μs`
    : `${Math.round(elapsedMs)}ms`;

export const consoleLine = (
  at: Date,
  phase: "Start" | "Finish",
  query: string,
  elapsedMs?: number,
): string =>
  elapsedMs === undefined
    ? `[${at.toISOString()}]-[datasource][${phase}]-[${query}]`
    : `[${at.toISOString()}]-[datasource][${phase}]-[${query}] ${formatElapsed(elapsedMs)}`;

export const commandQueries = (spec: CommandSpec): string[] =>
  spec.steps.flatMap((step) => {
    const sql = step.execute?.sql ?? step.executeReader?.sql;
    return sql === undefined ? [] : [sql.trim()];
  });

export const consoleSignature = (line: string): string =>
  line.replace(/^\[[^\]]+\]-/, "").replace(/ \d+(?:ms|μs)$/, "");

export const expectedConsoleSignatures = (spec: CommandSpec): string[] =>
  commandQueries(spec).flatMap((query) => [
    `[datasource][Start]-[${query}]`,
    `[datasource][Finish]-[${query}]`,
  ]);

export const expectedConsoleOutput = (
  spec: CommandSpec,
  options: ExpectedConsoleOptions = {},
): string[] => {
  const elapsedAt =
    typeof options.elapsedMs === "function"
      ? options.elapsedMs
      : () => options.elapsedMs ?? 1;
  let ms = new Date(options.start ?? "2026-08-28T01:54:12.101Z").getTime();
  const lines: string[] = [];
  for (const [index, query] of commandQueries(spec).entries()) {
    const elapsedMs = elapsedAt(index, query);
    lines.push(consoleLine(new Date(ms), "Start", query));
    ms += Math.max(1, Math.round(elapsedMs));
    lines.push(consoleLine(new Date(ms), "Finish", query, elapsedMs));
    ms += 1;
  }
  return lines;
};
