use crate::node::Node;


pub struct Graph {
    nodes: Vec<Box<Node>>,
}

impl Graph {
    pub fn add_node(&mut self, node: Box<Node>) -> &Box<Node> {
        self.nodes.push(node);
        self.nodes.last().unwrap()
    }
}

impl Graph {
    pub fn new() -> Graph {
        Graph { nodes: vec![] }
    }
    pub fn nodes(&self) -> &Vec<Box<Node>> {
        &self.nodes
    }
}
