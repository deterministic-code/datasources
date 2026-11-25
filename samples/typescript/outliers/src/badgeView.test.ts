import { Datasource, Native } from "@deterministic-code/datasources";
import { SqliteDriver } from "@deterministic-code/datasources-sqlite";
import { describe, expect, it } from "vitest";
import { BadgeViewsRepository } from "../generated/repositories/badgeViewsRepository.ts";
import { badgeAlias } from "../generated/types/datasource/badgeAlias.ts";
import { badgeNode } from "../generated/types/datasource/badgeNode.ts";
import { badgeView } from "../generated/types/view/badgeView.ts";

const schema = async (connection: Awaited<ReturnType<Datasource["createConnection"]>>): Promise<void> => {
  for (const sql of [
    `CREATE TABLE "badges" (
      "badge_id" TEXT NOT NULL,
      "code" VARCHAR(64) NOT NULL,
      "name" VARCHAR(128) NOT NULL,
      CONSTRAINT "badges_primary_key" PRIMARY KEY ("badge_id", "code")
    )`,
    `CREATE TABLE "badge_aliases" (
      "id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
      "parent_badge_id" TEXT NOT NULL,
      "parent_code" TEXT NOT NULL,
      "name" VARCHAR(128) NOT NULL
    )`,
    `CREATE TABLE "badge_settings" (
      "id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
      "parent_badge_id" TEXT NOT NULL,
      "parent_code" TEXT NOT NULL,
      "name" VARCHAR(64) NOT NULL,
      "value" VARCHAR(256) NOT NULL
    )`,
    `CREATE TABLE "badge_nodes" (
      "id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
      "parent_badge_id" TEXT NOT NULL,
      "parent_code" TEXT NOT NULL,
      "name" VARCHAR(128) NOT NULL
    )`,
  ])
    await connection.createCommand().executeNative(new Native(sql));
};

describe("badge view", () => {
  it("creates, reads, updates, and deletes", async () => {
    const ds = new Datasource("sqlite", new SqliteDriver(), { DATASOURCE_URL: "sqlite:///:memory:" });
    const connection = await ds.createConnection();
    await connection.open();
    await schema(connection);
    const repo = new BadgeViewsRepository(connection);
    const created = await repo.create(
      badgeView({
        badgeId: "11111111-1111-1111-1111-111111111111",
        code: "alpha",
        name: "north",
        aliases: [badgeAlias({ name: "uptown" })],
        settings: { color: "blue" },
        nodes: [badgeNode({ name: "dock" })],
      }),
    );
    const loaded = await repo.readSingleById({ badgeId: created.badgeId, code: created.code });
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
    const updated = await repo.readSingleById({ badgeId: loaded!.badgeId, code: loaded!.code });
    expect(updated?.name).toBe("south");
    expect(updated?.aliases[0]?.name).toBe("downtown");
    expect(updated?.settings.color).toBe("red");
    expect(updated?.nodes[0]?.name).toBe("pier");
    await repo.delete(updated!);
    expect(await repo.readSingleById({ badgeId: updated!.badgeId, code: updated!.code })).toBeUndefined();
  });
});
