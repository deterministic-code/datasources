import {
  TypeConverter,
  identityProviderTypes,
  type ProviderTypes,
} from "@datasources/core";

export const yamlTypes: ProviderTypes = identityProviderTypes();

export const registerConverters = () =>
  TypeConverter.use("yaml", yamlTypes);

registerConverters();
