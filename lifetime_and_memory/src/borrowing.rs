pub fn main() {
  let v = vec![3,2,1];

  // instead let foo function copy ref of v (borrow v)
  // we pass reference v to it
  // We will pass case ownership
  let foo = |v: &Vec<i32>| {
    println!("{:?}", v);
  };

  foo(&v);
  println!("v[0]: {}", v[0]);

  let mut a = 40;
  let b = &mut a;
  *b += 2;
  println!("{:?}", a);

  for i in v
  {
    println!("{}", i);
  }
}
