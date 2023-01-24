# Implementing a basic STM

Now im going to implement a basic STM


```rust,editable
struct STM {
  name: String
}

impl STM {
  fn handle(&self) {
    println!("[STM] name:{}", self.name);
  }
}
fn main() {
  let stm = STM{name: String::from("My STM")};
  stm.handle();
}

```

