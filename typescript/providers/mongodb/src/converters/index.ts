import {
  TypeConverter,
  identityProviderTypes,
  type ProviderTypes,
} from "@datasources/core";

export const mongodbTypes: ProviderTypes = identityProviderTypes();

export const registerConverters = () =>
  TypeConverter.use("mongodb", mongodbTypes);

registerConverters();
