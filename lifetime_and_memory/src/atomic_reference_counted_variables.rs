// cannot share rc type between thread
// because it has not been implemented with trait Send => unsafe
// So we instead of using Rc -> use Arc

// use std::rc::Rc;
use std::sync::Arc;
use std::thread;

struct Person
{
  name: Arc<String>
}

impl Person
{
  fn new(name: Arc<String>) -> Person
  {
    Person {name}
  }

  fn greeting(&self)
  {
    println!("Hello my name is {}", self.name)
  }
}

pub fn main() {
  let name = Arc::new("Thinh".to_string());

  let person = Person::new(name.clone());

  let new_thread = thread::spawn(move || {
    person.greeting();
  });
  
  new_thread.join().unwrap();
  println!("name: {}", name);
}
