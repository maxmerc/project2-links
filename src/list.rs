#[allow(unused_imports)]
use std::{fmt::Display, mem};

#[derive(Debug)]
pub enum ListNode<T> {
    Nil,
    Cons(T, Box<ListNode<T>>),
}

impl<T> ListNode<T> {
  // Use the implementation of this method to guide your implementation of
  // `insert` and `reverse`
  /// Deletes a node from the list
  pub fn delete(&mut self) {
    // Temporarily replaces the current node with default value (Nil).
    // In exchange, we get to take ownership of the current node instead of just
    // having it by mutable reference.
    let as_owned: ListNode<T> = mem::take(self);
    match as_owned {
      ListNode::Nil => {}
      ListNode::Cons(_, next) => {
        // Write the next node to the current node
        *self = *next;
      }
    }
  }
}

// Required methods for `ListNode<T>`
impl<T> ListNode<T> {
    /// Creates a new empty list
    pub fn new() -> Self {
        todo!()
    }
    /// Inserts a new list node with value `value` after `self` and returns a reference to the new
    /// node
    pub fn insert(&mut self, value: T) -> &mut Self {
        todo!()
    }
    /// Reverses the list in place.
    pub fn reverse(&mut self) {
        todo!()
    }
}

// Implement `Default` for `ListNode<T>`
impl<T> Default for ListNode<T> {
    fn default() -> Self {
        todo!()
    }
}

// Implement `PartialEq` for `ListNode<T>`
// TODO:

// Implement `Eq` for `ListNode<T>`
// TODO:

// Implement `Display` for `ListNode<T>`
// TODO:

// Implement `From<Vec<T>>` for `ListNode<T>`
// TODO:

// Implement `From<ListNode<T>>` for `Vec<T>`
// TODO:
