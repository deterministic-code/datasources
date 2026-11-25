use crate::generated::{
    BadgeAlias, BadgeNode, BadgeView, CategoryTree, NodeGraph, RegionCategoryTree, Repositories, WardAlias, WardNode,
    WardView,
};
use crate::{transcript, GraphNode, RegionNode, TreeNode};
use datasources::uuid::Uuid;
use datasources::{default, Result};
use futures_util::TryStreamExt;
use std::collections::HashMap;

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
    ds.ward_views()
        .create(WardView {
            region_id: 1,
            ward_id: 1,
            name: "north".into(),
            aliases: vec![WardAlias {
                name: "uptown".into(),
                ..Default::default()
            }],
            settings: HashMap::from([("color".into(), "blue".into())]),
            nodes: vec![WardNode {
                name: "dock".into(),
                ..Default::default()
            }],
        })
        .await?;
    ds.ward_views()
        .create(WardView {
            region_id: 1,
            ward_id: 2,
            name: "south".into(),
            aliases: vec![WardAlias {
                name: "midtown".into(),
                ..Default::default()
            }],
            settings: HashMap::from([("color".into(), "red".into())]),
            nodes: vec![WardNode {
                name: "pier".into(),
                ..Default::default()
            }],
        })
        .await?;
    let created: Vec<WardView> = ds.ward_views().read(None, None).try_collect().await?;
    let mut north = created
        .iter()
        .find(|row| row.ward_id == 1)
        .cloned()
        .expect("north");
    let south = created.iter().find(|row| row.ward_id == 2).cloned().expect("south");
    let create_north = format!(
        "ward create {}-{}/{} alias {}-{}/{} node {}-{}/{} setting {}",
        north.region_id,
        north.ward_id,
        north.name,
        north.aliases[0].parent_region_id,
        north.aliases[0].parent_ward_id,
        north.aliases[0].name,
        north.nodes[0].parent_region_id,
        north.nodes[0].parent_ward_id,
        north.nodes[0].name,
        north.settings["color"]
    );
    let create_south = format!(
        "ward create {}-{}/{} alias {}-{}/{} node {}-{}/{} setting {}",
        south.region_id,
        south.ward_id,
        south.name,
        south.aliases[0].parent_region_id,
        south.aliases[0].parent_ward_id,
        south.aliases[0].name,
        south.nodes[0].parent_region_id,
        south.nodes[0].parent_ward_id,
        south.nodes[0].name,
        south.settings["color"]
    );
    north.aliases = vec![WardAlias {
        name: "downtown".into(),
        ..Default::default()
    }];
    north.settings.insert("color".into(), "green".into());
    north.nodes = vec![WardNode {
        name: "quay".into(),
        ..Default::default()
    }];
    ds.ward_views().update(north, None).await?;
    let updated: Vec<WardView> = ds.ward_views().read(None, None).try_collect().await?;
    let updated = updated.into_iter().find(|row| row.ward_id == 1).expect("updated");
    let update = format!(
        "ward update {}-{}/{} alias {}-{}/{} node {}-{}/{} setting {}",
        updated.region_id,
        updated.ward_id,
        updated.name,
        updated.aliases[0].parent_region_id,
        updated.aliases[0].parent_ward_id,
        updated.aliases[0].name,
        updated.nodes[0].parent_region_id,
        updated.nodes[0].parent_ward_id,
        updated.nodes[0].name,
        updated.settings["color"]
    );
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
    let connection = ds.create_connection().await?;
    if !connection.is_open() {
        connection.open().await?;
    }
    ds.badge_views()
        .create(BadgeView {
            badge_id: Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap(),
            code: "alpha".into(),
            name: "north".into(),
            aliases: vec![BadgeAlias {
                name: "uptown".into(),
                ..Default::default()
            }],
            settings: HashMap::from([("color".into(), "blue".into())]),
            nodes: vec![BadgeNode {
                name: "dock".into(),
                ..Default::default()
            }],
        })
        .await?;
    ds.badge_views()
        .create(BadgeView {
            badge_id: Uuid::parse_str("22222222-2222-2222-2222-222222222222").unwrap(),
            code: "beta".into(),
            name: "south".into(),
            aliases: vec![BadgeAlias {
                name: "midtown".into(),
                ..Default::default()
            }],
            settings: HashMap::from([("color".into(), "red".into())]),
            nodes: vec![BadgeNode {
                name: "pier".into(),
                ..Default::default()
            }],
        })
        .await?;
    let created_badges: Vec<BadgeView> = ds.badge_views().read(None, None).try_collect().await?;
    let mut alpha = created_badges
        .iter()
        .find(|row| row.code == "alpha")
        .cloned()
        .expect("alpha");
    let beta = created_badges
        .iter()
        .find(|row| row.code == "beta")
        .cloned()
        .expect("beta");
    let create_alpha = format!(
        "badge create {}/{}/{} alias {}/{}/{} node {}/{}/{} setting {}",
        alpha.badge_id,
        alpha.code,
        alpha.name,
        alpha.aliases[0].parent_badge_id,
        alpha.aliases[0].parent_code,
        alpha.aliases[0].name,
        alpha.nodes[0].parent_badge_id,
        alpha.nodes[0].parent_code,
        alpha.nodes[0].name,
        alpha.settings["color"]
    );
    let create_beta = format!(
        "badge create {}/{}/{} alias {}/{}/{} node {}/{}/{} setting {}",
        beta.badge_id,
        beta.code,
        beta.name,
        beta.aliases[0].parent_badge_id,
        beta.aliases[0].parent_code,
        beta.aliases[0].name,
        beta.nodes[0].parent_badge_id,
        beta.nodes[0].parent_code,
        beta.nodes[0].name,
        beta.settings["color"]
    );
    alpha.aliases = vec![BadgeAlias {
        name: "downtown".into(),
        ..Default::default()
    }];
    alpha.settings.insert("color".into(), "green".into());
    alpha.nodes = vec![BadgeNode {
        name: "quay".into(),
        ..Default::default()
    }];
    ds.badge_views().update(alpha, None).await?;
    let updated_badges: Vec<BadgeView> = ds.badge_views().read(None, None).try_collect().await?;
    let updated_badge = updated_badges
        .into_iter()
        .find(|row| row.code == "alpha")
        .expect("updated badge");
    let badge_update = format!(
        "badge update {}/{}/{} alias {}/{}/{} node {}/{}/{} setting {}",
        updated_badge.badge_id,
        updated_badge.code,
        updated_badge.name,
        updated_badge.aliases[0].parent_badge_id,
        updated_badge.aliases[0].parent_code,
        updated_badge.aliases[0].name,
        updated_badge.nodes[0].parent_badge_id,
        updated_badge.nodes[0].parent_code,
        updated_badge.nodes[0].name,
        updated_badge.settings["color"]
    );
    let maps = crate::mapping::run(connection).await;
    let mut lines = maps;
    lines.extend([create_north, create_south, update, create_alpha, create_beta, badge_update]);
    Ok(transcript(&trees, &regions, &graphs, &lines))
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
