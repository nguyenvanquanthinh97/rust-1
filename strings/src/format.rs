pub fn format(){
  let name = "Thinh";
  let greeting = format!("Hello! Good morning {}", name);
  println!("{}", greeting);

  let hello_rust = format!("{} {}", "hello", "rust");
  println!("{}", hello_rust);

  let forest_run = format!("{0} {1} {0}", "run", "forest");
  println!("{}", forest_run);

  let info = format!("the name 's {last}. {first} {last}", first = "John", last = "Anderson");
  println!("{}", info);

  let mixed = format!(
    "{1} {} {0} {} {data}",
    "alpha",
    "beta",
    data = "delta"
  );
  println!("{}", mixed);
}
