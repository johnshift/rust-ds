// Given an array, return the first repeating element
// [2,5,1,2,3,5,1,2,4] -> 2
// [2,1,1,2,3,5,1,2,4] -> 1
// [2,3,4,5] -> None

// Results:
//  - on very large arrays, hashmaps are way much faster than loops
//  - on small arrays, vanilla for loops are faster

pub fn e1_naive(arr: Vec<u32>) -> Option<u32> {
  for (i_index, i) in arr.iter().enumerate() {
    'inner: for (j_index, j) in arr.iter().enumerate() {
      if i_index <= j_index {
        continue 'inner;
      }
      if i == j {
        return Some(*i);
      }
    }
  }

  None
}

pub fn e1_manual_array(arr: Vec<u32>) -> Option<u32> {
  let mut buf: Vec<u32> = Vec::new();

  for i in &arr {
    for j in &buf {
      if i == j {
        return Some(*j);
      }
    }

    buf.push(*i);
  }

  None
}

use std::collections::HashMap;
pub fn e1_hashmap1(arr: Vec<u32>) -> Option<u32> {
  let mut hm = HashMap::with_capacity(arr.len());

  for i in &arr {
    if hm.contains_key(i) {
      return Some(*i);
    } else {
      hm.insert(*i, true);
    }
  }

  None
}

pub fn e1_hashmap2(arr: Vec<u32>) -> Option<u32> {
  let mut hm = HashMap::with_capacity(arr.len());

  for i in &arr {
    if let Some(_) = hm.get(i) {
      return Some(*i);
    }
    hm.insert(*i, true);
  }

  None
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_e1_naive() {
    // random
    let arr = vec![2, 5, 1, 2, 3, 5, 1, 2, 4];
    assert_eq!(e1_naive(arr), Some(2));

    // in succession
    let arr = vec![2, 1, 1, 2, 3, 5, 1, 2, 4];
    assert_eq!(e1_naive(arr), Some(1));

    // non-repeating
    let arr = vec![2, 3, 4, 5];
    assert_eq!(e1_naive(arr), None);
  }

  #[test]
  fn test_e1_manual_array() {
    // random
    let arr = vec![2, 5, 1, 2, 3, 5, 1, 2, 4];
    assert_eq!(e1_manual_array(arr), Some(2));

    // in succession
    let arr = vec![2, 1, 1, 2, 3, 5, 1, 2, 4];
    assert_eq!(e1_manual_array(arr), Some(1));

    // non-repeating
    let arr = vec![2, 3, 4, 5];
    assert_eq!(e1_manual_array(arr), None);
  }

  #[test]
  fn test_e1_hashmap1() {
    // random
    let arr = vec![2, 5, 1, 2, 3, 5, 1, 2, 4];
    assert_eq!(e1_hashmap1(arr), Some(2));
    // in succession
    let arr = vec![2, 1, 1, 2, 3, 5, 1, 2, 4];
    assert_eq!(e1_hashmap1(arr), Some(1));
    // non-repeating
    let arr = vec![2, 3, 4, 5];
    assert_eq!(e1_hashmap1(arr), None);
  }

  #[test]
  fn test_e1_hashmap2() {
    // random
    let arr = vec![2, 5, 1, 2, 3, 5, 1, 2, 4];
    assert_eq!(e1_hashmap2(arr), Some(2));
    // in succession
    let arr = vec![2, 1, 1, 2, 3, 5, 1, 2, 4];
    assert_eq!(e1_hashmap2(arr), Some(1));
    // non-repeating
    let arr = vec![2, 3, 4, 5];
    assert_eq!(e1_hashmap2(arr), None);
  }
}
