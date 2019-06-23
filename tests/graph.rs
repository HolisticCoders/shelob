extern crate shelob;

use shelob::graph::Graph;
use shelob::math_nodes::AddNode;


#[test]
fn create_graph() {
    Graph::new();
}

#[test]
fn add_node() {
    let mut graph = Graph::new();
    let node = Box::new(AddNode::new());
    graph.add_node(node);
    assert_eq!(graph.nodes().len(), 1)
}
