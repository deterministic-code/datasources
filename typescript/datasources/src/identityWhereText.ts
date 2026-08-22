export function identityWhereText(obj: Record<string, unknown>): string {
  if (Object.prototype.hasOwnProperty.call(obj, "id")) {
    return "id = $id";
  }
  const keys = Object.keys(obj);
  if (keys.length === 0) {
    return "1 = 1";
  }
  return keys.map((key) => `${key} = $${key}`).join(" AND ");
}
