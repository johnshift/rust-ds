// Note: only SINGLY LINKED LIST
//  - we will use Box here

// 1 -> 2 -> 3 -> None

// Link can either be empty or a pointer to a Node in heap
pub enum Link {
  // final size: 8 bytes
  Empty,           // 0 bytes
  Next(Box<Node>), // Box is a pointer. pointer is always as large as usize -> 8 bytes
}

// Node has an associated 'value',
// and 'next' which can either be empty or a pointer to next node
pub struct Node {
  // final size: 16 bytes -> largest=8bytes multipled by two fields
  value: i32, // 4 bytes
  next: Link, // 8 bytes
}

pub struct LinkedList {
  // final size: 8 bytes
  head: Link, // Link -> 8 bytes
}

impl LinkedList {
  fn new() -> Self {
    LinkedList { head: Link::Empty }
  }

  // prepend adds new node to the head
  fn prepend(&mut self, value: i32) {
    let new_node = Node {
      value,
      /*
        We're trying to move the self.head field out to next, but Rust doesn't want us doing that. This would leave self only partially initialized when we end the borrow and "give it back" to its rightful owner.

        std::mem::replace returns the previous value of self.head,
        then change it to Link::Empty

        if self.head implements Default trait, you can directly use self.head.take(),
        and self.head would be equal to the default value e.g. None for Option<T>
      */
      // put old self.head into the new_node.next
      next: std::mem::replace(&mut self.head, Link::Empty),
    };

    self.head = Link::Next(Box::new(new_node));
  }

  fn pop(&mut self) -> Option<i32> {
    /*
      we want to obtain the old value and replace it with Link::Empty
      because we might replace the head inside the branch
    */
    match std::mem::replace(&mut self.head, Link::Empty) {
      Link::Empty => None,
      Link::Next(node) => {
        self.head = node.next;
        Some(node.value)
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::{Link, LinkedList, Node};
  use std::mem::size_of_val;

  #[test]
  pub fn test_bad_linked_list_pop_push() {
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
  pub fn test_bad_linked_list_sizes() {
    let empty_link = Link::Empty;
    assert_eq!(size_of_val(&empty_link), 8);

    let last_node = Node {
      value: 999,
      next: empty_link,
    };
    assert_eq!(size_of_val(&last_node.value), 4);
    assert_eq!(size_of_val(&last_node.next), 8);
    assert_eq!(size_of_val(&last_node), 16);

    let last_node_link = Link::Next(Box::new(last_node));
    assert_eq!(size_of_val(&last_node_link), 8);

    let first_node = Node {
      value: 111,
      next: last_node_link,
    };
    assert_eq!(size_of_val(&first_node), 16);
    let first_node_link = Link::Next(Box::new(first_node));
    assert_eq!(size_of_val(&first_node_link), 8);

    let my_linked_list = LinkedList {
      head: first_node_link,
    };
    assert_eq!(size_of_val(&my_linked_list), 8);
  }
}
