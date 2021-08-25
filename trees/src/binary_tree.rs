use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
  value: T,
  left: Link<T>,
  right: Link<T>,
}

impl<T> Node<T> {
  fn new(value: T) -> Self {
    Self {
      value,
      left: None,
      right: None,
    }
  }

  fn assign_left(&mut self, link: Link<T>) {
    self.left = link;
  }

  fn assign_right(&mut self, link: Link<T>) {
    self.right = link;
  }
}

struct BinarySearchTree<T> {
  root: Link<T>,
}

impl<T: PartialOrd> BinarySearchTree<T> {
  fn new() -> Self {
    Self { root: None }
  }

  fn insert(&mut self, value: T) {
    // first insert
    if self.root.is_none() {
      self.root = Some(Rc::new(RefCell::new(Node::new(value))));
      return;
    }

    let mut current_node = self.root.clone();
    'outer: loop {
      if let Some(ref node) = current_node {
        if &value < &node.borrow().value {
          // left

          match &node.borrow().left {
            None => {
              (*node.as_ref().borrow_mut())
                .left
                .replace(Rc::new(RefCell::new(Node {
                  value,
                  left: None,
                  right: None,
                })));
              break 'outer;
            }
            Some(n) => {
              // let ambot = (*n.as_ref().borrow_mut()).left;
              // current_node.replace((*n.as_ref().borrow_mut()).left.as_ref().unwrap());
              current_node = (*n.as_ref().borrow_mut()).left;
            }
          };
          // if node.borrow().left.is_none() {
          //   (*node.as_ref().borrow_mut())
          //     .left
          //     .replace(Rc::new(RefCell::new(Node {
          //       value,
          //       left: None,
          //       right: None,
          //     })));

          //   break 'outer;
          // } else {
          //   current_node = (*node.as_ref().borrow_mut()).left;
          // }
        }
      }
    }
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
  fn bst_basics() {
    //         9
    //    4        20
    //  1   6   15    170

    let mut tree = BinarySearchTree::new();
    // todo: check root is None
    tree.insert(9);
    // todo: check root is 9
    // tree.insert(4);
    // tree.insert(6);
    // tree.insert(20);
    // tree.insert(170);
    // tree.insert(15);
    // tree.insert(1);
  }
}
