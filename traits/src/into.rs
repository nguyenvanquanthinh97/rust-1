use std::fmt::Debug;

#[derive(Debug)]
struct Person
{
  name: String
}

impl Person
{
  // fn new(name: &str) -> Person
  // {
  //   Person{name: name.to_string()}
  // }

  fn new<S: Into<String>>(name:S) -> Person
  {
    Person {name: name.into()}
  }
}

pub fn into()
{
  let p1 = Person::new("Thinh");
  // let p2 = Person::new(String::from("Dat").as_ref());
  let p2 = Person::new(String::from("Dat"));

  println!("{:?}", p1);
  println!("{:?}", p2);
}
