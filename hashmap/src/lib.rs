// Rust HashMap notes:
//  - Copy is copied, others are owned
//  - using the same key will overwrite the value

// HashMap pros:
//  - O(1) lookups

// HashMap cons:
//  - collisions make O(n) lookups

pub mod exercise;

#[cfg(test)]
mod tests {

  use std::collections::HashMap;

  #[test]
  fn hashmap_demo() {
    let mut my_hashmap = HashMap::new();

    // add values
    my_hashmap.insert("key_1", "value_1");
    my_hashmap.insert("key_2", "value_2");
    my_hashmap.insert("key_3", "value_3");

    // how many kv does the hashmap have
    assert_eq!(my_hashmap.len(), 3);

    // get values
    if let Some(v) = my_hashmap.get("key_1") {
      assert_eq!(*v, "value_1")
    }
    if let Some(v) = my_hashmap.get("key_2") {
      assert_eq!(*v, "value_2")
    }
    if let Some(v) = my_hashmap.get("key_3") {
      assert_eq!(*v, "value_3")
    }

    // loop kv in hashmap
    // note: we borrow my_hashmap in for loop
    // if you want move, use my_hashmap instead
    for (k, v) in &my_hashmap {
      println!("k={}, v={}", k, v);
    }

    // check for existing keys
    assert_eq!(my_hashmap.contains_key("key_999"), false);

    // insert only if no value
    my_hashmap.entry("key_3").or_insert("value_999");
    my_hashmap.entry("key_4").or_insert("value_4");
    if let Some(v) = my_hashmap.get("key_3") {
      assert_eq!(*v, "value_3") // should have not overwritten
    }
    if let Some(v) = my_hashmap.get("key_4") {
      assert_eq!(*v, "value_4") // should have new value
    }
  }
}
