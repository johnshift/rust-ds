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

  fn has(&self, value: T) -> bool {
    let mut current = self;

    while let Some(ref node) = current.0 {
      match node.value.cmp(&value) {
        Ordering::Equal => {
          return true;
        }
        Ordering::Greater => current = &node.right,
        Ordering::Less => current = &node.left,
      }
    }

    false
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn bst_insert() {
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

  #[test]
  fn bst_has() {
    let mut tree = Tree::new(9);
    tree.insert(4);
    tree.insert(20);
    tree.insert(1);
    tree.insert(6);
    tree.insert(15);
    tree.insert(170);

    assert!(tree.has(9));
    assert!(tree.has(4));
    assert!(tree.has(20));
    assert!(tree.has(1));
    assert!(tree.has(6));
    assert!(tree.has(15));
    assert!(tree.has(170));

    assert!(!tree.has(99));
    assert!(!tree.has(0));
    assert!(!tree.has(44));
  }
}
