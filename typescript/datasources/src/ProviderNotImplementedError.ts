export class ProviderNotImplementedError extends Error {
  readonly provider: string;

  constructor(provider: string) {
    super(`Provider "${provider}" is not implemented`);
    this.name = "ProviderNotImplementedError";
    this.provider = provider;
  }
}
