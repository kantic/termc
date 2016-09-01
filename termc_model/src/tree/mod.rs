
use std::fmt;

/// Defines a binary tree node structure
//Todo: remove clone?
#[derive(Clone)]
pub struct TreeNode<T: Clone> {
    /// the content of the tree node
    pub content : T,
    pub successors : Vec<Box<TreeNode<T>>>
}

impl<'a, T: Clone> TreeNode<T> {

    /// Creates a new instance of type TreeNode<T>
    pub fn new(c : T) -> TreeNode<T> {
        TreeNode {content: c, successors: Vec::new()}
    }
}

impl<'a, T: fmt::Display + Clone> fmt::Display for TreeNode<T> {

    /// Returns a formatted representation of the tree.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut repr = String::new();
        repr.push_str(& format!("({} ", self.content));

        for n in & self.successors {
            repr.push_str(& format!("{}, ", n));
        }

        repr.push(')');

        write!(f, "{}", repr)
    }
}