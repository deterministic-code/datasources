use crate::generated::{Node, NodeEdge, RegionCategory, Repositories};
use crate::{transcript, GraphNode, RegionNode, TreeNode};
use datasources::{default, Result};
use futures_util::TryStreamExt;

pub async fn run() -> Result<Vec<String>> {
    crate::datasources_providers::register();
    let ds = default()?;
    ds.region_categories()
        .create(RegionCategory { region_id: 1, category_id: 1, name: "west".into(), ..Default::default() })
        .await?;
    ds.region_categories()
        .create(RegionCategory {
            region_id: 1,
            category_id: 2,
            parent_region_id: Some(1),
            parent_category_id: Some(1),
            name: "coast".into(),
            ..Default::default()
        })
        .await?;
    let core = ds.nodes().create(Node { value: "core".into(), ..Default::default() }).await?;
    let blade = ds.nodes().create(Node { value: "blade".into(), ..Default::default() }).await?;
    let potion = ds.nodes().create(Node { value: "potion".into(), ..Default::default() }).await?;
    ds.node_edges()
        .create(NodeEdge { from_node_id: core.id, to_node_id: blade.id })
        .await?;
    ds.node_edges()
        .create(NodeEdge { from_node_id: core.id, to_node_id: potion.id })
        .await?;
    ds.node_edges()
        .create(NodeEdge { from_node_id: blade.id, to_node_id: core.id })
        .await?;
    let trees = ds
        .category_trees()
        .read(None, None)
        .try_collect::<Vec<_>>()
        .await?
        .into_iter()
        .map(tree_of)
        .collect::<Vec<_>>();
    let regions = ds
        .region_category_trees()
        .read(None, None)
        .try_collect::<Vec<_>>()
        .await?
        .into_iter()
        .map(region_of)
        .collect::<Vec<_>>();
    let graphs = ds
        .node_graphs()
        .read(None, None)
        .try_collect::<Vec<_>>()
        .await?
        .into_iter()
        .map(graph_of)
        .collect::<Vec<_>>();
    Ok(transcript(&trees, &regions, &graphs))
}

fn tree_of(node: crate::generated::CategoryTree) -> TreeNode {
    TreeNode { name: node.name, children: node.children.into_iter().map(tree_of).collect() }
}

fn region_of(node: crate::generated::RegionCategoryTree) -> RegionNode {
    RegionNode {
        region_id: node.region_id,
        category_id: node.category_id,
        name: node.name,
        children: node.children.into_iter().map(region_of).collect(),
    }
}

fn graph_of(node: crate::generated::NodeGraph) -> GraphNode {
    GraphNode { value: node.value, edges: node.edges.into_iter().map(graph_of).collect() }
}
