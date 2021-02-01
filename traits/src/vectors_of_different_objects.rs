trait Animal {
  fn name(&self) -> &'static str;

  fn talk(&self);
}

struct Cat {
  name: &'static str
}

impl Animal for Cat {
  fn name(&self) -> &'static str {
    self.name
  }

  fn talk(&self) {
    println!("{} says meow meow", self.name());
  }
}

struct Human {
  name: &'static str
}

impl Animal for Human {
  fn name(&self) -> &'static str {
    self.name
  }

  fn talk(&self) {
    println!("Hi! My name is {}", self.name());
  }
}

enum Creature {
  Human(Human),
  Cat(Cat)
}

pub fn main() {
  // Best way, use pointer (Box)
  let mut animals:Vec<Box<Animal>> = Vec::new();

  animals.push(Box::new(Human{name: "John"}));
  animals.push(Box::new(Cat{name: "Tom"}));

  for animal in animals
  {
    animal.talk();
  }

  // Alter way, use enum
  let mut animals:Vec<Creature> = Vec::new();

  animals.push(Creature::Human(Human {name: "Jack"}));
  animals.push(Creature::Cat(Cat {name: "Oggy"}));

  for animal in animals
  {
    match animal {
      Creature::Human(h) => h.talk(),
      Creature::Cat(c) => c.talk()
    }
  }
}
