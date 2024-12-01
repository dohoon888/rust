struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// 1. immuteral definition and declaration Struct User
// fn main() {
//     let user1 = User {
//         active: true,
//         username: String::from("someusername123"),
//         email: String::from("someone@example.com"),
//         sign_in_count: 1,
//     };
// }

// 2. muteral definition and declaration Struct User
// fn main() {
//     let mut user1 = User {
//         active: true,
//         username: String::from("someusername123"),
//         email: String::from("someone@example.com"),
//         sign_in_count: 1,
//     };

//     user1.email = String::from("anotheremail@example.com");
// }

// 3. return by defining all fields
// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username: username,
//         email: email,
//         sign_in_count: 1,
//     }
// }

// 4. create user2 and update use struct user1
// uesr1 ownership is move to use2. (user1 is no more valid.) 
// fn main() {
//         let mut user1 = User {
//             active: true,
//             username: String::from("someusername123"),
//             email: String::from("someone@example.com"),
//             sign_in_count: 1,
//         };
    
//         user1.email = String::from("anotheremail@example.com");

//     let user2 = User {
//         active: user1.active,
//         username: user1.username,
//         email: String::from("another@example.com"),
//         sign_in_count: user1.sign_in_count,
//     };
// }


// 5. tuple stucts
// field is same type but it's different struct type.
// so it's impossible Color type copy to Point type
// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

// fn main() {
//     let black = Color(0, 0, 0);
//     let origin = Point(0, 0, 0);
// }

// 6. unit-like structsS
// struct AlwaysEqual;

// fn main() {
//     let subject = AlwaysEqual;
// }

// 7. struct ownership.
// struct User {
//     active: bool,
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
// }

// fn main() {
//     let user1 = User {
//         active: true,
//         username: "someusername123",
//         email: "someone@example.com",
//         sign_in_count: 1,
//     };
// }