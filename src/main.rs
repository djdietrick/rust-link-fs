mod node;
use node::{Type, Node};

fn main() {
    let _node = Node::new(String::from("Test"), Type::Directory, None);
}
