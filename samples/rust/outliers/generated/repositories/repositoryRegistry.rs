use datasources::{Datasource, DatasourceError, ICRUDRepository, IReadonlyRepository, RepositoryFactory};
use super::super::Category;
use super::super::CategoryTree;
use super::super::Node;
use super::super::NodeEdge;
use super::super::NodeGraph;
use super::super::RegionCategory;
use super::super::RegionCategoryTree;
use super::super::WardAlias;
use super::super::WardNode;
use super::super::WardSetting;
use super::super::WardView;
use std::sync::Arc;

pub fn register(factory: &RepositoryFactory) {
    factory.register_named(
        "Category",
        "categories",
        |connection| Ok(Arc::new(super::categoriesRepository::CategoriesRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<Category>>),
        |connection| Arc::new(super::categoriesRepository::CategoriesRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<Category>>,
    );
    factory.register_named(
        "CategoryTree",
        "category_trees",
        |connection| Ok(Arc::new(super::categoryTreesRepository::CategoryTreesRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<CategoryTree>>),
        |connection| Arc::new(super::categoryTreesRepository::CategoryTreesRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<CategoryTree>>,
    );
    factory.register_named(
        "Node",
        "nodes",
        |connection| Ok(Arc::new(super::nodesRepository::NodesRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<Node>>),
        |connection| Arc::new(super::nodesRepository::NodesRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<Node>>,
    );
    factory.register_named(
        "NodeEdge",
        "node_edges",
        |connection| Ok(Arc::new(super::nodeEdgesRepository::NodeEdgesRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<NodeEdge>>),
        |connection| Arc::new(super::nodeEdgesRepository::NodeEdgesRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<NodeEdge>>,
    );
    factory.register_named(
        "NodeGraph",
        "node_graphs",
        |connection| Ok(Arc::new(super::nodeGraphsRepository::NodeGraphsRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<NodeGraph>>),
        |connection| Arc::new(super::nodeGraphsRepository::NodeGraphsRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<NodeGraph>>,
    );
    factory.register_named(
        "RegionCategory",
        "region_categories",
        |connection| Ok(Arc::new(super::regionCategoriesRepository::RegionCategoriesRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<RegionCategory>>),
        |connection| Arc::new(super::regionCategoriesRepository::RegionCategoriesRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<RegionCategory>>,
    );
    factory.register_named(
        "RegionCategoryTree",
        "region_category_trees",
        |connection| Ok(Arc::new(super::regionCategoryTreesRepository::RegionCategoryTreesRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<RegionCategoryTree>>),
        |connection| Arc::new(super::regionCategoryTreesRepository::RegionCategoryTreesRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<RegionCategoryTree>>,
    );
    factory.register_named(
        "WardAlias",
        "ward_aliases",
        |connection| Ok(Arc::new(super::wardAliasesRepository::WardAliasesRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<WardAlias>>),
        |connection| Arc::new(super::wardAliasesRepository::WardAliasesRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<WardAlias>>,
    );
    factory.register_named(
        "WardNode",
        "ward_nodes",
        |connection| Ok(Arc::new(super::wardNodesRepository::WardNodesRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<WardNode>>),
        |connection| Arc::new(super::wardNodesRepository::WardNodesRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<WardNode>>,
    );
    factory.register_named(
        "WardSetting",
        "ward_settings",
        |connection| Ok(Arc::new(super::wardSettingsRepository::WardSettingsRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<WardSetting>>),
        |connection| Arc::new(super::wardSettingsRepository::WardSettingsRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<WardSetting>>,
    );
    factory.register_named(
        "WardView",
        "ward_views",
        |connection| Ok(Arc::new(super::wardViewsRepository::WardViewsRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<WardView>>),
        |connection| Arc::new(super::wardViewsRepository::WardViewsRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<WardView>>,
    );
}

datasources::bind_repositories!(register);

pub trait Repositories {
    fn categories(&self) -> Arc<super::categoriesRepository::CategoriesRepository>;
    fn category_trees(&self) -> Arc<super::categoryTreesRepository::CategoryTreesRepository>;
    fn nodes(&self) -> Arc<super::nodesRepository::NodesRepository>;
    fn node_edges(&self) -> Arc<super::nodeEdgesRepository::NodeEdgesRepository>;
    fn node_graphs(&self) -> Arc<super::nodeGraphsRepository::NodeGraphsRepository>;
    fn region_categories(&self) -> Arc<super::regionCategoriesRepository::RegionCategoriesRepository>;
    fn region_category_trees(&self) -> Arc<super::regionCategoryTreesRepository::RegionCategoryTreesRepository>;
    fn ward_aliases(&self) -> Arc<super::wardAliasesRepository::WardAliasesRepository>;
    fn ward_nodes(&self) -> Arc<super::wardNodesRepository::WardNodesRepository>;
    fn ward_settings(&self) -> Arc<super::wardSettingsRepository::WardSettingsRepository>;
    fn ward_views(&self) -> Arc<super::wardViewsRepository::WardViewsRepository>;
}

impl Repositories for Arc<dyn Datasource> {
    fn categories(&self) -> Arc<super::categoriesRepository::CategoriesRepository> {
        Arc::new(super::categoriesRepository::CategoriesRepository::new(self.bind_connection(None)).expect("categories"))
    }
    fn category_trees(&self) -> Arc<super::categoryTreesRepository::CategoryTreesRepository> {
        Arc::new(super::categoryTreesRepository::CategoryTreesRepository::new(self.bind_connection(None)).expect("category_trees"))
    }
    fn nodes(&self) -> Arc<super::nodesRepository::NodesRepository> {
        Arc::new(super::nodesRepository::NodesRepository::new(self.bind_connection(None)).expect("nodes"))
    }
    fn node_edges(&self) -> Arc<super::nodeEdgesRepository::NodeEdgesRepository> {
        Arc::new(super::nodeEdgesRepository::NodeEdgesRepository::new(self.bind_connection(None)).expect("node_edges"))
    }
    fn node_graphs(&self) -> Arc<super::nodeGraphsRepository::NodeGraphsRepository> {
        Arc::new(super::nodeGraphsRepository::NodeGraphsRepository::new(self.bind_connection(None)).expect("node_graphs"))
    }
    fn region_categories(&self) -> Arc<super::regionCategoriesRepository::RegionCategoriesRepository> {
        Arc::new(super::regionCategoriesRepository::RegionCategoriesRepository::new(self.bind_connection(None)).expect("region_categories"))
    }
    fn region_category_trees(&self) -> Arc<super::regionCategoryTreesRepository::RegionCategoryTreesRepository> {
        Arc::new(super::regionCategoryTreesRepository::RegionCategoryTreesRepository::new(self.bind_connection(None)).expect("region_category_trees"))
    }
    fn ward_aliases(&self) -> Arc<super::wardAliasesRepository::WardAliasesRepository> {
        Arc::new(super::wardAliasesRepository::WardAliasesRepository::new(self.bind_connection(None)).expect("ward_aliases"))
    }
    fn ward_nodes(&self) -> Arc<super::wardNodesRepository::WardNodesRepository> {
        Arc::new(super::wardNodesRepository::WardNodesRepository::new(self.bind_connection(None)).expect("ward_nodes"))
    }
    fn ward_settings(&self) -> Arc<super::wardSettingsRepository::WardSettingsRepository> {
        Arc::new(super::wardSettingsRepository::WardSettingsRepository::new(self.bind_connection(None)).expect("ward_settings"))
    }
    fn ward_views(&self) -> Arc<super::wardViewsRepository::WardViewsRepository> {
        Arc::new(super::wardViewsRepository::WardViewsRepository::new(self.bind_connection(None)).expect("ward_views"))
    }
}
