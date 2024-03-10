
//! Create a node with a given name.
pub fn create_node(name: String) -> Node {

    let new_node: Node = {
        name,
        publishers: Vec::new(),
        subscribers: Vec::new()
    };
}

pub fn add_subscriber(topic_name: String, ) {
}

