pub mod cast_shadow_const_static;
pub mod closures;
pub mod collections;
#[allow(unused_variables)]
pub mod conditions;
pub mod enums;
pub mod exceptions;
mod functions;
pub mod generics;
pub mod hof;
pub mod iterators; // look first in iterations.rs, then in iterations/mod.rs
pub mod lifetimes;
pub mod ownership_borrowing;
pub mod primitives;
pub mod printing;
pub mod strings;
pub mod structs_traits;
pub mod tuples;
fn main() {
    println!("Hello World");

    // Primative Types
    println!("\n---Primitives---");
    primitives::play();

    // String and String Slices (groupings of u8 chars)
    println!("\n---Strings---");
    strings::play();

    // Functions and  Procedures
    println!("\n---Functions & Procedures---");
    functions::some_procedure(10.1, 1);
    let _some_fn_val = functions::some_fn(10.1, 100);

    // Strings and ownership and borrowing???
    let my_string_var = String::from("I'm going to be a String");
    functions::print_string(my_string_var);

    // print_string(my_string_var);  this will generate a compiler error; to do with borrowing.

    // Conditions
    println!("\n---Conditions---");
    conditions::play();

    // Tuples
    println!("\n---Tuples---");
    tuples::play();

    // Collections
    println!("\n---Collections---");
    collections::play();

    // Structs, Traits, and implementations
    println!("\n---Structs, Traits, and implementations---");
    structs_traits::play();

    // Enums
    println!("\n---Enum---");
    enums::play();

    // Generics
    println!("\n---Generics---");
    generics::play();

    // Ownership and Borrowing
    println!("\n---Ownership and Borrowing---");
    ownership_borrowing::play();

    // Lifetimes
    println!("\n---Lifetimes---");
    lifetimes::play();

    // Higher Order Functions
    println!("\n---HOF---");
    hof::play();

    // cast_shadow_const_static
    println!("\n---Casting Shadowing Constants Static---");
    cast_shadow_const_static::play();

    // printing
    println!("\n---Printing---");
    printing::play();

    // exceptions
    println!("\n---Exceptions---");
    exceptions::play();

    // Closures
    println!("\n---Closures---");
    closures::play();

    // Iterators
    println!("\n---Iterators---");
    iterators::play();
}
