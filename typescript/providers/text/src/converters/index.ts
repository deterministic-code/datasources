import {
  TypeConverter,
  identityProviderTypes,
  type ProviderTypes,
} from "@datasources/core";

export const textTypes: ProviderTypes = identityProviderTypes();

export const registerConverters = () =>
  TypeConverter.use("text", textTypes);

registerConverters();
