use std::fmt::Debug;

trait Shape
{
  fn area(&self) -> f64;
}

#[derive(Debug)]
struct Square
{
  side: f64
}

impl Shape for Square
{
  fn area(&self) -> f64
  {
    self.side * self.side
  }
}

#[derive(Debug)]
struct Circle
{
  radius: f64
}

impl Shape for Circle
{
  fn area(&self) -> f64
  {
    self.radius * self.radius * std::f64::consts::PI
  }
}

// Three ways to declare trait type in function parameter
// fn print_info(shape: impl Shape + Debug)
// Trade bound syntax:
// fn print_info<T: Shape + Debug>(shape: T)
fn print_info<T>(shape: T)
  where T: Shape + Debug
{
  // print debug
  // to eneble print debug, that type has to be implemented with Debug method
  println!("{:?}", shape);
  println!("The area is {}", shape.area());
}

pub fn trait_parameters()
{
  let square = Square {side: 2.0};
  let circle = Circle {radius: 3.0};

  print_info(square);
  print_info(circle);
}
