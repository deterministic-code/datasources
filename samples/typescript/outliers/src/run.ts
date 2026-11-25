import "../generated/repositories/repositoryRegistry.ts";
import { Datasource } from "@deterministic-code/datasources";
import {
  badgeAlias,
  badgeNode,
  badgeView,
  categoryTree,
  nodeGraph,
  regionCategoryTree,
  wardAlias,
  wardNode,
  wardView,
} from "../generated/repositories/repositoryRegistry.ts";
import { transcript } from "./format.ts";
import { runMapping } from "./mapping.ts";

const collect = async <T>(rows: AsyncIterable<T>): Promise<T[]> => {
  const out: T[] = [];
  for await (const row of rows) out.push(row);
  return out;
};

export const run = async (): Promise<string[]> => {
  await using ds = Datasource.default;
  await ds.categoryTrees.create(
    categoryTree({
      name: "food",
      children: [categoryTree({ name: "fruit", children: [categoryTree({ name: "apple" })] })],
    }),
  );
  await ds.regionCategoryTrees.create(
    regionCategoryTree({
      regionId: 1,
      categoryId: 1,
      name: "west",
      children: [regionCategoryTree({ regionId: 1, categoryId: 2, name: "coast" })],
    }),
  );
  await ds.nodeGraphs.create(
    nodeGraph({
      value: "core",
      edges: [
        nodeGraph({ value: "blade", edges: [nodeGraph({ value: "core" })] }),
        nodeGraph({ value: "potion" }),
      ],
    }),
  );
  await ds.wardViews.create(
    wardView({
      regionId: 1,
      wardId: 1,
      name: "north",
      aliases: [wardAlias({ name: "uptown" })],
      settings: { color: "blue" },
      nodes: [wardNode({ name: "dock" })],
    }),
  );
  await ds.wardViews.create(
    wardView({
      regionId: 1,
      wardId: 2,
      name: "south",
      aliases: [wardAlias({ name: "midtown" })],
      settings: { color: "red" },
      nodes: [wardNode({ name: "pier" })],
    }),
  );
  const created = await collect(ds.wardViews.read());
  const north = created.find((row) => row.wardId === 1)!;
  const south = created.find((row) => row.wardId === 2)!;
  const createNorth = `ward create ${north.regionId}-${north.wardId}/${north.name} alias ${north.aliases[0]!.parentRegionId}-${north.aliases[0]!.parentWardId}/${north.aliases[0]!.name} node ${north.nodes[0]!.parentRegionId}-${north.nodes[0]!.parentWardId}/${north.nodes[0]!.name} setting ${north.settings.color}`;
  const createSouth = `ward create ${south.regionId}-${south.wardId}/${south.name} alias ${south.aliases[0]!.parentRegionId}-${south.aliases[0]!.parentWardId}/${south.aliases[0]!.name} node ${south.nodes[0]!.parentRegionId}-${south.nodes[0]!.parentWardId}/${south.nodes[0]!.name} setting ${south.settings.color}`;
  north.aliases = [wardAlias({ name: "downtown" })];
  north.settings.color = "green";
  north.nodes = [wardNode({ name: "quay" })];
  await ds.wardViews.update(north);
  const updated = (await collect(ds.wardViews.read())).find((row) => row.wardId === 1)!;
  const connection = await ds.createConnection();
  if (!connection.isOpen) await connection.open();
  await ds.badgeViews.create(
    badgeView({
      badgeId: "11111111-1111-1111-1111-111111111111",
      code: "alpha",
      name: "north",
      aliases: [badgeAlias({ name: "uptown" })],
      settings: { color: "blue" },
      nodes: [badgeNode({ name: "dock" })],
    }),
  );
  await ds.badgeViews.create(
    badgeView({
      badgeId: "22222222-2222-2222-2222-222222222222",
      code: "beta",
      name: "south",
      aliases: [badgeAlias({ name: "midtown" })],
      settings: { color: "red" },
      nodes: [badgeNode({ name: "pier" })],
    }),
  );
  const createdBadges = await collect(ds.badgeViews.read());
  const alpha = createdBadges.find((row) => row.code === "alpha")!;
  const beta = createdBadges.find((row) => row.code === "beta")!;
  const createAlpha = `badge create ${alpha.badgeId}/${alpha.code}/${alpha.name} alias ${alpha.aliases[0]!.parentBadgeId}/${alpha.aliases[0]!.parentCode}/${alpha.aliases[0]!.name} node ${alpha.nodes[0]!.parentBadgeId}/${alpha.nodes[0]!.parentCode}/${alpha.nodes[0]!.name} setting ${alpha.settings.color}`;
  const createBeta = `badge create ${beta.badgeId}/${beta.code}/${beta.name} alias ${beta.aliases[0]!.parentBadgeId}/${beta.aliases[0]!.parentCode}/${beta.aliases[0]!.name} node ${beta.nodes[0]!.parentBadgeId}/${beta.nodes[0]!.parentCode}/${beta.nodes[0]!.name} setting ${beta.settings.color}`;
  alpha.aliases = [badgeAlias({ name: "downtown" })];
  alpha.settings.color = "green";
  alpha.nodes = [badgeNode({ name: "quay" })];
  await ds.badgeViews.update(alpha);
  const updatedBadge = (await collect(ds.badgeViews.read())).find((row) => row.code === "alpha")!;
  const maps = await runMapping(connection);
  return transcript(
    await collect(ds.categoryTrees.read()),
    await collect(ds.regionCategoryTrees.read()),
    await collect(ds.nodeGraphs.read()),
    [
      ...maps,
      createNorth,
      createSouth,
      `ward update ${updated.regionId}-${updated.wardId}/${updated.name} alias ${updated.aliases[0]!.parentRegionId}-${updated.aliases[0]!.parentWardId}/${updated.aliases[0]!.name} node ${updated.nodes[0]!.parentRegionId}-${updated.nodes[0]!.parentWardId}/${updated.nodes[0]!.name} setting ${updated.settings.color}`,
      createAlpha,
      createBeta,
      `badge update ${updatedBadge.badgeId}/${updatedBadge.code}/${updatedBadge.name} alias ${updatedBadge.aliases[0]!.parentBadgeId}/${updatedBadge.aliases[0]!.parentCode}/${updatedBadge.aliases[0]!.name} node ${updatedBadge.nodes[0]!.parentBadgeId}/${updatedBadge.nodes[0]!.parentCode}/${updatedBadge.nodes[0]!.name} setting ${updatedBadge.settings.color}`,
    ],
  );
};
