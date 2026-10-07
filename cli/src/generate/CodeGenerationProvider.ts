import type { GenerateEntry } from "@deterministic-code/generators-common/generate-entry";
import type { GenerateRequest } from "./GenerateRequest.ts";

export type CodeGenerationProvider = {
  readonly name: string;
  generateTypes(request: GenerateRequest): Promise<GenerateEntry[]>;
  generateRepositories(request: GenerateRequest): Promise<GenerateEntry[]>;
  generateDatasource(request: GenerateRequest): Promise<GenerateEntry[]>;
};
