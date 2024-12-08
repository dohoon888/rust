// Start with the crate root:
//      When compiling a crate, the compiler first views the crate root file.
//      (usually src/lib.rs for library crate or src/main.rs for binary crate).
// Module Declaration: In the Crate root file,
//       you can declare a new module; you can declare a module 'garden' with the code mod garden;
//       the compiler will look at the code for this module in the following places:
//           Inline with the code inside using braces instead of semicolons after mod garden
//           Inside the src/garden.rs file
//           Inside the src/garden/mod.rs file
// Declaring a submodule:
//       A file other than the Crate root can declare a submodule.
//       For example, you can declare mod vegetables; in src/garden.rs .
//       The compiler will look at the places below that are located inside the directory of
//       the parent module name for codes for this submodule:
//           Mod vegetables, with braces instead of semicolons, in-line with code inside
//           Inside the src/garden/vegetables.rs 
//           Inside the src/garden/vegetables/mod.rs
// Path to code within the module: Once a module is configured as part of a crate,
// it can be used to refer to the code in the same crate as long as the disclosure rule allows.
// For example, the Asparagus type within the garden vegetables module can be found and
//  written as crate::garden::vegetables::Asparagus.
// Private vs. Public: The code within the module is private to the parent module by default.
// To make the module public, use pub mod instead of mod to declare it.
// To reveal the items in the public module, put pub before the declaration as well.
// use keyword: Within a scope,
// the use keyword creates a shortcut to an item to reduce repetition of a long path.
//  In any scope that can refer to crate::garden::vegetables::Asparagus,
//  you can create a shortcut to use crate::garden:vegetables::Asparagus; from then on,
// you only need to create an Asparagus to use this type in the scope.

use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}