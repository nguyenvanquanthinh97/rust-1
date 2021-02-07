// Only a single variable kind of owns the memory at any given time
pub fn main() {
  let v = vec![1,2,3];
  
  // let v2 = v;
  // because v2 has already take ref value of v => v is invalid to access
  // println!("{:?}", v);
  let foo = |x:Vec<i32>| {};
  // foo(v);
  // v has been borrowed by foo function
  // println!("{:?}", v);

  let u = 1;
  let u1 = u;
  // because u's value is primitive => u1 will copy u's value instead of copy reference
  println!("{}", u);

  // This will take reference and get problem with borrowing
  // let u = Box::new(1);
  // let u1 = u;
  // println!("{}", u);

  let print_vector = |x:Vec<i32>| -> Vec<i32> {
    println!("{:?}", x);
    x
  };

  // print_vector borrow reference of v, and then return the ownership of vector
  // which mean we can access vv as vv will be the one who ref to vector (first assigned to v)
  let vv = print_vector(v);

  println!("{:?}", vv[1]);
}
