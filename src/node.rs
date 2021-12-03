
#[derive(PartialEq)]
pub enum Type {
    File,
    Directory
}

pub struct Node {
    _children: Vec<Node>,
    _type: Type,
    _name: String,
    _url: Option<String>
}

impl Node {
    pub fn new(n: String, t: Type, u: Option<String>) -> Node {
        Node {
            _children: Vec::new(),
            _type: t,
            _name: n,
            _url: u
        }
    }

    fn mkdir(&mut self, n: String) -> Result<(), String> {
        if self._children.iter().any(|c| c._name == n) {
            return Err(String::from("Directory already exists"));
        }
        self._children.push(Node::new(n, Type::Directory, None));
        Ok(())
    }

    fn touch(&mut self, n: String, url: String) {
        self._children.push(Node::new(n, Type::File, Some(url)));
    }

    fn cd(self, n: String) -> Result<Node, String> {
        for child in self._children {
            if child._name == n && child._type == Type::Directory {
                return Ok(child)
            }
        }
        return Err(String::from("No subdirectory called ") + &n)
    }


}