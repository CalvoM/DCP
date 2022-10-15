// Good morning! Here's your coding interview problem for today.

// This problem was asked by Google.

// Given the root to a binary tree, implement serialize(root), which serializes the tree into a string, and deserialize(s), which deserializes the string back into the tree.
type OptBoxNode = Option<Box<Node>>;
#[derive(Debug)]
pub struct Node {
    pub val: String,
    pub right: OptBoxNode,
    pub left: OptBoxNode,
}

pub fn serialize(root: Node) -> String {
    preorder_traversal(Box::new(root))
}

pub fn deserialize(serial: String) -> Node {
    let nodes: Vec<&str> = serial.split('-').collect();
    let mut node_index = 0;
    Node {
        val: String::from(nodes[node_index]),
        left: deserialize_node(&nodes, &mut node_index),
        right: deserialize_node(&nodes, &mut node_index),
    }
}
fn deserialize_node(nodes: &Vec<&str>, index: &mut usize) -> OptBoxNode {
    *index += 1;
    if nodes[*index] == "+" {
        None
    } else {
        Some(Box::new(Node {
            val: String::from(nodes[*index]),
            left: deserialize_node(nodes, index),
            right: deserialize_node(nodes, index),
        }))
    }
}

fn preorder_traversal(root: Box<Node>) -> String {
    let mut content: String;
    content = format!("{}-", root.val);
    if let Some(left) = root.left {
        content = format!("{}{}", content, preorder_traversal(left));
    } else {
        content = format!("{}+-", content);
    }
    if let Some(right) = root.right {
        content = format!("{}{}", content, preorder_traversal(right));
    } else {
        content = format!("{}+-", content);
    }
    content
}
