import {
  TypeConverter,
  identityProviderTypes,
  type ProviderTypes,
} from "@datasources/core";

export const dynamodbTypes: ProviderTypes = identityProviderTypes();

export const registerConverters = () =>
  TypeConverter.use("dynamodb", dynamodbTypes);

registerConverters();
