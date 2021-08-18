// Exercise 1:
//  - given two arrays, merge both arrays into one that is sorted
fn array_exercise1<T: Ord>(a1: Vec<T>, a2: Vec<T>) -> Vec<T> {
  let mut result: Vec<T> = a1.into_iter().chain(a2.into_iter()).collect();
  result.sort();
  result
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn array_exercise1_test() {
    // unequal number of elements
    let a1 = vec![0, 3, 4, 31];
    let a2 = vec![4, 6, 30];
    let result = array_exercise1(a1, a2);
    assert_eq!(result, [0, 3, 4, 4, 6, 30, 31]);

    // one empty
    let a1 = vec![0, 3, 4, 31];
    let a2 = vec![];
    let result = array_exercise1(a1, a2);
    assert_eq!(result, [0, 3, 4, 31])
  }
}
