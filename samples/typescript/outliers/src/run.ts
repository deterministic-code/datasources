import "../generated/repositories/repositoryRegistry.ts";
import { Datasource } from "@deterministic-code/datasources";
import { node, nodeEdge, regionCategory } from "../generated/repositories/repositoryRegistry.ts";
import { transcript } from "./format.ts";

const collect = async <T>(rows: AsyncIterable<T>): Promise<T[]> => {
  const out: T[] = [];
  for await (const row of rows) out.push(row);
  return out;
};

export const run = async (): Promise<string[]> => {
  await using ds = Datasource.default;
  await ds.regionCategories.create(regionCategory({ regionId: 1, categoryId: 1, parentRegionId: null, parentCategoryId: null, name: "west" }));
  await ds.regionCategories.create(regionCategory({ regionId: 1, categoryId: 2, parentRegionId: 1, parentCategoryId: 1, name: "coast" }));
  const core = await ds.nodes.create(node({ value: "core" }));
  const blade = await ds.nodes.create(node({ value: "blade" }));
  const potion = await ds.nodes.create(node({ value: "potion" }));
  await ds.nodeEdges.create(nodeEdge({ fromNodeId: core.id, toNodeId: blade.id }));
  await ds.nodeEdges.create(nodeEdge({ fromNodeId: core.id, toNodeId: potion.id }));
  await ds.nodeEdges.create(nodeEdge({ fromNodeId: blade.id, toNodeId: core.id }));
  return transcript(
    await collect(ds.categoryTrees.read()),
    await collect(ds.regionCategoryTrees.read()),
    await collect(ds.nodeGraphs.read()),
  );
};
