// Note: rust standard library recommends std::collections::VecDequeue in implementing stacks

use std::collections::VecDeque;

struct Queue<T> {
  elements: VecDeque<T>,
}

impl<T> Queue<T> {
  fn new() -> Self {
    Self {
      elements: VecDeque::new(),
    }
  }

  // peek returns pointer to first element
  fn peek(&self) -> Option<&T> {
    self.elements.front()
  }

  // enqueue adds to the queue (as last)
  fn enqueue(&mut self, value: T) {
    // push_back = appends
    // push_front = prepend --> (sal.ot)
    self.elements.push_back(value)
  }
  // dequeue removes from the queue (from first)
  fn dequeue(&mut self) -> Option<T> {
    self.elements.pop_front()
  }
}

#[cfg(test)]
mod tests {

  use super::Queue;

  #[test]
  fn dequeue_basics() {
    let mut q = Queue::new();

    assert_eq!(q.peek(), None);
    q.enqueue(0);
    q.enqueue(1);
    q.enqueue(2);

    assert_eq!(q.peek(), Some(&0));
    assert_eq!(q.dequeue(), Some(0));
    assert_eq!(q.peek(), Some(&1));
    assert_eq!(q.dequeue(), Some(1));
    assert_eq!(q.peek(), Some(&2));
    assert_eq!(q.dequeue(), Some(2));
    assert_eq!(q.dequeue(), None);
  }
}
