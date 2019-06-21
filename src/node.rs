#[allow(dead_code)]
pub struct Node {
    name: String,
}

impl Node {
    pub fn new(name: &String) -> Node {
        Node{
            name: name.clone(),
        }
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn create_node() {
        let name = "Node".to_string();
        Node::new(&name);
    }
}
