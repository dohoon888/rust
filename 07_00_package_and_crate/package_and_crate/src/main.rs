// Crate is has two type.
// Binary Crate and Library Crate.
// 1. Binary Crate must define fn main().
// 2. Library Crate is define fn for shared in projects.
// 3. Crate root is first compiled by rust compiler.
// 4.package is a bundle consisting of one or more crates that provide a set of features.
// 5. "Cargo.toml" have description about how to build crate.
// 6. The cargo package also includes the library package on which this binary creep depends.

fn main() {
    println!("Hello, world!");
}



