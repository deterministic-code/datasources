import { register } from "@deterministic-code/datasources-code-generators/common/sql-dialect";
import { MysqlDriver } from "./MysqlDriver.ts";

register("mysql", new MysqlDriver());
