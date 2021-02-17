extern crate rand;
extern crate Phrases;

use rand::Rng;
use Phrases::greetings::french;

fn main()
{
  // let mut rng = rand::thread_rng();
  // let rand_number:u32 = rng.gen();

  // println!("random: {}", rand_number);
  println!("English: {}, {}",
    Phrases::greetings::english::hello(),
    Phrases::greetings::english::goodbye()
  );

  println!("French: {}, {}",
    french::hello(),
    french::goodbye()
  );
}
