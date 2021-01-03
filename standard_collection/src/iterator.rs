pub fn iterator() {
  let mut vec = vec![3,2,1];

  // if we use: for x in vec => we have move all items in vec (vec.into_iter())
  // use & to reference item in vector
  for x in &vec
  {
    // rust will auto detect if x is in println line
    // so we don't need to use *x in a println line
    println!("x = {}", x);
  }

  // same with for x in &vec
  for x in vec.iter()
  {
    println!("{}", x);
  }

  for x in &mut vec
  {
    *x += 2;
  }

  // same with for x in &mut vec
  for x in vec.iter_mut()
  {
    *x += 2;
  }

  for y in &vec
  {
    println!("y = {}", y);
  }

  // reverse iteration
  for z in vec.iter().rev()
  {
    println!("z = {}", z);
  }

  println!("{:?}", vec);

  let mut vec2 = vec![1,2,3];

  // equal to: 
  // let it = vec.into_iter();
  // vec2.extend(it);
  // Because we have move all items in vec into iteration
  // We're no longer to use vec
  vec2.extend(vec);
  println!("{:?}", vec2);
  // println!("{:?}", vec);

  for x in vec2.iter()
  {
    println!("we got {}", x);
  }
  println!("length of vec2 = {}", vec2.len());
}
