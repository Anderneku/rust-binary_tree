use std::vec;

#[derive(Debug)]
struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

#[derive(Debug)]
struct BinaryTree {
    root: Option<Box<TreeNode>>,
}

impl BinaryTree {
    fn new() -> Self {
        Self { root: None }
    }
    fn insert(&mut self, arr: Vec<i32>) {
        for num in arr{
            let mut current_node = &mut self.root;
            loop {
                match current_node {
                    Some(node) => {
                        if num < node.value {
                            current_node = &mut node.left;
                        } else if num > node.value {
                            current_node = &mut node.right;
                        } else {
                            break;
                        }
                    }
                    None => {
                        *current_node = Some(Box::new(TreeNode { value: num, left: None, right: None }));
                    }
                }
            }
        } 
    }
    fn search(&self, value :i32) {
        let mut current_node = &self.root;
        loop {
            match current_node {
                Some(node) => {
                    if value < node.value {
                        current_node = &node.left;
                    } else if value > node.value {
                        current_node = &node.right;
                    } else {
                        println!("Found!");
                        break;
                    }
                }
                None => {
                    println!("Not Found!");
                    break;
                }
            }
        }
    }
}

fn main() {
    let mut tree = BinaryTree::new();
    tree.insert(vec![10, 4, 32, 3, 1, 5]);
    tree.insert(vec![566]);
    tree.search(566);
}
