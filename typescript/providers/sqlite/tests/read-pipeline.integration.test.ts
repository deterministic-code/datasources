import { beforeEach, describe, expect, it } from "vitest";
import {
  Datasource,
  DatasourceFactory,
  DictionaryMapper,
  MetaDataRowTypeConverter,
  RowTypeConverter,
} from "@deterministic-code/datasources";
import { SqliteDriver } from "../src/SqliteDriver.ts";
import { applySchema } from "../../../schema-spec.ts";

class User {
  Id = 0;
  Uuid = "";
  Username = "";
  IsEnabled = false;
  Created = new Date(0);
}

describe("read pipeline", () => {
  beforeEach(() => {
    DatasourceFactory.reset();
    DatasourceFactory.register("sqlite", new SqliteDriver());
    delete process.env.INSTALLED_PROVIDERS;
    delete process.env.DATASOURCE_URL;
    process.env.PROVIDER = "sqlite";
  });

  it("maps and converts users through the read pipeline", async () => {
    const ada = "11111111-1111-1111-1111-111111111111";
    const grace = "22222222-2222-2222-2222-222222222222";
    const lin = "33333333-3333-3333-3333-333333333333";
    const ds = Datasource.default;
    const command = await ds.createCommand();
    await applySchema(command.connection, "pipeline-users");
    await command.execute(
      "INSERT INTO users (id, uuid, name, is_enabled, created) VALUES ($id, $uuid, $name, $is_enabled, $created)",
      [
        { name: "id", value: 1 },
        { name: "uuid", value: ada },
        { name: "name", value: "ada" },
        { name: "is_enabled", value: 1 },
        { name: "created", value: "2001-01-01T00:00:00.000Z" },
      ],
    );
    await command.execute(
      "INSERT INTO users (id, uuid, name, is_enabled, created) VALUES ($id, $uuid, $name, $is_enabled, $created)",
      [
        { name: "id", value: 2 },
        { name: "uuid", value: grace },
        { name: "name", value: "grace" },
        { name: "is_enabled", value: 1 },
        { name: "created", value: "2002-02-02T00:00:00.000Z" },
      ],
    );
    await command.execute(
      "INSERT INTO users (id, uuid, name, is_enabled, created) VALUES ($id, $uuid, $name, $is_enabled, $created)",
      [
        { name: "id", value: 3 },
        { name: "uuid", value: lin },
        { name: "name", value: "lin" },
        { name: "is_enabled", value: 0 },
        { name: "created", value: "2003-03-03T00:00:00.000Z" },
      ],
    );

    const propertyToColumnMapper = new DictionaryMapper([
      ["Id", "id"],
      ["Uuid", "uuid"],
      ["Username", "name"],
      ["IsEnabled", "is_enabled"],
      ["Created", "created"],
    ]);

    await using reader = await command.executeReader(
      "SELECT id, uuid, name, is_enabled, created FROM users",
    );
    const unmapped = await Array.fromAsync(reader.read());

    const columnMappedRows = unmapped.map((row) =>
      Object.fromEntries(
        Object.entries(row).map(([key, value]) => [propertyToColumnMapper.reverseMap(key), value]),
      ),
    );

    const rowConverter = RowTypeConverter.from(new SqliteDriver().rowTypeConverters, {
      Uuid: "uuid",
      IsEnabled: "bool",
      Created: "datetime",
    });
    const convertedRows = columnMappedRows.map((row) => rowConverter.convert(row));

    const objectConverter = new MetaDataRowTypeConverter<User>();
    const finalEntities = convertedRows.map((row) => objectConverter.convert(row, User));

    const finalManualEntities = convertedRows.map(
      (row) =>
        Object.assign(new User(), {
          Id: row.Id,
          Uuid: row.Uuid,
          Username: row.Username,
          IsEnabled: row.IsEnabled,
          Created: row.Created,
        }),
    );

    expect(unmapped).toHaveLength(3);
    expect(unmapped[0]).toMatchObject({ id: 1, uuid: ada, name: "ada" });
    expect(columnMappedRows[0]).toMatchObject({ Id: 1, Uuid: ada, Username: "ada" });
    expect(convertedRows[0].Uuid).toBe(ada);
    expect(convertedRows[0].IsEnabled).toBe(true);
    expect(convertedRows[0].Created).toEqual(new Date("2001-01-01T00:00:00.000Z"));
    expect(finalEntities[0]).toMatchObject({
      Id: 1,
      Uuid: ada,
      Username: "ada",
      IsEnabled: true,
    });
    expect(finalEntities[0].Created).toEqual(new Date("2001-01-01T00:00:00.000Z"));
    expect(finalManualEntities[1].Username).toBe("grace");
    expect(finalManualEntities[2].IsEnabled).toBe(false);
  });
});
