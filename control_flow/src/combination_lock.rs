use std::io::stdin;

enum STATE {
  LOCKED,
  FAILED,
  UNLOCKED
}

pub fn combination_lock() {
  let code = String::from("1234");
  let mut state = STATE::LOCKED;
  let mut entry = String::new();

  loop {
    match state {
      STATE::LOCKED => {
        let mut input = String::new();
        match stdin().read_line(&mut input) {
          Ok(_) => {
            entry.push_str(&input.trim_end());
            if entry == code {
              state = STATE::UNLOCKED;
            }
            else if !code.starts_with(&entry) {
              state = STATE::FAILED;
            }
          }
          Err(_) => continue
        }
      }
      STATE::FAILED => {
        state = STATE::LOCKED;
        entry.clear();
        println!("INPUT FAILED, LOCKED AGAIN");
        continue;
      }
      STATE::UNLOCKED => {
        println!("UNLOCK");
        return;
      }
    }
  }
}
