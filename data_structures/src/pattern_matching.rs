fn how_many(x:i32) -> &'static str {
  match x {
    0 => "no",
    1 | 2 => "one or two",
    12 => "a dozen",
    // to use range somewhere inside the processing of that particular case
    z @ 9..=11 => {
      println!("x = {}", z);
      "lots of"
    },
    // we don't care what is value, we just care expression
    _ if (x % 2 == 0) => "evenly",
    _ => "a few"
  }
}

pub fn pattern_matching() {
  for x in 0..13
  {
    println!("{}: I have {} oranges", x, how_many(x))
  }

  let point = (3,4);
  match point {
    (0,0) => println!("origin"),
    (x,0) => println!("x axis, x = {}", x),
    (0,y) => println!("y axix, y = {}", y),
    // we don't care what is x, we just only care y
    (_,y) => println!("(?,{})", y)

    // # wrong: (_,y) => println!("{},{}", _, y);
    // _ is reserved indentifier, can not access its value

    // (x,y) => println!("({},{})",x,y)
  }
}
