use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_numnber = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess = guess.trim().parse::<u32>().expect("Please type a number!");

    match guess.cmp(&secret_numnber) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

    println!("You guessed: {}", guess);

    let mut rng = rand::thread_rng();
    let x: u32 = rng.gen();
    println!("{}", x);
    println!("{:?}", rng.gen::<(f64, bool)>());
}
