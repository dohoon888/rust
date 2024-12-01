// 1. enums.
/*
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("Hello, world!");

    // struct  definition
    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };


    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method be definition in hear.
        // e.g. println!
        println!("{}", &self);
    }
}

fn main() {   
    let m = Message::Write(String::from("hello"));
    m.call();

}
*/

// 1.1. enums.
/*
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method be definition in hear.
        // e.g. println!
        println!("{:?}", &self);
    }
}

fn main() {   
    let m = Message::Write(String::from("hello"));
    m.call();

}
*/

// 2. Option<T>. it's template.
// sum is not work. because it's defferet Types.
/*
#[derive(Debug)]
enum MyOption<T> {
    None,
    Some(T),
}

fn main() {   
    let some_number = MyOption::Some(5);
    let some_char = MyOption::Some('e');

    let absent_number: MyOption<i32> = MyOption::None;
    println!("some_number{:?}", some_number);
    println!("some_char{:?}", some_char);
    println!("absent_number{:?}", absent_number);

    let x: i8 = 5;
    let y: MyOption<i8> = MyOption::Some(5);

    println!("x{:?}", x);
    println!("y{:?}", y);
    // let sum = x + y;
    // println!("sum{:?}", sum);
}
*/
