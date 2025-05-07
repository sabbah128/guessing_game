use rand::Rng;
use rand::thread_rng;
use std::cmp::Ordering;
use std::io::stdin;


fn main() {

    let mut i = 0;

    println!("Guess the number!");
    let secret_number = thread_rng().gen_range(1..=100);

    loop {
        i += 1;
        println!("Please input your guess ({i}).");
        let mut guess = String::new();

        stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        // let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too BIG!"),
            Ordering::Equal => {
                println!("You win!!!");
                println!("You can guess in {i}th times.");
                break;
            }
        }
    }
}
