import { ProviderNotImplementedError } from "../ProviderNotImplementedError.ts";

export const notImplemented = (provider: string): never => {
  throw new ProviderNotImplementedError(provider);
};
