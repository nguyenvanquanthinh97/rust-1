struct Person<'a>
{
  name: &'a str
}

impl<'b> Person<'b>
{
  fn talk(&self)
  {
    println!("{}", self.name);
  }
}

pub fn main() {
  let person = Person {name: "Thinh"};

  person.talk();
}
