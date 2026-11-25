using System.Runtime.CompilerServices;
using Datasources;

public static class RepositoryRegistry
{
    [ModuleInitializer]
    internal static void Bind() => RepositoryFactory.Bind(Register);

    public static void Register(RepositoryFactory factory)
    {
        factory.Register("BadgeAlias", "BadgeAliases", connection => new BadgeAliasesRepository(connection), connection => BadgeAliasesRepository.CreateReadonly(connection));
        factory.Register("BadgeNode", "BadgeNodes", connection => new BadgeNodesRepository(connection), connection => BadgeNodesRepository.CreateReadonly(connection));
        factory.Register("BadgeSetting", "BadgeSettings", connection => new BadgeSettingsRepository(connection), connection => BadgeSettingsRepository.CreateReadonly(connection));
        factory.Register("BadgeView", "BadgeViews", connection => new BadgeViewsRepository(connection), connection => BadgeViewsRepository.CreateReadonly(connection));
        factory.Register("Category", "Categories", connection => new CategoriesRepository(connection), connection => CategoriesRepository.CreateReadonly(connection));
        factory.Register("CategoryTree", "CategoryTrees", connection => new CategoryTreesRepository(connection), connection => CategoryTreesRepository.CreateReadonly(connection));
        factory.Register("Node", "Nodes", connection => new NodesRepository(connection), connection => NodesRepository.CreateReadonly(connection));
        factory.Register("NodeEdge", "NodeEdges", connection => new NodeEdgesRepository(connection), connection => NodeEdgesRepository.CreateReadonly(connection));
        factory.Register("NodeGraph", "NodeGraphs", connection => new NodeGraphsRepository(connection), connection => NodeGraphsRepository.CreateReadonly(connection));
        factory.Register("RegionCategory", "RegionCategories", connection => new RegionCategoriesRepository(connection), connection => RegionCategoriesRepository.CreateReadonly(connection));
        factory.Register("RegionCategoryTree", "RegionCategoryTrees", connection => new RegionCategoryTreesRepository(connection), connection => RegionCategoryTreesRepository.CreateReadonly(connection));
        factory.Register("WardAlias", "WardAliases", connection => new WardAliasesRepository(connection), connection => WardAliasesRepository.CreateReadonly(connection));
        factory.Register("WardNode", "WardNodes", connection => new WardNodesRepository(connection), connection => WardNodesRepository.CreateReadonly(connection));
        factory.Register("WardSetting", "WardSettings", connection => new WardSettingsRepository(connection), connection => WardSettingsRepository.CreateReadonly(connection));
        factory.Register("WardView", "WardViews", connection => new WardViewsRepository(connection), connection => WardViewsRepository.CreateReadonly(connection));
    }
}
