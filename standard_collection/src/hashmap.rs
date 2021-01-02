use std::collections::HashMap;

pub fn hashmap() {
  let mut shapes = HashMap::new();

  shapes.insert("triangle", 3);
  shapes.insert("square", 4);

  match shapes.get("triangle")
  {
    Some(z) => println!("Triangle has {} sides", z),
    None => println!("shape doesn't contain key triangle")
  }

  // If hashmap not exist this key => insert the key with value (Upsert)
  shapes.entry("Circle").or_insert(1);

  println!("{:?}", shapes);

  {
    // return the reference value of key "Circle"
    let actual = shapes.entry("Circle").or_insert(0);
    println!("shapes['Circle'] = {}", actual);
    *actual = 0;
    println!("{:?}", actual);
  }
}
