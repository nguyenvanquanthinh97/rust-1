struct Person
{
  name: String
}

impl Person {
  fn get_name(&self) -> &String
  {
    &self.name
  }
}

// This will get problem in borrowing
// because company.boss has borrowed boss reference to Person obj
// ------------
// struct Company
// {
//   name: String,
//   boss: Person
// }

// pub fn main() {
//   let boss = Person {name: String::from("Elon Musk")};
//   let company = Company {name: String::from("Telsa"), boss: boss};

//   println!("boss name get directly from boss var: {}", boss.name);
//   let boss_name = company.boss.get_name();

//   println!("{}", boss_name);
// }

// ---------------------------------------

// This will not get problem in borrowing
// because company.boss reference boss's address, not the boss's reference to Elon Musk object
struct Company<'z>
{
  name: String,
  boss: &'z Person
}

pub fn main() {
  let boss = Person {name: String::from("Elon Musk")};
  let company = Company {name: String::from("Telsa"), boss: &boss};

  println!("boss name get directly from boss var: {}", boss.name);
  let boss_name = company.boss.get_name();

  println!("{}", boss_name);
}
