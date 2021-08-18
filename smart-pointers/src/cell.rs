use std::cell::Cell;

// Notes:
//  - it is NOT THREAD SAFE
//  - gives multiple &T and still be able to mutate it
//  - avoid getting references of the value inside the cell
//  - only use the methods provided by Cell

//	enables mutability inside an immutable struct
struct SomeStruct {
  regular_field: u8,
  special_field: Cell<u8>,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn cell_mutability_in_immutable_struct() {
    let my_struct = SomeStruct {
      regular_field: 0,
      special_field: Cell::new(1),
    };

    my_struct.special_field.set(99);

    assert_eq!(my_struct.special_field.get(), 99);

    // As you can see, even do my_struct is immutable,
    // we are able to change the value of special_field
    // because it is a Cell
  }
}
