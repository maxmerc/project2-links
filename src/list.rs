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
        ListNode::Nil
    }
    /// Inserts a new list node with value `value` after `self` and returns a reference to the new
    /// node
    pub fn insert(&mut self, value: T) -> &mut Self {
        let mut current = mem::take(self); // Take ownership of the current list node
        if let ListNode::Nil = current {
            *self = ListNode::Cons(value, Box::new(ListNode::Nil));
        } else {
            let mut node = &mut current;
            while let ListNode::Cons(_, ref mut next) = node {
                node = next;
            }
            *node = ListNode::Cons(value, Box::new(ListNode::Nil));
            *self = current;
        }
        self
    }  
  
    /// Reverses the list in place.
    pub fn reverse(&mut self) {
        let mut prev = ListNode::Nil;
        let mut curr = mem::take(self);
        while let ListNode::Cons(value, next) = curr {
            curr = *next;
            prev = ListNode::Cons(value, Box::new(prev));
        }
        *self = prev;
    }
}

// Implement `Default` for `ListNode<T>`
impl<T> Default for ListNode<T> {
    fn default() -> Self {
        ListNode::Nil
    }
}

impl<T: PartialEq> PartialEq for ListNode<T> {
  fn eq(&self, other: &Self) -> bool {
      match (self, other) {
          (ListNode::Nil, ListNode::Nil) => true,
          (ListNode::Cons(val1, next1), ListNode::Cons(val2, next2)) => val1 == val2 && next1 == next2,
          _ => false,
      }
  }
}

impl<T: Eq> Eq for ListNode<T> {}

impl<T: Display> Display for ListNode<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
          ListNode::Nil => write!(f, "Nil"),
          ListNode::Cons(value, next) => write!(f, "{} -> {}", value, next),
      }
  }
}


impl<T> From<Vec<T>> for ListNode<T> {
  fn from(vec: Vec<T>) -> Self {
      let mut list = ListNode::Nil;
      for value in vec {
          list.insert(value);
      }
      list
  }
}


impl<T> From<ListNode<T>> for Vec<T> {
  fn from(mut list: ListNode<T>) -> Self {
      let mut vec = Vec::new();
      while let ListNode::Cons(value, next) = list {
          vec.push(value);
          list = *next;
      }
      vec
  }
}

