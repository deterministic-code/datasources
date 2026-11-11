export type JournalMode = "wal" | "delete" | "truncate" | "persist" | "memory" | "off";
export type Synchronous = "off" | "normal" | "full" | "extra";

export type SqliteOpenOptions = {
  journalMode?: JournalMode;
  busyTimeout?: number;
  poolSize?: number;
  synchronous?: Synchronous;
};

export type SqlitePathResult = {
  filename: string;
  options: SqliteOpenOptions;
  error?: string;
};

const KNOWN = ["journal_mode", "busy_timeout", "pool_size", "synchronous"] as const;

const JOURNAL: Record<string, JournalMode> = {
  wal: "wal",
  delete: "delete",
  truncate: "truncate",
  persist: "persist",
  memory: "memory",
  off: "off",
};

const SYNCHRONOUS: Record<string, Synchronous> = {
  off: "off",
  normal: "normal",
  full: "full",
  extra: "extra",
};

export const sqliteFilename = (url: string): string | undefined => sqlitePath(url)?.filename;

export const sqlitePath = (url: string): SqlitePathResult | undefined => {
  if (!/^sqlite:/i.test(url)) return undefined;
  const path = url.replace(/^sqlite:(?:\/\/\/|\/\/)?/i, "");
  const normalized = path === "" || path === ":memory" || path === ":memory:" ? ":memory:" : path;
  return parseFilenameQuery(normalized);
};

export const filenameOf = (options: { filename?: string; url?: string }): string =>
  options.filename ?? (options.url ? sqliteFilename(options.url) ?? ":memory:" : ":memory:");

export const optionsOf = (options: { filename?: string; url?: string }): SqlitePathResult => {
  if (options.filename) return { filename: options.filename, options: {} };
  if (options.url) return sqlitePath(options.url) ?? { filename: ":memory:", options: {} };
  return { filename: ":memory:", options: {} };
};

export const isMemoryFilename = (filename: string): boolean => {
  /* v8 ignore next */
  const base = filename.split("?")[0] ?? filename;
  return base === ":memory:" || base === ":memory" || filename.toLowerCase().includes("mode=memory");
};

export const resolvedOptions = (
  filename: string,
  options: SqliteOpenOptions,
): Required<Pick<SqliteOpenOptions, "busyTimeout" | "poolSize">> & SqliteOpenOptions => {
  const memory = isMemoryFilename(filename);
  const journalMode = memory ? undefined : (options.journalMode ?? "wal");
  return {
    journalMode,
    busyTimeout: options.busyTimeout ?? 5000,
    poolSize: memory ? 0 : (options.poolSize ?? (journalMode === "wal" ? 8 : 0)),
    synchronous: options.synchronous,
  };
};

export const parseFilenameQuery = (path: string): SqlitePathResult => {
  const q = path.indexOf("?");
  if (q < 0) return { filename: path, options: {} };
  const file = path.slice(0, q);
  const kept: string[] = [];
  const parsed: SqliteOpenOptions = {};
  let error: string | undefined;
  for (const pair of path.slice(q + 1).split("&")) {
    if (!pair) continue;
    const eq = pair.indexOf("=");
    const key = eq < 0 ? pair : pair.slice(0, eq);
    const value = eq < 0 ? "" : pair.slice(eq + 1);
    if (!KNOWN.some(name => name === key.toLowerCase())) {
      kept.push(pair);
      continue;
    }
    const next = assignOption(parsed, key, value);
    if (next) error ??= next;
  }
  const filename = kept.length === 0 ? file : `${file}?${kept.join("&")}`;
  return { filename, options: parsed, error };
};

const assignOption = (options: SqliteOpenOptions, key: string, value: string): string | undefined => {
  const handlers: Record<string, () => string | undefined> = {
    journal_mode: () => {
      const mode = JOURNAL[value.toLowerCase()];
      if (!mode) return `Invalid sqlite option journal_mode=${value}`;
      options.journalMode = mode;
    },
    busy_timeout: () => {
      const timeout = Number(value);
      if (!Number.isInteger(timeout) || timeout < 0) return `Invalid sqlite option busy_timeout=${value}`;
      options.busyTimeout = timeout;
    },
    pool_size: () => {
      const size = Number(value);
      if (!Number.isInteger(size) || size < 0 || size > 64) return `Invalid sqlite option pool_size=${value}`;
      options.poolSize = size;
    },
    synchronous: () => {
      const mode = SYNCHRONOUS[value.toLowerCase()];
      if (!mode) return `Invalid sqlite option synchronous=${value}`;
      options.synchronous = mode;
    },
  };
  const handler = handlers[key.toLowerCase()];
  /* v8 ignore next */
  if (!handler) throw new Error(`missing sqlite option ${key}`);
  return handler();
};
