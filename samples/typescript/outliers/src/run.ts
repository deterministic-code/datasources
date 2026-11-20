import "../generated/repositories/repositoryRegistry.ts";
import { Datasource } from "@deterministic-code/datasources";
import { categoryTree, nodeGraph, regionCategoryTree } from "../generated/repositories/repositoryRegistry.ts";
import { transcript } from "./format.ts";

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
  return transcript(
    await collect(ds.categoryTrees.read()),
    await collect(ds.regionCategoryTrees.read()),
    await collect(ds.nodeGraphs.read()),
  );
};
