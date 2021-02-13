// Arc can only help us to read a memory between threads, but not modify it because it unsafe (thread race)
// Mutex help us to modify a variable between threads, thanks to MutexGuard
// MutexGuard will ensure only a thread can access (read or edit) a variable by lock it.

use std::sync::{Arc, Mutex};

struct Person
{
  name: Arc<String>,
  state: Arc<Mutex<String>>
}

impl Person
{
  fn new(name: Arc<String>, state: Arc<Mutex<String>>) -> Person
  {
    Person {name, state}
  }

  fn greeting(&self)
  {
    let mut state = self.state.lock().unwrap();
    state.clear();
    state.push_str("exciting");
    println!("Hi my name is {}, and i'm {}", self.name, state.to_string());
  }
}

pub fn main()
{
  let name = Arc::new("John_Anderson".to_string());
  let state = Arc::new(Mutex::new("normal".to_string()));

  let person = Person::new(name.clone(), state.clone());

  let new_thread = std::thread::spawn(move || {
    person.greeting();
  });

  new_thread.join().unwrap();
  println!("state: {}", state.lock().unwrap().to_string());
}
