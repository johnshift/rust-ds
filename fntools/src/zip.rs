// demonstrate how zip iter function works
// generic type should implement Display trait to enable println
pub fn zip_demo<T>(arr1: Vec<T>, arr2: Vec<T>)
where
  T: std::fmt::Display,
{
  for (i, j) in arr1.iter().zip(arr2.iter()) {
    println!("i = {}, j = {}", i, j);
  }
}

// given two arrays, return an array
// whose members is composed of alternating elements
// [1,2,3] + [4,5,6] -> [1,4,2,5,3,6]
// ['a', 'b', 'c'] + ['e', 'f', 'g'] -> ['a', 'e', 'b', 'f', 'c', 'g']
// ["one", "two"] + ["three", "four"] -> ["one", "three", "two", "four"]
pub fn zip_merge<T>(arr1: Vec<T>, arr2: Vec<T>) -> Vec<T>
where
  T: Copy + Clone,
{
  let mut res: Vec<T> = Vec::new();

  for (i, j) in arr1.iter().zip(arr2.iter()) {
    res.push(*i);
    res.push(*j);
  }

  res
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn zip_merge_test() {
    let a1 = vec![1, 2, 3];
    let a2 = vec![4, 5, 6];
    assert_eq!(zip_merge(a1, a2), vec![1, 4, 2, 5, 3, 6]);

    let a1 = vec!['a', 'b', 'c'];
    let a2 = vec!['e', 'f', 'g'];
    assert_eq!(zip_merge(a1, a2), vec!['a', 'e', 'b', 'f', 'c', 'g']);

    let a1 = vec!["one", "two"];
    let a2 = vec!["three", "four"];
    assert_eq!(zip_merge(a1, a2), vec!["one", "three", "two", "four"]);
  }

  #[test]
  fn zip_demo_print() {
    // run with -- --nocapture to show printed lines
    let arr1 = vec![1, 2, 3, 4, 5];
    let arr2 = vec![6, 7, 8, 9, 11];

    zip_demo(arr1, arr2);
  }
}
