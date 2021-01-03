pub fn strings() {
  let s:&'static str = "hello there!"; // &str = string slice
  // Althought s is type as vector of chars, but we cannot access a character at index (example below)
  // let h = s[0];

  // return &str into iteration of chars
  println!("Print in order: ");
  for c in s.chars()
  {
    println!("{}", c);
  }

  // because s.chars() return iteration => we can use it to reverse
  println!("Print in reverse order: ");
  for c in s.chars().rev()
  {
    println!("{}", c);
  }
  
  println!("Access a char with an index at: ");
  if let Some(first_char) = s.chars().nth(0)
  {
    println!("{}", first_char);
  }

  // String: -> heap allocated construct
  let mut letters = String::new();
  let mut a = 'a' as u8;
  while a < ('z' as u8)
  {
    // push a char
    letters.push(a as char);
    // push string
    letters.push_str(",");
    a += 1;
  }

  println!("{}", letters);

  // &tr <> String
  let u:&str = &letters; // Convert from String to &str
  println!("Convert from String to &str u: {}", u);

  // concatentation
  // String + &str
  let str_tmp = String::from("This is a test");
  let z = letters + &str_tmp; // letters + "ABC";
  println!("z = {}", z);

  // we are no longer to use letters anymore
  // Becuase letters have been moved all to iter
  // println!("letters = {}", letters);
  println!("str_tmp = {}", str_tmp);
  

  // String <> &str
  let mut abc:String = "Hello World".to_string(); // Convert from &str into String
  abc.remove(0);
  abc.push_str("!!!");
  println!("{}", abc.replace("ello", "Goodbye"));
}
