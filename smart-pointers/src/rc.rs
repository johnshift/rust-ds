// Notes:
//  - Rc is NOT thread safe
//  - allow multiple &T without giving &mut T
//  - useful if you want multiple &T and not think about whether pointer is 'alive'
//  - if you want to mutate data within, use RefCell
//  - Rc::clone DOES NOT DEEP COPY, it only increments the reference count

// Implement Rc in ConsList
//  - create list a that contains 5 and then 10.
//  - make two more lists: b that starts with 3 and c that starts with 4.
//  - both b and c lists will then continue on to the first a list containing 5 and 10.
//  - both lists will share the first list containing 5 and 10.
// a -> 5 -> 10 -> Nil
// b -> 3 -> a
// c -> 4 -> a

use std::rc::Rc;

enum ConsList {
  // Cons(i32, Box<ConsList>), // doesn't work
  Cons(i32, Rc<ConsList>),
  Nil,
}

#[cfg(test)]
mod tests {

  use super::ConsList::{Cons, Nil};
  use std::rc::Rc;

  #[test]
  fn rc_demo() {
    // The following lines won't compile:
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a)); // Error: a is moved to b

    // working code:
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
  }
}
