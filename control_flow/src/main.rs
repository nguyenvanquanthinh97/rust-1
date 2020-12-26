mod combination_lock;

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

fn for_loop(){
    // 1..11 the upper bound will be 11 (exclusive)
    // to include upper bound (11) in to range, we use this
    // 1..=11 (include 11)
    for x in 1..11
    {
        if x == 3 {continue;}
        
        if x == 8 {break;}

        println!("x = {}", x);
    }

    println!("----");

    for x in 10..=21
    {
        println!("x = {}", x);
    }

    println!("-----");

    for (idx, val) in (30..41).enumerate()
    {
        println!("{}: {}", idx, val);
    }
}

// to get the result of match (will return type &str) => use the comment function below
// fn match_statement() -> 'static str
fn match_statement(){
    let country_code = 46;

    // we have to include all kind of cases that can happen base on type of variable
    // rust is smart enough to be aware of the kind of variable that you have and depending
    // on this variable it will actually perform the appropriate checks.
    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=1000 => "Unknown",
        _ => "Invalid"
    };

    println!("the country with code {} is {}", country_code, country);

    // return country

    let x = false;

    let s = match x {
        true => "true",
        false => "false"
    };
}

fn main() {
    println!("if_statement: ");
    if_statement();
    println!("--------------");
    println!("while_and_loop: ");
    while_and_loop();
    println!("--------------");
    println!("for_loop: ");
    for_loop();
    println!("--------------");
    match_statement();
    // let result = match_statement();
    combination_lock::combination_lock();
}
