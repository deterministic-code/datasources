#[derive(Clone)]
pub struct TreeNode {
    pub name: String,
    pub children: Vec<TreeNode>,
}

#[derive(Clone)]
pub struct RegionNode {
    pub region_id: i32,
    pub category_id: i32,
    pub name: String,
    pub children: Vec<RegionNode>,
}

#[derive(Clone)]
pub struct GraphNode {
    pub value: String,
    pub edges: Vec<GraphNode>,
}

pub fn tree_line(node: &TreeNode) -> String {
    let mut kids = node.children.clone();
    kids.sort_by(|left, right| left.name.cmp(&right.name));
    let rest = kids.iter().map(tree_line).collect::<Vec<_>>();
    if rest.is_empty() {
        node.name.clone()
    } else {
        format!("{} {}", node.name, rest.join(" "))
    }
}

pub fn region_line(node: &RegionNode) -> String {
    let mut kids = node.children.clone();
    kids.sort_by(|left, right| left.name.cmp(&right.name));
    let label = format!("{}-{}/{}", node.region_id, node.category_id, node.name);
    let rest = kids.iter().map(region_line).collect::<Vec<_>>();
    if rest.is_empty() {
        label
    } else {
        format!("{} {}", label, rest.join(" "))
    }
}

pub fn graph_line(node: &GraphNode) -> String {
    let mut edges = node.edges.iter().map(|edge| edge.value.clone()).collect::<Vec<_>>();
    edges.sort();
    if edges.is_empty() {
        node.value.clone()
    } else {
        format!("{} {}", node.value, edges.join(","))
    }
}

pub fn map_error(method: &str, message: &str) -> Vec<String> {
    vec![format!("map {method} error {message}")]
}

pub fn map_lines(method: &str, cells: &[(&str, String)]) -> Vec<String> {
    cells
        .iter()
        .map(|(column, value)| {
            if value.is_empty() {
                format!("map {method} {column}")
            } else {
                format!("map {method} {column} {value}")
            }
        })
        .collect()
}

pub fn transcript(
    trees: &[TreeNode],
    regions: &[RegionNode],
    graphs: &[GraphNode],
    maps: &[String],
) -> Vec<String> {
    let mut lines = trees
        .iter()
        .map(|row| format!("tree {}", tree_line(row)))
        .chain(regions.iter().map(|row| format!("region {}", region_line(row))))
        .chain(graphs.iter().map(|row| format!("graph {}", graph_line(row))))
        .chain(maps.iter().cloned())
        .collect::<Vec<_>>();
    lines.sort();
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transcript_covers_tree_region_and_graph() {
        let apple = TreeNode { name: "apple".into(), children: vec![] };
        let fruit = TreeNode { name: "fruit".into(), children: vec![apple.clone()] };
        let food = TreeNode { name: "food".into(), children: vec![fruit] };
        assert_eq!(tree_line(&food), "food fruit apple");
        assert_eq!(tree_line(&apple), "apple");
        let child = RegionNode { region_id: 1, category_id: 2, name: "coast".into(), children: vec![] };
        let root = RegionNode {
            region_id: 1,
            category_id: 1,
            name: "west".into(),
            children: vec![child],
        };
        assert_eq!(region_line(&root), "1-1/west 1-2/coast");
        assert_eq!(
            graph_line(&GraphNode {
                value: "core".into(),
                edges: vec![
                    GraphNode { value: "potion".into(), edges: vec![] },
                    GraphNode { value: "blade".into(), edges: vec![] },
                ],
            }),
            "core blade,potion"
        );
        assert_eq!(graph_line(&GraphNode { value: "blade".into(), edges: vec![] }), "blade");
        assert_eq!(
            transcript(
                &[food],
                &[root],
                &[GraphNode {
                    value: "core".into(),
                    edges: vec![GraphNode { value: "blade".into(), edges: vec![] }],
                }],
                &["map create text_to_integer 10".into()]
            ),
            vec![
                "graph core blade".to_string(),
                "map create text_to_integer 10".to_string(),
                "region 1-1/west 1-2/coast".to_string(),
                "tree food fruit apple".to_string()
            ]
        );
        assert_eq!(
            map_lines("create", &[("text_to_guid", "550e8400-e29b-41d4-a716-446655440000".into())]),
            vec!["map create text_to_guid 550e8400-e29b-41d4-a716-446655440000".to_string()]
        );
        assert_eq!(map_lines("create", &[("guid_to_guid", String::new())]), vec!["map create guid_to_guid".to_string()]);
        assert_eq!(map_error("read", "\"10.9\" is not a long"), vec!["map read error \"10.9\" is not a long".to_string()]);
    }
}
