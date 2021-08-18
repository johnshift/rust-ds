// note: fold accepts default value as first parameter

// function that returns sum of squared values
// using start and end (inclusive) values
fn fold_startend(start: i32, end: i32) -> i32 {
  let result = (start..=end).fold(0, |acc, x| acc + x * x);

  result
}

#[cfg(test)]
mod tests {
  use super::fold_startend;

  #[test]
  fn fold_test() {
    let start = 0;
    let end = 5;
    let result = fold_startend(start, end);
    assert_eq!(result, 55)
  }
}
