import type { CodeGenerationProvider } from "./CodeGenerationProvider.ts";
import { deterministic } from "./DeterministicProvider.ts";

const providers: Record<string, CodeGenerationProvider> = { deterministic };

export const providerByName = (name: string): CodeGenerationProvider => {
  const provider = providers[name];
  if (provider === undefined) throw new Error(`Unknown provider: ${name}`);
  return provider;
};
