import { TransformMapper } from "@datasources/core";

export const columns = new TransformMapper(
  (name: string) =>
    ({ name: "ugly_name", role_id: "bad_role_id" }[name] ?? name),
  (name: string) =>
    ({ ugly_name: "name", bad_role_id: "role_id" }[name] ?? name),
);
