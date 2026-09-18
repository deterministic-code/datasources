import { catalog, providers, type Provider } from "./catalog.ts";

export type NpmManifest = {
  dependencies?: Record<string, string>;
  devDependencies?: Record<string, string>;
};

export const npmProviders = (manifest: NpmManifest): Provider[] => {
  const deps = { ...manifest.dependencies, ...manifest.devDependencies };
  return providers.filter((name) => catalog[name].npm in deps);
};

export const cargoProviders = (toml: string): Provider[] => {
  const features = toml.match(/datasources-register[\s\S]*?features\s*=\s*\[([^\]]*)\]/);
  const fromFeatures = features
    ? splitTomlStrings(features[1]!).filter((name): name is Provider =>
        (providers as readonly string[]).includes(name),
      )
    : [];
  const fromCrates = providers.filter((name) => hasCargoCrate(toml, catalog[name].cargo));
  return [...new Set([...fromFeatures, ...fromCrates])];
};

export const csharpProviders = (csproj: string): Provider[] =>
  providers.filter((name) => hasCsharpPackage(csproj, catalog[name].nuget));

export const withRegisterFeatures = (toml: string, features: string[]): string =>
  /datasources-register[\s\S]*?features\s*=\s*\[[^\]]*\]/.test(toml)
    ? toml.replace(
        /(datasources-register[\s\S]*?features\s*=\s*)\[[^\]]*\]/,
        `$1[${features.map((name) => `"${name}"`).join(", ")}]`,
      )
    : toml;

const hasCargoCrate = (toml: string, crate: string): boolean =>
  new RegExp(`(^|[\\s{,])${crate}\\s*=`, "m").test(toml) ||
  new RegExp(`"${crate}"`).test(toml);

const hasCsharpPackage = (xml: string, id: string): boolean =>
  xml.includes(`"${id}"`) || xml.includes(`${id}.csproj`);

const splitTomlStrings = (value: string): string[] =>
  [...value.matchAll(/"([^"]+)"/g)].map((match) => match[1]!);
