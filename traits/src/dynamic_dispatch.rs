trait Printable {
  fn format(&self) -> String;
}

impl Printable for i32 {
  fn format(&self) -> String {
    format!("i32: {}", *self)
  }
}

impl Printable for String {
  fn format(&self) -> String {
    format!("String: {}", *self)
  }
}

// Dynamic dispatch:
// Rust will decided which format function will be use at run-time
// Idea for using in an unknow type
// For instanch:: let test:[Printable] = [1, "haha"]; (Polymorphism)
fn print_it(item: &Printable) {
  println!("{}", item.format());
}

pub fn main() {
  let a:i32 = 123;
  let b:String = "This is a test".to_string();

  print_it(&a);
  print_it(&b);
}
