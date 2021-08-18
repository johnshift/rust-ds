// Notes:
//  - RefCell is NOT THREAD SAFE
//  - ENFORCED AT RUNTIME
//  - you'll get panic! instead of compile error
//  - useful only if your code follows borrowing rules but compiler is unable to comprehend

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum ConsList {
  // Cons(i32, Box<ConsList>), // doesn't work
  Cons(Rc<RefCell<i32>>, Rc<ConsList>),
  Nil,
}

#[cfg(test)]
mod tests {

  use super::ConsList::{Cons, Nil};
  use std::cell::RefCell;
  use std::rc::Rc;

  #[test]
  fn refcell_demo() {
    let five = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&five), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));

    println!("a before = {:?}", a);
    println!("b before = {:?}", b);
    println!("c before = {:?}", c);

    // while we have many other references to five, we can still mutate five
    *five.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    // As you can see, five becomes '15'
  }
}
