use super::node::Node;
use std::fmt;

#[derive(Debug)]
pub struct XPath {
    root: Option<Box<Node>>,
}

impl XPath {
    pub fn new() -> Self {
        XPath { root: None }
    }

    pub fn append_node(&mut self, node: Node) {
        let new_node = Some(Box::new(node));

        if let Some(root) = self.root.as_mut() {
            let mut current = root;

            // Traverse to the last node
            while let Some(next) = current.next.take() {
                current.next = Some(next);
                current = current.next.as_mut().unwrap();
            }

            current.next = new_node;
        } else {
            // If there is no root, set the new node as root
            self.root = new_node;
        }
    }
}

impl fmt::Display for XPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref root) = self.root {
            write!(f, "{}", root)
        } else {
            write!(f, "") // Optionally handle the case of an empty XPath
        }
    }
}
