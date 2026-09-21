export const PostgresQuote = {
  quote: (id: string): string => `"${id.replaceAll('"', '""')}"`,
  qualified: (id: string): string => id.split(".").map(PostgresQuote.quote).join("."),
};
