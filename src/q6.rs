// Good morning! Here's your coding interview problem for today.
//
// This problem was asked by Google.
//
// An XOR linked list is a more memory efficient doubly linked list. Instead of each node holding next and prev fields, it holds a field named both, which is an XOR of the next node and the previous node. Implement an XOR linked list; it has an add(element) which adds the element to the end, and a get(index) which returns the node at index.
//
// If using a language that has no pointers (such as Python), you can assume you have access to get_pointer and dereference_pointer functions that converts between nodes and memory addresses.

type OptBoxNode = Option<Box<Node>>;

#[derive(Debug)]
pub struct Node {
    pub val: String,
    pub both: OptBoxNode,
}

#[derive(Debug)]
pub struct DLList {
    pub head: OptBoxNode,
    pub tail: OptBoxNode,
}

impl Default for DLList {
    fn default() -> Self {
        DLList {
            head: None,
            tail: None,
        }
    }
}
impl DLList {
    pub fn add(&mut self, node: OptBoxNode) {}
    pub fn get(index: usize) -> OptBoxNode {}
}
