import { register } from "@deterministic-code/datasources-code-generators/common/sql-dialect";
import { SqliteDriver } from "./SqliteDriver.ts";

register("sqlite", new SqliteDriver());
