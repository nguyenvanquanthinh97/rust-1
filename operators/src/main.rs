use std::mem;

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
    print!("scope_and_shadowing: ");
    scope_and_shadowing();
    println!("--------------------");
    print!("operators: ");
    operators();
}
