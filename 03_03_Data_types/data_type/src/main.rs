use std::io;
fn main() {
    
    // 1. rust datatype is definited in compiling time.
    // when variable guess is defined u32. 
    // let guess: u32 = "42".parse().expect("Not a number!");
    // println!("gusss! : {}", guess);

    // 2. float type
    // f64
    // let x = 2.0;

    // f 32
    // let y : f32 = 3.0;

    // basic operations
    // add
    // let sum = 5 + 10;

    // sub
    // let difference = 95.5 - 4.3;

    // mul
    // let product = 4 * 30;

    // div
    // Result value is  -1.
    // let quotient = 56.7 / 32.2;
    // let truncated = -5 / 3; 
    
    //  rem
    // let remainder = 43 % 5;

    // 3. bool type
    // old school definition
    // let t = true;

    // explicit type annotation
    // let f: bool = false; 

    //4. character type

    // let c = 'z';
    // explicit type annotation
    // let z: char = 'Z';
    // let heart_eyed_cat = '😻';

    // 5. compound type
    // let tup: (i32, f64, u8) = (500, 6.4, 1);

    // let tup = (500, 6.4, 1);

    // let (x, y, z) = tup;

    // println!("The value of x is: {x}");
    // println!("The value of y is: {y}");
    // println!("The value of z is: {z}");

    // tuple is cant approach to Values using index
    // let x: (i32, f64, u8) = (500, 6.4, 1);
    // let five_hundred = x.0;
    // let six_point_four = x.1;
    // let one = x.2;

    // println!("The value of x0 is: {five_hundred}");
    // println!("The value of x1 is: {six_point_four}");
    // println!("The value of x2 is: {one}");

    // 6. array.
    // array type is has static. vector's size if more flexible
    // let a = [1, 2, 3, 4, 5];
    // let months = ["January", "February", "March", "April", "May", "June", "July",
    //           "August", "September", "October", "November", "December"];
    // explicit type annotation
    // let a: [i32; 5] = [1, 2, 3, 4, 5];
    // it's filled to size 5.
    // let a = [3; 5];

    // old school array type's index approach
    // let a = [1, 2, 3, 4, 5];
    // let first = a[0];
    // let second = a[1];

    // e.g. ) wrong approach array index.
    // this code is success to compile and run
    // but it's have posiible throw exception::out_of_range, when if input value is bigger than 4 or less than 0.

    // so it's need to add size check after input index size.

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
        

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
