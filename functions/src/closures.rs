fn say_hello() {
  println!("Hello!");
}

pub fn closures() {
  let sh = say_hello;
  sh();

  let plus_one = |x:i32| -> i32 {x+1};
  let a:i32 = 6;
  println!("{} + 1 = {}", a, plus_one(a));

  // let the compiler figure type by itselft
  let two = 2;
  let plus_two = |x|
  {
    let mut z = x;
    z += two;
    return z;
  };
  
  let b = 3;
  println!("{} + 2 = {}", b, plus_two(b));

  // rust compiler are smarter, we no longer get error about borrowed error
  let borrow_two = &two;
  println!("borrow_two {}", borrow_two);

  // pass by reference
  let plus_three = |x: &mut i32| { *x += 3 };

  let mut x = 12;
  plus_three(&mut x);

  println!("x = {}", x);
}
