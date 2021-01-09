struct Point {
  x: f64,
  y: f64
}

struct Line {
  start: Point,
  end: Point
}

// function in a struct => call method
impl Line {
  // &self: refer to its class
  fn len(&self) -> f64 {
    let dx = self.end.x - self.start.x;
    let dy = self.end.y - self.end.y;

    (dx*dx + dy*dy).sqrt()
  }
}

pub fn methods() {
  let start = Point {x: 1.0, y: 1.0};
  let end = Point {x: 3.0, y: 3.0};

  // let line = Line {start: start, end: end};
  let line = Line {start, end};
  println!("length = {}", line.len());
}
