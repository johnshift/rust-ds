// demonstrate how zip iter function works
// generic type should implement Display trait to enable println
pub fn demo_zip<T>(arr1: Vec<T>, arr2: Vec<T>)
where
  T: std::fmt::Display,
{
  for (i, j) in arr1.iter().zip(arr2.iter()) {
    println!("i = {}, j = {}", i, j);
  }
}

#[cfg(test)]
mod tests {

  use super::demo_zip;

  #[test]
  fn zip_test() {
    // run with -- --nocapture to show printed lines
    let arr1 = vec![1, 2, 3, 4, 5];
    let arr2 = vec![6, 7, 8, 9, 11];

    demo_zip(arr1, arr2);
  }
}
