use super::node::Node;

#[allow(dead_code)]
pub struct Graph {
    nodes: Vec<Node>
}

impl Graph {
    #[allow(dead_code)]
    pub fn new() -> Graph {
        Graph {
            nodes: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn add_node(&mut self) {
        let node_count = self.nodes.len();
        let node_name = format!("Node{:03}", node_count);
        let node = Node::new(&node_name);
        self.nodes.push(node);
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn create_graph() {
        Graph::new();
    }

    #[test]
    fn add_node() {
        let mut graph = Graph::new();
        graph.add_node();
        assert_eq!(graph.nodes.len(), 1);
    }
}
