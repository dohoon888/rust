
// 1. implements about coin's match 
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }
// fn main() {
//     let penny = Coin::Penny;
//     println!("Value of penny: {} cents", value_in_cents(penny));
// }


// 1-1. implements about coin's match 
// it's use #[derive(Clone)]
/*
#[derive(Clone)] 
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn value_in_cents_2(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}


fn main() {
    
    let penny = Coin::Penny;
    println!("Value of penny: {} cents", value_in_cents(penny.clone()));

    println!("Value of penny: {} cents", value_in_cents_2(penny.clone()));
}

*/

// 2. matching using Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("five {:?}", five);
    println!("six {:?}", six);
    println!("none {:?}", none);
}
