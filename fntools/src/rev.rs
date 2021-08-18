// given some generic vector type T
// return it reversed
fn reverse<T>(arr: Vec<T>) -> Vec<T> {
  arr.into_iter().rev().collect()
}

#[cfg(test)]
mod tests {
  use super::reverse;

  #[test]
  fn reverse_test() {
    let arr = vec![0, 1, 2, 3, 4, 5];
    let result = reverse(arr);
    assert_eq!(result, vec![5, 4, 3, 2, 1, 0]);
  }

  #[test]
  fn reverse_demo() {
    for i in (0..6).rev() {
      println!("i = {}", i);
    }
  }
}
