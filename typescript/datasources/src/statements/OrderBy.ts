export type SortDirection = "asc" | "desc";

export type OrderBy = {
  column: string;
  direction: SortDirection;
};

const directionSql: Record<SortDirection, string> = {
  asc: "ASC",
  desc: "DESC",
};

export const OrderBy = {
  asc: (column: string): OrderBy => ({ column, direction: "asc" }),
  desc: (column: string): OrderBy => ({ column, direction: "desc" }),
};

export const orderText = (order: OrderBy): string =>
  `${order.column} ${directionSql[order.direction]}`;
