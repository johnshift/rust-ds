// Why Linked List is better than Arrays/HashMap?
//  - insert/remove have fewer allocations -> vs array
//  - elements are ordered -> vs hashmaps

// Note:
//  - you should always prefer: (read the documentation)
//  - standard library: see std::collections
//  - other crates: w/ specialized approach

// Use std::collections::LinkedList only when:
//  - You want a Vec or VecDeque of unknown size, and can’t tolerate amortization.
//  - You want to efficiently split and append lists.
//  - You are absolutely certain you really, truly, want a doubly linked list.
//  - NOTE: It is almost always better to use Vec or VecDeque
//          because array-based containers are generally faster, more memory efficient,
//          and make better use of CPU cache.

// For linked lists, we will use Vec only (std recommendeds to use Vec)

pub mod bad_linked_list;
pub mod better_linked_list;
pub mod ok_linked_list;

struct SinglyLinkedList<T> {
  elements: Vec<T>,
}

impl<T: Sized> SinglyLinkedList<T> {
  fn new(value: T) -> Self {
    SinglyLinkedList {
      elements: vec![value],
    }
  }

  fn len(&self) -> usize {
    self.elements.len()
  }

  // returns first element from Vec
  // we let rust internals do the work
  // instead of manually handling state
  fn head(&self) -> Option<&T> {
    match self.len() {
      0 => None,
      _ => Some(&self.elements[0]),
    }
  }

  // returns last element from Vec
  // we let rust internals do the work
  // instead of manually handling state
  fn tail(&self) -> Option<&T> {
    let length = self.len();
    match length {
      0 => None,
      _ => Some(&self.elements[length - 1]),
    }
  }

  // append sets the value as last element
  fn append(&mut self, value: T) {
    self.elements.push(value);
  }

  // prepend sets the value as the first element
  fn prepend(&mut self, value: T) {
    let mut new_vec = vec![value];
    new_vec.append(&mut self.elements);
    self.elements = new_vec;
  }

  // insert appends the element in index i
  fn insert(&mut self, index: usize, value: T) {
    self.elements.insert(index, value)
  }

  // removes element at index i
  fn remove(&mut self, index: usize) {
    self.elements.remove(index);
  }

  // reverses elements
  fn reverse(&mut self) {
    self.elements.reverse();
  }
}

#[cfg(test)]
mod tests {
  use crate::SinglyLinkedList;

  #[test]
  fn singly_linked_list_reverse() {
    let mut singly_list = SinglyLinkedList::new(10);
    singly_list.append(16);
    singly_list.append(5);

    singly_list.reverse();
    assert_eq!(singly_list.elements, vec![5, 16, 10]);
  }

  #[test]
  #[should_panic(expected = "removal index (is 99) should be < len (is 2)")]
  fn singly_linked_list_remove() {
    let mut singly_list = SinglyLinkedList::new(10);
    singly_list.append(16);
    singly_list.append(5);

    singly_list.remove(1);
    assert_eq!(singly_list.elements, vec![10, 5]);

    singly_list.remove(99);
  }

  #[test]
  #[should_panic(expected = "insertion index (is 99) should be <= len (is 5)")]
  fn singly_linked_list_insert() {
    let mut singly_list = SinglyLinkedList::new(10);
    singly_list.append(16);
    singly_list.append(5);

    singly_list.insert(0, 99);
    assert_eq!(singly_list.elements, vec![99, 10, 16, 5]);

    singly_list.insert(2, 69);
    assert_eq!(singly_list.elements, vec![99, 10, 69, 16, 5]);

    singly_list.insert(99, 55555);
  }

  #[test]
  fn singly_linked_list_basics() {
    let mut singly_list = SinglyLinkedList::new(10);
    singly_list.append(16);
    singly_list.append(5);

    assert_eq!(singly_list.elements, vec![10, 16, 5]);
    assert_eq!(singly_list.len(), 3);
    assert_eq!(singly_list.head(), Some(&10));
    assert_eq!(singly_list.tail(), Some(&5));

    singly_list.prepend(99);
    assert_eq!(singly_list.elements, vec![99, 10, 16, 5]);
  }
}
