import { catalog, providers, type Provider } from "./catalog.ts";

export type NpmManifest = {
  dependencies?: Record<string, string>;
  devDependencies?: Record<string, string>;
};

export const npmProviders = (manifest: NpmManifest): Provider[] => {
  const deps = { ...manifest.dependencies, ...manifest.devDependencies };
  return providers.filter((name) => catalog[name].npm in deps);
};

export const cargoProviders = (toml: string): Provider[] =>
  providers.filter((name) => hasCargoCrate(toml, catalog[name].cargo));

export const csharpProviders = (csproj: string): Provider[] =>
  providers.filter((name) => hasCsharpPackage(csproj, catalog[name].nuget));

export const hasCsharpProjectReference = (xml: string, id: string): boolean => xml.includes(`${id}.csproj`);

const hasCargoCrate = (toml: string, crate: string): boolean =>
  new RegExp(`(^|[\\s{,])${crate}\\s*=`, "m").test(toml) ||
  new RegExp(`"${crate}"`).test(toml);

const hasCsharpPackage = (xml: string, id: string): boolean =>
  xml.includes(`"${id}"`) || xml.includes(`${id}.csproj`);
