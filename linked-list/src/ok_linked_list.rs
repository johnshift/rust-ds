// final size: 8bytes -> from Box=8bytes
type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
  // final size: 8bytes * 2 fields ===> 16bytes
  value: T,      // ? -> but 4 since we only use i32
  next: Link<T>, // 8 bytes
}

pub struct LinkedList<T> {
  // final size: 8bytes
  head: Link<T>, // 8 bytes
}

impl<T> LinkedList<T> {
  fn new() -> Self {
    Self { head: None }
  }

  fn prepend(&mut self, value: T) {
    let new_node = Node {
      value,
      // self.head.take() replace self.head with default value None
      next: self.head.take(),
    };
    self.head = Some(Box::new(new_node));
  }

  fn pop(&mut self) -> Option<T> {
    /*
      Option.map takes a function that executes on Some(_)
    */
    self.head.take().map(|node| {
      self.head = node.next;
      node.value
    })
  }

  // return a reference if it exist
  fn peek(&self) -> Option<&T> {
    self.head.as_ref().map(|node| &node.value)
  }

  // return a mutable reference if it exist
  fn peek_mut(&mut self) -> Option<&mut T> {
    self.head.as_mut().map(|node| &mut node.value)
  }
}

// ================ into_iter() =================

// Tuple structs are an alternative form of struct,
// useful for trivial wrappers around other types.
pub struct IntoIter<T>(LinkedList<T>);
impl<T> Iterator for IntoIter<T> {
  type Item = T;
  fn next(&mut self) -> Option<Self::Item> {
    // access fields of a tuple struct numerically
    self.0.pop()
  }
}

impl<T> LinkedList<T> {
  pub fn into_iter(self) -> IntoIter<T> {
    IntoIter(self)
  }
}

// ==============================================

// ================== iter() ====================
pub struct Iter<'a, T> {
  next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
  type Item = &'a T;

  fn next(&mut self) -> Option<Self::Item> {
    self.next.map(|node| {
      self.next = node.next.as_deref();
      &node.value
    })
  }
}

impl<T> LinkedList<T> {
  pub fn iter(&self) -> Iter<T> {
    Iter {
      // next: self.head.as_ref().map(|node| node),
      next: self.head.as_deref(),
    }
  }
}
// ==============================================

// ================ iter_mut() ==================
pub struct IterMut<'a, T> {
  next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
  type Item = &'a mut T;

  fn next(&mut self) -> Option<Self::Item> {
    self.next.take().map(|node| {
      self.next = node.next.as_deref_mut();
      &mut node.value
    })
  }
}

impl<T> LinkedList<T> {
  pub fn iter_mut(&mut self) -> IterMut<T> {
    IterMut {
      // next: self.head.as_ref().map(|node| node),
      next: self.head.as_deref_mut(),
    }
  }
}
// ==============================================

#[cfg(test)]
mod tests {

  use super::*;
  use std::mem::size_of_val;

  #[test]
  fn test_ok_linked_list_iter_mut() {
    let mut list = LinkedList::new();
    list.prepend(1);
    list.prepend(2);
    list.prepend(3);

    let mut iter_mut = list.iter_mut();
    assert_eq!(iter_mut.next(), Some(&mut 3));
    assert_eq!(iter_mut.next(), Some(&mut 2));
    assert_eq!(iter_mut.next(), Some(&mut 1));
  }

  #[test]
  fn test_ok_linked_list_iter() {
    let mut list = LinkedList::new();
    list.prepend(1);
    list.prepend(2);
    list.prepend(3);

    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));
  }

  #[test]
  pub fn test_ok_linked_list_into_iter() {
    let mut list = LinkedList::new();
    list.prepend(1);
    list.prepend(2);
    list.prepend(3);

    let mut iter = list.into_iter();
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), None);
  }

  #[test]
  pub fn test_ok_linked_list_pop_push() {
    let mut list = LinkedList::new();

    // Check empty list behaves right
    assert_eq!(list.pop(), None);

    // Populate list
    list.prepend(1);
    list.prepend(2);
    list.prepend(3);

    // Check normal removal
    assert_eq!(list.pop(), Some(3));
    assert_eq!(list.pop(), Some(2));

    // Add some more just to make sure nothing's corrupted
    list.prepend(4);
    list.prepend(5);

    // Check normal removal
    assert_eq!(list.pop(), Some(5));
    assert_eq!(list.pop(), Some(4));

    // Check exhaustion
    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), None);
  }

  #[test]
  pub fn test_ok_linked_list_peek() {
    let mut list = LinkedList::new();
    assert_eq!(list.peek(), None);
    assert_eq!(list.peek_mut(), None);
    list.prepend(1);
    list.prepend(2);
    list.prepend(3);

    assert_eq!(list.peek(), Some(&3));
    assert_eq!(list.peek_mut(), Some(&mut 3));
  }

  #[test]
  pub fn test_ok_linked_list_sizes() {
    let empty_link = None;
    assert_eq!(size_of_val(&empty_link), 8);

    let last_node = Node {
      value: 999,
      next: empty_link,
    };
    assert_eq!(size_of_val(&last_node.value), 4);
    assert_eq!(size_of_val(&last_node.next), 8);
    assert_eq!(size_of_val(&last_node), 16);

    let last_node_link = Some(Box::new(last_node));
    assert_eq!(size_of_val(&last_node_link), 8);

    let first_node = Node {
      value: 111,
      next: last_node_link,
    };
    assert_eq!(size_of_val(&first_node), 16);
    let first_node_link = Some(Box::new(first_node));
    assert_eq!(size_of_val(&first_node_link), 8);

    let my_linked_list = LinkedList {
      head: first_node_link,
    };
    assert_eq!(size_of_val(&my_linked_list), 8);
  }
}
