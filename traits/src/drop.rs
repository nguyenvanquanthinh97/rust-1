struct Creature {
  name: String
}

impl Creature {
  fn new(name: &str) -> Creature {
    println!("{} is in the game", name);
    Creature {name: name.into()}
  }
}

impl Drop for Creature {
  fn drop(&mut self) {
    println!("{} is dead", self.name);
  }
}

// Rust only call drop if that variable will no longer need to use again
// This works as "Mark and Sweep" in Javascript
pub fn drops() {
  let mut clever:Creature;
  {
    let goblin = Creature::new("Thinh");
    clever = goblin
  }
  // drop(clever);
  println!("The game is in process");
}
