fn square(x:i32) -> i32
{
  x * x
}

fn is_even(x:i32) -> bool
{
  x % 2 == 0
}

fn greater_than(limit: i32) -> impl Fn(i32) -> bool
{
  return move |x| {x > limit};
}

pub fn high_order_functions()
{
  // sum of all even squares < 500
  let limit = 500;
  let mut sum = 0;

  // x: &i32 (pattern:type)
  // let is_greater_than = |&x:&i32| {x > limit};
  /*
  src: https://www.udemy.com/course/rust-lang/learn/lecture/4236024#questions/6396010
  1. Lambda functions support automatic type deduction for parameters:
    You can simply write |x| {...} and Rust will figure out the type of parameter 'x' if it can. If it can't, that's a compiler error. You can explicitly specify the type of 'x', e.g. | x : &i32 | {...},  but you don't have to. It is optional.

  2. Function parameters can also be patterns:
    You can specify a pattern in place of a single parameter and Rust will try to match the argument against it. This is 'only' syntatic sugar but does not influence the type of the parameter at all! This is what you are seing in the filter example. The declaration '&x' does not specify the type of the parameter, it is a pattern. Rust will dereference the argument and bind its contents to x. This is convenient because you don't have to dereference x everytime in the function body. (Sidenote: Rust also has the Deref trait which is intendend to facilitate automatic dereferencing which can further confuse newcomers).
  */

  let is_greater_than = greater_than(limit);

  for i in (0..)
  {
    let isq = square(i);
    
    if is_greater_than(isq)
    {
      break;
    }

    if is_even(isq)
    {
      sum += isq;
    }
  }

  // another solution
  let sum2 = (0..)
    .map(|x| {x * x})
    .take_while(|&x| {x < limit})
    .filter(|&x| {is_even(x)})
    .fold(0,|sum, x| {sum+x});

  let sum3 = (0..)
    .map(|x| {x * x})
    .take_while(|x: &i32| {*x < limit})
    .filter(|x:&i32| {is_even(*x)})
    .fold(0, |sum, x| {sum+x});

  println!("sum = {}", sum);
  println!("sum2 = {}", sum2);
  println!("sum3 = {}", sum3);
}
