use std::cmp::{Ord, Ordering};

#[derive(Debug)]
struct Node<T: Ord> {
  value: T,
  left: Tree<T>,
  right: Tree<T>,
}

impl<T: Ord> Node<T> {
  fn new(value: T) -> Self {
    Self {
      value,
      left: Tree(None),
      right: Tree(None),
    }
  }
}

#[derive(Debug)]
struct Tree<T: Ord>(Option<Box<Node<T>>>);

impl<T: Ord> Tree<T> {
  fn new(value: T) -> Self {
    Self(Some(Box::new(Node::new(value))))
  }

  fn insert(&mut self, value: T) {
    let mut current = self;

    while let Some(ref mut node) = current.0 {
      match node.value.cmp(&value) {
        Ordering::Equal => current = &mut node.right,
        Ordering::Greater => current = &mut node.right,
        Ordering::Less => current = &mut node.left,
      }
    }

    current.0 = Some(Box::new(Node::new(value)));
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn bst_basics() {
    //         9
    //    4        20
    //  1   6   15    170

    let mut tree = Tree::new(9);
    tree.insert(4);
    tree.insert(20);
    tree.insert(1);
    tree.insert(6);
    tree.insert(15);
    tree.insert(170);

    println!("{:#?}", tree);
  }
}
