export function identityFilter(obj: Record<string, unknown>): Record<string, unknown> {
  if (Object.prototype.hasOwnProperty.call(obj, "id")) {
    return { id: obj.id };
  }
  return { ...obj };
}
