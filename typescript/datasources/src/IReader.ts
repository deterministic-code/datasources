export interface IReader<T = unknown> {
  readonly current: T | undefined;
  next(): Promise<boolean>;
  nextResult(): Promise<boolean>;
  dispose(): Promise<void>;
  [Symbol.asyncDispose](): Promise<void>;
}
