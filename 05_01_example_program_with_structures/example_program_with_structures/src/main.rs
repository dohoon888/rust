// 1. rectangles.

// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// 1.1 refactoring e.g.) rectangles
// it's better but We don't know which argument is width and height.
// fn main() {
//     let rect1 = (30, 50);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// 1.2 second refactoring e.g.) rectangles
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rect1)
//     );
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

/* 2. example trade 
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // (it's not working)
    // println!("rect1 is {}", rect1);

    // (it's working)
    println!("rect1 is {}, {}", rect1.width, rect1.height);
}
*/

// 2.1 example trade (#[derive(Debug)])
// {} Adding :? to me is like communicating to println! that you want to use the output format Debug.
// if you have more fields, you'll need a more readable format,
//  but in that case, you can use {:#?} instead of {:?} in the println! string.
// when use "#[derive(Debug)]" must be every time before declaration.

/*
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);
}
*/

// 3. use dbg! macro
// Macros can be used for specific fields or structures themselves.
// second macro is never have rect1's ownership (because use reference/&)

/*
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: 30 * scale,//dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}
    */