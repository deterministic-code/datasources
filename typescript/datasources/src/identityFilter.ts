export function identityFilter<T>(obj: Record<string, T>): Record<string, T> {
  if (Object.prototype.hasOwnProperty.call(obj, "id")) {
    return { id: obj.id };
  }
  return { ...obj };
}
