// Implement Drop trait

struct Custom {
  data: String,
}

impl Drop for Custom {
  fn drop(&mut self) {
    println!("dropping \"{}\"", self.data)
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn drop_demo() {
    let c = Custom {
      data: String::from("c_data"),
    };
    println!("c created.");
    drop(c); // manual calling drop

    let d = Custom {
      data: String::from("d_data"),
    };
    println!("d created.");
  }
}
