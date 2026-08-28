import {
  TypeConverter,
  identityProviderTypes,
  type ProviderTypes,
} from "@datasources/core";

export const xmlTypes: ProviderTypes = identityProviderTypes();

export const registerConverters = () =>
  TypeConverter.use("xml", xmlTypes);

registerConverters();
