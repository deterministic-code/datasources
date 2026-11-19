using System.Runtime.CompilerServices;
using Datasources;

public static class RepositoryRegistry
{
    [ModuleInitializer]
    internal static void Bind() => RepositoryFactory.Bind(Register);

    public static void Register(RepositoryFactory factory)
    {
        factory.Register("Category", "Categories", connection => new CategoriesRepository(connection), connection => CategoriesRepository.CreateReadonly(connection));
        factory.Register("CategoryTree", "CategoryTrees", connection => new CategoryTreesRepository(connection), connection => CategoryTreesRepository.CreateReadonly(connection));
        factory.Register("Node", "Nodes", connection => new NodesRepository(connection), connection => NodesRepository.CreateReadonly(connection));
        factory.Register("NodeEdge", "NodeEdges", connection => new NodeEdgesRepository(connection), connection => NodeEdgesRepository.CreateReadonly(connection));
        factory.Register("NodeGraph", "NodeGraphs", connection => new NodeGraphsRepository(connection), connection => NodeGraphsRepository.CreateReadonly(connection));
        factory.Register("RegionCategory", "RegionCategories", connection => new RegionCategoriesRepository(connection), connection => RegionCategoriesRepository.CreateReadonly(connection));
        factory.Register("RegionCategoryTree", "RegionCategoryTrees", connection => new RegionCategoryTreesRepository(connection), connection => RegionCategoryTreesRepository.CreateReadonly(connection));
    }
}
