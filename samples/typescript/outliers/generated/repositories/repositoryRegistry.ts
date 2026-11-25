import { RepositoryFactory } from "@deterministic-code/datasources";
import { BadgeAliasesRepository } from "./badgeAliasesRepository";
import { BadgeNodesRepository } from "./badgeNodesRepository";
import { BadgeSettingsRepository } from "./badgeSettingsRepository";
import { BadgeViewsRepository } from "./badgeViewsRepository";
import { CategoriesRepository } from "./categoriesRepository";
import { CategoryTreesRepository } from "./categoryTreesRepository";
import { NodesRepository } from "./nodesRepository";
import { NodeEdgesRepository } from "./nodeEdgesRepository";
import { NodeGraphsRepository } from "./nodeGraphsRepository";
import { RegionCategoriesRepository } from "./regionCategoriesRepository";
import { RegionCategoryTreesRepository } from "./regionCategoryTreesRepository";
import { WardAliasesRepository } from "./wardAliasesRepository";
import { WardNodesRepository } from "./wardNodesRepository";
import { WardSettingsRepository } from "./wardSettingsRepository";
import { WardViewsRepository } from "./wardViewsRepository";
export { badgeAlias } from "../types/datasource/badgeAlias";
export { badgeNode } from "../types/datasource/badgeNode";
export { badgeSetting } from "../types/datasource/badgeSetting";
export { badgeView } from "../types/view/badgeView";
export { category } from "../types/datasource/category";
export { categoryTree } from "../types/view/categoryTree";
export { node } from "../types/datasource/node";
export { nodeEdge } from "../types/datasource/nodeEdge";
export { nodeGraph } from "../types/view/nodeGraph";
export { regionCategory } from "../types/datasource/regionCategory";
export { regionCategoryTree } from "../types/view/regionCategoryTree";
export { wardAlias } from "../types/datasource/wardAlias";
export { wardNode } from "../types/datasource/wardNode";
export { wardSetting } from "../types/datasource/wardSetting";
export { wardView } from "../types/view/wardView";

export const register = (factory: RepositoryFactory): void => {
  factory.register("BadgeAlias", "badgeAliases", (connection) => new BadgeAliasesRepository(connection), (connection) => BadgeAliasesRepository.createReadonly(connection));
  factory.register("BadgeNode", "badgeNodes", (connection) => new BadgeNodesRepository(connection), (connection) => BadgeNodesRepository.createReadonly(connection));
  factory.register("BadgeSetting", "badgeSettings", (connection) => new BadgeSettingsRepository(connection), (connection) => BadgeSettingsRepository.createReadonly(connection));
  factory.register("BadgeView", "badgeViews", (connection) => new BadgeViewsRepository(connection), (connection) => BadgeViewsRepository.createReadonly(connection));
  factory.register("Category", "categories", (connection) => new CategoriesRepository(connection), (connection) => CategoriesRepository.createReadonly(connection));
  factory.register("CategoryTree", "categoryTrees", (connection) => new CategoryTreesRepository(connection), (connection) => CategoryTreesRepository.createReadonly(connection));
  factory.register("Node", "nodes", (connection) => new NodesRepository(connection), (connection) => NodesRepository.createReadonly(connection));
  factory.register("NodeEdge", "nodeEdges", (connection) => new NodeEdgesRepository(connection), (connection) => NodeEdgesRepository.createReadonly(connection));
  factory.register("NodeGraph", "nodeGraphs", (connection) => new NodeGraphsRepository(connection), (connection) => NodeGraphsRepository.createReadonly(connection));
  factory.register("RegionCategory", "regionCategories", (connection) => new RegionCategoriesRepository(connection), (connection) => RegionCategoriesRepository.createReadonly(connection));
  factory.register("RegionCategoryTree", "regionCategoryTrees", (connection) => new RegionCategoryTreesRepository(connection), (connection) => RegionCategoryTreesRepository.createReadonly(connection));
  factory.register("WardAlias", "wardAliases", (connection) => new WardAliasesRepository(connection), (connection) => WardAliasesRepository.createReadonly(connection));
  factory.register("WardNode", "wardNodes", (connection) => new WardNodesRepository(connection), (connection) => WardNodesRepository.createReadonly(connection));
  factory.register("WardSetting", "wardSettings", (connection) => new WardSettingsRepository(connection), (connection) => WardSettingsRepository.createReadonly(connection));
  factory.register("WardView", "wardViews", (connection) => new WardViewsRepository(connection), (connection) => WardViewsRepository.createReadonly(connection));
};
RepositoryFactory.bind(register);

declare module "@deterministic-code/datasources" {
  interface Datasource {
    readonly badgeAliases: BadgeAliasesRepository;
    readonly badgeNodes: BadgeNodesRepository;
    readonly badgeSettings: BadgeSettingsRepository;
    readonly badgeViews: BadgeViewsRepository;
    readonly categories: CategoriesRepository;
    readonly categoryTrees: CategoryTreesRepository;
    readonly nodes: NodesRepository;
    readonly nodeEdges: NodeEdgesRepository;
    readonly nodeGraphs: NodeGraphsRepository;
    readonly regionCategories: RegionCategoriesRepository;
    readonly regionCategoryTrees: RegionCategoryTreesRepository;
    readonly wardAliases: WardAliasesRepository;
    readonly wardNodes: WardNodesRepository;
    readonly wardSettings: WardSettingsRepository;
    readonly wardViews: WardViewsRepository;
  }
}
