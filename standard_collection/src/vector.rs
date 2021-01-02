pub fn vector() {
  let mut a = Vec::new();
  a.push(1);
  a.push(2);
  a.push(3);

  println!("{:?}", a);

  a.push(44);
  println!("{:?}", a);

  // index of an array or vector can only be usize or isize
  // Because depend on the system or OS, the address will be 32 bits or 64 bits
  let idx:usize = 0;
  a[idx] = 433;
  println!("a[{}] = {}", idx, a[idx]);
  // This would crash the program because vector does not have idx 33 to access
  // println!("a[33] = {}", a[33]);
  println!("{:?}", a);

  // Option: this would return option type
  println!("{:?}", a.get(3));

  let idx:usize = 233;
  // Option type
  match a.get(idx)
  {
    Some(z) => println!("a[2]={}",z),
    None => println!("error, no such element")
  }

  for x in &a
  {
    println!("{}", x);
  }

  // This would return Option type too
  let last_element = a.pop();
  println!("Last element is {:?}, a = {:?}", last_element, a);
}
