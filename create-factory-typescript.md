# Create factory (TypeScript)

Register a factory, then create by provider name. Install only the dialect packages you use.

| Dialect | Provider | Package |
| --- | --- | --- |
| json | `json` | `@datasources/json` |
| memory | `memory` | `@datasources/memory` |
| sqlite | `sqlite` | `@datasources/sqlite` |
| text | `text` | `@datasources/text` |
| xml | `xml` | `@datasources/xml` |
| yaml | `yaml` | `@datasources/yaml` |
| mysql | `mysql` | `@datasources/mysql` |
| sqlserver | `sqlserver` | `@datasources/sqlserver` |
| postgres | `postgres` | `@datasources/postgres` |
| oracle | `oracle` | `@datasources/oracle` |
| mongodb | `mongodb` | `@datasources/mongodb` |
| dynamodb | `dynamodb` | `@datasources/dynamodb` |

```ts
import { Datasource } from "@datasources/core";
import { JsonDatasource } from "@datasources/json";
import { MemoryDatasource } from "@datasources/memory";
import { SqliteDatasource } from "@datasources/sqlite";
import { TextDatasource } from "@datasources/text";
import { XmlDatasource } from "@datasources/xml";
import { YamlDatasource } from "@datasources/yaml";
import { MysqlDatasource } from "@datasources/mysql";
import { SqlServerDatasource } from "@datasources/sqlserver";
import { PostgresDatasource } from "@datasources/postgres";
import { OracleDatasource } from "@datasources/oracle";
import { MongoDbDatasource } from "@datasources/mongodb";
import { DynamoDbDatasource } from "@datasources/dynamodb";

Datasource.use(
  JsonDatasource.factory,
  MemoryDatasource.factory,
  SqliteDatasource.factory,
  TextDatasource.factory,
  XmlDatasource.factory,
  YamlDatasource.factory,
  MysqlDatasource.factory,
  SqlServerDatasource.factory,
  PostgresDatasource.factory,
  OracleDatasource.factory,
  MongoDbDatasource.factory,
  DynamoDbDatasource.factory,
);

const json = Datasource.create("json");
const memory = Datasource.create("memory");
const sqlite = Datasource.create("sqlite");
const text = Datasource.create("text");
const xml = Datasource.create("xml");
const yaml = Datasource.create("yaml");
const mysql = Datasource.create("mysql");
const sqlserver = Datasource.create("sqlserver");
const postgres = Datasource.create("postgres");
const oracle = Datasource.create("oracle");
const mongodb = Datasource.create("mongodb");
const dynamodb = Datasource.create("dynamodb");
```

Working dialects (json, memory, sqlite) accept options on `create`. Stub dialects return a stub datasource.
