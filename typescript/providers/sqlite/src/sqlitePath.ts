export const sqliteFilename = (url: string): string | undefined => {
  if (!/^sqlite:/i.test(url)) return undefined;
  const path = url.replace(/^sqlite:(?:\/\/\/|\/\/)?/i, "");
  return path === "" || path === ":memory" || path === ":memory:" ? ":memory:" : path;
};

export const filenameOf = (options: { filename?: string; url?: string }): string =>
  options.filename ?? (options.url ? sqliteFilename(options.url) ?? ":memory:" : ":memory:");
