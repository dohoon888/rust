
// 1. Control flow statement :: if
// fn main() {
//     let number = 8;

//     if number < 5 {
//         println!("condition was true");
//     } else {
//         println!("condition was false");
//     }
// }

// 1-1. Control flow statement :: if
// a conditional expression is must be return boolean.
// it's need to fix like below
// if number == 3 {
//     println!("number was three");
// }

// fn main() {
//     let number = 3;

//     if number {
//         println!("number was three");
//     }
// }


// 2. multiple Control flow statement :: elseif(if)
// fn main() {
//     let number = 6;

//     if number % 4 == 0 {
//         println!("number is divisible by 4");
//     } else if number % 3 == 0 {
//         println!("number is divisible by 3");
//     } else if number % 2 == 0 {
//         println!("number is divisible by 2");
//     } else {
//         println!("number is not divisible by 4, 3, or 2");
//     }
// }


// 3. a three-way operator
// it's old school style. rust is must be use if-else
// let number =  conditio == true ?  5  : 6 ;
// fn main() {
//     let condition = true;
    
//     let number = if condition { 5 } else { 6 };
//     println!("The value of number is: {number}");
// }

// 4.  Control flow statement :: loop
// it's print "again" infinity.
// this program is end, when user input "ctrl+c"
// fn main() {
//     loop {
//         println!("again!");
//     }
// }

// 5. Control flow statement :: loop-break and return value
// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("The result is {result}");
// }

// 5. Control flow statement :: loop label
//Loop labels eliminate ambiguity between multiple anti-claims

// fn main() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;

//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }

//         count += 1;
//     }
//     println!("End count = {count}");
// }


// 5. Control flow statement :: while
// fn main() {
//     let mut number = 3;

//     while number != 0 {
//         println!("{number}!");

//         number -= 1;
//     }

//     println!("LIFTOFF!!!");
// }


// 5. Control flow statement :: for
// same code. but more simple.
// "For" smaple code is used iterator
// fn main() {
//     let a = [10, 20, 30, 40, 50];
//     let mut index = 0;

//     while index < 5 {
//         println!("the value is: {}", a[index]);

//         index += 1;
//     }
// }

// fn main() {
//     let a = [10, 20, 30, 40, 50];

//     for element in a {
//         println!("the value is: {element}");
//     }
// }

// 5-1. Control flow statement :: for (use fn::rev)
// fn main() {
//     for number in (1..4).rev() {
//         println!("{number}!");
//     }
//     println!("LIFTOFF!!!");
// }


// e.g) conversion from Fahrenheit to Celsius
// [°C] = ([°F] − 32) / 1.8 
// Celsius to Fahrenheit
// [°F] = [°C] × 1.8 + 32 
// fn main() {
//     let i_temp :f64 = 32.3;
//     let temp =  (i_temp - 32.0) / 1.8;
//     println!("temperature is {}", temp);
// }

// e.g) Generating *n*th Fibonacci number
//  fn main() {
//     let n = 5;
//     let mut i :u32 = 0;
//     let mut num_1 :u32 = 0;
//     let mut num_2 :u32 = 0;
    
//     loop  {
//         if i == n {
//             break;
//         }
//         let sum = num_1 + num_2;
//         num_1 = num_2;
//         num_2 = sum;
//         println!("count {}", i);
//         if (5 - n)  > 1 {
//             println!("f({}) = f({}) - f{}", i, (i-1), (i-2));
//             println!("sumd : {}", sum);
//         } else {
//             println!("f({}) = f({})", i, (i-1));
//             println!("sumd : {}", sum);
//         }
//         i += 1;
//     }

//  }
use std::io;
fn main() {
    // let n = 5; // 피보나치 수열에서 몇 번째 항까지 계산할 것인지

    // 사용자로부터 n을 입력받기
    println!("Enter the value of n:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let n: u32 = input.trim().parse().expect("Please enter a valid number");
    let mut i: u32 = 0;
    let mut num_1: u32 = 0; // F(0)
    let mut num_2: u32 = 1; // F(1)
    
    loop {
        if i == n {
            break;
        }

        let sum = num_1 + num_2;
        num_1 = num_2;
        num_2 = sum;
        
        // 피보나치 수열의 출력
        println!("count {}", i);
        
        if i > 1 { // i가 2 이상일 때 f(i) = f(i-1) + f(i-2) 출력
            println!("f({}) = f({}) + f({})", i, i - 1, i - 2);
        } else if i == 1 { // i가 1일 때 f(1) = f(0)
            println!("f({}) = f({})", i, i - 1);
        } else { // i가 0일 때 출력
            println!("f({}) = 0", i); // f(0) = 0
        }

        println!("sumd : {}", sum); // 현재 합 출력
        i += 1;
    }
}
// e.g) The repeatability of "The Twelve Days of Christmas" by Christmas Carol Use it to print out lyrics
