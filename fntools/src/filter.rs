// given two numbers as start and finish
// return all numbers divisible by x argument (except 0)
// note that end should be included
fn filter_demo(start: usize, end: usize, x: usize) -> Vec<usize> {
  let mut result = Vec::new();

  // note we need to do *n != 0 to exclude 0 in result
  for i in (start..end + 1).filter(|n| (n % x == 0 && *n != 0)) {
    result.push(i);
  }

  result
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn filter_demo_test() {
    let start = 0;
    let end = 20;
    let x = 5;

    let ans = filter_demo(start, end, x);
    assert_eq!(ans, [5, 10, 15, 20])
  }
}
