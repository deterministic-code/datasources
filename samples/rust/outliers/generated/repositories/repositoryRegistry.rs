use datasources::{Datasource, DatasourceError, ICRUDRepository, IReadonlyRepository, RepositoryFactory};
use super::super::Category;
use super::super::CategoryTree;
use super::super::Node;
use super::super::NodeEdge;
use super::super::NodeGraph;
use super::super::RegionCategory;
use super::super::RegionCategoryTree;
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
}
