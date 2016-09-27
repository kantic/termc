
extern crate serde_json;

use std::fmt;
use serde_json::{Value, Map};
use serialization::{Serialization, SerializationError};

/// Defines a binary tree node structure
#[derive(Clone)]
pub struct TreeNode<T: Clone + Serialization> {
    /// the content of the tree node
    pub content : T,
    pub successors : Vec<Box<TreeNode<T>>>
}

impl<T: Clone + Serialization> Serialization for TreeNode<T> {

    /// Generates the JSON object for serialization.
    fn build_value(& self) -> Value {
        let mut m : Map<String, Value> = Map::new();
        let content_val = self.content.build_value();
        let mut succ_val_vec : Vec<Value> = Vec::new();
        for succ in self.successors.as_slice() {
            succ_val_vec.push(succ.build_value());
        }
        let succ_val = Value::Array(succ_val_vec);

        m.insert(String::from("content"), content_val);
        m.insert(String::from("successors"), succ_val);

        Value::Object(m)
    }

    /// Generates a deserialized instance from the specified JSON object.
    fn build_instance(v: Value) -> Result<TreeNode<T>, SerializationError> {

        let mut m = match v {
            Value::Object(map) => map,
            _ => return Err(SerializationError::ValueTypeError(String::from("Object")))
        };

        let content_val = match m.remove("content") {
            Some(v) => try!(T::build_instance(v)),
            None => return Err(SerializationError::MissingValueError(String::from("TreeNode: content")))
        };

        let succ_val = match m.remove("successors") {
            Some(v) => v,
            None => return Err(SerializationError::MissingValueError(String::from("TreeNode: successors")))
        };

        let mut succ_val_vec : Vec<Box<TreeNode<T>>> = Vec::new();
        match succ_val {
            Value::Array(vec) => {
                for elem in vec {
                    succ_val_vec.push(Box::new(try!(TreeNode::build_instance(elem))));
                }
            },

            _ => return Err(SerializationError::ValueTypeError(String::from("Array")))
        }

        Ok(TreeNode {content: content_val, successors: succ_val_vec})
    }
}

impl<'a, T: Clone + Serialization> TreeNode<T> {

    /// Creates a new instance of type TreeNode<T>
    pub fn new(c : T) -> TreeNode<T> {
        TreeNode {content: c, successors: Vec::new()}
    }
}

impl<'a, T: fmt::Display + Clone + Serialization> fmt::Display for TreeNode<T> {

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