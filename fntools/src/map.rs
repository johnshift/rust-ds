// given an Array of int
// return an array of squared values
fn map_squared(a: Vec<i32>) -> Vec<i32> {
  a.iter().map(|x| x * x).collect()
}

#[cfg(test)]
mod tests {
  use super::map_squared;

  #[test]
  fn map_squared_test() {
    let a = vec![1, 2, 3, 4, 5];
    let result = map_squared(a);
    assert_eq!(result, [1, 4, 9, 16, 25])
  }
}
