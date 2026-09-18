const aliases: Record<string, string> = {
  postgresql: "postgres",
  mssql: "sqlserver",
  sqlite3: "sqlite",
};

export const providerFromUrl = (url: string): string | undefined => {
  const scheme = url.match(/^([a-zA-Z][a-zA-Z0-9+.-]*):/)?.[1]?.toLowerCase();
  return scheme ? (aliases[scheme] ?? scheme) : undefined;
};
