export type TreeNode = { name: string; children: TreeNode[] };
export type RegionNode = { regionId: number; categoryId: number; name: string; children: RegionNode[] };
export type GraphNode = { value: string; edges: { value: string }[] };

export const treeLine = (node: TreeNode): string => {
  const kids = [...node.children].sort((left, right) => left.name.localeCompare(right.name));
  return [node.name, ...kids.map(treeLine)].join(" ");
};

export const regionLine = (node: RegionNode): string => {
  const kids = [...node.children].sort((left, right) => left.name.localeCompare(right.name));
  return [`${node.regionId}-${node.categoryId}/${node.name}`, ...kids.map(regionLine)].join(" ");
};

export const graphLine = (node: GraphNode): string => {
  const edges = [...node.edges].map((edge) => edge.value).sort();
  return edges.length === 0 ? node.value : `${node.value} ${edges.join(",")}`;
};

export const transcript = (trees: TreeNode[], regions: RegionNode[], graphs: GraphNode[]): string[] =>
  [
    ...trees.map((row) => `tree ${treeLine(row)}`),
    ...regions.map((row) => `region ${regionLine(row)}`),
    ...graphs.map((row) => `graph ${graphLine(row)}`),
  ].sort();
