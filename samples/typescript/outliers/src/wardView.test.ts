import { Datasource, Native } from "@deterministic-code/datasources";
import { SqliteDriver } from "@deterministic-code/datasources-sqlite";
import { describe, expect, it } from "vitest";
import { WardViewsRepository } from "../generated/repositories/wardViewsRepository.ts";
import { wardAlias } from "../generated/types/datasource/wardAlias.ts";
import { wardNode } from "../generated/types/datasource/wardNode.ts";
import { wardView } from "../generated/types/view/wardView.ts";

const schema = async (connection: Awaited<ReturnType<Datasource["createConnection"]>>): Promise<void> => {
  for (const sql of [
    `CREATE TABLE "wards" (
      "region_id" INTEGER NOT NULL,
      "ward_id" INTEGER NOT NULL,
      "name" VARCHAR(128) NOT NULL,
      CONSTRAINT "wards_primary_key" PRIMARY KEY ("region_id", "ward_id")
    )`,
    `CREATE TABLE "ward_aliases" (
      "id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
      "parent_region_id" INTEGER NOT NULL,
      "parent_ward_id" INTEGER NOT NULL,
      "name" VARCHAR(128) NOT NULL
    )`,
    `CREATE TABLE "ward_settings" (
      "id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
      "parent_region_id" INTEGER NOT NULL,
      "parent_ward_id" INTEGER NOT NULL,
      "name" VARCHAR(64) NOT NULL,
      "value" VARCHAR(256) NOT NULL
    )`,
    `CREATE TABLE "ward_nodes" (
      "id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
      "parent_region_id" INTEGER NOT NULL,
      "parent_ward_id" INTEGER NOT NULL,
      "name" VARCHAR(128) NOT NULL
    )`,
  ])
    await connection.createCommand().executeNative(new Native(sql));
};

describe("ward view", () => {
  it("creates, reads, updates, and deletes", async () => {
    const ds = new Datasource("sqlite", new SqliteDriver(), { DATASOURCE_URL: "sqlite:///:memory:" });
    const connection = await ds.createConnection();
    await connection.open();
    await schema(connection);
    const repo = new WardViewsRepository(connection);
    const created = await repo.create(
      wardView({
        regionId: 1,
        wardId: 1,
        name: "north",
        aliases: [wardAlias({ name: "uptown" })],
        settings: { color: "blue" },
        nodes: [wardNode({ name: "dock" })],
      }),
    );
    const loaded = await repo.readSingleById({ regionId: created.regionId, wardId: created.wardId });
    expect(loaded?.name).toBe("north");
    expect(loaded?.aliases).toHaveLength(1);
    expect(loaded?.aliases[0]?.name).toBe("uptown");
    expect(loaded?.settings.color).toBe("blue");
    expect(loaded?.nodes).toHaveLength(1);
    expect(loaded?.nodes[0]?.name).toBe("dock");
    loaded!.name = "south";
    loaded!.aliases[0]!.name = "downtown";
    loaded!.settings.color = "red";
    loaded!.nodes[0]!.name = "pier";
    await repo.update(loaded!);
    const updated = await repo.readSingleById({ regionId: loaded!.regionId, wardId: loaded!.wardId });
    expect(updated?.name).toBe("south");
    expect(updated?.aliases[0]?.name).toBe("downtown");
    expect(updated?.settings.color).toBe("red");
    expect(updated?.nodes[0]?.name).toBe("pier");
    await repo.delete(updated!);
    expect(await repo.readSingleById({ regionId: updated!.regionId, wardId: updated!.wardId })).toBeUndefined();
  });
});
