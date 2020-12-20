#[allow(dead_code)]
mod sh;

use std::mem;

// const in rust will only replace the value, and don't have address
// which mean if we reference a const like this
// mem::size_of_val(&MEANING_OF_LIFE) is same as mem::size_of_val(&42)
const MEANING_OF_LIFE: u8 = 42; // No fixed address

// static is same as global in JS
// Default static will same with all, is immutable
// to use as mutable, we add mut after static
// static mut Z:i8 = -69;
// however to use mutablt static variable, you have to use it in a unsafe scope
static Z:i8 = -64;

static mut I:i8 = -72;

fn scope_and_shadowing() {
    // suprisingly we can declare duplicate variable which the latter will override the former
    let a = 123;
    let a = 246;

    {
        let b = 456;
        println!("inside, b = {}", b);

        let a = 777;
        println!("inside, a = {}", a);
    }

    println!("outside, a = {}", a);
    // println!("b = {}", b);
}

fn operators() {
    // arithmetic
    let mut a = 2 + 3 * 4;// +-*/
    println!("{}", a);

    // -- ++. Unfortunately, we can't use them in rust
    a = a + 1;
    a -= 2; // a = a - 2;
            // -= += *= /= %=. Fortunately, we can use all of them in rust
    println!("result of 2 integers devided each other: {} / {} = {}", a, 3, a/3);
    println!("remainder of {} % {} = {}", a, 3, {a%3});

    // we can not use ** as pow in rust.
    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);

    println!("{} cubed = {}, {} ^ pi = {}", b, b_cubed, b, b_to_pi);

    // bitwise
    let c = 1 | 2; // | OR & AND ^ XOR ! NOT
                    // 01 OR 10 = 11 (binary) == 3 (decimal)
    println!("1|2 = {}", c);
    let two_to_10 = 1 << 10;
    println!("2 ^ 10 = {}", two_to_10);

    // logical
    let pi_less_4 = std::f64::consts::PI < 4.0; // true
    // > >= <= ==
    println!("pi_less_4 = {}", pi_less_4);
    println!("4_less_pi = {}", !pi_less_4);

    let x = 5;
    let x_is_5 = x == 5;// true

    println!("x_is_5 = {}", x_is_5);
}

fn main() {
    println!("global variable");
    println!("MEANING_OF_LIFE = {}, takes up {} bytes", MEANING_OF_LIFE, mem::size_of_val(&MEANING_OF_LIFE));
    println!("--------------------");
    println!("static variable");
    println!("Before static Z = {}", Z);
    unsafe {
        println!("Before static mut I = {}", I);
        I = -88;
        println!("After static mut I = {}", I);
    }
    println!("--------------------");
    print!("scope_and_shadowing: ");
    scope_and_shadowing();
    println!("--------------------");
    print!("operators: ");
    operators();
    println!("--------------------");
    sh::stack_and_heap();
}
