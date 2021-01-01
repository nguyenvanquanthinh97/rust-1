struct Point<T>
{
  x: T,
  y: T
}

/*
  struct Point<T,V>
  {
    x: T,
    y: V
  }
*/

struct Line<T>
{
  start: T,
  end: T
}

pub fn generics() {
  /*
  let a:Point<i32,f64> = Point {x:1, y:2.2};
  let b:Point<f64,f64> = Point {x:1.2,y:3.4};
  */
  let a:Point<i32> = Point {x: 0, y:2};
  let b:Point<f64> = Point {x: 1.1, y:2.2};
  let c: Point<f64> = Point {x: 3.3, y:5.5};

  // Line will auto detect generic type is Point<f64>
  let line = Line {start: c, end: b};
}
