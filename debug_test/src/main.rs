fn double_value(value: i32) -> i32 {
    return value * 2;
}

fn main() {
    let x:u32 = 37;
    let result = double_value(x as i32);
    println!("result = {}", result);
}
