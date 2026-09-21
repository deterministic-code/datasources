export const SqliteQuote = {
  quote: (id: string): string => `"${id.replaceAll('"', '""')}"`,
  qualified: (id: string): string => id.split(".").map(SqliteQuote.quote).join("."),
};
