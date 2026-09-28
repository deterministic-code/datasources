export interface IUnitOfWork {
  begin(): Promise<void>;
  commit(): Promise<void>;
  abort(): Promise<void>;
}
