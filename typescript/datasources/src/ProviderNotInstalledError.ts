export class ProviderNotInstalledError extends Error {
  readonly provider: string;

  constructor(provider: string) {
    super(`Provider "${provider}" is not installed`);
    this.name = "ProviderNotInstalledError";
    this.provider = provider;
  }
}
