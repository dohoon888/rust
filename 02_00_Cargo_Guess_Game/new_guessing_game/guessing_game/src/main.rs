use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");
    
    println!("Guess the number");
    loop {
        
        println!("Please input your guess.");
        let mut guess = String::new();

        // io::stdin().read_line(&mut guess).expect("Failed to read line");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        //let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // place holder {guess}
        println!("You Guessd: {guess}");

        // {}의 place holder의 경우 레퍼런스를 넘겨줘야한다.
    //println!("x = {x} and y + 2 = {}", y + 2);

        // match guess.cmp(&secret_number) {
        //     Ordering::Less => println!("Too small!"),
        //     Ordering::Greater => println!("Too big!"),
        //     Ordering::Equal => println!("You win!"),
        // }
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
    