use std::rc::Rc;

struct Node<T> {
  value: T,
  next: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

pub struct List<T> {
  head: Link<T>,
}

impl<T> List<T> {
  fn new() -> Self {
    List { head: None }
  }

  // prepend adds the new value as the new head
  // it takes a value, then returns the list
  fn prepend(&self, value: T) -> List<T> {
    List {
      head: Some(Rc::new(Node {
        value,
        next: self.head.clone(), // clone -> Rc::clone()
      })),
    }
  }

  // tail returns the list without the head
  fn tail(&self) -> List<T> {
    List {
      head: self.head.as_ref().and_then(|node| node.next.clone()),
    }
  }

  // head returns reference to the first element
  fn head(&self) -> Option<&T> {
    self.head.as_ref().map(|node| &node.value)
  }
}

pub struct Iter<'a, T> {
  next: Option<&'a Node<T>>,
}

impl<T> List<T> {
  pub fn iter(&self) -> Iter<'_, T> {
    Iter {
      next: self.head.as_deref(),
    }
  }
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

#[cfg(test)]
mod test {
  use super::List;

  #[test]
  fn better_linked_list_iter() {
    let list = List::new().prepend(1).prepend(2).prepend(3);

    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));
  }

  #[test]
  fn better_linked_list_demo() {
    let list = List::new();
    assert_eq!(list.head(), None);

    let list = list.prepend(1).prepend(2).prepend(3);
    assert_eq!(list.head(), Some(&3));

    let list = list.tail();
    assert_eq!(list.head(), Some(&2));

    let list = list.tail();
    assert_eq!(list.head(), Some(&1));

    let list = list.tail();
    assert_eq!(list.head(), None);

    // Make sure empty tail works
    let list = list.tail();
    assert_eq!(list.head(), None);
  }
}
