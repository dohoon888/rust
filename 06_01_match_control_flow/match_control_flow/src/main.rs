
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

// 1-2. Binding Enum to Enum Variant. 
// #[derive(Debug)] 
// enum UsState {
//     Alabma,
//     Alaska,
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {:?}!", state);
//             25
//         }
//     }
// }

// fn main() {
    
//     // let penny = Coin::Penny;
//     println!("Value of penny: {} cents", value_in_cents(Coin::Quarter(UsState::Alaska)));
    
// }

// 2. matching using Option<T>
// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }

// fn main() {
//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);

//     println!("five {:?}", five);
//     println!("six {:?}", six);
//     println!("none {:?}", none);
// }

// 3. other and "_"
// "_" is do not bind in matching.
// other is bind in matching.
// fn main() {
//     let dice_roll = 9;
//     match dice_roll {
//         3 => add_fancy_hat(),
//         7 => remove_fancy_hat(),
//         // other => move_player(other),
//         _ => reroll(),
//     }

// }
// fn add_fancy_hat() {
//     println!("add fancy hat");
// }

// fn remove_fancy_hat() {
//     println!("remove fancy hat");
// }

// fn move_player(num_spaces: u8) {
//     println!("move player {}", num_spaces);
// }

// fn reroll() {
//     println!("re-roll!!");
// }