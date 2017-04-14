
extern crate serde_json;
extern crate serde;

use self::serde::ser::Serialize;

use std::fmt;

/// Defines a binary tree node structure
#[derive(Clone, Serialize, Deserialize)]
pub struct TreeNode<T: Clone + Serialize> {
    /// the content of the tree node
    pub content : T,
    pub successors : Vec<Box<TreeNode<T>>>
}

impl<'a, T: Clone + Serialize> TreeNode<T> {

    /// Creates a new instance of type TreeNode<T>
    pub fn new(c : T) -> TreeNode<T> {
        TreeNode {content: c, successors: Vec::new()}
    }
}

impl<'a, T: fmt::Display + Clone + Serialize> fmt::Display for TreeNode<T> {

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
