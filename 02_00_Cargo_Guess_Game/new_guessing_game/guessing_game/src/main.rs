use std::io;

fn main() {
    println!("Guess the number");
    println!("Please input your guess.");
    let mut guess = String::new();

    // io::stdin().read_line(&mut guess).expect("Failed to read line");
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    // place holder {guess}
    println!("You Guessd: {guess}");

    // {}의 place holder의 경우 레퍼런스를 넘겨줘야한다.
   //println!("x = {x} and y + 2 = {}", y + 2);
}
