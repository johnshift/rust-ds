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

#[cfg(test)]
mod tests {
  use super::*;
  use std::mem::size_of_val;

  #[test]
  pub fn test_linked_list_bad_sizes() {
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
