export const IsolationLevel = {
  ReadUncommitted: "readUncommitted",
  ReadCommitted: "readCommitted",
  RepeatableRead: "repeatableRead",
  Serializable: "serializable",
} as const;

export type IsolationLevel = (typeof IsolationLevel)[keyof typeof IsolationLevel];
