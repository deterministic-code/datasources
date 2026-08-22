export interface IReader<T = unknown> {
  readonly current: T | undefined;
  next(): Promise<boolean>;
}
