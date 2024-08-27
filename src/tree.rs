struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: i32) {
        if value < self.value {
            match &mut self.left {
                Some(left) => left.insert(value),
                None => self.left = Some(Box::new(Node::new(value))),
            }
        } else {
            match &mut self.right {
                Some(right) => right.insert(value),
                None => self.right = Some(Box::new(Node::new(value))),
            }
        }
    }

    fn search(&self, value: i32) -> bool {
        if value == self.value {
            return true;
        }
        if value < self.value {
            if let Some(left) = &self.left {
                return left.search(value);
            }
        } else {
            if let Some(right) = &self.right {
                return right.search(value);
            }
        }
        false
    }
}

pub fn main() {
    let mut tree = Node::new(20);
    tree.insert(22);
    tree.insert(23);
    tree.insert(3);
    tree.insert(7);
    tree.insert(12);
    tree.insert(18);

    print_tree(&tree);
}

fn print_tree(node: &Node) {
    print_tree_helper(node, None);
}

fn print_tree_helper(node: &Node, parent: Option<i32>) {
    if let Some(parent_value) = parent {
        println!("{} -> {}", node.value, parent_value);
    } else {
        println!("{} -> Root", node.value);
    }

    if let Some(left) = &node.left {
        print_tree_helper(left, Some(node.value));
    }

    if let Some(right) = &node.right {
        print_tree_helper(right, Some(node.value));
    }
}
