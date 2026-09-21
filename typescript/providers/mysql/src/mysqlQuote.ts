export const MysqlQuote = {
  quote: (id: string): string => `\`${id.replaceAll("`", "``")}\``,
  qualified: (id: string): string => id.split(".").map(MysqlQuote.quote).join("."),
};
