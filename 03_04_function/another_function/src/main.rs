// 1. function definition
// the function is deinited main fuction.
// it's not problem(work!)

// fn main() {
//     println!("Hello, world!");

//     another_function();
// }

// fn another_function() {
//     println!("Another function.");
// }

// 2. function is have parameter.
// fn main() {
//     another_function(5);
// }

// fn another_function(x: i32) {
//     println!("The value of x is: {x}");
// }

// 3. function parameter must define data type.
// fn main() {
//     print_labeled_measurement(5, 'h');
// }

// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }

// 4. statement and expression.

// normally statement.
// this statement is not reuturn anything. 
// fn main() {
//     let y = 6;
// }

// abnormally statement.
// fn main() {
//     let x = (let y = 6);
// }

// expression is not use semicolon.
// fn main() {
//     let y = {
//         let x = 3;
//         x + 1
//     };

//     println!("The value of y is: {y}");
// }

// 5. function's return value.
// return value's type is must be defined to after arrow(->).
// fn five() -> i32 {
//     5
// }

// fn main() {
//     let x = five();

//     println!("The value of x is: {x}");
// }

// 5-1. 
// if expression:x+1 is ended by semilcolon, it's changed expression to statement and occurred Error.
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    // never ended by semilcolon.
    x + 1 
}