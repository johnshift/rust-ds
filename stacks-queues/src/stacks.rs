// Note: rust standard library recommends std::vec::Vec in implementing stacks

struct Stack<T> {
  elements: Vec<T>,
}

impl<T> Stack<T> {
  fn new() -> Self {
    Stack {
      elements: Vec::new(),
    }
  }

  // see the latest element
  fn peek(&self) -> Option<&T> {
    let length = self.elements.len();

    match self.elements.len() {
      0 => None,
      _ => Some(&self.elements[length - 1]),
    }
  }

  // add new element
  fn push(&mut self, value: T) {
    self.elements.push(value)
  }

  // remove latest element
  fn pop(&mut self) -> Option<T> {
    let length = self.elements.len();
    match self.elements.len() {
      0 => None,
      _ => self.elements.pop(),
    }
  }
}

#[cfg(test)]
mod tests {

  use super::Stack;

  #[test]
  fn stack_basics() {
    let mut stack = Stack::new();
    assert_eq!(stack.peek(), None);
    stack.push(0);
    assert_eq!(stack.peek(), Some(&0));
    stack.push(1);
    assert_eq!(stack.peek(), Some(&1));
    stack.push(2);
    assert_eq!(stack.peek(), Some(&2));
    assert_eq!(stack.pop(), Some(2));
    assert_eq!(stack.pop(), Some(1));
    assert_eq!(stack.pop(), Some(0));
    assert_eq!(stack.pop(), None);
  }
}
