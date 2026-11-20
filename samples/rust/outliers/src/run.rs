use crate::generated::{CategoryTree, NodeGraph, RegionCategoryTree, Repositories};
use crate::{transcript, GraphNode, RegionNode, TreeNode};
use datasources::{default, Result};
use futures_util::TryStreamExt;

pub async fn run() -> Result<Vec<String>> {
    crate::datasources_providers::register();
    let ds = default()?;
    ds.category_trees()
        .create(CategoryTree {
            name: "food".into(),
            children: vec![CategoryTree {
                name: "fruit".into(),
                children: vec![CategoryTree { name: "apple".into(), ..Default::default() }],
                ..Default::default()
            }],
            ..Default::default()
        })
        .await?;
    ds.region_category_trees()
        .create(RegionCategoryTree {
            region_id: 1,
            category_id: 1,
            name: "west".into(),
            children: vec![RegionCategoryTree {
                region_id: 1,
                category_id: 2,
                name: "coast".into(),
                ..Default::default()
            }],
            ..Default::default()
        })
        .await?;
    ds.node_graphs()
        .create(NodeGraph {
            value: "core".into(),
            edges: vec![
                NodeGraph {
                    value: "blade".into(),
                    edges: vec![NodeGraph { value: "core".into(), ..Default::default() }],
                    ..Default::default()
                },
                NodeGraph { value: "potion".into(), ..Default::default() },
            ],
            ..Default::default()
        })
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
