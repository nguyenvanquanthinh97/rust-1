// union will take memory depend on the largest part
// This will take 32 bits == 4 bytes
// The problem of union is not about initiate or update value of union
// The problem is about "How we get the value of union"
// For instance:
// we can update and changing value of iof
// as iof.i = 40 or iof.f = 30.5
// However, when we get value of union as println!("{}", iof.f)
// while iof is set to i with value is 1
// union will intepreter convert bit of i -> f
// the result will be: i:1 -> f:0.000000...01 (NOT the value you want)
// So you have to access union's value under unsafe scope
union IntOrFloat {
  i: i32,
  f: f32
}

// union
fn process_value(iof: IntOrFloat) {
  unsafe {
    match iof {
      IntOrFloat {i: 30} => println!("meaning of life value"),
      IntOrFloat {f} => println!("f = {}", f)
    }
  }
}

pub fn unions() {
  let mut iof = IntOrFloat {f: 25.5};
  iof.i = 30;
  process_value(iof);
}
