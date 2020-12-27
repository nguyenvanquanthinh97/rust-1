fn process_slice(slice: &mut [i32]) {
  println!("first element = {}, length = {}", slice[0], slice.len());
  slice[0] = 123;
}

pub fn slices() {
  let mut arr = [1,2,3,4,5];

  process_slice(&mut arr[1..4]);

  println!("Array after change slice[0]: {:?}", arr);
}
