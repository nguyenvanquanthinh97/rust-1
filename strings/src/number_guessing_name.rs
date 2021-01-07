use rand::Rng;
use std::io::stdin;

pub fn number_guessing_name () {
  let random = rand::thread_rng().gen_range(1..101);
  println!("{}", random);
  let mut buffer = String::new();

  loop {
    println!("Enter your guess: ");
    match stdin().read_line(&mut buffer) {
      Ok(_) => {
        println!("buffer {}", buffer.to_string());
        let parsed = buffer.trim_end().parse::<u8>();
        match parsed {
          Ok(guess) => {
            if guess < 1 || guess > 100 {
              println!("Out of scope");
            } else if guess < random {
              println!("Too low!");
            } else if guess > random {
              println!("Too high!");
            } else {
              println!("Congratulation! Matched");
              break;
            }
          },
          Err(error) => {
            println!("Can not parse with input, error: {}", error)
          }
        }
        // we must clear buffer, because we don't declare buffer out of loop
        // so it won't be renewed after looping
        buffer.clear();
      },
      Err(_) => continue
    }
  }
}
