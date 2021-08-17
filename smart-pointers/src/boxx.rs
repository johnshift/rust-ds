// 'boxx' because 'box'is reserved.

// Box is used to store values on the heap.
//  - a type with unknown size at compile time
//  - transfer ownership of large data to avoid copy
//  - want to implement trait object rather than specific type

fn box_demo() {
    // 5 is stored in the heap
    // the pointer is stored in the stack
    let b: Box<u64> = Box::new(5);

    println!("b = {}", b);
}

// ------------ IMPLEMENTATION --------------
// Notice that Box is used in Box<List>
// since ConsList is recursive (size cannot be known at compile time)
// we use Box to store it on the heap instead
enum ConsList {
    Cons(i32, Box<ConsList>),
    Nil,
}

fn box_cons() {
    use ConsList::{Cons, Nil};

    // line below won't work
    // let list = Cons(1, Cons(2, Cons(3, Nil)));

    // using Box
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn box_demo_test() {
        box_demo();
    }
}
