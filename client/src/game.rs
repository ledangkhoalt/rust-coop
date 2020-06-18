use rand::prelude::*;
use std::cmp::Ordering;
use std::io;
pub fn run() {
  let mut rng = rand::thread_rng();
  let serrect = rng.gen_range(0, 100);
  println!("Random number: {}", serrect);

  'looper: loop {
    let mut input = String::new();
    println!("Say something:");
    let mut _new_num = io::stdin()
      .read_line(&mut input)
      .expect("Failed to read line");

    let guess: u32 = input.trim().parse().expect("Please type a number!");
    println!("Random number: {}", guess);

    match guess.cmp(&serrect) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
        println!("You win!");
        break 'looper;
      }
    }
  }
}
