use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeNode {
    pub id: i64,
    pub name: String,
    pub children: Vec<TreeNode>,
}

pub fn build_tree(nodes: Vec<(i64, String, Option<i64>)>) -> Option<Vec<TreeNode>> {
    if nodes.is_empty() {
        // 如果输入的节点列表为空，则返回 None
        return None;
    }

    let mut tree = Vec::new();
    let mut children_by_parent: HashMap<i64, Vec<TreeNode>> = HashMap::new();

    // 创建所有节点，并根据parentId分类存储
    for (id, name, parent_id) in nodes {
        let node = TreeNode {
            id,
            name,
            children: Vec::new(),
        };
        if let Some(parent_id) = parent_id {
            children_by_parent.entry(parent_id).or_insert_with(Vec::new).push(node);
        } else {
            tree.push(node);
        }
    }

    // 递归构建树
    fn build_subtree(parent_id: i64, children_by_parent: &HashMap<i64, Vec<TreeNode>>) -> Vec<TreeNode> {
        children_by_parent.get(&parent_id)
            .map(|children| {
                children.iter().flat_map(|child| {
                    let mut node = child.clone();
                    node.children = build_subtree(child.id, children_by_parent);
                    vec![node]
                }).collect()
            }).unwrap_or_else(Vec::new)
    }

    // 从根节点开始构建整个树
    let root = build_subtree(0, &children_by_parent);
    if root.is_empty() {
        // 如果根节点为空，则返回 None
        None
    } else {
        // 否则返回 Some 包裹的树
        Some(root)
    }
}