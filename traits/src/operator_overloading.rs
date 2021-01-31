use std::ops::{Add, AddAssign, Neg};
use std::cmp::{Eq, PartialEq};

#[derive(Debug, Copy, Clone)]
struct Complex <T> {
  re: T,
  im: T
}

impl<T> Complex<T> {
  fn new(re: T, im: T) -> Complex<T> {
    Complex::<T> {re, im}
  }
}

// impl Add for Complex<i32> {
//   type Output = Complex<i32>;
  
//   // a + b
//   fn add(self, rhs: Self) -> Self::Output {
//     Complex {
//       re: self.re + rhs.re,
//       im: self.im + rhs.im
//     }
//   }
// }

impl<T> Add for Complex<T>
  where T: Add<Output=T>
{
  type Output = Complex<T>;
  fn add(self, rhs: Self) -> Self::Output {
    Complex {
      re: self.re + rhs.re,
      im: self.im + rhs.im
    }
  }
}

impl<T> AddAssign for Complex<T>
  where T: AddAssign<T>
{
  fn add_assign(&mut self, rhs:Self) {
    self.re += rhs.re;
    self.im += rhs.im;
  }
}

impl<T> Neg for Complex<T>
  where T: Neg<Output=T>
{
  type Output = Complex<T>;
  fn neg(self) -> Self::Output {
    Complex {
      re: -self.re,
      im: -self.im
    }
  }
}

impl<T> PartialEq for Complex<T>
  where T: PartialEq<T>
{
  fn eq(&self, rhs: &Self) -> bool {
    self.re == rhs.re && self.im == rhs.im
  }
}

// Eq will inherit from PartialEq
// don't need to re impl again
impl<T> Eq for Complex<T>
  where T: Eq{}

pub fn main() {
  let mut complex_1 = Complex::new(1, 2);
  let mut complex_2 = Complex::new(3, 4);

  println!("complex_1: {:?}", complex_1);
  println!("complex_2: {:?}", complex_2);

  // add
  println!("complex_1 + complex_2 = {:?}", complex_1 + complex_2);

  // add assign
  complex_1 += complex_2;
  println!("complex_1 after add with complex_2, {:?}", complex_1);

  // negation
  println!("-complex_1={:?}", -complex_1);

  println!("{}", complex_1 == complex_1);
}
