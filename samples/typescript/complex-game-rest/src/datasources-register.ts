import { DatasourceFactory } from "@deterministic-code/datasources";
import { SqliteDriver } from "@deterministic-code/datasources-sqlite";

DatasourceFactory.register("sqlite", new SqliteDriver());
