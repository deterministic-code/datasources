// schema-version: 1.0
/** View ContactCard. */
type Dictionary<K extends PropertyKey, V> = Record<K, V>;
export interface ContactCard {
  displayName: string;
  labels: Dictionary<string, string>;
}
