// Note: In rust 2018, you only need to use .clone() to freely sharing variable

// Rc will counting how many times a variable has been referenced to
// Thanks to that, that variable can freely share
use std::rc::Rc;

struct Person
{
  name: Rc<String>
}

impl Person
{
  fn new(name: Rc<String>) -> Person
  {
    Person {name}
  }

  fn greeting(&self)
  {
    println!("Hello my name is: {}", self.name)
  }
}

pub fn main() {
  let name = Rc::new("Thinh".to_string());
  println!("reference call before create a person: {}", Rc::strong_count(&name));
  {
    let thinh = Person::new(name.clone());
    thinh.greeting();
    println!("Reference call after creating a person: {}", Rc::strong_count(&name));
  }

  println!("Reference call after the scope: {}", Rc::strong_count(&name));
  println!("{}", name);
}
