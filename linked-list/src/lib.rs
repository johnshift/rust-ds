// Why Linked List is better than Arrays/HashMap?
//  - insert/remove have fewer allocations -> vs array
//  - elements are ordered -> vs hashmaps

//  prepend:  O(1)
//  append:   O(1)
//  lookup:   O(n)
//  insert:   O(n)
//  delete:   O(n)

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node<T> {
  value: T,
  next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
  fn new_with_sptr(value: T) -> Rc<RefCell<Node<T>>> {
    Rc::new(RefCell::new(Node { value, next: None }))
  }
}

#[derive(Debug)]
struct LinkedList<T> {
  head: Rc<RefCell<Node<T>>>,
  tail: Rc<RefCell<Node<T>>>,
  length: u64,
}

impl<T> LinkedList<T> {
  fn new(value: T) -> Self {
    let init_node = Node::new_with_sptr(value);

    LinkedList {
      head: Rc::clone(&init_node),
      tail: Rc::clone(&init_node),
      length: 1,
    }
  }

  fn append(&mut self, value: T) {
    // create new node with the value
    let new_node = Node::new_with_sptr(value);

    // change next of current tail
    self.tail.borrow_mut().next = Some(Rc::clone(&new_node));

    // change current_tail to new_node
    self.tail = Rc::clone(&new_node);

    // increment length
    self.length += 1;
  }

  fn prepend(&mut self, value: T) {
    // create new node with the value
    let new_node = Node::new_with_sptr(value);

    let old_head_ptr = &self.head.clone();

    // change current_head to new_node
    self.head = Rc::clone(&new_node);

    // change next of current_head
    self.head.borrow_mut().next = Some(Rc::clone(old_head_ptr));

    // increment length
    self.length += 1;
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_linked_list_append() {
    // 10 --> 5 --> 16 -> None

    let mut my_linked_list = LinkedList::new(10);
    my_linked_list.append(5);
    my_linked_list.append(16);
    assert_eq!(my_linked_list.length, 3);
    assert_eq!(my_linked_list.head.borrow_mut().value, 10);
    assert_eq!(my_linked_list.tail.borrow_mut().value, 16);
    match my_linked_list.tail.borrow().next {
      Some(_) => panic!("Should return None!"),
      None => {}
    };
  }

  #[test]
  fn test_linked_list_prepend() {
    // 2 --> 16 --> None
    // 69 --> 2 --> 16 --> None

    let mut my_linked_list = LinkedList::new(2);
    my_linked_list.append(16);
    my_linked_list.prepend(69);
    assert_eq!(my_linked_list.length, 3);
    assert_eq!(my_linked_list.head.borrow_mut().value, 69);
    assert_eq!(my_linked_list.tail.borrow_mut().value, 16);
    match &my_linked_list.head.borrow().next {
      Some(sptr) => {
        assert_eq!(sptr.borrow().value, 2);
      }
      None => panic!("Should not return None!"),
    };
  }
}
