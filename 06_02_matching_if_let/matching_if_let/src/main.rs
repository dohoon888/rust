
// 1. matching pattern use Some. it's boilerplate code.
// fn main() {
//     let config_max = Some(3u8);
//     match config_max {
//         Some(max) => println!("The maximum is configured to be {}", max),
//         _ => (),
//     }
// }


// 2. example "if let"

// fn main() {
//     let config_max = Some(3u8);
//     if let Some(max) = config_max {
//         println!("The maximum is configured to be {}!", max);
//     }
// }

// 2-1 example "if let"
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

// fn main() {
//     let mut count = 0;
//     let coin = Coin::Quarter(UsState::Alabma);
//     match coin {
//         Coin::Quarter(state) => println!("State quarter from {:?}!", state),
//         _ => count +=1,
//     }

// }

// 2-2. example "if let"
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

// fn main() {
//     let mut count = 0;
//     // let coin = Coin::Quarter(UsState::Alabma);
//     let coin = Coin::Penny;
//     if let Coin::Quarter(state) = coin {
//         println!("State quarter from {:?}!", state);
//     } else {
//         println!("knock knock knock. penny?");
//         count +=1;
//     }
//     println!("Count {} ", count);
// }