fn sum_and_product(x: i32, y:i32) -> (i32, i32) {
  (x+y, x*y)
}

// tuple != array
// tuple can contain a list of different type values
// meanwhile, array just can contain list of same type values
pub fn tuple(){
  let x = 3;
  let y = 4;
  let result = sum_and_product(x, y);

  println!("{:?}", result);

  println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, result.0, result.1);

  // destruct tuple
  let (a, b) = result;
  println!("Sum = {}, Product = {}", a, b);

  let result2 = sum_and_product(4, 7);
  let combined = (result, result2);
  println!("{:?}", combined);

  println!("last element = {}", (combined.1).1);

  // destruct combined tuples
  let ((c,d),(e,f)) = combined;
  println!("(c = {}, d = {}), (e = {}, f = {})", c, d, e, f);

  // tuple contains different types of values
  let various_tuple = (42.0, false, -1i8); // (f64, bool, i8);
  println!("various_tuple = {:?}", various_tuple);

  // tuple contains only single element
  let single_tuple = (-32f32,);
  println!("single_tuple = {:?}", single_tuple);
}
