const signature = (line: string): string => {
  const duration = line.lastIndexOf("] ");
  return duration >= 0 && (line.endsWith("ms") || line.endsWith("μs")) ? line.slice(0, duration + 1) : line;
};

const token = (value: string): string =>
  /\d/.test(value) && /[A-Za-z]/.test(value) ? "<token>" : value;

export const normalizeConsoleLine = (line: string): string =>
  signature(line)
    .replace(/\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{3}Z/g, "<date>")
    .replace(/\b[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}\b/g, "<uuid>")
    .replace(/["`]([A-Za-z_][A-Za-z0-9_]*)["`]/g, "$1")
    .replace(/password: [^,}\]]+/g, "password: <token>")
    .replace(/updated: [^,}\]]+/g, "updated: <token>")
    .replace(/[A-Za-z0-9+/]{4,}={0,2}/g, token);

export const sqlText = (lines: readonly string[]): string[] =>
  lines.filter((line) => line.includes("][Start]-") || line.includes("][Finish]-")).map(normalizeConsoleLine);
