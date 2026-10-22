import { register } from "@deterministic-code/datasources-code-generators/common/sql-dialect";
import { PostgresDriver } from "./PostgresDriver.ts";

register("postgres", new PostgresDriver());
