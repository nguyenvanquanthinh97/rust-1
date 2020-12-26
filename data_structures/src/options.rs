pub fn options() {
  let x = 3.0;
  let y = 0.0;

  // Option -> Some(z) | None
  let result =
  if y != 0.0 {Some(x/y)} else {None};

  match result {
    Some(z) => println!("{}/{}={}", x, y, z),
    None => println!("Can't divide to zero")
  }

  // Another way to specify Option
  // If return Some(z) then let can assign -> true
  // If return None then let can not assign -> false
  if let Some(z) = result {
    println!("The result is {}", z);
  }

  let mut count = 0;
  // Same logic with if let
  while let Some(z) = result {
    if count > 2 {
      break;
    }
    count += 1;
    println!("i = {}", count);
  }
}
