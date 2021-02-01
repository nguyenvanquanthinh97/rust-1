trait Shape {
  fn area(&self) -> f64;
}

struct Square {
  side: f64
}

struct Circle {
  radius: f64
}

impl Shape for Square {
  fn area(&self) -> f64 {
    self.side * self.side
  }
}

impl Shape for Circle {
  fn area(&self) -> f64 {
    self.radius * self.radius * std::f64::consts::PI
  }
}

pub fn main() {
  let shapes:[&Shape; 4] = [
    &Circle {radius: 2.0},
    &Square {side: 3.0},
    &Circle {radius: 4.0},
    &Square {side: 5.0}
  ];

  for (i, shape) in shapes.iter().enumerate() {
    println!("Area of {} is {}", i, shape.area());
  }
}
