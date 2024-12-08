use rand::Rng;

// Using nested paths to clean up a large list of uses.
// use std::cmp::Ordering;
// use std::io;
// use std::{cmp::Ordering, io};
// use std::io;
// use std::io::Write;
use std::io::{self, Write};

// Glop operator
use std::collections::*;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}
