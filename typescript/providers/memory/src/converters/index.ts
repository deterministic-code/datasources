import {
  TypeConverter,
  identityProviderTypes,
  type ProviderTypes,
} from "@datasources/core";

export const memoryTypes: ProviderTypes = identityProviderTypes();

export const registerConverters = () =>
  TypeConverter.use("memory", memoryTypes);

registerConverters();
