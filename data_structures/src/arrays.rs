use std::mem;

// single dimension array
fn single_dimension_array() {
  let mut arr:[i32; 5] = [1,2,3,4,5];
  
  println!("arr has {} elements, first is {}", arr.len(), arr[0]);
  arr[0] = 123;
  println!("arr[0] = {}", arr[0]);
  
  // loop all elements in array by for loop
  for idx in 0..arr.len()
  {
    println!("arr[{}]={}", idx, arr[idx]);
  }

  // loop and print all elements in arr by debug output
  println!("{:?}", arr);

  // compare between 2 arrays
  // Requirements:
  // 2 arrays must have same length and must have same type
  if arr == [123,2,3,4,5] {
    println!("match");
  }

  // initiate an array b with 10 items' value is 1
  let b:[u16;10] = [1; 10];
  // let b = [1u16; 10];
  println!("{:?}", b);

  // 20 bytes
  println!("b took up {} bytes", mem::size_of_val(&b));
}

// double dimension array
fn double_dimension_array() {
  let mtx:[[f32;3];2] = 
  [
    [1.0, 0.0, 0.0],
    [0.0, 2.0, 0.0]
  ];

  for i in 0..mtx.len() {
    for j in 0..mtx[i].len() {
      if i == j {
        println!("mtx[{}][{}] = {}", i, j, mtx[i][j]);
      }
    }
  }

  // 24 bytes
  println!("mtx took up {} bytes", mem::size_of_val(&mtx));
}

pub fn arrays() {
  // exercise on single dimension array
  single_dimension_array();

  // exercise on double dimension array
  double_dimension_array();
}
