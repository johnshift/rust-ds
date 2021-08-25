use std::rc::Rc;

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
  value: T,
  left: Link<T>,
  right: Link<T>,
}

struct BinarySearchTree<T> {
  root: Link<T>,
}

impl<T: PartialOrd> BinarySearchTree<T> {
  fn new() -> Self {
    Self { root: None }
  }

  fn insert(&mut self, value: T) {
    // if empty tree
    if self.root.is_none() {
      self.root = Some(Rc::new(Node {
        value,
        left: None,
        right: None,
      }));
      return;
    }

    let mut current_node = self.root.clone();
  }

  // simply returns the node or None
  fn lookup(&mut self) -> Link<T> {
    unimplemented!()
  }
}

#[cfg(test)]
mod test {

  use super::*;

  #[test]
  fn btnode_basics() {
    //         9
    //    4        20
    //  1   6   15    170

    let mut tree = BinarySearchTree::new();
    // todo: check root is None
    tree.insert(9);
    // todo: check root is 9
    tree.insert(4);
    tree.insert(6);
    tree.insert(20);
    tree.insert(170);
    tree.insert(15);
    tree.insert(1);
  }
}
