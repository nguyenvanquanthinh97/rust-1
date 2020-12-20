fn if_statement() {
    let tmp = 25;

    if tmp > 30 && tmp < 40 {
        println!("really hot outside");
    }
    else if tmp < 10 {
        println!("really cold!");
    }
    else {
        println!("temperature is OK");
    }

    // tail expression
    let day = if tmp > 20 {"sunny"} else {"cloudy"};
    println!("today is {}", day);

    println!("is it {}",
        if tmp > 20 {"hot"} else if tmp < 10 {"cold"} else {"OK"});

    println!("it is {}",
        if tmp > 20 {
            if tmp > 30 {"very hot"} else {"hot"}
        }
        else {"hot"}
    );

    let test = if tmp % 5 == 0 {
        let tmp = if tmp > 20 {5} else {2};
        tmp * 2
    } else {0};

    println!("test tail expression = {}", test);
}

fn while_and_loop(){
    let mut x = 1;

    while x < 1000 {
        x *= 2;
        if x == 64 {continue;}
        println!("x = {}", x);
    }

    // loop: equivalent to while(true)
    let mut y = 1;
    loop {
        y *= 2;
        println!("y = {}", y);
        if y == 1 << 10 {break;}
    }
}

fn main() {
    if_statement();
    while_and_loop();
}
