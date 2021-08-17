use std::ops::Deref;

struct Custom<T>(T);

impl<T> Custom<T> {
  fn new(x: T) -> Custom<T> {
    Custom(x)
  }
}

// Implement Deref trait for custom type
impl<T> Deref for Custom<T> {
  type Target = T;

  fn deref(&self) -> &T {
    &self.0 // we return the only element in our tuple struct
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn deref_test() {
    let x = 5;
    let y = Custom::new(5);

    assert_eq!(5, x); // true
    assert_eq!(5, *y); // error: cannot be dereferenced (if Deref trait is not implemented)
  }
}
