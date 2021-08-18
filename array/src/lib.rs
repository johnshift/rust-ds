mod exercise;

// This is a manual implementation of array
// The only purpose of this is for learning
// As much as possible:
//  - present generic parameters
//  - present idiomatic rust approach

// MyArray contains data of any type T
struct MyArray<T> {
  length: usize,
  data: Vec<T>,
}

impl<T> MyArray<T> {
  fn new(data: Vec<T>) -> Self {
    MyArray {
      length: data.len(),
      data,
    }
  }

  fn get(&self, index: usize) -> Option<&T> {
    self.data.get(index)
  }

  // returns length of self
  fn push(&mut self, item: T) -> usize {
    self.data.push(item);
    self.length += 1;
    self.length
  }

  // deletes and returns the item
  fn pop(&mut self) -> Option<T> {
    self.length -= 1;
    self.data.pop()
  }

  fn delete(&mut self, index: usize) -> Option<T> {
    self.length -= 1;
    if index >= self.length {
      return None;
    }

    Some(self.data.remove(index))
  }
}

#[cfg(test)]
mod tests {
  use super::MyArray;

  #[test]
  fn array_constructor() {
    let my_data = vec![1, 2, 3, 4, 5];
    let my_array = MyArray::new(my_data);
    assert_eq!(my_array.length, 5);
    assert_eq!(my_array.data, vec![1, 2, 3, 4, 5]);
  }

  #[test]
  fn array_get() {
    let my_data = vec![1, 2, 3, 4, 5];
    let my_array = MyArray::new(my_data);
    assert_eq!(my_array.get(0), Some(&1));
    assert_eq!(my_array.get(99), None);
  }

  #[test]
  fn array_push() {
    let my_data = vec![1, 2, 3, 4, 5];
    let mut my_array = MyArray::new(my_data);
    let new_length = my_array.push(20);
    assert_eq!(new_length, 6);
  }

  #[test]
  fn array_pop() {
    let my_data = vec![1, 2, 3, 4, 5, 6];
    let mut my_array = MyArray::new(my_data);
    let popped = my_array.pop();
    assert_eq!(popped, Some(6));
    assert_eq!(my_array.data, vec![1, 2, 3, 4, 5]);
  }

  #[test]
  fn array_delete() {
    let my_data = vec![1, 2, 3, 4, 5, 6];
    let mut my_array = MyArray::new(my_data);
    let deleted = my_array.delete(0);
    assert_eq!(deleted, Some(1));
    let deleted_impossible = my_array.delete(999);
    assert_eq!(deleted_impossible, None);
  }
}
