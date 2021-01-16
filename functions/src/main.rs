mod methods;
mod closures;
mod high_order_functions;

fn println_value(x: i32)
{
    println!("value = {}", x);
}

fn increase(x: &mut i32)
{
    *x += 1;
}

fn product(a: i32, b: i32) -> i32
{
    a * b
}

fn functions()
{
    println_value(22);

    let mut x = 34;
    increase(&mut x);
    println!("x = {}", x);

    let a = 3;
    let b = 5;
    let value = product(a, b);
    println!("product = {}", value);
}

fn main() {
    // functions();
    // methods::methods();
    // closures::closures();
    high_order_functions::high_order_functions();
}
