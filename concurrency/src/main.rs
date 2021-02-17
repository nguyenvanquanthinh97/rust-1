use std::thread;
use std::time;

fn print_n_times(s: char, n: i32, timeout: i32)
{
    for i in 1..=n
    {
        print!("{}", s);
        thread::sleep_ms(timeout as u32);
    }
}

fn main() {
    let n:i32 = 10;
    let new_thread = thread::spawn(move || {
        print_n_times('+', n, 500);
    });

    print_n_times('-', n, 300);
    new_thread.join().unwrap();
}
