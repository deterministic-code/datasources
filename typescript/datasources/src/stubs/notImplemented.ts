import { ProviderNotImplementedError } from "../ProviderNotImplementedError.ts";

export function notImplemented(provider: string): never {
  throw new ProviderNotImplementedError(provider);
}
