enum Color {
  Red,
  Blue,
  Green,
  rgb(u8, u8, u8), // tuple format
  Cmyk{cyan: u8, magneto: u8, yellow: u8, black: u8} // struct format
}

pub fn enums() {
  let c:Color = Color::Cmyk{cyan: 0, magneto: 0, yellow: 0, black: 255};

  match c {
    Color::Red => println!("r"),
    Color::Blue => println!("b"),
    Color::Green => println!("g"),
    Color::rgb(0,0,0)
    | Color::Cmyk{cyan: _, magneto: _, yellow: _, black: 255} => println!("black"),
    Color::rgb(r,g,b) => println!("rgb color (r: {}, g: {}, b: {})", r, g, b),
    _ => ()
  }
}
