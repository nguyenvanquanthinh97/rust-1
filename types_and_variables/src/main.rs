#[allow(dead_code)]
#[allow(unused_variables)]

use std::mem;

fn main() {
    let a: u8 = 128; // u = unsigned, 8 bits, 0 - 255
    println!("a = {}", a);

    // u = unsigned, 0 to 2 ^ N
    // i = signed, -2^(N-1) ... 2^(N-1) - 1
    // if we only declare "let" without "mut" that variable will be treated as being immutable
    // Otherwise, it will be treated as being mutable
    let mut b: i8 = 0; // -128 ... 127
    println!("BEFORE b = {}", b);
    b = -128;
    println!("AFTER b = {}", b);

    let mut c = 123456789; // 32 bits
    println!("BEFORE c = {}, takes up {} bytes", c, mem::size_of_val(&c));
    c = -1;
    println!("AFTER c = {}, takes up {} bytes", c, mem::size_of_val(&c));

    let d = 4;// 32 bits
    println!("c = {}, takes up {} bytes", d, mem::size_of_val(&d));

    // u8, u16, u32, u64, i8, i16, i32, i64

    // usize isize ( variable which is native to that processor, os )
    // usize: unsign type
    // isize: sign type
    let z: isize = 123;
    let size_of_z = mem::size_of_val(&z) as i8; // casting type from usize => i8
    println!("z = {}, takes up {} bytes, {}-bits OS", z, size_of_z, size_of_z * 8);
    println!("size_of_z = {}, takes up {} bytes",
     size_of_z, mem::size_of_val(&size_of_z));
    
    // char type is different from letter type
    // character isn't just a number or letter
    // char type can present punctuation: . ;
    let e: char = 'x'; // 32 bits
    println!("{} is a char, size = {} bytes", e, mem::size_of_val(&e));

    // f32 f64 IEEE754
    // there is no signed/unsigned for floating point => they 're all signed floating point
    // if not declare default type => the default will be f64
    let f: f32 = 0.1; // 32 bits
    println!("{}, size = {} bytes", f, mem::size_of_val(&f));

    let g = 0.8; // 64 bits
    println!("{}, size = {} bytes", g, mem::size_of_val(&g));

    let h: bool = false; // 8 bits
    println!("{}, size = {} bytes", h, mem::size_of_val(&h));
}
