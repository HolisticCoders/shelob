use crate::node::Node;

pub struct AddNode {
    name: String,
}
impl Node for AddNode {
    fn evaluate(&self) {
        println!("Evaluating {}", self.name)
    }
}
impl AddNode {
    pub fn new() -> AddNode {
        AddNode {
            name: "AddNode".to_string(),
        }
    }
}
