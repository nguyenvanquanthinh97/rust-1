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

// Static dispatch:
// Rust will decide which format function will be used at compiler-time
// Idea for using if we alread know its type
// Ex: let a:i32 = 123;
fn print_it<T: Printable>(item: T) {
  println!("{}", item.format());
} // monomorphisation

pub fn main() {
  let a:i32 = 123;
  let b:String = "This is a test".to_string();

  print_it(a);
  print_it(b);
}
