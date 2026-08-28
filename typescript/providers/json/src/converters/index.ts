import {
  TypeConverter,
  identityProviderTypes,
  type ProviderTypes,
} from "@datasources/core";

export const jsonTypes: ProviderTypes = identityProviderTypes();

export const registerConverters = () =>
  TypeConverter.use("json", jsonTypes);

registerConverters();
